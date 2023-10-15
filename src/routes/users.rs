use axum::{extract::Query, http::StatusCode, Extension, Json};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, Set,
};
use serde::{Deserialize, Serialize};

use crate::database::users::Entity as Users;
use crate::database::users::{self, Model};
use crate::utils::jwt::create_jwt;

#[derive(Deserialize, Serialize, Debug)]
pub struct RequestUser {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseUser {
    username: String,
    id: i32,
    token: String,
}

pub async fn create_users(
    Query(query_params): Query<RequestUser>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let jwt = create_jwt()?;

    let new_users = users::ActiveModel {
        username: Set(query_params.username),
        password: Set(hash_password(query_params.password)?),
        token: Set(Some(jwt)),
        ..Default::default()
    }
    .save(&database)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ResponseUser {
        username: new_users.username.unwrap(),
        id: new_users.id.unwrap(),
        token: new_users.token.unwrap().unwrap(),
    }))
}

pub async fn login(
    Extension(database): Extension<DatabaseConnection>,
    Json(request_user): Json<RequestUser>,
) -> Result<Json<ResponseUser>, StatusCode> {
    let mut conditions = Condition::all();

    conditions = conditions.add(users::Column::Username.eq(request_user.username));

    let db_user = Users::find()
        .filter(conditions)
        .one(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Some(db_user) = db_user {
        if !verify_password(request_user.password, &db_user.password)? {
            return Err(StatusCode::UNAUTHORIZED);
        }
        let new_token = create_jwt()?;

        let mut user = db_user.into_active_model();
        user.token = Set(Some(new_token));

        let saved_user = user
            .save(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        Ok(Json(ResponseUser {
            username: saved_user.username.unwrap(),
            token: saved_user.token.unwrap().unwrap(),
            id: saved_user.id.unwrap(),
        }))
    } else {
        return Err(StatusCode::NOT_FOUND);
    }
}

pub async fn logout(
    Extension(database): Extension<DatabaseConnection>,
    Extension(user): Extension<Model>,
) -> Result<(), StatusCode> {
    let mut user = user.into_active_model();

    user.token = Set(None);

    user.save(&database)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}

fn hash_password(password: String) -> Result<String, StatusCode> {
    bcrypt::hash(password, 14).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

fn verify_password(password: String, hash: &str) -> Result<bool, StatusCode> {
    bcrypt::verify(password, hash).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
