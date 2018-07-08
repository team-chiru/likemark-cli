#[macro_use] extern crate serde_derive;
#[macro_use] extern crate clap;
extern crate hyper;
mod model;

use model::likemark::Likemark;
use clap::App;
use std::io::{self, Write};
use hyper::Client;
use hyper::rt::{self, Future, Stream};

fn main() {
    let yaml = load_yaml!("../res/cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let client = Client::new();

    if matches.is_present("list") {
        let uri: hyper::Uri = "http://localhost:42506/likemark/list".parse().unwrap();

        let request = client
        // Fetch the url...
        .get(uri)
        // And then, if we get a response back...
        .and_then(|res| {
            println!("Response: {}", res.status());
            println!("Headers: {:#?}", res.headers());

            // The body is a stream, and for_each returns a new Future
            // when the stream is finished, and calls the closure on
            // each chunk of the body...
            println!("{}", "Here is the likemark list.");
            res.into_body().for_each(|chunk| {
                io::stdout().write_all(&chunk)
                    .map_err(|e| panic!("example expects stdout is open, error={}", e))
            })
        })
        // If all good, just tell the user...
        .map(|_| {
            println!("\n\nDone.");
        })
        // If there was an error, let the user know...
        .map_err(|err| {
            eprintln!("Error {}", err);
        });

        rt::run(request);
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
