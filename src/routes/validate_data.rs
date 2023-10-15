use axum::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RequestUser {
    _username: Option<String>,
    _password: String,
}

pub async fn valdiate_data(Json(user): Json<RequestUser>) -> String {
    dbg!(user);
    "Ok".to_owned()
}
