use std::env;
use std::error::Error;
use std::sync::Arc;
use rust_bert::pipelines::sentence_embeddings::Embedding;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use uuid::Uuid;
use weaviate_community::collections::objects::{MultiObjects, Object, ObjectListParameters};
use weaviate_community::collections::query::{ExploreQuery, GetQuery};
use weaviate_community::collections::schema::{Class, Classes, Properties, Property};
use weaviate_community::WeaviateClient;
use crate::notes::entities::NoteEntity;
use crate::notes::service::NotesService;

const NOTE_CLASS: &str = "Note";

#[derive(Error, Debug)]
pub enum WeaviateServiceError {
    #[error("Weaviate client error")]
    WeaviateClientError
}

impl From<Box<dyn Error>> for WeaviateServiceError {
    fn from(_error: Box<dyn Error>) -> Self {
        WeaviateServiceError::WeaviateClientError
    }
}

#[derive(Clone)]
pub struct WeaviateService {
    client: Arc<WeaviateClient>,
}

pub trait FromEmbeddingToF64 {
    fn to_f64_vec(&self) -> Vec<f64>;
}

impl FromEmbeddingToF64 for &Embedding {
    fn to_f64_vec(&self) -> Vec<f64> {
        self.iter().map(|value| *value as f64).collect()
    }
}

#[derive(Serialize, Debug)]
struct ExploreQueryParams {
    vector: Vec<f64>,
}

#[derive(Deserialize, Debug)]
pub struct ExploreQueryResultDataItem {
    beacon: String,
    certainty: f64,
}

#[derive(Deserialize, Debug)]
pub struct ExploreQueryResultData {
    #[allow(non_snake_case)]
    Explore: Vec<ExploreQueryResultDataItem>,
}

#[derive(Deserialize, Debug)]
pub struct ExploreQueryResult {
    data: ExploreQueryResultData,
}

impl ExploreQueryResult {
    pub async fn get_notes(&self, notes_service: &NotesService) -> anyhow::Result<Vec<NoteEntity>> {
        let mut result = Vec::new();

        for item in &self.data.Explore {
            let splitted = &item.beacon.split("/").collect::<Vec<&str>>();
            let input = splitted.get(splitted.len() - 1).expect("No uuid found");
            let id = Uuid::parse_str(input).unwrap();

            let note_entity = notes_service.find_by_id(id).await.unwrap().expect("Weaviate returned non existing note!");
            result.push(note_entity);
        }

        Ok(result)
    }
}

impl From<&NoteEntity> for Object {
    fn from(note: &NoteEntity) -> Self {
        let mut builder = Object::builder(NOTE_CLASS, note.into())
            .with_id(note.id);

        if let Some(vector) = &note.encoded {
            builder = builder.with_vector(vector.to_f64_vec());
        }

        builder.build()
    }
}

impl From<&NoteEntity> for Value {
    fn from(note: &NoteEntity) -> Self {
        serde_json::json!({
            "title": &note.title,
            "content": &note.body,
        })
    }
}

impl WeaviateService {
    pub async fn new() -> Result<Self, WeaviateServiceError> {
        println!("Creating Weaviate service");
        let hostname = env::var("WEAVIATE_HOSTNAME").unwrap();
        let port = env::var("WEAVIATE_PORT").unwrap();
        let url = format!("http://{}:{}", hostname, port);

        println!("Weaviate URL: {}", url);

        let client = WeaviateClient::builder(&url).build()?;

        let service = Self {
            client: Arc::new(client),
        };

        service.setup_database().await?;

        Ok(service)
    }

    pub async fn insert_note(
        &self,
        note: &NoteEntity,
    ) -> Result<(), WeaviateServiceError> {
        self.client.objects.create(&(note.into()), None).await?;

        Ok(())
    }

    pub async fn find_note(
        &self,
        note_id: &Uuid,
    ) -> Result<Object, WeaviateServiceError> {
        let note = self.client.objects.get(NOTE_CLASS, note_id, None, None, None).await?;

        Ok(note)
    }

    pub async fn all_notes(&self) -> Result<MultiObjects, WeaviateServiceError> {
        let notes = self.client.objects.list(ObjectListParameters::builder().with_include("vector").with_class_name(NOTE_CLASS).build()).await?;

        Ok(notes)
    }

    pub async fn query_notes(
        &self,
        query_vector: Embedding,
    ) -> Result<ExploreQueryResult, WeaviateServiceError> {
        let query_data = serde_json::to_string(&ExploreQueryParams {
            vector: (&query_vector).to_f64_vec(),
        }).unwrap().replace("\"", "");

        let query = ExploreQuery::builder()
            .with_limit(5)
            .with_near_vector(&query_data)
            .with_fields(vec!["beacon", "certainty"])
            .build();

        let json_value = self.client.query.explore(query).await?;

        let deserialized: ExploreQueryResult = serde_json::from_value(json_value).unwrap();

        Ok(deserialized)
        // let notes = self.client.query.
    }

    pub async fn update_note(
        &self,
        note: &NoteEntity,
    ) -> Result<(), WeaviateServiceError> {
        match self.client.objects.delete(NOTE_CLASS, &note.id, None, None).await {
            Ok(_) => println!("Successfully deleted item for updating {:?}", note.id),
            Err(_) => println!("Coudln't delete item for updating {:?}", note.id)
        };

        self.insert_note(note).await?;

        Ok(())
    }

    async fn setup_database(&self) -> Result<(), WeaviateServiceError> {
        let schema = self.client.schema.get().await?;

        if !schema.classes.iter().any(|class| class.class == NOTE_CLASS) {
            println!("Creating Note class");
            let note_class = Class::builder(NOTE_CLASS)
                .with_properties(Properties::new(vec![
                    Property::builder("title", vec!["text"]).build(),
                    Property::builder("content", vec!["text"]).build(),
                ]))
                .build();

            self.client.schema.create_class(&note_class).await?;
        } else {
            println!("Note class already exists");
        }

        Ok(())
    }

    pub fn get_client(&self) -> Arc<WeaviateClient> {
        self.client.clone()
    }
}
