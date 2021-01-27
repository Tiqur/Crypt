use std::env;
use std::fs;
use std::str;
use std::io::{Cursor, Write};
extern crate base64;
extern crate clap;
extern crate libdeflater;
use clap::{Arg, App};
use libdeflater::{Compressor, CompressionLvl, Decompressor};

mod Functions;
use crate::Functions::enterDir::enterDir;
use crate::Functions::decodeFile::decodeFile;


#[derive(Clone, Copy)]
pub enum Mode {
   Encode,
   Decode
}

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
       .arg(Arg::with_name("compression")
           .short("c")
           .long("compression")
           .multiple(true)
           .help("Sets the level of compression"));

   let matches = app.get_matches();


   // load user input into variables
   let encrypt = matches.is_present("encrypt");
   let decrypt = matches.is_present("decrypt");
   let compress = matches.is_present("compress");
   let mut path: &str = "";

   // if path, execute on path
   if matches.is_present("path") {
      path = matches.value_of("path").unwrap();
   }

   if encrypt && decrypt {
      println!("Invalid user input: Cannot encrypt and decrypt at same time.");
      return;
   }

   if encrypt {

   } else {

   }


   // this is where the data will be stored
   let mut file = fs::OpenOptions::new()
       .append(true)
       .create(true)
       .open("protected.crypt")
       .unwrap();


   let compressionLevel = CompressionLvl::best();
   //enterDir(path.to_owned(), &mut file, compressionLevel, 0, Mode::Encode);
   let tempPath = "protected.crypt";
   decodeFile(tempPath.to_owned());

   println!("Done!");
}
