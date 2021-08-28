use actix_web::{HttpServer, HttpResponse, web, Responder, App};
use bienveillance_core::welcomer::Welcomer;
use bienveillance_compliments_stub::ComplimentsStub;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .data(setup_welcomer_with_stub())
            .service(web::resource("/")
                .route(web::get().to(hello))
            )
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn hello(welcomer: web::Data<Welcomer>) -> impl Responder {
    HttpResponse::Ok().body(&welcomer.compute_default_message())
}

fn setup_welcomer_with_stub() -> Welcomer {
    let stub = ComplimentsStub {};

    Welcomer::new(Box::new(stub))
}
