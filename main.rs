use clap::Parser;
use num_bigint::{BigUint, RandomBits};
use num_traits::{One, Zero};
use num_primes::Generator;
use rand::Rng;
use rayon::prelude::*;
use std::cmp::min;

#[derive(Parser, Debug)]
#[command(author, version, about = "Custom RSA Key Generator")]
struct Args {
    /// RSA Key size in bits
    #[arg(long, default_value_t = 1024)]
    bits: usize,

    /// Maximum bit-length for the custom exponent 'e'
    #[arg(long)]
    max_e_bits: Option<usize>,

    /// Maximum attempts to find a valid 'e'
    #[arg(long, default_value_t = 1000)]
    max_attempts: usize,
}

fn main() {
    let args = Args::parse();
    
    // 1. Determine constraints
    let bits = args.bits;
    let max_e_bits = args.max_e_bits.unwrap_or_else(|| min(bits / 4, 256));
    
    println!("Generating {}-bit RSA key...", bits);
    println!("Searching for random 'e' up to {} bits...", max_e_bits);

    // 2. Generate Primes P and Q in parallel using the Rayon thread pool
    let (p, q) = rayon::join(
        || generate_prime(bits / 2),
        || generate_prime(bits / 2)
    );

    let n = &p * &q;
    let phi = (&p - BigUint::one()) * (&q - BigUint::one());

    // 3. Find a random 'e' using parallel attempts
    // We chunk the attempts to allow the thread pool to work efficiently
    let e = (0..args.max_attempts)
        .into_par_iter()
        .map(|_| {
            let mut rng = rand::thread_rng();
            let candidate_e: BigUint = rng.sample(RandomBits::new(max_e_bits as u64));
            
            // e must be > 1 and coprime to phi
            if candidate_e > BigUint::one() && gcd(candidate_e.clone(), phi.clone()) == BigUint::one() {
                Some(candidate_e)
            } else {
                None
            }
        })
        .find_first(|res| res.is_some())
        .flatten()
        .expect("Failed to find a valid 'e' within max-attempts. Try increasing max-attempts or changing bit size.");

    // 4. Calculate Private Exponent 'd'
    let d = mod_inverse(e.clone(), phi.clone()).expect("Failed to calculate modular inverse");

    println!("\n--- RSA Key Components ---");
    println!("N (Modulus): {}", n);
    println!("E (Public Exponent): {}", e);
    println!("D (Private Exponent): {}", d);
}

/// Generates a random prime of the specified bit length
fn generate_prime(bits: usize) -> BigUint {
    let p = Generator::new_prime(bits);
    BigUint::from_bytes_be(&p.to_bytes_be())
}

/// Standard Euclidean Algorithm to find GCD
fn gcd(mut a: BigUint, mut b: BigUint) -> BigUint {
    while !b.is_zero() {
        a %= &b;
        std::mem::swap(&mut a, &mut b);
    }
    a
}

/// Calculates the modular multiplicative inverse using Extended Euclidean Algorithm
fn mod_inverse(e: BigUint, phi: BigUint) -> Option<BigUint> {
    let mut mn = (phi.clone(), e);
    let mut xy = (BigUint::zero(), BigUint::one());
    let mut positive = (true, true); // Track signs manually for BigUint

    // This is a simplified version of the EEA for BigUint
    // In a production environment, use a library that handles signed BigInts
    // For this demonstration, we'll use a basic approach
    let e_signed = num_bigint::BigInt::from_biguint(num_bigint::Sign::Plus, mn.1.clone());
    let phi_signed = num_bigint::BigInt::from_biguint(num_bigint::Sign::Plus, phi.clone());
    
    use num_integer::ExtendedGcd;
    let egcd = num_integer::Integer::extended_gcd(&e_signed, &phi_signed);
    
    if egcd.gcd == num_bigint::BigInt::one() {
        let mut x = egcd.x;
        if x < num_bigint::BigInt::zero() {
            x += phi_signed;
        }
        x.to_biguint()
    } else {
        None
    }
}