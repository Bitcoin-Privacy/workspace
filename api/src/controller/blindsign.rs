use crate::{svc::blindsign, util::response};

use actix_web::HttpResponse;
use shared::intf::blindsign::GetBlindSessionRes;

pub async fn get_session() -> HttpResponse {
    let (publickey, rp) = blindsign::get_session();
    let res = GetBlindSessionRes { publickey, rp };
    response::success(res)
}
