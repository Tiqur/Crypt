use std::fs;
use crate::Functions::decompressBuffer::decompressBuffer;
use crate::Functions::compressBuffer::compressBuffer;
use std::io::Read;

pub fn decodeFile(path: String) {
    println!("Decrypting File: {}", path);

    // get file buffer
    let mut encodedCompressedBuffer = fs::read(&path).unwrap();

    // extract filename
    let indexOfComma = encodedCompressedBuffer.iter().rposition(|r| r == &b',').unwrap()+1;
    let bufferLen = encodedCompressedBuffer.len();
    let filename = &encodedCompressedBuffer.clone()[indexOfComma..bufferLen];

    // truncate to only hold file data
    encodedCompressedBuffer.truncate(indexOfComma-1);

    // decode from base64
    let compressedBuffer = base64::decode(encodedCompressedBuffer).unwrap();

    // decompress buffer
    let dataBuffer = decompressBuffer(compressedBuffer);

    // convert filename to str
    let strFilename = std::str::from_utf8(filename).unwrap();
    fs::write(strFilename, dataBuffer);

    // delete old file
    fs::remove_file(path).unwrap();
}