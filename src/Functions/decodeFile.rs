use std::fs;
use crate::Functions::decompressBuffer::decompressBuffer;
use crate::Functions::compressBuffer::compressBuffer;
use std::io::Read;

pub fn decodeFile(path: String) {
    println!("Decoding File: {}", path);

    // get file buffer
    let mut encodedCompressedBuffer = fs::read(&path).unwrap();

    // decode from base64
    let compressedBuffer = base64::decode(&encodedCompressedBuffer).unwrap();

    // decompress buffer
    let mut dataBuffer = decompressBuffer(compressedBuffer);

    // extract filename
    let indexOfComma = dataBuffer.iter().rposition(|r| r == &b',').unwrap()+1;
    let filename = &dataBuffer.clone()[indexOfComma..dataBuffer.len()];

    // truncate to only hold file data
    dataBuffer.truncate(indexOfComma-1);

    // convert filename to str
    let strFilename = std::str::from_utf8(filename).unwrap();
    fs::write(strFilename, dataBuffer);

    // delete old file
    fs::remove_file(path).unwrap();
}