pub mod router;
#[allow(unused_imports)]

use actix_web::{get , web ,post , App , HttpServer , Responder};
use crate::router::RouterConfigure;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    RouterConfigure::new("0.0.0.0".to_string(), 8000).run().await
}
