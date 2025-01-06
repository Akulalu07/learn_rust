use regex::Regex;
use clap::{App,Arg};

fn main() {
    let args = App::new("grep-lite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();
    let quote = "";
    for line in quote.lines(){
       match re.find(line) {
           Some(_) => println!("{}", line),
           None => (),
       } 
    }
}

