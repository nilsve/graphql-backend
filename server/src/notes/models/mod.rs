use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::notes::entities::NoteEntity;

#[derive(Debug, Clone, Deserialize, ApiComponent, JsonSchema)]
pub struct NewNoteDTO {
    pub title: String,
    pub body: String,
}

impl From<NewNoteDTO> for NoteEntity {
    fn from(new_note: NewNoteDTO) -> Self {
        NoteEntity {
            id: Uuid::new_v4(),
            title: new_note.title,
            body: new_note.body,
            encoded: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct NoteDTO {
    pub id: Uuid,
    pub title: String,
    pub body: String,
}

impl From<NoteEntity> for NoteDTO {
    fn from(note: NoteEntity) -> Self {
        NoteDTO {
            id: note.id,
            title: note.title,
            body: note.body,
        }
    }
}

impl From<NoteDTO> for NoteEntity {
    fn from(note: NoteDTO) -> Self {
        NoteEntity {
            id: note.id,
            title: note.title,
            body: note.body,
            encoded: None,
        }
    }
}

