use actix_web::{
    App, HttpResponse, HttpServer, Responder,
    web::{self, Data},
};
use serde_json::json;
use tokio::sync::{mpsc, oneshot};

async fn index(state: Data<AppState>) -> impl Responder {
    let order_sender = state.order_sender.clone();

    let (oneshot_sender, oneshot_receiver) = oneshot::channel::<String>();

    let msg = EngineMessage {
        payload: "hello".to_string(),
        oneshot_sender: oneshot_sender,
    };

    tokio::spawn(async move {
        if let Err(_) = order_sender.send(msg).await {
            println!("receiver dropped");
            return;
        }
    });

    match oneshot_receiver.await {
        Ok(val) => println!("you received -> {}", val),
        Err(_) => println!("the sender dropped"),
    }

    HttpResponse::Ok().json(json!({"message": "data sent to the engine!"}))
}

#[derive(Clone)]
pub struct AppState {
    order_sender: mpsc::Sender<EngineMessage>,
}

pub struct EngineMessage {
    pub payload: String,
    pub oneshot_sender: oneshot::Sender<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (sender, receiver) = mpsc::channel::<EngineMessage>(100);

    tokio::spawn(run_engine(receiver));

    let state = web::Data::new(AppState {
        order_sender: sender,
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

async fn run_engine(mut receiver: mpsc::Receiver<EngineMessage>) {
    while let Some(i) = receiver.recv().await {
        i.oneshot_sender
            .send("hello from underworld".to_string())
            .unwrap();
        println!("got {} in payload", i.payload)
    }
}
