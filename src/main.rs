use axum_server::run;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = dotenv!("DATABASE_URL");
    println!("{}", database_url);
    run(database_url).await
}
