use actix_web::{get, web, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
//Struct for value return of score
struct Score {
    score_value: u8,
}


//Instantiates web server 
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};

    HttpServer::new(|| App::new().service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}



//Handles GET request to /req/score and returns score
#[get("/req/score")]
async fn index() -> Result<impl Responder> {
    let obj = Score {
        score_value: 12, //Holds arbitray value pass score value into here
    };
    Ok(web::Json(obj))
}