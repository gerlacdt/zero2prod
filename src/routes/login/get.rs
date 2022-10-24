use crate::startup::HmacSecret;
use actix_web::cookie::{time::Duration, Cookie};
use actix_web::{http::header::ContentType, web, HttpRequest, HttpResponse};

#[derive(serde::Deserialize)]
pub struct QueryParams {
    error: String,
    tag: String,
}

impl QueryParams {
    fn verify(self, secret: &HmacSecret) -> Result<String, anyhow::Error> {
        let query_string = format!("error={}", urlencoding::Encoded::new(&self.error));

        Ok(self.error)
    }
}

pub async fn login_form(
    query: Option<web::Query<QueryParams>>,
    secret: web::Data<HmacSecret>,
    request: HttpRequest,
) -> HttpResponse {
    let error_html = match request.cookie("_flash") {
        None => "".into(),
        Some(cookie) => {
            format!("<p><i>{}</i></p>", cookie.value())
        }
    };

    let body = format!(
        r#"
<!DOCTYPE html>
<html lang="en">
<head>
<meta http-equiv="content-type" content="text/html; charset=utf-8">
<title>Login
</title>
</head>
<body>
{error_html}
<form action="/login" method="post">
<label>Username
<input
type="text"
placeholder="Enter Username"
name="username"
>
</label>
<label>Password
<input
type="password"
placeholder="Enter Password"
name="password"
>
</label>
<button type="submit">Login
</button>
</form>
</body>
</html>
"#
    );

    HttpResponse::Ok()
        .content_type(ContentType::html())
        .cookie(Cookie::build("_flash", "").max_age(Duration::ZERO).finish())
        .body(body)
}
