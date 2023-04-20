mod models;
mod schema;

use actix_web::{
    web,
    get,
    post,
    App,
    HttpServer,
    Responder,
    HttpResponse
};

use crate::models::*;

#[derive(serde::Deserialize)]
struct FormData {
    email: String,
    username: String
}

#[get("/health_check")]
async fn health_check() -> impl Responder {
    return HttpResponse::Ok();
}

// Continue in 3.8.4 Database Setup
#[post("/subscriptions")]
async fn subscribe(form: web::Form<FormData>) -> impl Responder {
    let conn = &mut establish_connnection().await;
    let new_sub = create_subscription(conn, &form.email, &form.username).await;
    return HttpResponse::Ok().json(new_sub);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(subscribe)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
