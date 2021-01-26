use libdeflater::CompressionLvl;
use crate::Mode;
use std::fs;
use crate::Functions::encryptFile::encryptFile;
use crate::Functions::decryptFile::decryptFile;

pub fn enterDir(pathDir: String, compress: CompressionLvl, depth: i32, mode: Mode) {
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


        if isFile && fileName != "Crypt.exe" {
            match mode {
                Mode::Encrypt => {
                    fileIndex+=1;
                    encryptFile(path, fileName, compress, fileIndex);
                }
                Mode::Decrypt => {
                    if fileName.ends_with(".crypt") {
                        decryptFile(path);
                    }
                }
                Mode::Merge => {}
            }
        } else if isDir {
            println!("Entering Directory: {}", fileName);
            enterDir(path.clone(), compress, depth + 1, mode);
        } else { // unable to encrypt file
            println!("[ERROR] Unable to encrypt: {}", fileName);
        }



        // match mode {
        //     Mode::Encrypt => {
        //         if isFile && fileName != "Crypt.exe" {
        //             fileIndex+=1;
        //             encryptFile(path, fileName, compress, fileIndex);
        //         } else if isDir {
        //             println!("Entering Directory: {}", fileName);
        //             enterDir(path.clone(), compress, depth + 1, Mode::Encrypt);
        //         } else { // unable to encrypt file
        //             println!("[ERROR] Unable to encrypt: {}", fileName);
        //         }
        //     }
        //     Mode::Decrypt => {}
        //     Mode::Merge => {}
        // }

    }
}