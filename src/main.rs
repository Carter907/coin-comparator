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
async fn coin_values(params: web::Query<QueryParams>) -> impl Responder {// Create a reqwest Client
    let html_content = fs::read_to_string("app/index.html").unwrap();
    let mut template = TinyTemplate::new();
    template.add_template("coin_values_templ", &html_content).unwrap();

    let mut context: HashMap<String, f64> = HashMap::from(
        [
            ("bitcoin_price".to_string(), 0f64),
            ("ethereum_price".to_string(), 0f64),
            ("binance_coin_price".to_string(), 0f64)
        ]
    );

    if let Ok(bitcoin_price) = get_coin_price(&params.id).await {
        context.insert(format!("{}_price", &params.id), bitcoin_price[&params.id]["usd"]);

        let rendered = template.render("coin_values_templ", &context).unwrap();
        HttpResponse::Ok()
            .content_type("text/html")
            .body(rendered)
    } else {


        // Return an error if the request was not successful
        HttpResponse::InternalServerError().body("womp womp")
    }
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