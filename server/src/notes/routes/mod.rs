use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, put};
use apistos::api_operation;
use uuid::Uuid;

use crate::ai::service::AiService;
use orm::prelude::{CrudService, DynamoRepositoryError};

use crate::notes::entities::{NewNoteEntity, NoteEntity};

use crate::notes::service::NotesService;

pub fn get_routes() -> actix_web::Scope {
    actix_web::web::scope("/notes")
        .service(get_notes)
        .service(get_note_by_id)
        .service(create_note)
        .service(update_note)
}

// Actix route for retrieving all notes
// #[api_operation(summary = "List all notes")]
#[get("")]
async fn get_notes(
    notes_service: Data<NotesService>,
) -> Result<Json<Vec<NoteEntity>>, DynamoRepositoryError> {
    Ok(Json(notes_service.find_all().await?))
}

#[api_operation()]
#[get("/{id}")]
async fn get_note_by_id(
    path: Path<Uuid>,
    notes_service: Data<NotesService>,
) -> Json<Option<NoteEntity>> {
    Json(notes_service.find_by_id(path.into_inner()).await.unwrap())
}

#[post("")]
async fn create_note(
    note: Json<NewNoteEntity>,
    notes_service: Data<NotesService>,
    ai_service: Data<AiService>,
) -> Result<Json<NoteEntity>, DynamoRepositoryError> {
    Ok(Json(notes_service.create_note(&note, ai_service).await?))
}

#[put("/{id}")]
async fn update_note(
    path: Path<Uuid>,
    note: Json<NoteEntity>,
    notes_service: Data<NotesService>,
    ai_service: Data<AiService>,
) -> Result<Json<NoteEntity>, DynamoRepositoryError> {
    Ok(Json(
        notes_service
            .update_note(path.into_inner(), &note, ai_service)
            .await?,
    ))
}
