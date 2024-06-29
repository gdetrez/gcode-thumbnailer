use std::{env, error::Error, fs};

const EX_USAGE: i32 = 64;

fn main() -> Result<(), Box<dyn Error>> {
    let [_, input, output] = &std::env::args().collect::<Vec<_>>()[..] else {
        eprintln!("Usage: {} INPUT OUTPUT", env!("CARGO_BIN_NAME"));
        std::process::exit(EX_USAGE);
    };
    let input = input.trim_start_matches("file://");
    let output = output.trim_start_matches("file://");
    let file = fs::File::open(input)?;
    let mut zip = zip::ZipArchive::new(file)?;
    let mut thumbnail = zip.by_name("Metadata/thumbnail.png")?;
    std::io::copy(&mut thumbnail, &mut fs::File::create(output)?)?;
    Ok(())
}
