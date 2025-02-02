use num_bigint::{BigUint, BigInt, RandBigInt};
use rand::thread_rng;
use num_traits::{One, Zero};
use num_traits::Signed;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::time::Instant;
// use num_prime::nt_funcs::is_prime;
// use num_integer::Integer;
// use std::cmp::Ordering;


fn generate_large_prime(bit_size: usize) -> BigInt {
    let mut rng = thread_rng();
    loop {
        // Generate a random number with the specified bit size
        let num: BigUint = rng.gen_biguint(bit_size as u64);
        
        // Check if the number is prime using the Miller-Rabin test
        if miller_rabin_test(&num, 5) { // K is iteration value, can be increased for more accuracy.
            return BigInt::from(num);
        }
    }
}

fn miller_rabin_test(n: &BigUint, k: usize) -> bool {
    if n.is_zero() || n.is_one() {
        return false;
    }
    if n == &BigUint::from(2u32) || n == &BigUint::from(3u32) {
        return true;
    }
    if n % 2u32 == BigUint::zero() {
        return false;
    }

    // Decompose n-1 into d * 2^s
    let mut d = n - BigUint::one();
    let mut s = 0;
    while &d % 2u32 == BigUint::zero() {
        d /= 2u32;
        s += 1;
    }

    let mut rng = thread_rng();
    for _ in 0..k {
        let a = rng.gen_biguint_range(&BigUint::from(2u32), &(n - 2u32));
        let mut x = a.modpow(&d, n);

        if x == BigUint::one() || x == n - BigUint::one() {
            continue;
        }

        let mut is_witness = true;
        for _ in 0..s - 1 {
            x = x.modpow(&BigUint::from(2u32), n);
            if x == n - BigUint::one() {
                is_witness = false;
                break;
            }
        }

        if is_witness {
            return false; // n is composite
        }
    }

    true // n is probably prime
}
    


fn find_d(a: BigInt, m: BigInt) -> BigInt {
    let mut old_r = a.clone();
    let mut r = m.clone();
    let mut old_s = BigInt::one();
    let mut s = BigInt::zero();

    while r > BigInt::zero() {
        // BigInt division truncates, which is what we want.
        let quot = &old_r / &r;

        let temp_r = r.clone();
        r = &old_r - &quot * &r; // alternatively, r = oldR % r;
        old_r = temp_r;

        let temp_s = s.clone();
        s = &old_s - &quot * &s;
        old_s = temp_s;
    }

    // Ensure the result is non-negative
    if old_s < BigInt::zero() {
        old_s += &m;
    }

    return  BigInt::from(old_s);
}

fn manual_gcd(a: BigInt, b: BigInt) -> BigInt {
    let mut a = a.abs();
    let mut b = b.abs();
    while b != BigInt::zero() {
        let temp = b.clone();
        b = a % b;
        a = temp;
    }
    a
}
fn initilization(e: &BigInt)->(BigInt, BigInt, BigInt, BigInt, BigInt, BigInt){


    let bit_size = 1024;
    
    // Generate the first 1024-bit prime number
    let prime1 = generate_large_prime(bit_size);
    let prime_1= prime1.clone();
    println!("First 1024-bit prime number: {}", prime1);
    
    // Generate the second 1024-bit prime number
    let prime2 = generate_large_prime(bit_size);
    let prime_2= prime1.clone();
    println!("Second 1024-bit prime number: {}", prime2);
    let n= &prime1*& prime2;
    let p_1= prime1-1u32;
    let q_1= prime2-1u32;
    // let gcd = p_1.gcd(&q_1);
    let phi_n= &p_1*&q_1;
    let psi_gcd= manual_gcd(p_1, q_1);
    let psi_n= &phi_n/psi_gcd.clone();
    
    // let psi_n = &phi_n/gcd;
    let _coprime_check = manual_gcd(phi_n.clone(), e.clone());
    if _coprime_check == BigInt::one() {
        println!("phi_n and e are coprime (gcd = 1)");
        return (prime_1, prime_2, phi_n, psi_n, psi_gcd, n );
    } else {
        println!("Not coprime. Initializing again.");
        return initilization(&e) ;// Recursive call
    }
    // return (prime1, prime2, phi_n, psi_n, psi_gcd,n);
}


// fn big_int_pow(mut base: BigInt, mut exp: BigInt) -> BigInt {
//     if exp.is_zero() {
//         return BigInt::one();
//     }
//     let mut result = BigInt::one();

//     while !exp.is_zero() {
//         if &exp % 2u32 == BigInt::one() {
//             result *= &base;
//         }
//         exp /= 2u32;
//         base = &base * &base; // Avoid mutable and immutable borrow conflict
//     }

//     result
// }

//Ecryption function
fn encrypt(m: BigInt, e: &BigInt, n: &BigInt) -> BigInt {
    m.modpow(e, n)
}
// Decryption function
fn decrypt(c: BigInt, d: &BigInt, n: &BigInt) -> BigInt {
   
    c.modpow(d, n)
}

