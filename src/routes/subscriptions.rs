use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize, Debug)]
pub struct FormData {
    pub email: String,
    pub name: String,
}

pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    println!("{:?}", form);
    HttpResponse::Ok().finish()
}
