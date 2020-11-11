use actix_web::{web, App, HttpResponse, HttpServer, Responder};

use lindera::tokenizer::Tokenizer;
use lindera_core::core::viterbi::Mode;


use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize)]
struct Input {
    text: String
}

#[derive(Serialize)]
struct Morph {
    start: usize,
    end: usize,
    surface: String,
    features: Vec<String>,
}

fn _analyze(text: &String) -> Vec<Morph> {
    let mut morphs: Vec<Morph> = Vec::new();
    let mut tokenizer = Tokenizer::new(Mode::Normal, "");

    // tokenize the text
    let tokens = tokenizer.tokenize(text.as_str());

    // output the tokens
    tokens.iter().fold(0, |sum, token| {
        let length = token.text.chars().count();
        morphs.push(Morph {start: sum, end: sum + length, surface: String::from(token.text), features: token.detail.clone()});
        sum + length
    });
    morphs
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world! foo")
}

async fn analyze(input: web::Json<Input>) -> impl Responder {
    let a = _analyze(&input.text);
    web::Json(a)
}


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("server starting ...");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/analyze", web::post().to(analyze))
    })
        .bind("0.0.0.0:8088")?
        .run()
        .await
}
