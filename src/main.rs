use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
struct CatFacts {
    fact: String,
    length: i32,
}

#[derive(Serialize, Debug)]
struct UserInfo {
    email: String,
    name: String,
    stack: String,
}

#[derive(Serialize, Debug)]
struct ApiResponse {
    status: String,
    user: UserInfo,
    timestamp: String,
    fact: String,
}

async fn fetch_cat_fact(api_url: &str) -> Result<CatFacts, reqwest::Error> {
    let client = Client::new();
    let response = client.get(api_url)
    .send()
    .await?
    .json::<CatFacts>()
    .await?;
    Ok(response)
}


async fn cat_fact_handler() -> impl Responder {
    let api_url = "https://catfact.ninja/fact";

    match fetch_cat_fact(api_url).await {
        Ok(fact_data) => {
            let response = ApiResponse {
            status: "success".to_string(),
            user: UserInfo {
                email: "idowuseyi22@gmail.com".to_string(),
                name: "Oluwaseyi Idowu".to_string(),
                stack: "Rust/Actix".to_string(),
            },
            timestamp: Utc::now().to_rfc3339(),
            fact: fact_data.fact,
        };

        HttpResponse::Ok().json(response)
    }
    Err(err) => {
        eprintln!("Error fetching cat fact: {:?}", err);
        HttpResponse::InternalServerError()
            .body("Error fetching cat fact from external API")
    }
}
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Server running on http://127.0.0.1:8080/me");

    HttpServer::new(|| 
        App::new()
            .route("/me", web::get().to(cat_fact_handler))
    )
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
