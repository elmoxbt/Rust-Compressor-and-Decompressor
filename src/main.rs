use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use flate2::Compression;
use std::env::args;
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;
use std::io::copy;
use std::io::Read;

fn main() -> std::io::Result<()>{
    if args().len() < 3 {
        eprintln!("Usage: 'source' 'target'");
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput, 
            "Missing required arguments"
        ));
    }

    // Measure the time taken for compression and decompression
    let start = Instant::now();
    
    // Open the input file for reading
    let mut input = BufReader::new(File::open(&args().nth(1).ok_or(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Missing input file argument"))?)?);
    let output_path = args().nth(2).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Missing target file argument"))?;
    let output = File::create(&output_path)?;
    let mut encoder = GzEncoder::new(output, Compression::default());
    copy(&mut input, &mut encoder)?;
    let output = encoder.finish()?;
    
    // Decompress the output file for verification
    let compressed_path = &output_path;
    let decompressed_file = File::open(compressed_path)?;
    let mut decoder: GzDecoder<File> = GzDecoder::new(decompressed_file)?;
    let mut verify_data = Vec::new();
    decoder.read_to_end(&mut verify_data)?;

    // Read the original file for comparison
    let original_file_path = args().nth(1).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "Missing source file argument"))?;
    let mut original_file = File::open(&original_file_path)?;
    let mut original_data = Vec::new();
    original_file.read_to_end(&mut original_data)?;

    // Print results
    println!("Original: {}\n", String::from_utf8_lossy(&original_data));
    println!("Decompressed: {}\n", String::from_utf8_lossy(&verify_data));
    println!("Target length: {}", output.metadata()?.len());
    println!("Elapsed: {:?}", start.elapsed());

    // Verify that the decompressed data matches the original data
    assert_eq!(original_data, verify_data);

     Ok(())
}




