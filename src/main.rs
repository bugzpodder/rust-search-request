use actix_web::{get, post, web, App, HttpServer, Responder};
use itertools::Itertools;
use serde_json::to_value;
use std::collections::HashMap;

pub use types::WalletRequest;
mod types;

pub use query::Query;
mod query;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[post("/search")]
async fn search(body: web::Json<WalletRequest>) -> actix_web::Result<impl Responder> {
    let aliases = HashMap::from([
        ("wallet_id".to_string(), "id".to_string()),
        ("wallet_type".to_string(), "type".to_string()),
    ]);
    let req = body.into_inner();
    let query = "SELECT c.data, c.id, c.type FROM c";
    let value = to_value(&req).unwrap();
    let where_clause = &value["where_clause"];
    let order_clauses = &value["order_by"];
    let offset = value["offset"].as_u64().unwrap_or(0);
    let limit = value["limit"].as_u64().unwrap_or(0);
    let mut query_obj = Query::new(query.to_string(), aliases);
    let mut query_stmt = query_obj.get_query(where_clause, order_clauses, offset, limit);
    for key in query_obj.values.keys().sorted() {
        let value = &query_obj.values[key];
        query_stmt.push_str(&format!("\n{key}={value}").to_string());
    }
    Ok(query_stmt.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
            .service(search)
    })
    .bind(("0.0.0.0", 8088))?
    .run()
    .await
}
