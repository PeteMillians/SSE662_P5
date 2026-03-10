use std::env;
use std::fs;
use std::io::{self, Write};

fn xor_cipher(data: &[u8]) -> Vec<u8> {

    /*
    Computes the XOR Cipher algorithm on the data on a slice of bytes in memory

    Arguments:
        data (&[u8]): the memory address of the slice of an 8-bit unsigned integer referencing the input file bytes
    Returns:
        a vector of 8-bit unsigned integers containing encrypted bytes
    */

    // Hard-coded static encryption key
    let key = b"deadbeef";
    
    // XOR cipher algorithm (symmetric)
    data.iter() // Iterates through the data slice
        .enumerate()    // Enumerates the bytes in the slice into indices
        .map(|(i, &byte)| byte ^ key[i % key.len()]) // Apply the XOR cipher to each byte
        .collect()  // Collect the encrypted bytes and return them

}

fn main() -> io::Result<()> {
    /*
    Main method which reads command-line arguments at runtime and uses the XOR cipher to encrypt the data
    */

    // Collect command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure the required command-line arguments were input
    if args.len() != 3 {
        eprintln!("Usage: cargo run <input_file> <output_file>");
        std::process::exit(1);
    }

    // Read input file
    let input_data : Vec<u8> = fs::read(&args[1])?;

    // Encrypt or decrpy the file
    let encrypted : Vec<u8> = xor_cipher(&input_data);

    // Create the file at the output path
    let mut file = fs::File::create(&args[2])?;

    // Write the encryped data to the output path
    file.write_all(&encrypted)?;

    println!("Encrypted file written to {}", &args[2]);

    Ok(())

}
