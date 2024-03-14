use crate::repository::notes::entities::NoteEntity;
use aws_sdk_dynamodb::Client;
use orm::prelude::{DynamoRepository, RepositoryIndex};
use serde::Serialize;

const TABLE_NAME: &str = "notes";

pub struct DynamoNotesRepository {
    client: Client,
}

#[derive(Debug, Clone, Serialize)]
pub struct NotePrimaryIndex {
    pk: String,
    sk: String,
}

impl NotePrimaryIndex {
    pub fn find_by_id(id: i32) -> Self {
        Self {
            pk: "NOTE".to_string(),
            sk: format!("NOTE_ID#{}", id),
        }
    }
}

impl RepositoryIndex for NotePrimaryIndex {}

impl DynamoNotesRepository {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

impl DynamoRepository<NoteEntity> for DynamoNotesRepository {
    fn get_table_name(&self) -> &'static str {
        "notes"
    }

    fn get_client(&self) -> &'_ Client {
        &self.client
    }
}
