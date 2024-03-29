use std::collections::HashMap;
use std::fmt::Display;
use std::fs;

use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use reqwest::Client;
use tinytemplate::TinyTemplate;
use actix_rust_template::get_coin_price;


#[derive(serde::Deserialize)]
struct QueryParams {
    id: String,
}


#[get("/coin_values")]
async fn coin_values() -> impl Responder {// Create a reqwest Client

    const COINS: [&str; 5] =
        [
            "bitcoin",
            "ethereum",
            "solana",
            "cardano",
            "tether",


        ];
    let html_content = fs::read_to_string("app/index.html").unwrap();
    let mut template = TinyTemplate::new();
    template.add_template("coin_values_templ", &html_content).unwrap();

    let mut context: HashMap<String, String> = HashMap::new();

    for coin in COINS {
        if let Ok(bitcoin_price) = get_coin_price(coin).await {
            context.insert(
                coin.to_string(),
                format!("{} : ${}", coin, bitcoin_price[coin]["usd"]),
            );
        }
    }
    println!("{context:?}");
    println!("{}", &html_content);

    let rendered = template.render("coin_values_templ", &context).unwrap();

    println!("{}", &rendered);
    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/app")
                .service(coin_values)
            )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}