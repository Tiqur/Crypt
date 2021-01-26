use std::fs;
use libdeflater::CompressionLvl;
use crate::Functions::compressBuffer::compressBuffer;

pub fn encryptFile(path: String, fileName: String, compress: CompressionLvl, fileIndex: i32) {
    println!("Encrypting File: {}", path);

    // get buffer from file
    let dataBuffer = fs::read(&path).unwrap();

    // compress buffer
    let compressedBuffer = compressBuffer(dataBuffer.clone(), compress);

    // convert to base64
    let mut encodedCompressedBuffer = base64::encode(compressedBuffer);

    // append filename to end
    encodedCompressedBuffer = format!("{},{}", encodedCompressedBuffer, fileName);


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

    fs::write(newFileName, &encodedCompressedBuffer);

    // delete old file
    fs::remove_file(path).unwrap();

}