// fn decrypt(b: BigInt, e: BigInt, n: &BigInt) -> BigInt {
//     let mut res = BigInt::one(); // Initialize result to 1
//     let mut b = b % &n; // Reduce base modulo n
//     let mut e = e; // Copy the exponent

//     while e > BigInt::zero() {
//         if &e % 2u32 == BigInt::one() {
//             res = (res * &b) % &n; // Multiply result by base modulo n
//         }
//         e = e >> 1; // Divide exponent by 2 (right shift)
//         b = (&b * &b) % &n; // Square base modulo n
//     }

//     res
// }

fn main() {
    
    let e = BigInt::from(3u32); //3, 17, 65537, Fermit primes 
    // initializes two large prime number and recieves, phi_n, psi_n
    let (prime1, prime2, phi_n, psi_n, psi_gcd,n)= initilization(&e);

    

    
    //calculation of D1
    println!("value of phi_n{}",phi_n);
    let d_1= find_d(e.clone(),phi_n.clone());
    println!("value of d1: {:?}", &d_1);
    
    //calculation of D2
    println!("The value of psi_n={}", psi_n);
    let d_2 = find_d(e.clone(), psi_n.clone());
    println!("value of d2:{:?}", &d_2);

    // let statement= String::new();

    //checking which one is larger between D1 and D2
    match d_1.cmp(&d_2) {
        std::cmp::Ordering::Greater => println!("d_1 is greater than d_2"),
        std::cmp::Ordering::Less => println!("d_1 is less than d_2"), 
        std::cmp::Ordering::Equal => println!("d_1 is equal to d_2"),
    }

    //plaintext
    let m = BigInt::from(65931u32);
    println!("encrypting plaintext={}", &m);

    // standard RSA encrypting
    let s_rsa_start_encrypt = Instant::now();
    let s_rsa_ciphertext = encrypt(m.clone(), &e, &n);
    let s_rsa_encrypt_time = s_rsa_start_encrypt.elapsed();
    println!("ciphertext is {}", s_rsa_ciphertext);
    println!("encryption time: {:?}", s_rsa_encrypt_time);

    // Standard RSA decrypting
    println!("decrypting with d_1");
    let s_rsa_decrypt = Instant::now();//start timer
    let s_rsa_plaintext = decrypt(s_rsa_ciphertext.clone(), &d_1, &n);
    let s_rsa_decrypt_time = s_rsa_decrypt.elapsed();//stop timer
    println!("decryption time: {:?}", s_rsa_decrypt_time);
    println!("Decrypted message: {}", s_rsa_plaintext);

    //LCM RSA decrypting
    println!("now decrypting with d_2");
    let lcm_rsa_decrypt = Instant::now();//start timer
    let lcm_rsa_plaintext = decrypt(s_rsa_ciphertext.clone(), &d_2, &n);
    let lcm_rsa_decrypt_time = lcm_rsa_decrypt.elapsed();//stop timer
    println!("decryption time: {:?}", lcm_rsa_decrypt_time);
    println!("Decrypted message: {}", lcm_rsa_plaintext);

    
   
    let mut file = OpenOptions::new()
        .create(true) // Create the file if it doesn't exist
        .append(true) // Open the file in append mode
        .open("output.txt")
        .expect("Failed to open file");
        writeln!(file, "first prime(P):\t {}", prime1).expect("Failed to write to file");
        writeln!(file, "second prime(P):\t {}", prime2).expect("Failed to write to file");
        writeln!(file, "-----------------------------").expect("Failed to write to file");
        writeln!(file, "phi_n Standard RSA:\t {}", phi_n).expect("Failed to write to file");
        writeln!(file, "psi_n LCM RSA:\t {}", psi_n).expect("Failed to write to file");
        writeln!(file, "d_1:\t {:?}", d_1).expect("Failed to write to file");
        writeln!(file, "d_2:\t {:?}", d_2).expect("Failed to write to file");
        writeln!(file, "++++++++++++++++++++++++++++").expect("Failed to write to file");
        writeln!(file, "Now encryption with plaintext: \t{}", m).expect("Failed to write to file");
        writeln!(file, "Ciphertext with Standard RSA:\t{}",s_rsa_ciphertext).expect("Failed to write to file");
        writeln!(file, "****************************").expect("Failed to write to file");
        writeln!(file, "Now decryption").expect("Failed to write to file");
        writeln!(file, "with D1 decryption time: \t{:?}",s_rsa_decrypt_time).expect("Failed to write to file");
        writeln!(file, "with D2 decryption time: \t{:?}",lcm_rsa_decrypt_time).expect("Failed to write to file");
        writeln!(file, "#############################################################################################################################################").expect("Failed to write to file");
        writeln!(file, "=============================================================================================================================================").expect("Failed to write to file");    
}