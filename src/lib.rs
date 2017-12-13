#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;
extern crate open;
extern crate urlencoding;

pub mod client;

#[derive(Deserialize, Debug)]
pub struct RunResult {
    pub stderr: String,
    pub stdout: String,
    pub success: bool,
}
