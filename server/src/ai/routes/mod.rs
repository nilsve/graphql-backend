use actix_web::{post};
use actix_web::web::{Data, Json};
use serde::{Deserialize, Serialize};
use crate::ai::service::chatgpt::ChatGptService;
use crate::ai::service::encoder::SentenceEncoderService;
use crate::ai::service::weaviate::WeaviateService;
use crate::notes::service::NotesService;

pub fn get_routes() -> actix_web::Scope {
    actix_web::web::scope("/ai")
        .service(query)
}

#[derive(Debug, Deserialize)]
struct QuestionModel {
    question: String,
}

#[derive(Debug, Serialize)]
struct ResponseModel {
    answer: String,
}


#[post("")]
async fn query(
    question: Json<QuestionModel>,
    notes_service: Data<NotesService>,
    weaviate_service: Data<WeaviateService>,
    chatgpt_service: Data<ChatGptService>,
    encoding_service: Data<SentenceEncoderService>,
) -> Json<ResponseModel> {
    let encoded_question = encoding_service.encode_string(question.0.question.clone()).await;

    let result = weaviate_service.query_notes(encoded_question).await.unwrap();

    let result_notes = result.get_notes(&notes_service).await.unwrap();

    let response = chatgpt_service.ask_question(&question.0.question, &result_notes).await.unwrap();

    let response_message = response.message_choices.get(0).unwrap().message.content.clone();

    println!("Result: {:?}", response_message);

    Json(ResponseModel {
        answer: response_message,
    })
}