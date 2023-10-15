use axum::extract::Query;
use axum::{extract::Path, http::StatusCode, Extension, Json};
use chrono::{DateTime, Utc};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::database::tasks;
use crate::database::tasks::Entity as Tasks;

#[derive(Serialize)]
pub struct ResponseTask {
    id: i32,
    title: String,
    priority: Option<String>,
    description: Option<String>,
    deleted_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct QueryParams {
    priority: Option<String>,
}

pub async fn get_all_tasks(
    Query(query_params): Query<QueryParams>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<Vec<ResponseTask>>, StatusCode> {
    let mut priority_filter = Condition::all();

    if let Some(priority) = query_params.priority {
        priority_filter = priority_filter.add(tasks::Column::Priority.eq(priority));
    }

    let all_tasks = Tasks::find()
        .filter(priority_filter)
        .filter(tasks::Column::DeletedAt.is_null())
        .all(&database)
        .await
        .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?
        .into_iter()
        .map(|db_task| ResponseTask {
            id: db_task.id,
            title: db_task.title,
            priority: db_task.priority,
            description: db_task.description,
            deleted_at: db_task.deleted_at,
        })
        .collect();

    Ok(Json(all_tasks))
}

pub async fn get_one_task(
    Path(task_id): Path<i32>,
    Extension(database): Extension<DatabaseConnection>,
) -> Result<Json<ResponseTask>, (StatusCode, String)> {
    let task = Tasks::find_by_id(task_id)
        .filter(tasks::Column::DeletedAt.is_null())
        .one(&database)
        .await
        .unwrap();

    if let Some(task) = task {
        Ok(Json(ResponseTask {
            id: task.id,
            title: task.title,
            priority: task.priority,
            description: task.description,
            deleted_at: task.deleted_at,
        }))
    } else {
        Err((StatusCode::NOT_FOUND, "Couldn't find the task".to_owned()))
    }
}
