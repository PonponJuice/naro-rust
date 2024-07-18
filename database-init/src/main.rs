use sqlx::{mysql, prelude::FromRow};
use std::env;

// データベースに繋ぐための情報を保持する構造体
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

// IDのせいで#[sqlx(rename_all = "PascalCase")] が使えないので、手動でrenameを書く(なんかいい方法あったら教えてください)
#[derive(FromRow)]
struct City {
    #[sqlx(rename = "ID")]
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

// 国の人口を取得するための構造体
#[derive(FromRow)]
#[sqlx(rename_all = "PascalCase")]
struct Population {
    pub population: i32,
}

#[tokio::main]
async fn main() {
    // 環境変数からデータベースに繋ぐための情報を取得
    let config = Config {
        mysql_host: std::env::var("MYSQL_HOSTNAME").unwrap_or_else(|_| "localhost".to_string()),
        mysql_port: std::env::var("MYSQL_PORT").unwrap_or_else(|_| "3306".to_string()),
        mysql_user: std::env::var("MYSQL_USERNAME").unwrap_or_else(|_| "root".to_string()),
        mysql_password: std::env::var("MYSQL_PASSWORD")
            .unwrap_or_else(|_| "password".to_string()),
        mysql_database: std::env::var("MYSQL_DATABASE").unwrap_or_else(|_| "world".to_string()),
    };

    // データベースに接続
    let pool = mysql::MySqlPool::connect(&config.database_url())
        .await
        .unwrap();
    println!("connected");

    // コマンドライン引数から都市名を取得
    let args: Vec<String> = env::args().collect();
    let city_name = if args.len() < 2 {
        "Tokyo"
    } else {
        args[1].as_str()
    };

    // 都市名から都市の情報を取得
    let city = sqlx::query_as::<_, City>("SELECT * FROM city WHERE Name = ?")
        .bind(city_name)
        .fetch_optional(&pool)
        .await
        .unwrap();

    // 都市が存在しない場合はエラーを表示して終了
    let city = match city {
        Some(city) => city,
        None => {
            println!("no such city Name = '{}'\n", city_name);
            return;
        }
    };

    println!("{}の人口は{}人です", city_name, city.population);

    // 都市の国の人口を取得
    let population =
        sqlx::query_as::<_, Population>("SELECT Population FROM country WHERE Code = ?")
            .bind(&city.country_code)
            .fetch_one(&pool)
            .await
            .unwrap();

    // 都市の人口が国の人口の何%かを計算
    let percent = city.population as f64 / population.population as f64 * 100.0;

    println!("これは{}の人口の{}%です", city.country_code, percent);
}
