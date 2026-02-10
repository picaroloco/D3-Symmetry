/// Aritmetica modular sobre F_p usando u64 (sin dependencias externas).

pub fn mod_add(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 + b as u128) % p as u128) as u64
}

pub fn mod_sub(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 + p as u128 - b as u128) % p as u128) as u64
}

pub fn mod_mul(a: u64, b: u64, p: u64) -> u64 {
    ((a as u128 * b as u128) % p as u128) as u64
}

pub fn mod_pow(base: u64, mut exp: u64, p: u64) -> u64 {
    if p == 1 {
        return 0;
    }
    let mut result: u128 = 1;
    let mut b: u128 = (base % p) as u128;
    let m = p as u128;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * b % m;
        }
        exp >>= 1;
        b = b * b % m;
    }
    result as u64
}

/// Inverso modular via extended GCD. Panics si a == 0.
pub fn mod_inv(a: u64, p: u64) -> u64 {
    let a = a as i128;
    let p = p as i128;
    let (mut old_r, mut r) = (a, p);
    let (mut old_s, mut s) = (1i128, 0i128);

    while r != 0 {
        let q = old_r / r;
        let tmp = r;
        r = old_r - q * r;
        old_r = tmp;
        let tmp = s;
        s = old_s - q * s;
        old_s = tmp;
    }

    ((old_s % p + p) % p) as u64
}

/// Raiz cuadrada modular via Tonelli-Shanks. Retorna None si `a` no es QR.
pub fn mod_sqrt(a: u64, p: u64) -> Option<u64> {
    if a == 0 {
        return Some(0);
    }
    // Chequear si a es residuo cuadratico
    if mod_pow(a, (p - 1) / 2, p) != 1 {
        return None;
    }

    // Caso p ≡ 3 (mod 4)
    if p % 4 == 3 {
        return Some(mod_pow(a, (p + 1) / 4, p));
    }

    // Tonelli-Shanks general
    // Escribir p-1 = Q * 2^S
    let mut q = p - 1;
    let mut s = 0u32;
    while q % 2 == 0 {
        q /= 2;
        s += 1;
    }

    // Encontrar un no-residuo cuadratico z
    let mut z = 2u64;
    while mod_pow(z, (p - 1) / 2, p) != p - 1 {
        z += 1;
    }

    let mut m = s;
    let mut c = mod_pow(z, q, p);
    let mut t = mod_pow(a, q, p);
    let mut r = mod_pow(a, (q + 1) / 2, p);

    loop {
        if t == 1 {
            return Some(r);
        }
        // Encontrar el menor i tal que t^(2^i) = 1
        let mut i = 1u32;
        let mut tmp = mod_mul(t, t, p);
        while tmp != 1 {
            tmp = mod_mul(tmp, tmp, p);
            i += 1;
        }
        let b = mod_pow(c, 1u64 << (m - i - 1), p);
        m = i;
        c = mod_mul(b, b, p);
        t = mod_mul(t, c, p);
        r = mod_mul(r, b, p);
    }
}

/// Simbolo de Legendre: retorna 1 si QR, p-1 si NQR, 0 si a==0.
pub fn legendre(a: u64, p: u64) -> u64 {
    mod_pow(a % p, (p - 1) / 2, p)
}
