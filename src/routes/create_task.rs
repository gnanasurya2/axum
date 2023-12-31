use axum::{
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    Extension, Json, TypedHeader,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::Deserialize;

use crate::database::{
    tasks,
    users::{self, Entity as Users},
};
#[derive(Deserialize, Debug)]
pub struct CreateTaskRequestBody {
    title: String,
    description: Option<String>,
    priority: Option<String>,
}

pub async fn create_task(
    authorization: TypedHeader<Authorization<Bearer>>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_task): Json<CreateTaskRequestBody>,
) -> Result<(), StatusCode> {
    let token = authorization.token();

    let user = if let Some(user) = Users::find()
        .filter(users::Column::Token.eq(Some(token)))
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        user
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    };

    let new_task = tasks::ActiveModel {
        priority: Set(request_task.priority),
        title: Set(request_task.title),
        description: Set(request_task.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    let _result = new_task.save(&database).await.unwrap();

    Ok(())
}
