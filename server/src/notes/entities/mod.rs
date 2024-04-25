use apistos::ApiComponent;
use rust_bert::pipelines::sentence_embeddings::Embedding;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use orm::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct NoteEntity {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub encoded: Option<Vec<Embedding>>,
    // created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Deserialize, ApiComponent, JsonSchema)]
pub struct NewNoteEntity {
    pub title: String,
    pub body: String,
}

impl From<NewNoteEntity> for NoteEntity {
    fn from(new_note: NewNoteEntity) -> Self {
        NoteEntity {
            id: Uuid::new_v4(),
            title: new_note.title,
            body: new_note.body,
            encoded: None,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct NoteIndex {}

#[derive(Debug, Clone, Serialize)]
pub struct NotePrimaryKey {
    pub pk: String,
    pub sk: String,
}

impl Entity for NoteEntity {
    type PrimaryKey = NotePrimaryKey;
    type IndexFields = NoteIndex;

    fn get_primary_key(&self) -> Self::PrimaryKey {
        NotePrimaryKey {
            pk: "NOTE".to_string(),
            sk: format!("NOTE_ID#{}", self.id),
        }
    }

    fn get_index_fields(&self) -> Self::IndexFields {
        NoteIndex {}
    }
}
