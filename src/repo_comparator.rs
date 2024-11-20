use reqwest;
use serde_json::{Value, json};

pub async fn fetch_branch(branch: &str) -> Value {
	let responce = reqwest::get(format!("https://rdb.altlinux.org/api/export/branch_binary_packages/{branch}"))
		.await.expect(&format!("Cannot fetch the repository {branch}").as_str())
		.text()
		.await.expect("Cannot get response text");

	serde_json::from_str::<Value>(&responce)
		.expect("Unable to parse a responce")
}
