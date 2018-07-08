#[macro_use] extern crate serde_derive;
#[macro_use] extern crate clap;
extern crate curl;

// Import of likemark model
mod model;
use model::likemark::Likemark;

use clap::App;
use std::io::{stdout, Write};
use curl::easy::Easy;

fn main() {
    let yaml = load_yaml!("../res/cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let mut client = Easy::new();

    if matches.is_present("list") {
        let url = "http://localhost:42506/likemark/list";
        client.url(url).unwrap();
        client.write_function(|data| {
            stdout().write_all(data).unwrap();
            Ok(data.len())
        }).unwrap();
        client.perform().unwrap();

        println!("{}", "");
        println!("Here is the status code: {}", client.response_code().unwrap());
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
