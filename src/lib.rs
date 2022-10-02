use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("I am alive")
}

#[derive(serde::Deserialize, Debug)]
struct FormData {
    email: String,
    name: String,
}

async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    println!("{:?}", form);
    HttpResponse::Ok().finish()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

#[allow(dead_code)]
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_um() {
        let actual = sum(1, 3);
        let expected = 4;
        assert_eq!(expected, actual);
    }
}
