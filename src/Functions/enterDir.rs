use libdeflater::CompressionLvl;
use crate::Mode;
use std::{fs, thread};
use crate::Functions::encodeFile::encodeFile;
use crate::Functions::decodeFile::decodeFile;
use std::time::Duration;
use std::fs::File;


enum fileType {
    Folder,
    File
}

pub fn enterDir(pathDir: String, compress: CompressionLvl, depth: i32, mode: Mode) {
    let mut paths = fs::read_dir(pathDir.clone()).unwrap();
    for path in paths {
        let mut fileName = path.as_ref().unwrap().clone().file_name().into_string().unwrap();
        let isFile = path.as_ref().unwrap().clone().file_type().unwrap().is_file();
        let isDir = path.as_ref().unwrap().clone().file_type().unwrap().is_dir();
        let mut path = fileName.clone();

        if depth > 0 {
            path = format!("{}/{}", pathDir, fileName);
        } else {
            // this is where the data will be stored
            fs::write("protected.crypt", "");
        }

        if isFile && fileName != "Crypt.exe" {
            match mode {
                Mode::Encode => {
                    encodeFile(path,  "protected.crypt".to_owned(), fileName, compress);
                }
                Mode::Decode => {
                        //decodeFile(path);
                }
            }
        } else if isDir {
            enterDir(path.clone(), compress, depth + 1, mode);

            // remove dir when done
            fs::remove_dir(path);
        } else { // unable to encrypt file
            println!("[ERROR] Unable to encode / encrypt: {}", fileName);
        }

    }
}





