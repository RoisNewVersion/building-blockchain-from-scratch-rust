use actix_web::{web, App, HttpResponse, HttpServer};
#[actix_web::main]
async fn main() {
    let server = HttpServer::new(||{
        App::new().route("/", web::get().to(get_index))
    });
    println!("Server is listening on localhost:3000...");
    server.bind("127.0.0.1:3000")
    .expect("Error binding server to").run().await.expect("Error running server");
}

async  fn get_index()->HttpResponse{
    HttpResponse::Ok().content_type("text/html").body("It works!")
}