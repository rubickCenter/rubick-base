use lzma_rs;
use std::io::Write;

#[allow(dead_code)]
pub fn lzma_decompress(fromfile: &str, tofile: &str) -> Result<(), lzma_rs::error::Error> {
    // load
    let mut ff = std::io::BufReader::new(std::fs::File::open(fromfile)?);
    let mut decomp: Vec<u8> = Vec::new();
    // decompress
    lzma_rs::lzma2_decompress(&mut ff, &mut decomp)?;
    // dump
    let mut ft = std::io::BufWriter::new(std::fs::File::open(tofile)?);
    ft.write_all(&decomp)?;
    Ok(())
}

#[allow(dead_code)]
pub fn lzma_compress(fromfile: &str, tofile: &str) -> Result<(), lzma_rs::error::Error> {
    // load
    let mut ff = std::io::BufReader::new(std::fs::File::open(fromfile)?);
    let mut decomp: Vec<u8> = Vec::new();
    // decompress
    lzma_rs::lzma2_compress(&mut ff, &mut decomp)?;
    // dump
    let mut ft = std::io::BufWriter::new(std::fs::File::open(tofile)?);
    ft.write_all(&decomp)?;
    Ok(())
}
