/// Ataques ECDLP: BSGS estandar vs BSGS con automorfismos (D=-3).

use std::collections::HashMap;
use crate::field::*;
use crate::curve::*;
use crate::endomorphism::*;

/// BSGS estandar: resuelve Q = k*G en el grupo de orden n.
/// Retorna (k, numero_de_operaciones).
pub fn bsgs_standard(q: &Point, g: &Point, n: u64, p: u64) -> (u64, usize) {
    let m = (n as f64).sqrt().ceil() as u64;
    let mut ops = 0usize;

    // Baby steps: tabla[j*G] = j para j = 0..m-1
    let mut table: HashMap<(u64, u64), u64> = HashMap::new();
    let mut baby = Point::infinity();
    for j in 0..m {
        if !baby.infinity {
            table.insert((baby.x, baby.y), j);
        } else if j == 0 {
            // Representar el punto infinito con un valor especial
            table.insert((u64::MAX, u64::MAX), 0);
        }
        baby = point_add(&baby, g, p);
        ops += 1;
    }

    // Giant step: factor = m*G
    let factor = scalar_mul(m, g, p);
    let neg_factor = point_neg(&factor, p);

    // Giant steps: Q - i*m*G para i = 0..m
    let mut gamma = *q;
    for i in 0..=m {
        let key = if gamma.infinity {
            (u64::MAX, u64::MAX)
        } else {
            (gamma.x, gamma.y)
        };

        if let Some(&j) = table.get(&key) {
            let k = mod_add(mod_mul(i, m, n), j, n);
            return (k, ops);
        }
        gamma = point_add(&gamma, &neg_factor, p);
        ops += 1;
    }

    panic!("BSGS estandar no encontro solucion");
}

/// BSGS con automorfismos D=-3: usa clases de equivalencia de tamanio 6.
/// Almacena representantes canonicos en la tabla baby, reduciendo el espacio sqrt(6) veces.
/// Retorna (k, numero_de_operaciones).
pub fn bsgs_glv(q: &Point, g: &Point, n: u64, p: u64, beta: u64) -> (u64, usize) {
    // Con |Aut(E)| = 6, el tamano optimo de la tabla es sqrt(n/6)
    let m = ((n as f64) / 6.0).sqrt().ceil() as u64;
    let m = if m == 0 { 1 } else { m };
    let mut ops = 0usize;

    // Baby steps: almacenamos canonical_rep(j*G) -> j
    let mut table: HashMap<(u64, u64), u64> = HashMap::new();
    let mut baby = Point::infinity();
    for j in 0..m {
        if !baby.infinity {
            let canon = canonical_rep(&baby, beta, p);
            table.insert((canon.x, canon.y), j);
        } else if j == 0 {
            table.insert((u64::MAX, u64::MAX), 0);
        }
        baby = point_add(&baby, g, p);
        ops += 1;
    }

    // Giant step: factor = m*G
    let factor = scalar_mul(m, g, p);
    let neg_factor = point_neg(&factor, p);

    // Para recuperar k, necesitamos saber QUE automorfismo matcheo
    let lambda = find_lambda(n);
    let lambda2 = mod_mul(lambda, lambda, n);

    // Giant steps: Q - i*m*G para i = 0..ceil(n/m)+1
    let max_giant = n / m + 2;
    let mut gamma = *q;
    for i in 0..max_giant {
        if gamma.infinity {
            if table.contains_key(&(u64::MAX, u64::MAX)) {
                let j = table[&(u64::MAX, u64::MAX)];
                let k = mod_add(mod_mul(i, m, n), j, n);
                return (k, ops);
            }
        } else {
            let canon = canonical_rep(&gamma, beta, p);
            if let Some(&j) = table.get(&(canon.x, canon.y)) {
                // gamma esta en la orbita de j*G
                // gamma = sigma(j*G) para algun automorfismo sigma
                // Necesitamos encontrar cual sigma y ajustar k
                let jg = scalar_mul(j, g, p);
                let orbit_scalars = [1u64, lambda, lambda2,
                                     n - 1, n - lambda, n - lambda2];
                let orbit_points = automorphism_orbit(&jg, beta, p);

                for (idx, orbit_pt) in orbit_points.iter().enumerate() {
                    if *orbit_pt == gamma {
                        // gamma = [orbit_scalars[idx]] * (j*G)
                        // Q - i*m*G = [s] * j*G
                        // Q = i*m*G + s*j*G
                        let s = orbit_scalars[idx];
                        let sj = mod_mul(s, j, n);
                        let k = mod_add(mod_mul(i, m, n), sj, n);
                        return (k, ops);
                    }
                }
            }
        }
        gamma = point_add(&gamma, &neg_factor, p);
        ops += 1;
    }

    panic!("BSGS GLV no encontro solucion");
}
