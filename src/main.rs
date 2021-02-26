use std::time::Duration;
use actix_web::{HttpServer, App, web};
use actix::Actor;

#[macro_use]
extern crate serde_json;

mod donning;
mod fitness;
mod maze;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let donning_server = donning::server::DonningServer::new().start();
    let fitness_server = fitness::server::FitnessServer::new().start();

    HttpServer::new(move || {
        App::new()
            .data(donning_server.clone())
            .data(fitness_server.clone())
            .route("/donning", web::get().to(donning::websocket::donning_start))
            .route("/fitness", web::get().to(fitness::websocket::fitness_start))
            .route("/maze", web::get().to(maze::websocket::maze_start))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
