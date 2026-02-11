/// Pollard's Rho para ECDLP con Equivalence Class Search (D=-3).
///
/// A diferencia de BSGS (memoria O(√n)), Pollard's Rho usa memoria O(1)
/// y detecta colisiones via el ciclo de Floyd (tortuga y liebre).
///
/// La variante GLV explota los 6 automorfismos de curvas con |D|=3:
/// en cada paso, el punto se reduce a su representante canónico,
/// comprimiendo el espacio de búsqueda por un factor √6.

use crate::math::field::*;
use crate::math::curve::*;
use crate::endomorphism::*;

/// Resultado de Pollard's Rho: clave encontrada + estadísticas.
pub struct PollardRhoResult {
    pub k: u64,
    pub iterations: usize,
}

// ── Función de partición ──────────────────────────────────────
// Divide los puntos en 3 zonas según x mod 3 para definir
// el camino pseudoaleatorio. Cada zona aplica una operación
// distinta para mezclar los coeficientes (a, b) de R = aG + bQ.

/// Un paso del camino aleatorio: dado R = aG + bQ, produce R', a', b'.
/// La partición usa x mod 3 del punto (o hash del infinito).
fn walk_step(
    r: &Point, a: u64, b: u64,
    g: &Point, q: &Point,
    n: u64, p: u64,
) -> (Point, u64, u64) {
    let partition = if r.infinity { 0 } else { r.x % 3 };

    match partition {
        // Zona 0: R' = R + Q,  a' = a,      b' = b + 1
        0 => {
            let r_new = point_add(r, q, p);
            let b_new = mod_add(b, 1, n);
            (r_new, a, b_new)
        }
        // Zona 1: R' = 2R,     a' = 2a,     b' = 2b
        1 => {
            let r_new = point_add(r, r, p);
            let a_new = mod_mul(2, a, n);
            let b_new = mod_mul(2, b, n);
            (r_new, a_new, b_new)
        }
        // Zona 2: R' = R + G,  a' = a + 1,  b' = b
        _ => {
            let r_new = point_add(r, g, p);
            let a_new = mod_add(a, 1, n);
            (r_new, a_new, b)
        }
    }
}

// ── Pollard's Rho estándar ────────────────────────────────────

/// Pollard's Rho estándar: resuelve Q = k*G sin explotar automorfismos.
/// Memoria O(1), tiempo esperado O(√(π·n/2)).
/// Retorna PollardRhoResult con k y el número de iteraciones.
pub fn pollard_rho_standard(
    q: &Point, g: &Point, n: u64, p: u64,
) -> PollardRhoResult {
    // Tortuga: empieza en R_t = G (a=1, b=0)
    let mut rt = *g;
    let mut at: u64 = 1;
    let mut bt: u64 = 0;

    // Liebre: empieza en el mismo punto
    let mut rh = *g;
    let mut ah: u64 = 1;
    let mut bh: u64 = 0;

    let mut iterations = 0usize;

    loop {
        // Tortuga: 1 paso
        let (rt2, at2, bt2) = walk_step(&rt, at, bt, g, q, n, p);
        rt = rt2; at = at2; bt = bt2;

        // Liebre: 2 pasos
        let (rh2, ah2, bh2) = walk_step(&rh, ah, bh, g, q, n, p);
        let (rh3, ah3, bh3) = walk_step(&rh2, ah2, bh2, g, q, n, p);
        rh = rh3; ah = ah3; bh = bh3;

        iterations += 1;

        // ¿Colisión? Tortuga == Liebre
        if rt == rh {
            // rt = at*G + bt*Q = rh = ah*G + bh*Q
            // (at - ah)*G = (bh - bt)*Q
            // Si Q = k*G => (at - ah) = k*(bh - bt) mod n
            // k = (at - ah) * (bh - bt)^{-1} mod n
            let delta_b = mod_sub(bh, bt, n);
            if delta_b == 0 {
                // Colisión degenerada: reintentar con offset
                // (en curva toy esto es rarísimo, pero lo manejamos)
                rt = point_add(&rt, g, p);
                at = mod_add(at, 1, n);
                rh = rt; ah = at; bh = bt;
                continue;
            }
            let delta_a = mod_sub(at, ah, n);
            let k = mod_mul(delta_a, mod_inv(delta_b, n), n);
            return PollardRhoResult { k, iterations };
        }
    }
}

// ── Pollard's Rho con Equivalence Class Search (D=-3) ─────────

