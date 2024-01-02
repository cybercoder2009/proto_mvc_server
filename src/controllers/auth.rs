use actix_web::{web, Responder,HttpResponse};
use serde_json::json;
use handlebars::Handlebars;

pub async fn login(hb: web::Data<Handlebars<'_>>) -> impl Responder {
    
    let html: String = hb.render("layout", &json!({
        "title": "index | mvc",
        "content": hb.render("login", &json!({

        })).unwrap()
    })).unwrap();
    HttpResponse::Ok().content_type("text/html").body(html)
}