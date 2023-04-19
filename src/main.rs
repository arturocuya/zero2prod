use actix_web::{
    web,
    get,
    post,
    App,
    HttpServer,
    Responder,
    HttpResponse
};

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
    return format!("Hello {}!", form.username);
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
