use orm::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteEntity {
    pub id: i32,
    pub title: String,
    pub body: String,
    // created_at: NaiveDateTime,
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