/// Un paso del camino aleatorio con reducción canónica.
/// Después de cada paso, el punto se reemplaza por su representante
/// canónico (el menor de los 6 automorfismos), y los coeficientes
/// (a, b) se ajustan según qué automorfismo lo mapeó.
fn walk_step_canonical(
    r: &Point, a: u64, b: u64,
    g: &Point, q: &Point,
    n: u64, p: u64, beta: u64,
    lambda: u64, lambda2: u64,
) -> (Point, u64, u64) {
    // Primero: paso normal del camino aleatorio
    let (r_new, a_new, b_new) = walk_step(r, a, b, g, q, n, p);

    if r_new.infinity {
        return (r_new, a_new, b_new);
    }

    // Después: reducir a representante canónico
    // Encontrar cuál de los 6 automorfismos produce el representante
    let canon = canonical_rep(&r_new, beta, p);

    // Los 6 automorfismos con sus escalares asociados:
    // σ_i(P) = [s_i]*P donde s_i ∈ {1, λ, λ², -1, -λ, -λ²}
    let orbit_scalars = [1u64, lambda, lambda2,
                         n - 1, n - lambda, n - lambda2];
    let orbit = automorphism_orbit(&r_new, beta, p);

    for (idx, orbit_pt) in orbit.iter().enumerate() {
        if *orbit_pt == canon {
            // canon = σ_i(R') = [s_i] * R'
            // R' = a'G + b'Q, canon = [s_i]*(a'G + b'Q)
            // canon = (s_i*a')G + (s_i*b')Q
            let s = orbit_scalars[idx];
            let a_canon = mod_mul(s, a_new, n);
            let b_canon = mod_mul(s, b_new, n);
            return (canon, a_canon, b_canon);
        }
    }

    // Fallback: no debería llegar aquí
    (r_new, a_new, b_new)
}

/// Pollard's Rho con Equivalence Class Search: explota |Aut(E)| = 6.
/// Cada paso reduce el punto a su clase canónica, comprimiendo el
/// espacio de colisión por √6 ≈ 2.45x respecto al Rho estándar.
/// Memoria O(1), tiempo esperado O(√(π·n/12)).
pub fn pollard_rho_glv(
    q: &Point, g: &Point, n: u64, p: u64, beta: u64,
) -> PollardRhoResult {
    let lambda = find_lambda(n);
    let lambda2 = mod_mul(lambda, lambda, n);

    // Tortuga: empieza en canon(G)
    let start = canonical_rep(g, beta, p);
    // Necesitamos saber qué escalar mapea G → canon(G)
    let orbit_scalars = [1u64, lambda, lambda2,
                         n - 1, n - lambda, n - lambda2];
    let orbit = automorphism_orbit(g, beta, p);
    let mut a_start: u64 = 1;
    let b_start: u64 = 0;
    for (idx, orbit_pt) in orbit.iter().enumerate() {
        if *orbit_pt == start {
            a_start = orbit_scalars[idx];
            break;
        }
    }

    let mut rt = start;
    let mut at = a_start;
    let mut bt = b_start;

    let mut rh = start;
    let mut ah = a_start;
    let mut bh = b_start;

    let mut iterations = 0usize;

    loop {
        // Tortuga: 1 paso canónico
        let (rt2, at2, bt2) = walk_step_canonical(
            &rt, at, bt, g, q, n, p, beta, lambda, lambda2,
        );
        rt = rt2; at = at2; bt = bt2;

        // Liebre: 2 pasos canónicos
        let (rh2, ah2, bh2) = walk_step_canonical(
            &rh, ah, bh, g, q, n, p, beta, lambda, lambda2,
        );
        let (rh3, ah3, bh3) = walk_step_canonical(
            &rh2, ah2, bh2, g, q, n, p, beta, lambda, lambda2,
        );
        rh = rh3; ah = ah3; bh = bh3;

        iterations += 1;

        // ¿Colisión en espacio canónico?
        if rt == rh {
            let delta_b = mod_sub(bh, bt, n);
            if delta_b == 0 {
                // Colisión degenerada: perturbar y reintentar
                let (rt2, at2, bt2) = walk_step_canonical(
                    &rt, at, bt, g, q, n, p, beta, lambda, lambda2,
                );
                rt = rt2; at = at2; bt = bt2;
                rh = rt; ah = at; bh = bt;
                continue;
            }
            let delta_a = mod_sub(at, ah, n);
            let k = mod_mul(delta_a, mod_inv(delta_b, n), n);
            return PollardRhoResult { k, iterations };
        }
    }
}
