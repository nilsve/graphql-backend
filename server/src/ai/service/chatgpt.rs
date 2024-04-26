use std::env;
use chatgpt::client::ChatGPT;
use chatgpt::types::{CompletionResponse, Role};
use crate::notes::entities::NoteEntity;

#[derive(Clone)]
pub struct ChatGptService {
    client: ChatGPT,
}

impl ChatGptService {
    pub fn new() -> Self {
        let client = ChatGPT::new(env::var("OPENAI_API_KEY").expect("Couldnt find OpenAI api key in env")).expect("Couldnt create ChatGPT client");
        Self { client }
    }

    pub async fn ask_question(&self, question: &str, notes: &Vec<NoteEntity>) -> chatgpt::Result<CompletionResponse> {
        let mut starting_message = "Here comes a list of relevant notes. Derive your answer from these notes\n".to_string();

        for note in notes {
            starting_message.push_str(&format!("TITLE: {}, BODY: {}", note.title.clone(), note.body.clone()));
        }

        starting_message.push_str("\nHere comes the question:");

        let mut conversation = self.client.new_conversation_directed(starting_message);

        let response = conversation.send_role_message(Role::User, question).await?;

        Ok(response)
    }
}