use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, put};
use apistos::api_operation;
use uuid::Uuid;

use crate::ai::service::AiService;
use orm::prelude::{DynamoRepositoryError};
use crate::helpers::{Truncatable, TruncatedString};

use crate::notes::models::{NewNoteDTO, NoteDTO};

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
) -> Result<Json<Vec<NoteDTO>>, DynamoRepositoryError> {
    Ok(Json(notes_service.find_all().await?.into_iter().map(|note| {
        let mut note_dto: NoteDTO = note.into();

        note_dto.body = note_dto.body.truncate_with_dots(100);

        note_dto
    }).collect()))
}

#[api_operation()]
#[get("/{id}")]
async fn get_note_by_id(
    path: Path<Uuid>,
    notes_service: Data<NotesService>,
) -> Json<Option<NoteDTO>> {
    Json(notes_service.find_by_id(path.into_inner()).await.unwrap().map(|note| note.into()))
}

#[post("")]
async fn create_note(
    note: Json<NewNoteDTO>,
    notes_service: Data<NotesService>,
    ai_service: Data<AiService>,
) -> Result<Json<NoteDTO>, DynamoRepositoryError> {
    Ok(Json(notes_service.create_note(&note, ai_service).await?.into()))
}

#[put("/{id}")]
async fn update_note(
    path: Path<Uuid>,
    note: Json<NoteDTO>,
    notes_service: Data<NotesService>,
    ai_service: Data<AiService>,
) -> Result<Json<NoteDTO>, DynamoRepositoryError> {
    Ok(Json(
        notes_service
            .update_note(path.into_inner(), &note.into_inner().into(), ai_service)
            .await?.into(),
    ))
}
