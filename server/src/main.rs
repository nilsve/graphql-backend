use aws_config::load_from_env;
use aws_sdk_dynamodb::Client;

use orm::prelude::*;

use crate::notes::repository::DynamoNotesRepository;
use crate::notes::routes::get_routes;
use crate::notes::service::NotesService;

mod notes;

#[actix_web::main]
async fn main() -> Result<(), DynamoRepositoryError> {
    let config = load_from_env().await;
    let client = Client::new(&config);

    let repository = DynamoNotesRepository::new(client);
    let service = NotesService::new(repository);

    // Start actix server
    actix_web::HttpServer::new(move || {
        // let _spec = Spec {
        //     info: Info {
        //         title: "An API".to_string(),
        //         version: "1.0.0".to_string(),
        //         ..Default::default()
        //     },
        //     ..Default::default()
        // };

        actix_web::App::new()
            .app_data(actix_web::web::Data::new(service.clone()))
            // .document(spec)
            .service(get_routes())
    })
    .bind(("localhost", 8080))
    .unwrap()
    .run()
    .await
    .unwrap();

    Ok(())
}
