use dotenv::{dotenv, vars};

fn main() {
    let path = dotenv().unwrap();
    println!("cargo::rerun-if-changed={}", path.display());

    for (key, value) in vars() {
        println!("cargo::rustc-env={key}={value}")
    }
}