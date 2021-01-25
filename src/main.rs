use std::env;
use std::fs;
use std::str;
use std::io::Cursor;
extern crate base64;
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


   // load user input into variables
   let encrypt = matches.is_present("encrypt");
   let decrypt = matches.is_present("decrypt");
   let compress = matches.is_present("compress");
   let inplace = matches.is_present("inplace");
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

   enterDir(String::from(path), inplace, compress, 0);
   println!("Done!");
   loop {}
}



fn enterDir(pathDir: String, inplace: bool, compress: bool, depth: i32) {
   let mut paths = fs::read_dir(pathDir.clone()).unwrap();

   for path in paths {

      let mut fileName = path.as_ref().unwrap().clone().file_name().into_string().unwrap();
      let isFile = path.as_ref().unwrap().clone().file_type().unwrap().is_file();
      let isDir = path.as_ref().unwrap().clone().file_type().unwrap().is_dir();
      let mut path = fileName.clone();

      if depth > 0 {
         path = format!("{}/{}", pathDir, fileName);
      }

      if isFile {
         encryptFile(path, fileName, inplace, compress);
      } else if isDir {
         println!("Entering Directory: {}", fileName);
         enterDir(path, inplace, compress, depth+1);

         // replace folder with encrypted data to continue
         if !inplace {

         }
      } else { // unable to encrypt file
         println!("[ERROR] Unable to encrypt: {}", fileName);
      }
   }
}

fn encryptFile(pathDir: String, fileName: String, inplace: bool, compress: bool) {
   println!("Encrypting File: {}", pathDir);

   // get buffer from file
   let dataBuffer = fs::read(pathDir).unwrap();

   // convert to base64
   let mut b64data = base64::encode(dataBuffer);

   // append filename to end
   b64data = format!("{},{}", b64data, fileName);

   println!("{}", b64data)
}