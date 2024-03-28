use orm::prelude::{
    CrudService, DynamoRepositoryError, LastEvaluatedKey, QueryData, QueryResult, RepositoryIndex,
};

use crate::notes::entities::{NewNoteEntity, NoteEntity};
use crate::notes::repository::{DynamoNotesRepository, NotePrimaryIndex};

#[derive(Clone)]
pub struct NotesService {
    repository: DynamoNotesRepository,
}

impl NotesService {
    pub fn new(repository: DynamoNotesRepository) -> Self {
        Self { repository }
    }
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
    pub async fn find_by_id(
        &self,
        uuid: String,
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
        note: &NewNoteEntity,
    ) -> Result<NoteEntity, DynamoRepositoryError> {
        let note: NoteEntity = note.to_owned().into();
        self.create(note.clone()).await?;

        Ok(note)
    }
}
