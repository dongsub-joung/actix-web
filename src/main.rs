use actix_web::HttpServer;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(|| {
        actix_web::App::new()
            .route("/", actix_web::web::get().to(index))
    })
    .bind(("127.0.0.1", 8001))?
    .run()
    .await
}

async fn index() -> &'static str{
    "hello world"
}