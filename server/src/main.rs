use actix_cors::Cors;
use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::web::scope;
use aws_config::load_from_env;
use aws_sdk_dynamodb::Client;
use env_logger::Env;

use orm::prelude::*;

use crate::notes::repository::DynamoNotesRepository;
use crate::notes::routes::get_routes;
use crate::notes::service::NotesService;

mod notes;

#[actix_web::main]
async fn main() -> Result<(), DynamoRepositoryError> {
    println!("Loading AWS configurations...");

    let config = load_from_env().await;
    let client = Client::new(&config);

    let repository = DynamoNotesRepository::new(client);
    let service = NotesService::new(repository);

    println!("Starting server...");

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Start actix server
    actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .app_data(actix_web::web::Data::new(service.clone()))
            // .document(spec)
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .service(scope("/api").service(get_routes()))
            .service(Files::new("/", "static").index_file("index.html"))
    })
    .bind(("0.0.0.0", 8080))
    .unwrap()
    .run()
    .await
    .unwrap();

    Ok(())
}
