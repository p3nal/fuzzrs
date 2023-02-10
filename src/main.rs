use reqwest::{Url, blocking::Client};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Target {
    /// a valid url with no trailing /
    url: Url,
    /// an unchecked endpoint
    endpoint: String,
}

impl Target {
    fn new(url: &str, endpoint: &str) -> Target {
        let mut url = url.to_string();
        let endpoint = endpoint.to_string();
        if url.ends_with('/') {
            url.pop();
        }
        if !url.starts_with("http://") && !url.starts_with("https://") {
            url = format!("http://{url}");
        }
        let url =
            Url::parse(&url).expect("url cant be parsed, youve proly passed a bad written url");
        Target { url, endpoint }
    }

    fn get(&self, client: Client) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let url = Url::parse(&format!("{}/{}", self.url, self.endpoint.trim())).unwrap();
        Ok(client.get(url).send()?)
    }

    fn post(&self, client: Client) -> Result<reqwest::blocking::Response, reqwest::Error> {
        let url = Url::parse(&format!("{}/{}", self.url, self.endpoint.trim())).unwrap();
        Ok(client.post(url).send()?)
    }
}


fn main() {
    let url = env::args().nth(1).unwrap(); // first arg should be url
    let mut target = Target::new(&url, "");
    let wordlist = env::args().nth(2).unwrap(); // second argument is wordlist path
    println!("{:#?}", &wordlist);
    let wordlist =
        File::open(wordlist).expect("wordlist path is.. there's just something wrong with it");
    let reader = BufReader::new(wordlist);
    for line in reader.lines() {
        let word = line.expect("error in some word in the file");
        target.endpoint = word;
        let client = Client::new();
        let resp = target.get(client);
        match resp {
            Ok(r) => println!("status {}", r.status()),
            Err(e) => println!("error occured {e}"),
        }
    }
}
