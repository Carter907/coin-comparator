use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs;
use std::iter::Map;
use std::path::PathBuf;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use actix_files::NamedFile;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tinytemplate::TinyTemplate;




#[derive(serde::Deserialize)]
struct QueryParams {
    id: String
}



#[get("/coin_values")]
async fn coin_values(params: web::Query<QueryParams>) -> impl Responder {// Create a reqwest Client
    let client = Client::new();
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

    // Make a GET request to an example API
    let response = client.get(format!("https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd", &params.id))
        .send()
        .await
        .unwrap();

    // Check if the request was successful
    if response.status().is_success() {
        // Return the response body as a String
        let bitcoin_price = response.json::<HashMap<String, HashMap<String, f64>>>().await
            .unwrap();
        println!("{}", &params.id);
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