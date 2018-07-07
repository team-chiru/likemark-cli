#[macro_use] extern crate clap;
extern crate reqwest;

use clap::App;

fn main() {
    let yaml = load_yaml!("../res/cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();
    let client = reqwest::Client::new();

    if matches.is_present("list") {
        println!("je rentre dans list");
        let body = client.get("http://localhost:42506/likemark/list").send();
        println!("body = {:?}", body);
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
