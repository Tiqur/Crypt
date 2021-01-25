use std::env;
use std::fs;
use std::str;
use std::io::{Cursor, Write};
extern crate base64;
extern crate clap;
extern crate libdeflater;
use clap::{Arg, App};
use libdeflater::{Compressor, CompressionLvl, Decompressor};

enum Mode {
   Encrypt,
   Decrypt,
   Merge
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
           .help("Sets the level of compression"))
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

   enterDir(String::from(path), CompressionLvl::best(), 0, Mode::Encrypt);
   enterDir(String::from(path), CompressionLvl::best(), 0, Mode::Merge);
   println!("Done!");
   loop {}
}


fn enterDir(pathDir: String, compress: CompressionLvl, depth: i32, mode: Mode) {
   let mut paths = fs::read_dir(pathDir.clone()).unwrap();
   let mut fileIndex = 0;
   for path in paths {
      let mut fileName = path.as_ref().unwrap().clone().file_name().into_string().unwrap();
      let isFile = path.as_ref().unwrap().clone().file_type().unwrap().is_file();
      let isDir = path.as_ref().unwrap().clone().file_type().unwrap().is_dir();
      let mut path = fileName.clone();

      if depth > 0 {
         path = format!("{}/{}", pathDir, fileName);
      }

      match mode {
         Mode::Encrypt => {
            if isFile && fileName != "Crypt.exe" {
               fileIndex+=1;
               encryptFile(path, fileName, compress, fileIndex);
            } else if isDir {
               println!("Entering Directory: {}", fileName);
               enterDir(path.clone(), compress, depth + 1, Mode::Encrypt);
            } else { // unable to encrypt file
               println!("[ERROR] Unable to encrypt: {}", fileName);
            }
         }
         Mode::Decrypt => {}
         Mode::Merge => {}
      }

   }
}


fn compressBuffer(bytes: Vec<u8>, compLevel: CompressionLvl) -> Vec<u8> {
   let mut compressor = Compressor::new(compLevel);
   let max_sz = compressor.gzip_compress_bound(bytes.len());
   let mut compressed_data = Vec::new();
   compressed_data.resize(max_sz, 0);
   let actual_sz = compressor.gzip_compress(&bytes, &mut compressed_data).unwrap();
   compressed_data.resize(actual_sz, 0);
   return compressed_data;
}

fn decompressBuffer(bytes: Vec<u8>) -> Vec<u8> {

   let isize = {
      let isize_start = bytes.len() - 4;
      let isize_bytes = &bytes[isize_start..];
      let mut ret: u32 = isize_bytes[0] as u32;
      ret |= (isize_bytes[1] as u32) << 8;
      ret |= (isize_bytes[2] as u32) << 16;
      ret |= (isize_bytes[3] as u32) << 26;
      ret as usize
   };

   let mut decompressor = Decompressor::new();
   let mut outbuf = Vec::new();
   outbuf.resize(isize, 0);
   decompressor.gzip_decompress(&bytes, &mut outbuf).unwrap();
   return outbuf;
}



fn encryptFile(path: String, fileName: String, compress: CompressionLvl, fileIndex: i32) {
   println!("Encrypting File: {}", path);

   // get buffer from file
   let dataBuffer = fs::read(&path).unwrap();

   // compress buffer
   let compressedBuffer = compressBuffer(dataBuffer.clone(), compress);

   // convert to base64
   let mut b64data = base64::encode(compressedBuffer);

   // append filename to end
   b64data = format!("{},{}", b64data, fileName);


   /*
      ENCRYPT HERE
   */


   // https://stackoverflow.com/questions/37157926/is-there-a-method-like-javascripts-substr-in-rust
   trait StringUtils {
      fn substring(&self, start: usize, len: usize) -> Self;
   }

   impl StringUtils for String {
      fn substring(&self, start: usize, len: usize) -> Self {
         self.chars().skip(start).take(len).collect()
      }
   }

   // writing individual files to prevent potential running out of memory when dealing with EXTREMELY large folders
   // write encrypted contents to new file
   let mut newFileName = String::new();

   if path.rfind("/").unwrap_or(0) != 0 {
     newFileName.push_str(&path.substring(0, (path.rfind("/").unwrap()+1)))
   }

   newFileName.push_str(&*(fileIndex.to_string() + ".crypt"));

   fs::write(newFileName, &b64data);

   // delete old file
   fs::remove_file(path).unwrap();

}

