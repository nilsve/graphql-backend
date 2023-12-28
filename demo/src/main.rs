use crate::repository::notes::repository::NotesRepository;
use crate::repository::notes::service::NotesService;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenvy::dotenv;
use orm::prelude::*;
use std::env;

mod repository;
mod schema;

#[actix_web::main]
async fn main() -> QueryResult<()> {
    let connection_pool = get_connection_pool();

    let repository = NotesRepository::new(connection_pool);
    let service = NotesService::new(repository);

    let notes = service.find_all()?;
    println!("Displaying {} notes", notes.len());
    for note in notes {
        println!("{}", note.title);
        println!("----------\n");
        println!("{}", note.body);
    }

    Ok(())
}

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().unwrap();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}
