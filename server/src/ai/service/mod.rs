use crate::notes::entities::NoteEntity;
use rust_bert::pipelines::sentence_embeddings::{
    SentenceEmbeddingsBuilder, SentenceEmbeddingsModel, SentenceEmbeddingsModelType,
};
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
}

#[derive(Debug)]
enum ResponseMessage {
    NoteEncoded(NoteEntity),
}

#[derive(Clone)]
pub struct AiService {
    sync_sender: SyncSender<Message>,
    pub join_handle: Arc<Mutex<JoinHandle<Result<(), ()>>>>,
}

impl AiService {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::sync_channel(10);

        let handle = thread::spawn(move || BertServiceRunner::new().runner(receiver));

        Self {
            join_handle: Arc::from(Mutex::new(handle)),
            sync_sender: sender,
        }
    }

    pub async fn encode(&self, note: NoteEntity) -> NoteEntity {
        let (sender, receiver) = oneshot::channel();

        self.sync_sender
            .send((RequestMessage::EncodeNote(note), sender))
            .expect("sending request");

        let response = receiver.await.expect("receiving response");

        match response {
            ResponseMessage::NoteEncoded(note) => note,
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
                    println!("Encoding note {:?}", note.id);
                    let start = Instant::now();
                    let embeddings = self.sentence_embeddings_model
                        .encode(&vec![note.title.clone(), note.body.clone()].to_vec())
                        .unwrap();

                    let duration = start.elapsed();

                    println!(
                        "Done encoding. Size of vecs: {} {}. Took {:?}",
                        embeddings.get(0).unwrap().len(),
                        embeddings.get(1).unwrap().len(),
                        duration
                    );

                    let encoded_note = NoteEntity {
                        id: note.id,
                        title: note.title,
                        body: note.body,
                        encoded: Some(embeddings),
                    };

                    response
                        .send(ResponseMessage::NoteEncoded(encoded_note))
                        .expect("sending response");
                }
            }
        }

        Ok(())
    }
}
