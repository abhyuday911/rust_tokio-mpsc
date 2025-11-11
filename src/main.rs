use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use tokio::sync::mpsc;

async fn index() -> impl Responder {
    HttpResponse::Ok().body("hello from index route")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let (sender, mut receiver) = mpsc::channel(100);

    tokio::spawn(async move {
        for i in 0..10 {
            if let Err(_) = sender.send(i).await {
                println!("receiver dropped");
                return;
            }
        }
    });

    while let Some(i) = receiver.recv().await {
        println!("got = {}", i)
    }

    HttpServer::new(move || App::new().route("/", web::get().to(index)))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
