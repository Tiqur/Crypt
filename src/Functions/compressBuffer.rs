use libdeflater::{CompressionLvl, Compressor};

pub fn compressBuffer(bytes: Vec<u8>, compLevel: CompressionLvl) -> Vec<u8> {
    let mut compressor = Compressor::new(compLevel);
    let max_sz = compressor.gzip_compress_bound(bytes.len());
    let mut compressed_data = Vec::new();
    compressed_data.resize(max_sz, 0);
    let actual_sz = compressor.gzip_compress(&bytes, &mut compressed_data).unwrap();
    compressed_data.resize(actual_sz, 0);
    return compressed_data;
}

