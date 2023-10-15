use axum::http::StatusCode;
use axum::{extract::Path, Extension, Json};
use sea_orm::{prelude::DateTimeUtc, DatabaseConnection, Set};
use sea_orm::{ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
use serde::Deserialize;

use crate::database::tasks;
use crate::database::tasks::Entity as Tasks;

#[derive(Deserialize)]
pub struct RequestTask {
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub priority: Option<Option<String>>,
    pub title: Option<String>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub completed_at: Option<Option<DateTimeUtc>>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub description: Option<Option<String>>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub deleted_at: Option<Option<DateTimeUtc>>,
}

pub async fn partial_update(
    Path(task_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
    Json(request_task): Json<RequestTask>,
) -> Result<(), StatusCode> {
    let mut db_task = if let Some(task) = Tasks::find_by_id(task_id)
        .one(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        task.into_active_model()
    } else {
        return Err(StatusCode::NOT_FOUND);
    };

    if let Some(priority) = request_task.priority {
        db_task.priority = Set(priority);
    }

    if let Some(title) = request_task.title {
        db_task.title = Set(title);
    }

    if let Some(completed_at) = request_task.completed_at {
        db_task.completed_at = Set(completed_at);
    }

    if let Some(deleted_at) = request_task.deleted_at {
        db_task.deleted_at = Set(deleted_at);
    }

    if let Some(description) = request_task.description {
        db_task.description = Set(description);
    }

    Tasks::update(db_task)
        .filter(tasks::Column::Id.eq(task_id))
        .exec(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(())
}
