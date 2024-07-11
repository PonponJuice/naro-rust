use axum::{
    extract::{Path, Query},
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // GETリクエストの"/hello/:username"というパターンに対応するルートを設定し、
    // URLのパラメータ(:username)を使用してhelloHandler関数を呼び出す
    let app = Router::new().route("/hello/:username", get(hello_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

// クエリパラメータを取得するための構造体を定義する
#[derive(serde::Deserialize)]
pub struct QueryParam {
    lang: Option<String>,
    page: Option<String>,
}

async fn hello_handler(
    // パスパラメータを取得する
    Path(username): Path<String>,
    // クエリパラメータを取得する
    Query(query): Query<QueryParam>,
) -> String {
    format!(
        "Hello, {}!\nlanguage: {}\npage: {}\n",
        username,
        query.lang.unwrap_or(String::from("")),
        query.page.unwrap_or(String::from(""))
    )
}
