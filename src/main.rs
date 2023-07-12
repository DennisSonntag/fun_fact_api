// async fn hello_world() -> &'static str {
//     "Hello, world!"
// }

use axum::{routing::get, Json, Router};
use rand::Rng;
use tokio::fs;

use serde::Serialize;

#[derive(Serialize)]
struct Fact {
	fact: String,
}

async fn handler() -> Json<Fact> {
	let contents = fs::read_to_string("facts.txt").await.unwrap();
	let facts = contents.lines().collect::<Vec<_>>();
	let index = rand::thread_rng().gen_range(0..facts.len());
	Json(Fact {
		fact: facts[index].to_string(),
	})
}

#[shuttle_runtime::main]
async fn axum() -> shuttle_axum::ShuttleAxum {
	let router = Router::new().route("/", get(handler));

	Ok(router.into())
}
