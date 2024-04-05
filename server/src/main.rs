use actix_cors::Cors;
use actix_files::Files;
use actix_web::middleware::Logger;
use actix_web::web::scope;
use aws_config::load_from_env;
use aws_sdk_dynamodb::Client;
use dotenvy::dotenv;
use env_logger::Env;
use std::env;
use std::path::PathBuf;

use orm::prelude::*;

use crate::notes::repository::DynamoNotesRepository;
use crate::notes::routes::get_routes;
use crate::notes::service::NotesService;

mod notes;

#[actix_web::main]
async fn main() -> Result<(), DynamoRepositoryError> {
    dotenv().unwrap();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    println!("Loading AWS configurations...");

    let config = load_from_env().await;
    let client = Client::new(&config);

    let repository = DynamoNotesRepository::new(client);
    let service = NotesService::new(repository);

    let path: PathBuf = env::var("FRONTEND_LOCATION")
        .unwrap_or_else(|_| "static".to_string())
        .into();

    println!("Looking for frontend files at: {:?}", path);

    let mut frontend_path = None;

    match path.canonicalize() {
        Ok(path_buf) => {
            println!("Serving frontend from: {:?}", path_buf);

            frontend_path = Some(path_buf);
        }
        Err(_) => {
            println!("Frontend not found at: {:?}. Not serving", path);
        }
    }

    println!("Starting server...");

    // Start actix server
    actix_web::HttpServer::new(move || {
        let mut app = actix_web::App::new()
            .app_data(actix_web::web::Data::new(service.clone()))
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .service(scope("/api").service(get_routes()));

        if let Some(path) = &frontend_path {
            app = app.service(Files::new("/", path).index_file("index.html"));
        }

        app
    })
    .bind(("0.0.0.0", 8080))
    .unwrap()
    .run()
    .await
    .unwrap();

    Ok(())
}
