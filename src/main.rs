use std::env;
use std::process;
use reqwest;

fn main() {
    let args: Vec<String> = env::args().collect();

    let url = parse_url(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    let client = reqwest::Client::new();
    let res = client.get(&url);
    println!("fetching url: {}", &url);
}

fn parse_url(args: &[String]) -> Result<String, &'static str> {
    if args.len() < 2 {
        return Err("please pass a url");
    } else if args.len() > 2 {
        return Err("please pass only one arg");
    }

    // TODO: manage references better later
    let url = args[1].clone();

    Ok(url)
}
