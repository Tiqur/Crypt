use std::env;
use std::fs;
extern crate clap;
use clap::{Arg, App};

fn main() {
   let app = App::new("Crypt")
       .version("1.0")
       .about("A simple commandline utility to recursively encrypt all files / directories")
       .author("Trevor Brage")
       .arg(Arg::with_name("path")
           .short("p")
           .long("path")
           .value_name("FILE")
           .help("Path to be encrypted ( Otherwise will execute in current directory )")
           .takes_value(true))
       .arg(Arg::with_name("encrypt")
           .short("e")
           .long("encrypt")
           .help("Sets mode to 'encrypt'"))
       .arg(Arg::with_name("decrypt")
           .short("d")
           .long("decrypt")
           .help("Sets mode to 'decrypt'"))
       .arg(Arg::with_name("compress")
           .short("c")
           .long("compress")
           .help("Enables file compression"))
       .arg(Arg::with_name("inplace")
           .short("i")
           .long("inplace")
           .help("Will encrypt/compress all files inplace"));

   let matches = app.get_matches();

}