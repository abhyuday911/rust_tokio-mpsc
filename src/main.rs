use actix_web::{
    App, HttpResponse, HttpServer, Responder,
    web::{self, Data},
};
use serde_json::json;
use tokio::sync::mpsc;

async fn index(state: Data<AppState>) -> impl Responder {
    let order_sender = state.order_sender.clone();

    tokio::spawn(async move {
        for i in 0..10 {
            if let Err(_) = order_sender.send(i.to_string()).await {
                println!("receiver dropped");
                return;
            }
        }
    });

    HttpResponse::Ok().json(json!({"message": "data snent to the engine!"}))
}

#[derive(Clone)]
pub struct AppState {
    order_sender: mpsc::Sender<String>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (sender, receiver) = mpsc::channel(100);

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

async fn run_engine(mut receiver: mpsc::Receiver<String>) {
    while let Some(i) = receiver.recv().await {
        println!("got = {}", i)
    }
}
