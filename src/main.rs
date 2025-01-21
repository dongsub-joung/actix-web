use std::sync::Mutex;
use serde::Deserialize;

use actix_web::{web::{self, Json, Path}, App, HttpServer};

#[derive(Deserialize)]
struct EntityId{
    id: i64,
}

#[derive(Clone)]
struct FinalUser{
    id: i64,
    user_name: String,
    full_name: String,
}

#[derive(Deserialize)]
struct NewUser{
    user_name: String,
    full_name: String,
}

struct AppState{
    users: std::sync::RwLock<Vec<FinalUser>>
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    let app_data= web::Data::new(
        AppState{
            users: std::sync::RwLock::new(vec![
                FinalUser { id: 1, user_name: "sadf".to_string(), full_name: "sadffff".to_string()}, 
                FinalUser { id: 1, user_name: "qwer".to_string(), full_name: "qwerrrrr".to_string()}, 
                FinalUser { id: 1, user_name: "zxcv".to_string(), full_name: "zxcvvvvv".to_string()}, 
            ])
        }
    );

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(
                web::scope("/v1")
                    .service(
                        web::resource("/user/{id}")
                            .route(web::get().to(get_user_name))
                    )
                    .service(
                        web::resource("/user")
                            .route(web::post().to(insert_user))   
                    )
            )
    })
    .bind(("127.0.0.1", 8001))?
    .run()
    .await
}

async fn insert_user(app_data: web::Data<AppState>, new_user: Json<NewUser>)-> String{
    let mut users= app_data.users.write().unwrap();
    let max_id= users.iter().max_by_key(|usr| { usr.id }).unwrap().id;
    users.push(FinalUser { id: max_id+1, user_name: new_user.user_name.clone() , full_name: new_user.full_name.clone() });

    users.last().unwrap().id.to_string()
}

async fn get_user_name(app_data: web::Data<AppState>, params: Path<EntityId>) -> String{
    let users= app_data.users.read().unwrap();
    users
        .iter()
        .find(|usr| {
            usr.id == params.id
        })
        .unwrap()
        .clone()
        .user_name
}