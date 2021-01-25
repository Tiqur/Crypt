use libdeflater::Decompressor;

pub fn decompressBuffer(bytes: Vec<u8>) -> Vec<u8> {

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