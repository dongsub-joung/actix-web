use std::sync::Mutex;

use actix_web::{web, HttpServer};

struct Messenger{
    message: String,
}

struct MutableState{
    messenger: Mutex<Messenger>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let app_data= actix_web::web::Data::new(MutableState{
        messenger: Mutex::new(Messenger { message: "hello".to_string() })
    });

    // @todo "move" doc
    // global mutable status
    // + "Arc" type //smart pointer
    HttpServer::new(move || {
        actix_web::App::new()
        .app_data(app_data.clone())
        .route("/", web::post().to(update))
        .route("/", web::post().to(get))
    })
    .bind(("127.0.0.1", 8001))?
    .run()
    .await
}

async fn update(app_date: actix_web::web::Data<MutableState>) -> String{
    let mut messenger= app_date.messenger.lock().unwrap();
    messenger.message= format!("{} world", messenger.message);
    "".to_string()
}

async fn get(app_date: actix_web::web::Data<MutableState>) -> String{
    app_date.messenger.lock().unwrap().message.clone()
}