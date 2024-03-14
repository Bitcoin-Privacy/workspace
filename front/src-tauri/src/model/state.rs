use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "type", content = "password")]
pub enum InitState {
    BrandNew,
    CreatedPassword(String),
    CreatedWallet(String),
}
