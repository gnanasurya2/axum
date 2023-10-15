use axum::{
    headers::{authorization::Bearer, Authorization},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    TypedHeader,
};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    database::users::{self, Entity as Users},
    utils::{app_error::AppError, jwt::is_valid},
};

pub async fn guard<T>(
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    mut request: Request<T>,
    next: Next<T>,
) -> Result<Response, AppError> {
    let token = token.token().to_owned();

    is_valid(&token)?;

    let database = request
        .extensions()
        .get::<DatabaseConnection>()
        .ok_or_else(|| AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"))?;

    let user = Users::find()
        .filter(users::Column::Token.eq(token))
        .one(database)
        .await
        .map_err(|error| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        })?;

    let Some(user) = user else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "Not authorized, please login or create an account",
        ));
    };

    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
