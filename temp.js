// Helper function to generate a random bigint with a specific bit length
function generateRandomBigInt(bitLength) {
    const bytes = Math.ceil(bitLength / 8);
    const randomBytes = new Uint8Array(bytes);
    crypto.getRandomValues(randomBytes);
    randomBytes[0] |= 0x80; // Ensure the highest bit is set to make it the correct bit length
    return BigInt("0x" + Array.from(randomBytes).map(b => b.toString(16).padStart(2, "0")).join(""));
}

// Miller-Rabin primality test (simplified for demonstration)
function isPrime(n, k = 5) {
    if (n <= 1n) return false;
    if (n <= 3n) return true;
    if (n % 2n === 0n) return false;

    let d = n - 1n;
    let s = 0n;
    while (d % 2n === 0n) {
        d /= 2n;
        s++;
    }

    for (let i = 0; i < k; i++) {
        const a = generateRandomBigInt(64) % (n - 3n) + 2n;
        let x = powMod(a, d, n);
        if (x === 1n || x === n - 1n) continue;

        for (let j = 0; j < s - 1n; j++) {
            x = powMod(x, 2n, n);
            if (x === n - 1n) break;
        }

        if (x !== n - 1n) return false;
    }

    return true;
}

// Modular exponentiation (a^b mod m)
function powMod(a, b, m) {
    let result = 1n;
    a = a % m;
    while (b > 0n) {
        if (b % 2n === 1n) {
            result = (result * a) % m;
        }
        a = (a * a) % m;
        b = b / 2n;
    }
    return result;
}

// Generate a large prime number with a specific bit length
function generateLargePrime(bitLength) {
    while (true) {
        const num = generateRandomBigInt(bitLength);
        if (isPrime(num)) {
            return num;
        }
    }
}

// Extended Euclidean Algorithm to compute the modular inverse
function extendedGCD(a, b) {
    if (b === 0n) {
        return [a, 1n, 0n];
    } else {
        const [gcd, x1, y1] = extendedGCD(b, a % b);
        const x = y1;
        const y = x1 - (a / b) * y1;
        return [gcd, x, y];
    }
}

// Compute the modular inverse of a modulo m
function modInverse(a, m) {
    const [gcd, x, y] = extendedGCD(a, m);
    if (gcd !== 1n) {
        return null; // Modular inverse does not exist if gcd(a, m) != 1
    } else {
        return (x % m + m) % m; // Ensure the result is positive
    }
}

function modMultInverse (a, m) {
    // Extended Euclidian algorithm
    //   https://en.wikipedia.org/wiki/Modular_multiplicative_inverse#Computation
    //   https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm#Pseudocode
    var oldR = a;// old_r=e
    var r = m; //r==phi_n
    var oldS = 1n;
    var s = 0n;
    
    while(r > 0n) {
      // BigInt division truncates, which is what we want.
      var quot = oldR / r;
      
      var tempR = r;
      r = oldR - quot * r; // alternatively, r = oldR % r;
      oldR = tempR;
      
      var tempS = s;
      s = oldS - quot * s;
      oldS = tempS;
    }
    
    if(oldS < 0)
      oldS += m;
    return oldS;
  };

// Initialize primes and compute phi(n) such that gcd(e, phi(n)) = 1
function initialization(e) {
    const bitSize = 1024;

    while (true) {
        // Generate two 1024-bit primes
        const prime1 = generateLargePrime(bitSize);
        const prime2 = generateLargePrime(bitSize);

        // Compute n = p * q
        const n = prime1 * prime2;

        // Compute phi(n) = (p-1) * (q-1)
        const p1 = prime1 - 1n;
        const q1 = prime2 - 1n;
        const phiN = p1 * q1;

        // Check if gcd(e, phi(n)) == 1
        const gcdVal = extendedGCD(e, phiN)[0];
        if (gcdVal === 1n) {
            console.log(`phi_n and e are coprime with e = ${e} (gcd = 1)`);
            return { prime1, prime2, n, phiN };
        } else {
            console.log(`phi_n and e are not coprime. gcd(e, phi(n)) = ${gcdVal}. Regenerating primes...`);
        }
    }
}

// Main function
function main() {
    // Public exponent
    const e = 3n;

    // Initialize primes and compute phi(n)
    const { prime1, prime2, n, phiN } = initialization(e);
    console.log(`First 1024-bit prime number: ${prime1}`);
    console.log(`Second 1024-bit prime number: ${prime2}`);
    console.log(`n = ${n}`);
    console.log(`phi(n) = ${phiN}`);

    // Compute the private key d as the modular inverse of e modulo phi(n)
    const d = modMultInverse(e, phiN);
    if (d === null) {
        console.log("Modular inverse does not exist. gcd(e, phi(n)) != 1.");
    } else {
        console.log(`Private key d: ${d}`);
    }
}

// Run the main function
main();