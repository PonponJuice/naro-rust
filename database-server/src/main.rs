use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use sqlx::{mysql, prelude::FromRow};

struct Config {
    mysql_host: String,
    mysql_port: String,
    mysql_user: String,
    mysql_password: String,
    mysql_database: String,
}
impl Config {
    pub fn database_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.mysql_user,
            self.mysql_password,
            self.mysql_host,
            self.mysql_port,
            self.mysql_database,
        )
    }
}

#[tokio::main]
async fn main() {
    let config = Config {
        mysql_host: std::env::var("MYSQL_HOSTNAME").unwrap_or_else(|_| "localhost".to_string()),
        mysql_port: std::env::var("MYSQL_PORT").unwrap_or_else(|_| "3306".to_string()),
        mysql_user: std::env::var("MYSQL_USERNAME").unwrap_or_else(|_| "root".to_string()),
        mysql_password: std::env::var("MYSQL_PASSWORD")
            .unwrap_or_else(|_| "password".to_string()),
        mysql_database: std::env::var("MYSQL_DATABASE").unwrap_or_else(|_| "world".to_string()),
    };

    let pool = mysql::MySqlPool::connect(&config.database_url())
        .await
        .unwrap();

    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/cities/:cityname", get(get_city_handler))
        .route("/cities", post(post_city_handler))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

#[derive(FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct City {
    #[sqlx(rename = "ID")]
    #[serde(default)]
    pub id: i32,
    #[sqlx(rename = "Name")]
    pub name: String,
    #[sqlx(rename = "CountryCode")]
    pub country_code: String,
    #[sqlx(rename = "District")]
    pub district: String,
    #[sqlx(rename = "Population")]
    pub population: i32,
}

async fn get_city_handler(
    State(pool): State<mysql::MySqlPool>,
    Path(cityname): Path<String>,
) -> Result<Json<City>, (StatusCode, String)> {
    // データベースからcitynameに一致するデータを取得
    let city = sqlx::query_as::<_, City>("SELECT * FROM city WHERE Name = ?")
        .bind(&cityname)
        .fetch_optional(&pool)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from("internal server error"),
            )
        })?;

    match city {
        Some(city) => Ok(Json(city)),
        None => Err((
            StatusCode::NOT_FOUND,
            format!("No such city Name = {}", cityname),
        )),
    }
}

async fn post_city_handler(
    State(pool): State<mysql::MySqlPool>,
    Json(city): Json<City>,
) -> Result<Json<City>, (StatusCode, String)> {
    let result = sqlx::query(
        "INSERT INTO city (Name, CountryCode, District, Population) VALUES (?, ?, ?, ?)",
    )
    .bind(&city.name)
    .bind(&city.country_code)
    .bind(&city.district)
    .bind(city.population)
    .execute(&pool)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("internal server error"),
        )
    })?;

    let id = result.last_insert_id();

    let city = City {
        id: id as i32,
        ..city
    };

    Ok(Json(city))
}
