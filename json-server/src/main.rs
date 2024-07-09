use axum::{routing::get, Json, Router};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/json", get(json_handler).post(post_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

// JSONから構造体に変換するためにserde::Deserializeもderiveする
#[derive(serde::Serialize, serde::Deserialize)]
struct JsonData {
    number: i32,
    string: String,
    bool: bool,
}

async fn json_handler() -> Json<JsonData> {
    // レスポンスとして返す値を構造体として定義
    let response = JsonData {
        number: 42,
        string: String::from("hello"),
        bool: true,
    };

    // 構造体をJSONに変換してクライアントに返す
    Json(response)
}

async fn post_handler(
    // JSONを受け取るためにJson<JsonData>を引数に取る
    Json(data): Json<JsonData>,
) -> Json<JsonData> {
    // 構造体をJSONに変換してクライアントに返す
    Json(data)
}