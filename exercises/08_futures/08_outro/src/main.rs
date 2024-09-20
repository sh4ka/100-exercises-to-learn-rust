// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.
use actix_web::{web, App,HttpResponse, HttpServer, Responder};
use serde::Serialize;
use crate::data::{Status, Ticket};
use crate::store::TicketId;
use crate::title::TicketTitle;

pub mod data;
pub mod store;
pub mod title;
pub mod description;

#[derive(Serialize)]
struct Greeting {
    message: String,
}

async fn hello() -> impl Responder {
    let greeting = Greeting {
        message: String::from("¡Hola, mundo!"),
    };
    HttpResponse::Ok().json(greeting)
}

async fn 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Servidor corriendo en http://localhost:8080/");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}


