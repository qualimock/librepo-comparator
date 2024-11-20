use std::collections::HashMap;

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

pub fn collect_packages(branch_json: Value) -> HashMap<String, Vec<String>> {
	let packages_json = branch_json
		.get("packages")
		.expect("Cannot get field 'packages'")
		.as_array()
		.expect("Not an array")
		.to_vec();

	let mut packages = HashMap::new();

	for pkg in packages_json {
		packages.insert(pkg.get("name").expect("Cannot get field 'name'").to_string().replace("\"", ""),
						vec![pkg.get("version").expect("Cannot get field 'version'").to_string().replace("\"", ""),
							 pkg.get("release").expect("Cannot get field 'release'").to_string().replace("\"", ""),
							 pkg.get("arch").expect("Cannot get field 'version'").to_string().replace("\"", ""),
							 pkg.get("disttag").expect("Cannot get field 'release'").to_string().replace("\"", ""),
							 pkg.get("epoch").expect("Cannot get field 'version'").to_string().replace("\"", ""),
							 pkg.get("source").expect("Cannot get field 'version'").to_string().replace("\"", "")]);
	}

	packages
}

pub fn compare_branches(branch_a: &HashMap<String, Vec<String>>, branch_b: &HashMap<String, Vec<String>>) -> (HashMap<String, Vec<String>>, HashMap<String, Vec<String>>) {
	let mut in_a_not_in_b = HashMap::new();
	let mut in_b_not_in_a = HashMap::new();

	for pkg in branch_a {
		if !branch_b.contains_key(pkg.0) {
			in_a_not_in_b.insert(pkg.0.to_string(), pkg.1.to_vec());
		}
	}

	for pkg in branch_b {
		if !branch_a.contains_key(pkg.0) {
			in_b_not_in_a.insert(pkg.0.to_string(), pkg.1.to_vec());
		}
	}

	(in_a_not_in_b, in_b_not_in_a)
}

fn get_common_packages(branch_a: &HashMap<String, Vec<String>>, branch_b: &HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>> {
	let mut common_packages = HashMap::new();

	for pkg in branch_a {
		if branch_b.contains_key(pkg.0) {
			common_packages.insert(pkg.0.to_string(), pkg.1.to_vec());
		}
	}

	common_packages
}
