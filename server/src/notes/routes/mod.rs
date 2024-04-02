use actix_web::web::{Data, Json, Path};
use actix_web::{get, post};
use apistos::api_operation;

use orm::prelude::DynamoRepositoryError;

use crate::notes::entities::{NewNoteEntity, NoteEntity};

use crate::notes::service::NotesService;

pub fn get_routes() -> actix_web::Scope {
    actix_web::web::scope("/notes")
        .service(get_notes)
        .service(get_note_by_id)
        .service(create_note)
}

// Actix route for retrieving all notes
// #[api_operation(summary = "List all notes")]
#[get("")]
async fn get_notes(
    notes_service: Data<NotesService>,
) -> Result<Json<Vec<NoteEntity>>, DynamoRepositoryError> {
    println!("Getting all notes");
    let vec = notes_service.find_all().await.unwrap();
    Ok(Json(vec))
}

#[api_operation()]
#[get("/{id}")]
async fn get_note_by_id(
    path: Path<String>,
    notes_service: Data<NotesService>,
) -> Json<Option<NoteEntity>> {
    let uuid = path.into_inner();
    Json(notes_service.find_by_id(uuid).await.unwrap())
}

#[post("")]
async fn create_note(
    note: Json<NewNoteEntity>,
    notes_service: Data<NotesService>,
) -> Result<Json<NoteEntity>, DynamoRepositoryError> {
    Ok(Json(notes_service.create_note(&note).await?))
}
