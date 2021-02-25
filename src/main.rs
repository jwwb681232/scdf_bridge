use std::time::Duration;
use actix_web::{HttpServer, App, web};
use actix::Actor;

mod donning;
mod fitness;
mod maze;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = donning::server::DonningServer::new().start();

    HttpServer::new(move || {
        App::new()
            .data(server.clone())
            .route("/donning", web::get().to(donning::websocket::donning_start))
            .route("/fitness", web::get().to(fitness::websocket::fitness_start))
            .route("/maze", web::get().to(maze::websocket::fitness_start))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
