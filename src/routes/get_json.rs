use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Data {
    message: String,
    count: i32,
    username: String,
}
pub async fn get_json() -> Json<Data> {
    let data = Data { message: "Data".to_owned(), count: 32, username: "gnanasurya".to_owned() };
    
    Json(data)
}