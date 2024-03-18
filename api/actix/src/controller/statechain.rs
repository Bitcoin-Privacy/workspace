use actix_web::HttpResponse;

use crate::util::response;

pub async fn hello() -> HttpResponse {
    response::success("hello from statechain endpoint")
}
