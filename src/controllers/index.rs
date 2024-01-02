use actix_web::{web, Responder,HttpResponse};
use serde_json::json;
use handlebars::Handlebars;

use utilities_rs::rand::rand_string;
use crate::models::index::Record;

pub async fn index(hb: web::Data<Handlebars<'_>>) -> impl Responder {

    let mut Records: Vec<Record> = vec![];
    for i in 0..20 {
        Records.push(Record {
            id: i.to_string(),
            title: rand_string(5, 50),
        });
    }

    let html: String = hb.render("layout", &json!({
        "title": "index | mvc",
        "content": hb.render("index", &json!({
            "records": Records
        })).unwrap()
    })).unwrap();
    HttpResponse::Ok().content_type("text/html").body(html)
}