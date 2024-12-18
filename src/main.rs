mod primes;
mod math;

use clap::Parser;
use base64::{engine::general_purpose, Engine as _};
use num::{bigint::ToBigInt, BigInt};
use num_bigint::Sign;
use primes::Bint;
use core::str;
use std::{env, fs::File, io::{Read, Write}};


#[derive(Parser)]
#[command(name = "ALGOLAB: RSA Key Generator")]
#[command(about = "One of the most efficien RSA-tools known to mankind.", version = "1.0", author = "Riku")]
struct Cli {
    /// Command to generate key pair
    #[arg(short, long)]
    generate_keys: bool,

    /// Input file to encrypt (must exist in the same directory)
    #[arg(short, long)]
    file_name: Option<String>,

    /// Output file name to save the encrypted content
    #[arg(short, long)]
    output: Option<String>,

    /// Encrypted file to decrypt 
    #[arg(short, long)]
    encrypted_file: Option<String>,

    /// Output file name to save the decrypted content
    #[arg(short, long)]
    decrypted_output: Option<String>,
}


const PUBLIC_KEY_FILE_NAME: &str = "public_key.txt";
const PRIVATE_KEY_FILE_NAME: &str = "private_key.txt";

fn main() {
    //TODO: Make this more user-friendly - seperate commands instead of param-diff
    env::set_var("RUST_BACKTRACE", "1");

    let cli = Cli::parse();

    // --generate-keys --> generate and save the keys
    if cli.generate_keys {
        let (public_key, private_key) = generate_key_pair();
        println!("Keys generated!");
        save_keys_to_files(public_key, private_key);
    } 
    
    //First iteration of cli - clunkily check if certain optional values are passed and execute based on that
    // File encryption
    if let (Some(input_file), Some(output_file)) = (cli.file_name, cli.output) {
        println!("Encrypting the file: {} to {}", input_file, output_file);
        encrypt_file(&input_file, &output_file);
    }
    // File decryption
    if let (Some(encrypted_file), Some(decrypted_output)) = (cli.encrypted_file, cli.decrypted_output) {
        println!("Decrypting the file: {} to {}", encrypted_file, decrypted_output);
        decrypt_file(&encrypted_file, &decrypted_output);
    }
    
}


fn construct_public_key(n: BigInt, e: BigInt) -> (BigInt, BigInt) {
    return (n, e)
}

fn construct_private_key(n: BigInt, d: BigInt) -> (BigInt, BigInt) {
    return (n, d)
}

fn generate_key_pair() -> ((BigInt, BigInt), (BigInt, BigInt)){
    let (p1, p2) = primes::get_primes();
    let n = p1.clone() * p2.clone();
    let totient = math::euler_totient(p1.clone(), p2.clone());
    let e = Bint!(65537);
    let d = math::modular_multip_inverse(e.clone(), totient);
    let public_key = construct_public_key(n.clone(), e.clone());
    let private_key = construct_private_key(n.clone(), d.clone());
    return (public_key, private_key)
}


//TODO: Duplicate code, can be cleaned
fn encrypt_file(input_file: &str, output_file: &str) {
    // Harcoded for now: Load the public key from "public_key.txt"
    let (n, e) = load_public_key_from_file(PUBLIC_KEY_FILE_NAME);
    let mut file = File::open(input_file).expect("Failed to open input file, check that it exists.");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Failed to read input file");
    let encrypted_message = encrypt(&contents, (n, e));
    let mut output = File::create(output_file).expect("Failed to create output file");
    write!(output, "{}", encrypted_message).expect("Failed to write encrypted content");

    println!("Encrypted contents saved to '{}'", output_file);
}

fn decrypt_file(encrypted_file: &str, output_file: &str) {
    // Harcoded for now: Load the public key from "private_key.txt"
    let (n, d) = load_private_key_from_file(PRIVATE_KEY_FILE_NAME);
    let mut file = File::open(encrypted_file).expect("Failure to open encrypted file, make sure it exists.");
    let mut encrypted_contents = String::new();
    file.read_to_string(&mut encrypted_contents).expect("Failure to read encrypted file");
    let decrypted_message = decrypt(&encrypted_contents, (n, d));
    let mut output = File::create(output_file).expect("Failed to create output file");
    write!(output, "{}", decrypted_message).expect("Failed to write decrypted content");

    println!("Decrypted contents saved to '{}'", output_file);
}

fn load_public_key_from_file(filename: &str) -> (BigInt, BigInt) {
    let mut file = File::open(filename).expect("Unable to open public key file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read public key file");

    // Parse public key - this needs better error msg funcs
    let parts: Vec<&str> = contents.split(": ").collect();
    let key_part = parts[1].trim_matches(|c| c == '(' || c == ')' || c == '\n').split(", ").collect::<Vec<&str>>();
    let n = BigInt::parse_bytes(key_part[0].as_bytes(), 10).unwrap();
    let e = BigInt::parse_bytes(key_part[1].as_bytes(), 10).unwrap();

    return (n, e)
}

// Load the private key from a file (for decryption)
fn load_private_key_from_file(filename: &str) -> (BigInt, BigInt) {
    let mut file = File::open(filename).expect("Unable to open private key file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read private key file");
    let parts: Vec<&str> = contents.split(": ").collect();
    let key_part = parts[1].trim_matches(|c| c == '(' || c == ')' || c == '\n').split(", ").collect::<Vec<&str>>();
    let n = BigInt::parse_bytes(key_part[0].as_bytes(), 10).unwrap();
    let d = BigInt::parse_bytes(key_part[1].as_bytes(), 10).unwrap();

    return (n, d)
}

