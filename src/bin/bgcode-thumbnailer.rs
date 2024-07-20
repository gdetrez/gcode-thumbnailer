use byteorder::{ReadBytesExt, LE};
use std::{
    env, fs,
    io::{prelude::*, SeekFrom},
};
use anyhow::Result;

const EX_USAGE: i32 = 64;

fn main() -> Result<()> {
    let [_, input, output] = &std::env::args().collect::<Vec<_>>()[..] else {
        eprintln!("Usage: {} INPUT OUTPUT", env!("CARGO_BIN_NAME"));
        std::process::exit(EX_USAGE);
    };
    let input = input.trim_start_matches("file://");
    let output = output.trim_start_matches("file://");
    let mut file = fs::File::open(input)?;
    let checksum_type = parse_header(&mut file)?;
    loop {
        let thumb = parse_block(&mut file, checksum_type)?;
        if thumb.is_empty() {
            continue;
        }
        let mut ofile = fs::File::create(output)?;
        ofile.write_all(b"\x89PNG\r\n")?;
        ofile.write_all(&thumb[..])?;
        return Ok(());
    }
}

#[derive(Clone, Copy, Debug)]
enum ChecksumType {
    None,
    Crc32,
}

fn parse_header<R: Read>(input: &mut R) -> Result<ChecksumType> {
    let mut buf = [0; 4];
    input.read_exact(&mut buf)?;
    if &buf != b"GCDE" {
        anyhow::bail!("not bgcode: bad magic number");
    }
    let version = input.read_u32::<LE>()?;
    if version != 1 {
        anyhow::bail!("bgcode version not supported");
    }
    match input.read_u16::<LE>().unwrap() {
        0 => Ok(ChecksumType::None),
        1 => Ok(ChecksumType::Crc32),
        n => anyhow::bail!("unknown checksum type: {}", n),
    }
}

fn parse_block<R: Read + Seek>(
    input: &mut R,
    _checksum_type: ChecksumType,
) -> Result<Vec<u8>> {
    let block_type = input.read_u16::<LE>().unwrap();
    match block_type {
        0 => println!("File Metadata Block"),
        1 => println!("GCode Block"),
        2 => println!("Slicer Metadata Block"),
        3 => println!("Printer Metadata Block"),
        4 => println!("Print Metadata Block"),
        5 => println!("Thumbnail Block"),
        n => anyhow::bail!("unknown block type: {n}"),
    }
    let parameters_size = if block_type == 5 { 6 } else { 2 };
    let compression = input.read_u16::<LE>().unwrap();
    match compression {
        0 => println!("No compression"),
        1 => println!("Deflate algorithm"),
        2 => println!("Heatshrink algorithm with window size 11 and lookahead size 4"),
        3 => println!("Heatshrink algorithm with window size 12 and lookahead size 4"),
        n => anyhow::bail!("unknown compression type: {n}"),
    }
    let uncompressed_size = input.read_u32::<LE>().unwrap();
    if block_type == 5 {
        assert_eq!(compression, 0, "compression not implemented");
        assert_eq!(parameters_size, 6);
        let format = input.read_u16::<LE>().unwrap();
        let _width = input.read_u16::<LE>().unwrap();
        let _height = input.read_u16::<LE>().unwrap();
        println!("FORMAT: {}", format);
        if format != 0 {
            // != png
            // Skip block
            input.seek(SeekFrom::Current(uncompressed_size as i64)).unwrap();
            input.seek(SeekFrom::Current(4)).unwrap();
            return Ok(vec![]);
        }
        input.seek(SeekFrom::Current(parameters_size)).unwrap();
        let mut dst = vec![0; uncompressed_size as usize];
        input.read_exact(&mut dst[..]).unwrap();
        input.seek(SeekFrom::Current(4)).unwrap();
        return Ok(dst);
    }
    // Skip block
    if compression == 0 {
        input.seek(SeekFrom::Current(parameters_size)).unwrap();
        input.seek(SeekFrom::Current(uncompressed_size as i64)).unwrap();
    } else {
        let compressed_size = input.read_u32::<LE>().unwrap();
        input.seek(SeekFrom::Current(parameters_size)).unwrap();
        input.seek(SeekFrom::Current(compressed_size as i64)).unwrap();
    }
    // Checksum
    input.seek(SeekFrom::Current(4)).unwrap();
    Ok(vec![])
}
