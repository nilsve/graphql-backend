use crate::notes::entities::NoteEntity;
use rust_bert::pipelines::sentence_embeddings::{Embedding, SentenceEmbeddingsBuilder, SentenceEmbeddingsModel, SentenceEmbeddingsModelType};
use std::sync::mpsc::{Receiver, SyncSender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use tokio::sync::oneshot;
use tokio::time::Instant;

type Message = (RequestMessage, oneshot::Sender<ResponseMessage>);

#[derive(Debug, Clone)]
enum RequestMessage {
    EncodeNote(NoteEntity),
    EncodeString(String),
}

#[derive(Debug)]
enum ResponseMessage {
    NoteEncoded(NoteEntity),
    StringEncoded(Embedding),
}

#[derive(Clone)]
pub struct SentenceEncoderService {
    sync_sender: SyncSender<Message>,
    pub join_handle: Arc<Mutex<JoinHandle<Result<(), ()>>>>,
}

impl SentenceEncoderService {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::sync_channel(10);

        let handle = thread::spawn(move || BertServiceRunner::new().runner(receiver));

        Self {
            join_handle: Arc::from(Mutex::new(handle)),
            sync_sender: sender,
        }
    }

    pub async fn encode_string(&self, string: String) -> Embedding {
        let (sender, receiver) = oneshot::channel();

        self.sync_sender
            .send((RequestMessage::EncodeString(string), sender))
            .expect("sending request");

        let response = receiver.await.expect("receiving response");

        match response {
            ResponseMessage::StringEncoded(embedding) => embedding,
            _ => panic!("Unexpected response")
        }
    }

    pub async fn encode_note(&self, note: NoteEntity) -> NoteEntity {
        let (sender, receiver) = oneshot::channel();

        self.sync_sender
            .send((RequestMessage::EncodeNote(note), sender))
            .expect("sending request");

        let response = receiver.await.expect("receiving response");

        match response {
            ResponseMessage::NoteEncoded(note) => note,
            _ => panic!("Unexpected response"),
        }
    }
}

struct BertServiceRunner {
    sentence_embeddings_model: SentenceEmbeddingsModel,
}

impl BertServiceRunner {
    fn new() -> Self {
        let model = SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL12V2)
            .create_model()
            .expect("??????");

        Self {
            sentence_embeddings_model: model,
        }
    }

    fn runner(self, receiver: Receiver<Message>) -> Result<(), ()> {
        println!("AI service runner started");

        println!("AI model created");

        while let Ok((request, response)) = receiver.recv() {
            match request {
                RequestMessage::EncodeNote(note) => {
                    let formatted_note = format!("TITLE: {} BODY: {}", note.title, note.body);

                    let embeddings = self.encode_string(&formatted_note);

                    let encoded_note = NoteEntity {
                        id: note.id,
                        title: note.title,
                        body: note.body,
                        encoded: Some(embeddings.get(0).unwrap().to_owned()),
                    };

                    response
                        .send(ResponseMessage::NoteEncoded(encoded_note))
                        .expect("sending response");
                }
                RequestMessage::EncodeString(string) => {
                    let embeddings = self.encode_string(&string);

                    response
                        .send(ResponseMessage::StringEncoded(embeddings.get(0).unwrap().to_owned()))
                        .expect("sending response");
                }
            }
        }

        Ok(())
    }

    fn encode_string(&self, formatted_note: &String) -> Vec<Embedding> {
        let start = Instant::now();

        println!("Encoding text");

        let embeddings = self.sentence_embeddings_model
            .encode(&vec![formatted_note])
            .unwrap();

        let duration = start.elapsed();

        println!(
            "Done encoding. Size of vec: {}. Took {:?}",
            embeddings.get(0).unwrap().len(),
            duration
        );

        embeddings
    }
}
