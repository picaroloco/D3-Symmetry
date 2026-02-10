/// Endomorfismo GLV para curvas con D=-3.
///
/// En una curva y^2 = x^3 + b con D=-3, existe beta tal que beta^3 = 1 (mod p).
/// El endomorfismo phi(x, y) = (beta*x, y) es un automorfismo del grupo.

use crate::math::field::*;
use crate::math::curve::*;

/// Encuentra beta: raiz cubica primitiva de 1 mod p.
/// Requiere p ≡ 1 (mod 3).
pub fn find_beta(p: u64) -> u64 {
    assert!(p % 3 == 1, "p debe ser ≡ 1 (mod 3) para que exista beta");

    // Buscamos g tal que g^((p-1)/3) != 1, luego beta = g^((p-1)/3)
    let exp = (p - 1) / 3;
    for g in 2..p {
        let candidate = mod_pow(g, exp, p);
        if candidate != 1 {
            // candidate es una raiz cubica primitiva de 1
            return candidate;
        }
    }
    panic!("No se encontro beta");
}

/// Encuentra lambda: raiz de x^2 + x + 1 ≡ 0 (mod n).
/// lambda es el escalar tal que phi(P) = [lambda]P.
pub fn find_lambda(n: u64) -> u64 {
    // x^2 + x + 1 = 0 (mod n) => x = (-1 ± sqrt(1 - 4)) / 2 = (-1 ± sqrt(-3)) / 2
    // Equivale a buscar por fuerza bruta en curva toy
    for x in 2..n {
        let val = mod_add(mod_add(mod_mul(x, x, n), x, n), 1, n);
        if val == 0 {
            return x;
        }
    }
    panic!("No se encontro lambda (n no es compatible con D=-3)");
}

/// Aplica el endomorfismo phi(P) = (beta * x, y).
pub fn apply_endo(pt: &Point, beta: u64, p: u64) -> Point {
    if pt.infinity {
        return Point::infinity();
    }
    Point::new(mod_mul(beta, pt.x, p), pt.y)
}

/// Genera las 6 imagenes de P bajo el grupo de automorfismos Aut(E) para |D|=3.
/// Son: P, phi(P), phi^2(P), -P, -phi(P), -phi^2(P)
pub fn automorphism_orbit(pt: &Point, beta: u64, p: u64) -> Vec<Point> {
    if pt.infinity {
        return vec![Point::infinity()];
    }
    let beta2 = mod_mul(beta, beta, p);

    let p1 = *pt;                                              // [1]P
    let p2 = apply_endo(pt, beta, p);                          // [zeta3]P
    let p3 = apply_endo(pt, beta2, p);                         // [zeta3^2]P
    let p4 = point_neg(pt, p);                                 // [-1]P
    let p5 = point_neg(&p2, p);                                // [-zeta3]P
    let p6 = point_neg(&p3, p);                                // [-zeta3^2]P

    vec![p1, p2, p3, p4, p5, p6]
}

/// Representante canonico de la clase de equivalencia bajo Aut(E).
/// Devuelve el punto con la menor (x, y) lexicograficamente.
pub fn canonical_rep(pt: &Point, beta: u64, p: u64) -> Point {
    if pt.infinity {
        return Point::infinity();
    }
    let orbit = automorphism_orbit(pt, beta, p);
    let mut best = orbit[0];
    for &q in &orbit[1..] {
        if q.infinity {
            continue;
        }
        if q.x < best.x || (q.x == best.x && q.y < best.y) {
            best = q;
        }
    }
    best
}
