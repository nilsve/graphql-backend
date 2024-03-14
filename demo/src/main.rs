use crate::repository::notes::entities::NoteEntity;
use crate::repository::notes::repository::{DynamoNotesRepository, NotePrimaryIndex};
use crate::repository::notes::service::NotesService;
use aws_config::load_from_env;
use aws_sdk_dynamodb::Client;
use orm::prelude::*;
use std::error::Error;

mod repository;

#[actix_web::main]
async fn main() -> Result<(), DynamoRepositoryError> {
    let config = load_from_env().await;
    let client = Client::new(&config);

    let repository = DynamoNotesRepository::new(client);
    let service = NotesService::new(repository);

    service
        .upsert(NoteEntity {
            id: 2,
            title: "Hello, world! 2".to_string(),
            body: "This is a test note".to_string(),
        })
        .await
        .unwrap();

    let result = service.find_by_id(2).await?;

    println!("{:?}", result);

    // let notes = service.find()?;
    // println!("Displaying {} notes", notes.len());
    // for note in notes {
    //     println!("{}", note.title);
    //     println!("----------\n");
    //     println!("{}", note.body);
    // }

    Ok(())
}
