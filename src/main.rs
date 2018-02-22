extern crate clap;

use clap::{ App, Arg };

fn main() {
    let matches = App::new("trustlr")
        .version("0.0.1")
        .about("Helps bulk download Tumblr blogs")
        .author("Yalin Gunayer")
        .arg(Arg::with_name("name")
            .required(true)
            .help("The name of the blog to download"))
        .get_matches();
    
    let blog_name = matches.value_of("name").unwrap();
    println!("Will download blog {}", blog_name);
}