pub fn encrypt(message: &str, public_key: (BigInt, BigInt)) -> String {

    let max_size = 255;
    let content_bytes = message.as_bytes();

    // Check if the message size exceeds the limit
    if content_bytes.len() > max_size {
        panic!(
            "Message size exceeds the RSA encryption limit! Maximum allowed is {} bytes, but got {} bytes.",
            max_size, content_bytes.len()
        );
    }
    //TODO: Consider using BigUint across application.
    let encoded = BigInt::from_bytes_be(Sign::Plus, message.as_bytes());
    //Encrypt via modpow by using public key parts as exponent and modulus
    let encrypted = encoded.modpow(&public_key.1, &public_key.0);
    let encrypted_bytes = encrypted.to_bytes_be().1;
    let encrypted_base64 = general_purpose::STANDARD.encode(encrypted_bytes);
    return encrypted_base64
}

pub fn decrypt(ciphertext: &str, private_key: (BigInt, BigInt)) -> String {
    let decoded_bytes = general_purpose::STANDARD.decode(ciphertext).expect("Invalid Base64 conversion");
    let decoded = BigInt::from_bytes_be(Sign::Plus, &decoded_bytes);
    let decrypted = decoded.modpow(&private_key.1, &private_key.0);
    let decrypted_bytes = decrypted.to_bytes_be().1;
    return String::from_utf8(decrypted_bytes).expect("Invalid UTF-8 byte-representation");
}

// Function to save keys to files
fn save_keys_to_files(public_key: (BigInt, BigInt), private_key: (BigInt, BigInt)) {
    let mut public_file = File::create("public_key.txt").expect("Unable to create public key file");
    write!(public_file, "Pub-Key: {:?}\n", public_key).expect("Unable to write public key");

    let mut private_file = File::create("private_key.txt").expect("Unable to create private key file");
    write!(private_file, "Priv-Key: {:?}\n", private_key).expect("Unable to write private key");

    println!("Public key saved to 'public_key.txt'");
    println!("Private key saved to 'private_key.txt'");
}


//Initial tests for prime-gen checking
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prime_generation() {
        let some_known_primes: [i32; 5] = [11, 13, 443, 1289, 2027];
        for prime in some_known_primes {
            let is_p = primes::is_prime(&primes::Bint!(prime));
            assert_eq!(is_p, true);
         }
         let some_non_primes: [i32; 5] = [33, 68, 559, 1120, 4880];
         for non_prime in some_non_primes {
            let is_p = primes::is_prime(&primes::Bint!(non_prime));
            assert_eq!(is_p, false);
         }
    }
    
    #[test]
    fn test_is_rb_prime_with_300_digit_primes() {
        // Using two known large primes, that closely match the scale we use in this application
        let large_primes = vec![
            BigInt::parse_bytes(b"245332586188658183944499887005401082192662104197862402666504581444741980802463343669777652960150365572686150406028449000402715114148932314861443501702815341764211457722702561071759672876952376140221899408388109049581877397848130603301357443383926859437040070405422466207856822897209735260983241546009", 10).unwrap(),
            BigInt::parse_bytes(b"980269168439402415976282861949874043634013156920705880010086326521936806687754187106313049210341316079327871960377290671695123027195996409794020335351913447952704592856671242982287613805794745292858748242832923204848198742658586084574052571703180373739521293005052367505797637575449840521470134313523", 10).unwrap(),
        ];

        // Same for large non-primes, including numbers that only have 2 large primes as factors
        let large_non_prime_w_primes_as_factor = &large_primes[0] * &large_primes[1];
        let large_non_primes = vec![
            BigInt::parse_bytes(b"445332586188658183944499887005401082192662104197862402666504581444741980802463343669777652960150365572686150406028449000402715114148932314861443501702815341764211457722702561071759672876952376140221899408388109049581877397848130603301357443383926859437040070405422466207856822897209735260983241546009", 10).unwrap(),
            BigInt::parse_bytes(b"480269168439402415976282861949874043634013156920705880010086326521936806687754187106313049210341316079327871960377290671695123027195996409794020335351913447952704592856671242982287613805794745292858748242832923204848198742658586084574052571703180373739521293005052367505797637575449840521470134313523", 10).unwrap(),
            large_non_prime_w_primes_as_factor,
        ];

        for prime in large_primes {
            assert!(primes::is_rb_prime(&prime), "Failed to verify very large prime: {}", prime);
        }

        
        for non_prime in large_non_primes {
            assert!(!primes::is_rb_prime(&non_prime), "Incorrectly identified non-prime as prime: {}", non_prime);
        }
    }
    

    #[test]
    fn test_encryption_decryption() {
        let (public_key, private_key) = generate_key_pair();
        let original_message = "Algolabra Test String";
        let encrypted_message = encrypt(original_message, public_key);
        let decrypted_message = decrypt(&encrypted_message, private_key);
        assert_eq!(original_message, decrypted_message, "The decrypted message does not match the original one.");
    }
    
    #[test]
    fn test_encryption_size_limit() {
        let (public_key, private_key) = generate_key_pair();
        //Init content just under 2048 bits (256 bytes)
        let message_within_limit = "X".repeat(255); 
        let encrypted_message_within = encrypt(&message_within_limit, public_key.clone());
        let decrypted_message_within = decrypt(&encrypted_message_within, private_key.clone());
        assert_eq!(message_within_limit, decrypted_message_within, "Decryption failed for message within limit.");
         //Init 257 bytes string -> just over the limit
        let message_over_limit = "X".repeat(257); 
    
        // Expect this to panic 
        let result = std::panic::catch_unwind(|| encrypt(&message_over_limit, public_key.clone()));
        // Check if encryption fails(as expected)
        assert!(result.is_err(), "Encryption should fail for message over the limit.");
    }
}