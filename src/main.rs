extern crate curl;
extern crate regex;

use std::str;
use std::str::Utf8Error;
use std::io::prelude::*;
use std::fs::File;

use curl::http;
use curl::http::handle::Handle;
use regex::Regex;

fn main() {
    let resp = http::handle()
                   .get("http://www.ishuhui.com/archives/375140")
                   .exec()
                   .unwrap();

    let re = Regex::new(r#"https?://img[0-9]+[^"]*.png"#).unwrap();
    let handle = &mut http::handle();

    match str::from_utf8(resp.get_body()) {
        Ok(v) => {
            for caps in re.captures_iter(v) {
                let url = caps.at(0).unwrap();

                download(handle, &url);
                println!("{:?}", url);
            }

        }
        Err(err) => {
            println!("{:?}", err);
        }
    }

}

fn download(handle: &mut Handle, url: &str) -> Result<(), std::io::Error> {
    let spd: Vec<&str> = url.split("/").collect();
    if spd.len() > 0 {
        let name = spd.last().unwrap();
        let rsp = handle.get(url).exec().unwrap();
        let mut f = try!(File::create(name));
        try!(f.write_all(rsp.get_body()));
    }
    Ok(())

}
