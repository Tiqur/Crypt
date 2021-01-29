use std::fs;
use crate::Functions::decompressBuffer::decompressBuffer;
use crate::Functions::compressBuffer::compressBuffer;
use std::io::Read;

pub fn decodeFile(path: String) {

    // get file buffer
    let mut fileString = fs::read_to_string(&path).unwrap();

    // removes hanging comma from file
    fileString.pop();

    // split data into chunks that represent files
    let encodedFiles = fileString.split(",");

    // write each file
    for encodedFile in encodedFiles {
        // convert to bytes
        let encodedFileBytes = encodedFile.as_bytes();

        // decode from base64
        let compressedBuffer = base64::decode(&encodedFileBytes).unwrap();

        // decompress buffer
        let mut dataBuffer = decompressBuffer(compressedBuffer);

        // extract path with name
        let indexOfComma = dataBuffer.iter().rposition(|r| r == &b',').unwrap()+1;
        let filePath = &dataBuffer.clone()[indexOfComma..dataBuffer.len()];
        let fileNameDelimiterIndex = filePath.iter().rposition(|r| r == &b'/').unwrap();
        let filePathWithoutName = String::from_utf8(Vec::from(&filePath.clone()[0..fileNameDelimiterIndex])).unwrap();

        println!("Decoding File: {}",  String::from_utf8(Vec::from(filePath)).unwrap());

        // truncate to only hold file data
        dataBuffer.truncate(indexOfComma-1);

        // convert filename to str
        let strPath = std::str::from_utf8(filePath).unwrap();

        // create directory if needed
        fs::create_dir_all(filePathWithoutName);

        // write file
        fs::write(strPath, dataBuffer);
    }

    // delete old file
    fs::remove_file(path).unwrap();
}