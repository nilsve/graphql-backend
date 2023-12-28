use crate::repository::notes::entities::{NewNoteEntity, NoteEntity};
use crate::repository::notes::repository::NotesRepository;
use orm::prelude::{CrudService, Model};

pub struct NoteModel {
    pub id: i32,
    pub title: String,
    pub body: String,
}

pub struct NewNoteModel {
    pub title: String,
    pub body: String,
}

impl Model for NewNoteModel {
    type Entity = NewNoteEntity;

    fn to_entity(self) -> Self::Entity {
        NewNoteEntity {
            title: self.title,
            body: self.body,
        }
    }

    fn from_entity(entity: Self::Entity) -> Self {
        Self {
            title: entity.title,
            body: entity.body,
        }
    }
}

impl Model for NoteModel {
    type Entity = NoteEntity;

    fn to_entity(self) -> Self::Entity {
        NoteEntity {
            id: self.id,
            title: self.title,
            body: self.body,
        }
    }

    fn from_entity(entity: Self::Entity) -> Self {
        Self {
            id: entity.id,
            title: entity.title,
            body: entity.body,
        }
    }
}

pub struct NotesService {
    repository: NotesRepository,
}

impl NotesService {
    pub fn new(repository: NotesRepository) -> Self {
        Self { repository }
    }
}

impl CrudService<NoteModel, NewNoteModel, NotesRepository> for NotesService {
    fn get_repository(&self) -> &NotesRepository {
        &self.repository
    }
}
