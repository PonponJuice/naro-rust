use std::env;
use sqlx::{mysql, prelude::FromRow};

struct Config {
    mariadb_host: String,
    mariadb_port: String,
    mariadb_user: String,
    mariadb_password: String,
    mariadb_database: String,
}

impl Config {
    pub fn database_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.mariadb_user,
            self.mariadb_password,
            self.mariadb_host,
            self.mariadb_port,
            self.mariadb_database
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

#[derive(FromRow)]
#[sqlx(rename_all = "PascalCase")]
struct Population {
    pub population: i32,
} 


#[tokio::main]
async fn main(){
    let hostname = std::env::var("MYSQL_HOSTNAME").unwrap_or_else(|_| "localhost".to_string());
    let port     = std::env::var("MYSQL_PORT").unwrap_or_else(|_| "3306".to_string());
    let username = std::env::var("MYSQL_USERNAME").unwrap_or_else(|_| "root".to_string());
    let password = std::env::var("MYSQL_PASSWORD").unwrap_or_else(|_| "password".to_string());
    let database = std::env::var("MYSQL_DATABASE").unwrap_or_else(|_| "world".to_string());

    let config = Config {
        mariadb_host: hostname,
        mariadb_port: port,
        mariadb_user: username,
        mariadb_password: password,
        mariadb_database: database,
    };

    let pool = mysql::MySqlPool::connect(&config.database_url()).await.unwrap();
    println!("connected");

    let args: Vec<String> = env::args().collect();
    let city_name = if args.len() < 2 {
        "Tokyo"
    } else {
        args[1].as_str()
    };

    let city = sqlx::query_as::<_, City>("SELECT * FROM city WHERE Name = ?")
        .bind(city_name)
        .fetch_optional(&pool)
        .await
        .unwrap();

    let city = match  city {
        Some(city) => city,
        None => {
            println!("no such city Name = '{}'\n", city_name);
            return;
        }
    };

    
    println!("{}の人口は{}人です", city_name, city.population);

    
    let population = sqlx::query_as::<_, Population>("SELECT Population FROM country WHERE Code = ?")
        .bind(&city.country_code)
        .fetch_one(&pool)
        .await
        .unwrap();

    let percent = city.population as f64 / population.population as f64 * 100.0;

    println!("これは{}の人口の{}%です", city.country_code, percent);
}