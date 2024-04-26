use crate::ai::service::AiService;
use actix_web::web::Data;
use orm::prelude::{
    CrudService, DynamoRepositoryError, LastEvaluatedKey, QueryData, QueryResult, RepositoryIndex,
};
use uuid::Uuid;

use crate::notes::entities::{NoteEntity};
use crate::notes::models::NewNoteDTO;
use crate::notes::repository::{DynamoNotesRepository, NotePrimaryIndex};

#[derive(Clone)]
pub struct NotesService {
    repository: DynamoNotesRepository,
}

impl CrudService<NoteEntity, DynamoNotesRepository> for NotesService {
    fn get_repository(&self) -> &DynamoNotesRepository {
        &self.repository
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct QueryNoteIndex {
    pk: String,
}

impl QueryNoteIndex {
    pub fn find_all() -> Self {
        Self {
            pk: "NOTE".to_string(),
        }
    }
}

impl RepositoryIndex for QueryNoteIndex {}

impl NotesService {
    pub fn new(repository: DynamoNotesRepository) -> Self {
        Self { repository }
    }

    pub async fn find_by_id(
        &self,
        uuid: Uuid,
    ) -> Result<Option<NoteEntity>, DynamoRepositoryError> {
        self.find(NotePrimaryIndex::find_by_id(uuid)).await
    }

    pub async fn find_all_paged(
        &self,
        last_evaluated_key: Option<LastEvaluatedKey>,
    ) -> Result<QueryResult<NoteEntity>, DynamoRepositoryError> {
        self.query(QueryData::new(
            QueryNoteIndex::find_all(),
            last_evaluated_key,
        ))
            .await
    }

    pub async fn find_all(&self) -> Result<Vec<NoteEntity>, DynamoRepositoryError> {
        self.query_all(QueryNoteIndex::find_all()).await
    }

    pub async fn create_note(
        &self,
        note: &NewNoteDTO,
        ai_service: Data<AiService>,
    ) -> Result<NoteEntity, DynamoRepositoryError> {
        let note: NoteEntity = note.to_owned().into();
        self.create(note.clone()).await?;

        let encoded_note = ai_service.encode(note.clone()).await;

        self.upsert(encoded_note).await?;

        Ok(note)
    }

    pub async fn update_note(
        &self,
        note_id: Uuid,
        note: &NoteEntity,
        ai_service: Data<AiService>,
    ) -> Result<NoteEntity, DynamoRepositoryError> {
        // Check if note exists
        self.find(NotePrimaryIndex::find_by_id(note_id))
            .await?
            .ok_or(DynamoRepositoryError::ItemNotFoundError)?;

        let entity = NoteEntity {
            id: note_id,
            ..note.clone()
        };

        self.upsert(entity.clone()).await?;

        let encoded_note = ai_service.encode(entity.clone()).await;

        self.upsert(encoded_note).await?;

        Ok(entity)
    }
}
