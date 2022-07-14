mod router;

use actix_web::middleware::{Compress, NormalizePath};
use actix_web::web::{scope, Data};
use actix_web::{App, HttpServer};
use mongodb::Client;

// Use Jemalloc for musl 64-bit platforms
#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    // Create a mongo client
    let mongo_uri = std::env::var("MONGO_URI").expect("MONGO_URI must be provided.");
    let mongo = Client::with_uri_str(mongo_uri)
        .await
        .expect("Failed to connect to provided MongoDB URI.");

    // Start actix server
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(mongo.clone()))
            .wrap(NormalizePath::trim())
            .wrap(Compress::default())
            .service(router::index)
            .service(scope("/api").service(router::healthcheck))
    })
    .bind("0.0.0.0:8888")?
    .run()
    .await
}
