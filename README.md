# librepo-comparator
A Rust crate which is used to work with AltRepo API export method.

The crate is used in [Repo Comparator](https://github.com/qualimock/repo-comparator) utility.

## Prerequisites
To build the crate you should use Rust package manager [Cargo](https://doc.rust-lang.org/stable/cargo/)

It is available systemlessly on [rustup](https://rustup.rs/)

Or can be installed on your ALT distro via apt-get:
```
apt-get install rust-cargo
```

## How to build
Clone the repository:
```
git clone https://github.com/qualimock/librepo-comparator.git
cd repo-comparator
```

Build the program:
```
cargo build --release --lib
```

## How to use
Include the crate into Cargo.toml of your project:
```
...
[dependencies]
librepo-comparator = { git = "https://github.com/qualimock/librepo-comparator.git", branch="master" }
...
```

Use it in your project:
```rust
...
use repo_comparator::repo_comparator;
...
```

## Public Functions

```rust
pub async fn fetch_branch(branch: &str) -> Value
```
Returns [serde_json](https://crates.io/crates/serde_json) object of selected branch.

---

```rust
pub fn collect_packages(branch_json: Value) -> HashMap<String, Vec<String>>
```
Collects packages of branch into HashMap where the key is name of package.

---

```rust
pub fn compare_branches(branch_a: &HashMap<String, Vec<String>>, branch_b: &HashMap<String, Vec<String>>) -> (HashMap<String, Vec<String>>, HashMap<String, Vec<String>>)
```
Compares two branches and returns a tuple where first HashMap contains packages which exist only in branch A, and the second contains packages which exist only in branch B

---

```rust
pub fn compare_versions(branch_a: &HashMap<String, Vec<String>>, branch_b: &HashMap<String, Vec<String>>) -> HashMap<String, Vec<String>>
```
Compares versions of packages of both branches and returns HashMap that contains packages which version in branch A is greater than in branch B.

If package exists only in one branch, it is not compared with.

---

```rust
pub fn packages_to_json(branch: &HashMap<String, Vec<String>>) -> Vec<Value>
```
Packs HashMap with packages into JSON object.
