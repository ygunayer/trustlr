extern crate clap;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;
extern crate futures;

use clap::{ App, Arg };
use hyper::{ Uri, Body, Client };
use hyper::client::FutureResponse;
use hyper_tls::HttpsConnector;
use std::io::{self, Write};
use futures::{Future, Stream};
use tokio_core::reactor::Core;
use std::str::FromStr;

pub struct Downloader<'a> {
    blog_name: String,
    blog_url: String,
    core: &'a Core
}

impl<'a> Downloader<'a> {
    pub fn new(core: &Core, blog_name: String) -> Downloader {
        Downloader {
            blog_name: blog_name.clone(),
            blog_url: format!("https://{}.tumblr.com", blog_name.clone()),
            core: &core
        }
    }

    #[inline]
    pub fn go(&self) -> Future<Item = String, Error = hyper::Error> {
        let client = Client::configure()
            .connector(HttpsConnector::new(4, &self.core.handle()).unwrap())
            .build(&self.core.handle());
        println!("Downloading from {:?}", &self.blog_url);
        
        let uri = Uri::from_str(&self.blog_url).unwrap();
        let result = client.get(uri).map(|res| {
            println!("Response Status: {}", res.status());

            res.body().concat2().map(|chunk| String::from_utf8(chunk.to_vec()));
        });

        result
    }
}

fn main() {
    let matches = App::new("trustlr")
        .version("0.0.1")
        .about("Helps bulk download Tumblr blogs")
        .arg(Arg::with_name("name")
            .required(true)
            .help("The name of the blog to download"))
        .get_matches();
    
    let blog_name = matches.value_of("name").unwrap();
    println!("Will download blog {}", blog_name);

    let core = Core::new();

    match core {
        Ok(c) => {
            let downloader = Downloader::new(&c, blog_name.to_string());

            let work = downloader.go();

            c.run(work)?;
        }
        Err(e) => println!("Failed to initialize I/O core due to {:?}", e)
    }
}
