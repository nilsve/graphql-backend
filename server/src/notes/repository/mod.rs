use aws_sdk_dynamodb::Client;
use serde::Serialize;
use uuid::Uuid;

use orm::prelude::{DynamoRepository, RepositoryIndex};

use crate::notes::entities::NoteEntity;

const TABLE_NAME: &str = "notes";

#[derive(Clone)]
pub struct DynamoNotesRepository {
    client: Client,
}

#[derive(Debug, Clone, Serialize)]
pub struct NotePrimaryIndex {
    pk: String,
    sk: String,
}

impl NotePrimaryIndex {
    pub fn find_by_id(uuid: Uuid) -> Self {
        Self {
            pk: "NOTE".to_string(),
            sk: format!("NOTE_ID#{}", uuid),
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
        TABLE_NAME
    }

    fn get_client(&self) -> &'_ Client {
        &self.client
    }
}
