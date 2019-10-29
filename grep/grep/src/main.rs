use std::env;
use std::process;
use grep;
use grep::Config;



fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("{:?}",args);

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);


    if let Err(e) = grep::run(config){
        eprintln!("Application error: {}", e);

        process::exit(1);
    }
}



