extern crate serde_json;
use self::serde_json::{Value};

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
  success: bool,
  message: Value
}