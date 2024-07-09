use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // ロギングを有効にする
    tracing_subscriber::fmt::init();

    let app = Router::new() // ルーターを作成する
        .route("/hello", get("Hello, World.\n")); // GET /hello にアクセスしたときに "Hello, World." を返す

    // 8000番ポートでリクエストを待ち受けるListenerを作成する
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    // ローカルアドレスを表示する
    println!("listening on {}", listener.local_addr().unwrap());

    //　サーバーを起動する
    axum::serve(listener, app).await.unwrap();
}