#![feature(proc_macro_hygiene)]

use clap::{clap_app, crate_version};
use maud::html;
use pulldown_cmark::{html::push_html, Event, Parser};

fn wrap_html(s:&str, css: Option<&str>) -> String {

   let res = html!{
        (maud::DOCTYPE)
        html {
           head {
               meta charset="utf-8";
               @if let Some(s) = css {
                  link rel="stylesheet" type="text/css" href= (s) {}
               }
           }
           body {
             (maud::PreEscaped(s))
           }
        }
   };

   res.into_string()

}

fn main() {

    let clap = clap_app!( mdrend =>
               (version: crate_version!())
               (author:"Mohamed Magdi")
               (about:"Make an a Command Clap with Rust!")
               (@arg input: +required "Set The Input File")
               (@arg wrap: -w "wrap in html")
               (@arg event: -e "Print Event")
               (@arg css: --css +takes_value "Link to css")
    ).get_matches();
    
    println!("Input : {:?}\n", clap.value_of("input").unwrap());

    let file_name = clap.value_of("input").unwrap();

    if !file_name.ends_with(".md") {
       panic!("Wrong File Extention!");
    }

    let infile = std::fs::read_to_string(clap.value_of("input").unwrap()).expect("Could Not Read The File!");

    let mut res = String::new();
    let ps = Parser::new(&infile);

    let ps: Vec<Event> = ps.into_iter().collect();

    if clap.is_present("event") {
       for p in &ps {
	if p ==&ps[&ps.len() -1] {
            println!("{:?}\n", p);
        }else{
            println!("{:?}", p);
        }
       }
    }

    push_html(&mut res, ps.into_iter());

    if clap.is_present("wrap") {
        res = wrap_html(&res, clap.value_of("css"));
    }

    println!("{}", res);

    println!("Done!");
}
