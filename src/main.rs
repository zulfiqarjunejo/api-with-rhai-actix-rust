use std::env;

use actix_web::{self, get, web::Path, App, HttpServer, Responder};

use rhai::Engine;

#[get("/multiply/{num1}/{num2}")]
async fn multiply(path: Path<(i64, i64)>) -> impl Responder {
    let (num1, num2) = path.into_inner();

    let mut engine = Engine::new();
    engine.register_fn("num1", move || num1);
    engine.register_fn("num2", move || num2);

    let result = engine.eval_file::<i64>("src/multiply.rhai".into()).unwrap();
    format!("{result}")
}

#[get("/add/{num1}/{num2}")]
async fn add(path: Path<(i64, i64)>) -> impl Responder {
    let (num1, num2) = path.into_inner();

    let mut engine = Engine::new();
    engine.register_fn("num1", move || num1);
    engine.register_fn("num2", move || num2);

    let result = engine.eval_file::<i64>("src/add.rhai".into()).unwrap();
    format!("{result}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = match env::var("PORT") {
        Ok(val) => val.parse().expect("port should be a number"),
        Err(_) => 9090,
    };

    HttpServer::new(|| App::new().service(multiply).service(add))
        .bind(("127.0.0.1", port))
        .unwrap()
        .run()
        .await
}
