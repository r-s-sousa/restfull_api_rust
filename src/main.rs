use actix_web::{get, web, App, HttpServer, Responder};
use serde_json::json; // Import the json! macro

// Define a simple handler for the /hello endpoint
#[get("/hello")]
async fn hello() -> impl Responder {
    // Return a JSON response with a greeting
    web::Json(json!({"message": "Hello, World!"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Start the HTTP server
    HttpServer::new(|| {
        App::new()
            // Register the hello handler
            .service(hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
