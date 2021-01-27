use std::fs;
use libdeflater::CompressionLvl;
use crate::Functions::compressBuffer::compressBuffer;
use std::io::Write;


// https://stackoverflow.com/questions/37157926/is-there-a-method-like-javascripts-substr-in-rust
trait StringUtils {
    fn substring(&self, start: usize, len: usize) -> Self;
}

impl StringUtils for String {
    fn substring(&self, start: usize, len: usize) -> Self {
        self.chars().skip(start).take(len).collect()
    }
}

pub fn encodeFile(path: String, mut mergeFilePath: String, fileName: String, compress: CompressionLvl) {
    println!("Encoding File: {} -> {}.crypt", path, mergeFilePath.clone());

    // get buffer from file
    let mut dataBuffer = fs::read(&path).unwrap();

    // append filename to end of buffer
    dataBuffer.append(&mut Vec::from(format!(",{}", fileName)));

    // compress buffer
    let compressedBuffer = compressBuffer(dataBuffer.clone(), compress);

    // convert to base64
    let mut encodedCompressedBuffer = base64::encode(compressedBuffer);


    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(mergeFilePath)
        .unwrap();

    file.write_all(format!("{},", encodedCompressedBuffer).as_ref());

   //  delete old file
   fs::remove_file(path).unwrap();

}