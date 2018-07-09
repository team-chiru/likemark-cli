#[macro_use] extern crate serde_derive;
#[macro_use] extern crate clap;
extern crate serde_json;
extern crate curl;

// Import of likemark model
mod model;
use model::likemark::Likemark;
use model::response::Response;

use clap::App;
use std::io::{stdout, Write};
use curl::easy::Easy;
use std::str;
use serde_json::{Value, Error};

fn parse_bytes (data: &[u8]) -> Response {
    let on_str = str::from_utf8(data).unwrap();
    serde_json::from_str(on_str).unwrap()
}

fn main() {
    let yaml = load_yaml!("../res/cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let mut client = Easy::new();

    if matches.is_present("list") {
        let url = "http://localhost:42506/likemark/list";
        client.url(url).unwrap();
        client.write_function(|data| {
            println!("{:?}", parse_bytes(data));
            Ok(data.len())
        }).unwrap();
        client.perform().unwrap();
        println!("\nHere is the status code: {}", client.response_code().unwrap());
    } 

    if let Some(submatches) = matches.subcommand_matches("import") {
        println!("{}", "likemark import");
        if let Some(src) = submatches.value_of("SRC"){
            println!("{}", src);
        }
    }   

    if let Some(submatches) = matches.subcommand_matches("export") {
        println!("{}", "likemark export");
        if let Some(dest) = submatches.value_of("DEST"){
            println!("{}", dest);
        }
    }   
}
