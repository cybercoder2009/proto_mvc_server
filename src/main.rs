mod config;
mod state;
mod controllers;
pub mod models;
extern crate utilities_rs;

use std::fs;
use actix_files::Files;
use std::sync::Arc;
use actix_web::{HttpServer,web,App};
use handlebars::Handlebars;
use mongodb::{Client,Database};
use mongodb::options::ClientOptions;
use config::Config;
use state::State;
use controllers::index::index;
use controllers::auth::login;

fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(Files::new("/static", "./static").show_files_listing());
    cfg.route("/", web::get().to(index));
    cfg.route("/login", web::get().to(login));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config_str: String = fs::read_to_string("config.toml")
    .expect("Error on reading config.toml");
    let config: Config = toml::from_str(&config_str)
    .expect("Error on parsing config.toml");
    println!("{:?}", config);

    let opt: ClientOptions = ClientOptions::parse(config.mongo_binding)
        .await.expect("Error on mongo connection string");
    let db: Database = Client::with_options(opt)
        .expect("Error on connecting mongodb")
        .database(&config.mongo_db);
    let state: State = State {
        db: Arc::new(db)
    };

    env_logger::init();

    let mut hb = Handlebars::new();
    hb.register_template_file("layout", "views/layout.html").unwrap();
    hb.register_template_file("index", "views/index.html").unwrap();
    hb.register_template_file("login", "views/login.html").unwrap();
    let hb_ref = web::Data::new(hb);

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(state.clone()))
        .app_data(hb_ref.clone())
        .configure(routes)
    })
    .bind(&config.server_rest)
    .expect("Error on rest binding")
    .run().await
}