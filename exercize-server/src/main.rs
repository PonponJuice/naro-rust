use axum::{
    extract::{Query, rejection::JsonRejection},
    extract::Path,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use std::clone;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // pongを返す「/ping」というエンドポイントを作成する
    let app = Router::new()
        .route("/ping", get("pong\n"))
        .route("/fizzbuzz", get(fizzbuzz_handler))
        .route("/add", post(add_handler))
        .route(
            "/students/:classNumber/:studentNumber",
            get(get_student_handler),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

// クエリパラメータを受け取るための構造体を定義する
#[derive(serde::Deserialize)]
struct FizzBuzzQuery {
    pub count: Option<String>,
}

async fn fizzbuzz_handler(Query(query): Query<FizzBuzzQuery>) -> (StatusCode, String) {
    // クエリパラメータが指定されていない場合はデフォルト値を使用する
    let mut n: i32 = 30;
    // クエリパラメータが指定されている場合はその値を調べる
    if let Some(count) = query.count {
        let count = count.parse();
        match count {
            // 数値に変換できた場合はその値を使用する
            Ok(count) => n = count,
            // 数値に変換できない場合はエラーを返す
            Err(_) => return (StatusCode::BAD_REQUEST, String::from("Bad Request\n")),
        }
    }

    // FizzBuzzの結果を生成する
    let mut result = String::new();
    for i in 1..=n {
        if i % 3 == 0 && i % 5 == 0 {
            result += "FizzBuzz\n";
        } else if i % 3 == 0 {
            result += "Fizz\n";
        } else if i % 5 == 0 {
            result += "Buzz\n";
        } else {
            result += &format!("{}\n", i);
        }
    }

    // statuscode 200(OK)と結果を返す
    (StatusCode::OK, result)
}

// クエリを受け取るための構造体
#[derive(serde::Deserialize)]
struct AddQuery {
    left: f64,
    right: f64,
}

// クライアントに返すレスポンスの構造体
#[derive(serde::Serialize)]
struct AddResponse {
    answer: i64,
}

// エラー時のレスポンスの構造体
#[derive(serde::Serialize)]
struct AddError {
    error: String,
}

async fn add_handler(
    query: Result<Json<AddQuery>, JsonRejection>,
) -> Result<Json<AddResponse>, (StatusCode, Json<AddError>)> {
    match query {
        // クエリが正しく受け取れた場合、クライアントに結果を返す
        Ok(query) => Ok(Json(AddResponse {
            answer: (query.left + query.right) as i64,
        })),

        // クエリが正しく受け取れなかった場合、エラーを返す
        Err(_) => Err((
            StatusCode::BAD_REQUEST,
            Json(AddError {
                error: String::from("Bad Request"),
            }),
        )),
    }
}

// クラスと生徒のデータを定義
const CLASS_DATA: &str = r#"
[
  {"class_number": 1, "students": [
    {"student_number": 1, "name": "pikachu"},
    {"student_number": 2, "name": "ikura-hamu"},
    {"student_number": 3, "name": "noc7t"}
  ]},
  {"class_number": 2, "students": [
    {"student_number": 1, "name": "Sora"},
    {"student_number": 2, "name": "Kaito"},
    {"student_number": 3, "name": "Haruka"},
    {"student_number": 4, "name": "Shingo"}
  ]},
  {"class_number": 3, "students": [
    {"student_number": 1, "name": "Hikaru"},
    {"student_number": 2, "name": "Eri"},
    {"student_number": 3, "name": "Ryo"}
  ]},
  {"class_number": 4, "students": [
    {"student_number": 1, "name": "Marina"},
    {"student_number": 2, "name": "Takumi"}
  ]}
]"#;

// 生徒のデータを受け渡しするための構造体
#[derive(serde::Serialize, serde::Deserialize, clone::Clone)]
struct Student {
    student_number: u32,
    name: String,
}

// クラスのデータを作るための構造体
#[derive(serde::Deserialize)]
struct Class {
    class_number: u32,
    students: Vec<Student>,
}

async fn get_student_handler(
    Path((class_number, student_number)): Path<(u32, u32)>,
) -> Result<Json<Student>, (StatusCode, Json<serde_json::Value>)> {
    // クラスの情報を取得する
    let class: Vec<Class> = serde_json::from_str(CLASS_DATA).unwrap();

    // クラス番号と生徒番号から生徒の情報を取得する
    let student = class
        .iter()
        .find(|c| c.class_number == class_number)
        .and_then(|c| {
            c.students
                .iter()
                .find(|s| s.student_number == student_number)
        });

    match student {
        // 生徒の情報があればその情報を返す
        Some(student) => Ok(Json(student.clone())),

        // 生徒の情報がなければエラーを返す
        None => Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": "Student Not Found",})),
        )),
    }
}
