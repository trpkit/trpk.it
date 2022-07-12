use actix_web::http::header;
use actix_web::{Error, HttpResponse};

// Default route should redirect to our company site.
#[actix_web::get("/")]
pub async fn index() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::PermanentRedirect()
        .insert_header((header::LOCATION, "https://trpkit.com"))
        .finish())
}
