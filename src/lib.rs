extern crate url;
use url::Url;
use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub fn say(s: &str) -> String {
  let url = Url::parse(s).unwrap();
  println!("The Rust function say() received {}", s);
  let r = String::from("User name: ");
  return r + url.username();
}