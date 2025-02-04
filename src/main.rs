use num_bigint::{BigUint, BigInt, RandBigInt};
use rand::thread_rng;
use num_traits::{One, Zero};
use num_traits::Signed;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::time::Instant;
use std::str::FromStr;
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
    


fn find_d(e: BigInt, n: BigInt) -> BigInt {
    let mut old_r = e.clone();
    let mut r = n.clone();
    let mut old_s = BigInt::one();
    let mut s = BigInt::zero();

    while r > BigInt::zero() {//will stop when r is 0
        
        let quot = &old_r / &r;

        let temp_r = r.clone();
        r = &old_r - &quot * &r; // r = oldR % r;
        old_r = temp_r;

        let temp_s = s.clone();
        s = &old_s - &quot * &s;
        old_s = temp_s;
    }

    // if d is negative
    if old_s < BigInt::zero() {
        old_s += &n;
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

fn pre_initialized(e: &BigInt)->(BigInt, BigInt, BigInt, BigInt, BigInt, BigInt){

 let p_str = "96558248052282147526391182286665006314117848345484525076355431840149439003724986445913635448855014940982565499416613932973592902322959421967957268300679284031279587288424292695941524786350152190906640105711127601822493983422519024768880179684079307556749885699494609222085795277721644743481467937637199325917";
 let q_str= "29980273465391173109616012765531821967603585050808878210552383945074469961561472918563426452752208707746778581193889965810674111027142816470507082291123959483256344772891966107830171652515293535021361888534788746146562715271056246135217036771706557912569717468616464261028846704797695270457321067742864025497";
 let p = BigInt::from_str(p_str).expect("Failed to parse number");
 let q = BigInt::from_str(q_str).expect("Failed to parse number");
 let n= &p*&q;
 println!("value of n: {}", n);
 let p_1= &p-1u32;
 let q_1= &q-1u32;
 let phi_n= &p_1*&q_1;
 let psi_gcd= manual_gcd(p_1, q_1);
 let psi_n= &phi_n/psi_gcd.clone();

 return (p, q, phi_n, psi_n, psi_gcd, n );
    
}

fn initilization(e: &BigInt)->(BigInt, BigInt, BigInt, BigInt, BigInt, BigInt){


    let bit_size = 1024;
    
    // Generate the first 1024-bit prime number
    let prime1 = generate_large_prime(bit_size);
    let prime_1= prime1.clone();
    println!("First 1024-bit prime number: {}", prime1);
    
    // Generate the second 1024-bit prime number
    let prime2 = generate_large_prime(bit_size);
    let prime_2= prime2.clone();
    println!("Second 1024-bit prime number: {}", prime2);
    let n= &prime1*& prime2;
    println!("value of n: {}",n);
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

fn decrypt_crt(c: &BigInt, d: &BigInt, p: &BigInt, q: &BigInt) -> BigInt {
    // Step 1: Precompute d_p and d_q
    let dp = d % (p - BigInt::one());
    let dq = d % (q - BigInt::one());

    // Step 2: Compute m_p and m_q
    let mp = c.modpow(&dp, p);
    let mq = c.modpow(&dq, q);

    // Step 3: Compute q_inv = q^{-1} mod p
    let q_inv = find_d(q.clone(), p.clone());

    // Step 4: Recombine using CRT
    let h = (&mp - &mq) * &q_inv % p;
    let m = mq + h * q;

    m
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
    let mut prime1= BigInt::from(0u32);
    let mut prime2= BigInt::from(0u32);
    let mut phi_n= BigInt::from(0u32);
    let mut psi_n= BigInt::from(0u32);
    let mut psi_gcd= BigInt::from(0u32);
    let mut n= BigInt::from(0u32);
    let mut _choice= 0;

    loop {
        // Display menu
        println!("Please choose an option:");
        println!("1. Run with preinitalized value");
        println!("2. Run with random value(Might take some time)");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap(); // Ensure the prompt is displayed immediately

        // Read user input
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");
        // let _choice= choice.clone();
        // Parse input as an integer
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        // Perform action based on choice
        match choice {
            1 => {
                println!("You chose Action 1.");
                // Call a function or perform an action for choice 1
                (prime1, prime2, phi_n, psi_n, psi_gcd,n)= pre_initialized(&e);
            }
            2 => {
                println!("You chose Action 2.");
                // Call a function or perform an action for choice 2
                (prime1, prime2, phi_n, psi_n, psi_gcd,n)= initilization(&e);
            }
            _ => {
                println!("Invalid choice. Please choose again.");
                continue;
            }
        }
        _choice=choice.clone();
        // Exit the loop after a valid choice is processed
        break;
    }

    // println!("1. try with fixed value\n 2. Try with Random value");
    // let mut choice = String::new();

    // initializes two large prime number and recieves, phi_n, psi_n
    // let (prime1, prime2, phi_n, psi_n, psi_gcd,n)= initilization(&e);

    

    
    //calculation of D1
    println!("value of phi_n{}",phi_n);
    let d_1= find_d(e.clone(),phi_n.clone());
    println!("value of d1: {:?}", &d_1);
    
    //calculation of D2
    println!("The value of psi_n={}", psi_n);
    let d_2 = find_d(e.clone(), psi_n.clone());
    println!("value of d2:{:?}", &d_2);

    // let statement= String::new();
    let mut ratio_d1_d2= BigInt::from(0u32);
    //checking which one is larger between D1 and D2
    match d_1.cmp(&d_2) {
        std::cmp::Ordering::Greater => println!("d_1 is greater than d_2"),
        std::cmp::Ordering::Less => println!("d_1 is less than d_2"), 
        std::cmp::Ordering::Equal => println!("d_1 is equal to d_2"),
    }

    if d_1>d_2{
        ratio_d1_d2=&d_1/&d_2;
        println!("d1 is {} times higher than d2", ratio_d1_d2);
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

    // using CRT for decryption_D1
    let crt_decrypt_d1= Instant::now();
    let decrypted_message_crt_d1 = decrypt_crt(&s_rsa_ciphertext, &d_1, &prime1, &prime2);
    let crt_decrypt_d1_time = crt_decrypt_d1.elapsed();
    println!("decryption time with CRT, D1: {:?}",crt_decrypt_d1_time);
    println!("decrypted msg with CRT, D1: \t {}",decrypted_message_crt_d1); 

    let crt_decrypt_d2= Instant::now();
    let decrypted_message_crt_d2 = decrypt_crt(&s_rsa_ciphertext, &d_2, &prime1, &prime2);
    let crt_decrypt_d2_time = crt_decrypt_d2.elapsed();
    println!("decryption time with CRT, D2: {:?}",crt_decrypt_d2_time);
    println!("decrypted msg with CRT, D2: \t {}",decrypted_message_crt_d2); 

    
   
    if _choice==2{
        let mut file = OpenOptions::new()
        .create(true) // Create the file if it doesn't exist
        .append(true) // Open the file in append mode
        .open("output.txt")
        .expect("Failed to open file");
        writeln!(file, "first prime(P):\t {}", prime1).expect("Failed to write to file");
        writeln!(file, "second prime(P):\t {}", prime2).expect("Failed to write to file");
        writeln!(file, "-----------------------------").expect("Failed to write to file");
        writeln!(file, "N:\t {}", n).expect("Failed to write to file");
        writeln!(file, "phi_n Standard RSA:\t {}", phi_n).expect("Failed to write to file");
        writeln!(file, "psi_n LCM RSA:\t {}", psi_n).expect("Failed to write to file");
        writeln!(file, "d_1:\t {:?}", d_1).expect("Failed to write to file");
        writeln!(file, "d_2:\t {:?}", d_2).expect("Failed to write to file");
        writeln!(file, "D1 is \t {:?} times higher than D2", ratio_d1_d2).expect("Failed to write to file");
        writeln!(file, "++++++++++++++++++++++++++++").expect("Failed to write to file");
        writeln!(file, "Now encryption with plaintext: \t{}", m).expect("Failed to write to file");
        writeln!(file, "Ciphertext with Standard RSA:\t{}",s_rsa_ciphertext).expect("Failed to write to file");
        writeln!(file, "****************************").expect("Failed to write to file");
        writeln!(file, "Now decryption").expect("Failed to write to file");
        writeln!(file, "with D1 decryption time: \t{:?}",s_rsa_decrypt_time).expect("Failed to write to file");
        writeln!(file, "with D2 decryption time: \t{:?}",lcm_rsa_decrypt_time).expect("Failed to write to file");
        writeln!(file, "with CRT and D1 decryption time: \t{:?}",crt_decrypt_d1_time).expect("Failed to write to file");
        writeln!(file, "with CRT and D2 decryption time: \t{:?}",crt_decrypt_d2_time).expect("Failed to write to file");
        writeln!(file, "#############################################################################################################################################").expect("Failed to write to file");
        writeln!(file, "=============================================================================================================================================").expect("Failed to write to file");    
    }
}