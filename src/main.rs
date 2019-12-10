#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use rocket_contrib::json::{Json, JsonValue};
use rocket_contrib::databases::redis::{self, Commands};


#[database("distreqs")]
struct DistReqsConn(redis::Connection);

#[derive(Serialize, Deserialize)]
struct Content {
    url: String,
    content: String,
}

#[derive(Serialize, Deserialize)]
struct Contents {
    url: String,
    content: Vec<String>,
}
#[derive(Serialize, Deserialize)]
struct ContentResponse {
    url: String,
    elements: Option<i64>,
}

#[get("/work")]
fn work(conn: DistReqsConn) -> JsonValue {
    let val : Option<String> = match conn.lpop("work_items") {
        Ok(val) => val,
        _ => None
    };
    json!({"url": val})
}
#[get("/work/<url>")]
fn content_for_url(conn: DistReqsConn, url: String) -> Json<Contents> {
    let content: Option<Vec<String>> = match conn.lrange(&url, 0, -1) {
        Ok(val) => val,
        Err(_err) => {
            println!("Error: {}", _err);
            None
        }
    };
    Json(Contents {url: url, content: content.unwrap()})
}

#[post("/work", data = "<url>")]
fn create_work(conn: DistReqsConn, url: String) -> Json<ContentResponse> {
    let elements: Option<i64> = match conn.rpush("work_items", &url) {
        Ok(val) => val,
        Err(_err) => {
            println!("Error: {}", _err);
            None
        }
    };
    Json(ContentResponse {url: url, elements: elements})
}

#[post("/work/content", data = "<content>")]
fn content(conn: DistReqsConn, content: Json<Content>) -> Json<ContentResponse> {
    let url = content.url.clone();
    let elements: Option<i64> = match conn.rpush(&content.url, &content.content) {
        Ok(val) => val,
        Err(_err) => {
            println!("Error: {}", _err);
            None
        }
    };
    Json(ContentResponse {url: url, elements: elements})
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .attach(DistReqsConn::fairing())
        .mount("/", routes![work, create_work, content, content_for_url])
}

fn main() {
    rocket().launch();
}
