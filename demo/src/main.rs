use aws_config::load_from_env;
use aws_sdk_dynamodb::Client;

use orm::prelude::*;

use crate::repository::notes::entities::NoteEntity;
use crate::repository::notes::repository::DynamoNotesRepository;
use crate::repository::notes::service::NotesService;

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

    service
        .upsert(NoteEntity {
            id: 1,
            title: "Hello, world! 1".to_string(),
            body: "This is another note".to_string(),
        })
        .await
        .unwrap();

    let all = service.find_all_paged(None).await?;

    println!("{:?}", all);

    let next = service.find_all().await?;

    println!("{:?}", next);

    Ok(())
}
