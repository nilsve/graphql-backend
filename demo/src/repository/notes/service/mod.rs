use crate::repository::notes::entities::NoteEntity;
use crate::repository::notes::repository::{DynamoNotesRepository, NotePrimaryIndex};
use orm::prelude::{CrudService, DynamoRepository, DynamoRepositoryError};

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

impl NotesService {
    pub async fn find_by_id(&self, id: i32) -> Result<Option<NoteEntity>, DynamoRepositoryError> {
        self.repository.find(NotePrimaryIndex::find_by_id(id)).await
    }
}
