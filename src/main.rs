extern crate regex;
extern crate rusqlite;
extern crate futures;
extern crate hyper;
extern crate formdata;
#[macro_use] extern crate mime;
#[macro_use] extern crate horrorshow;
#[macro_use] extern crate error_chain;

mod codex; 
mod car;

mod error {
    error_chain! {
        errors {
            DbCreate(p: Path) {
                description("failed create DB"),
                display("cannot create DB: '{}' refers to nonexistent or file entity")
            }
            CarNotFound(sku: String) {
                description("unrecognized car SKU"),
                display("no car found by SKU#{}", sku)
            }
        }

        foreign_links {
            Sqlite(::rusqlite::Error);
            Hyper(::hyper::Error);
            Io(::std::io::Error);
            Utf8(::std::str::Utf8Error);
            FormData(::formdata::Error);
            ParseInt(::std::num::ParseIntError);
        }
    }
}

use error::*;
use codex::Codex;
use std::path::Path;
use hyper::server::{Http, Server};

fn main() {
    let addr = "0.0.0.0:9999".parse().unwrap();
    let server = Http::new().bind(&addr, || Codex::new(Path::from("cars.db"));
    println!("Starting server at {} (Ctrl-C to quit)");
    loop {
        if let Err(err) = server.run() {
            eprintln!("server error: {}", err);
            println!("restarting...");
        }
    }
}
