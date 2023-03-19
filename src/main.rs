use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
use flate2::{Compression, write::GzEncoder};

fn main() {
    let input_file = "input.txt";
    let output_file = "compressed.gz";

    if let Err(e) = compress_file(input_file, output_file) {
        eprintln!("Error compressing file: {}", e);
    } else {
        println!("File compressed successfully.");
    }
}

fn compress_file(input_path: &str, output_path: &str) -> std::io::Result<()> {
    let input_file = File::open(input_path)?;
    let mut reader = BufReader::new(input_file);

    let output_file = File::create(output_path)?;
    let mut writer = BufWriter::new(output_file);
    let mut encoder = GzEncoder::new(writer, Compression::default());

    let mut buffer = [0; 4096];
    loop {
        let read_bytes = reader.read(&mut buffer)?;
        if read_bytes == 0 {
            break;
        }
        encoder.write_all(&buffer[..read_bytes])?;
    }

    encoder.finish()?;
    Ok(())
}
