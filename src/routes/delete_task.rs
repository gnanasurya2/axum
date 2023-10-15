use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Extension,
};
use chrono;
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel, Set};
use serde::Deserialize;

use crate::database::tasks::Entity as Tasks;

#[derive(Deserialize)]
pub struct QueryParams {
    soft: bool,
}

pub async fn delete_tasks(
    Path(task_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Query(query_params): Query<QueryParams>,
) -> Result<(), StatusCode> {
    // Tasks::delete(task)
    //     .exec(&database)
    //     .await
    //     .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    println!("printing {}", query_params.soft);
    if query_params.soft {
        let mut task = if let Some(task) = Tasks::find_by_id(task_id)
            .one(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        {
            task.into_active_model()
        } else {
            return Err(StatusCode::NOT_FOUND);
        };

        let now = chrono::Utc::now();

        task.deleted_at = Set(Some(now.into()));

        Tasks::update(task)
            .exec(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        Tasks::delete_by_id(task_id)
            .exec(&database)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(())
}
