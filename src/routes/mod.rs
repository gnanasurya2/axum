mod always_errors;
mod create_task;
mod delete_task;
mod get_json;
mod get_tasks;
mod guard;
mod hello_world;
mod middleware_message;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_custom_header;
mod mirror_user_agent;
mod partial_update_task;
mod path_variables;
mod query_params;
mod read_middleware_custom_header;
mod set_middleware_custom_headers;
mod update_tasks;
mod users;
mod validate_data;

use axum::{
    http::Method,
    middleware,
    routing::{delete, get, patch, post, put},
    Extension, Router,
};
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};

use always_errors::always_errors;
use create_task::create_task;
use delete_task::delete_tasks;
use get_json::get_json;
use get_tasks::{get_all_tasks, get_one_task};
use guard::guard;
use hello_world::hello_world;
use middleware_message::middleware_message;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_header::mirror_custom_header;
use mirror_user_agent::mirror_user_agent;
use partial_update_task::partial_update;
use path_variables::hard_coded_path;
use path_variables::path_variables;
use query_params::query_params;
use read_middleware_custom_header::read_middleware_custom_header;
use set_middleware_custom_headers::set_middleware_custom_header;
use update_tasks::atomic_update;
use users::create_users;
use users::login;
use users::logout;
use validate_data::valdiate_data;

#[derive(Clone)]
pub struct SharedData {
    pub message: String,
}

pub fn create_routes(database: DatabaseConnection) -> Router {
    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_origin(Any);

    let shared_data = SharedData {
        message: "Hello from shared data".to_owned(),
    };

    Router::new()
        .route(
            "/read_middleware_custom_header",
            get(read_middleware_custom_header),
        )
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/users/logout", post(logout))
        .route("/hello", get(hello_world))
        .route_layer(middleware::from_fn(guard))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/15", get(hard_coded_path))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .route("/always_errors", get(always_errors))
        .route("/get_json", get(get_json))
        .route("/validate_data", post(valdiate_data))
        .route("/tasks/create", post(create_task))
        .route("/tasks/:task_id", get(get_one_task))
        .route("/tasks/all", get(get_all_tasks))
        .route("/task/:task_id", put(atomic_update))
        .route("/task/:task_id", patch(partial_update))
        .route("/tasks/:task_id", delete(delete_tasks))
        .route("/users", post(create_users))
        .route("/users/login", post(login))
        .layer(cors)
        .layer(Extension(database))
        .layer(Extension(shared_data))
}
