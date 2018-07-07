#[macro_use] extern crate clap;

use clap::App;

fn main() {
    let yaml = load_yaml!("../res/cli.yaml");
    let matches = App::from_yaml(yaml).get_matches();

    if let Some(submatches) = matches.subcommand_matches("export"){
        println!("{}", "likemark export");
        if let Some(dest) = submatches.value_of("DEST"){
            println!("{}", dest);
        }
    }   
}
