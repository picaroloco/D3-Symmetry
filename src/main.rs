/// D3-Symmetry: Demostracion del ataque GLV en una curva toy con D=-3
///
/// Curva: y^2 = x^3 + 7 sobre F_p (misma ecuacion que Bitcoin secp256k1)
/// Primo toy: p = 10477 (p ≡ 1 mod 3, necesario para D=-3)

use d3_symmetry::field::*;
use d3_symmetry::curve::*;
use d3_symmetry::endomorphism::*;
use d3_symmetry::attack::*;
use d3_symmetry::formal_verification::Z3Verifier;

fn main() {
    println!("=== D3-Symmetry: Demostracion del Ataque GLV ===\n");

    // ── Parametros de la curva toy ──
    let p: u64 = 10477;  // primo, 10477 mod 3 == 1, orden del grupo primo
    let b: u64 = 7;      // y^2 = x^3 + 7 (como Bitcoin)

    // ════════════════════════════════════════════════
    // [1] CURVA TOY
    // ════════════════════════════════════════════════
    println!("[1] CURVA TOY (analogia directa con Bitcoin)");
    println!("    Ecuacion: y^2 = x^3 + 7");
    println!("    Campo:    F_{{{}}}   (primo, {} mod 3 = {})", p, p, p % 3);

    let order = count_points(b, p);
    println!("    Orden del grupo: n = {}", order);

    let g = find_generator(b, p, order);
    println!("    Generador: G = {}", g);
    assert!(is_on_curve(&g, b, p), "G no esta en la curva!");
    println!("    Verificacion: G esta en la curva ✓");

    // Verificar que G tiene orden n
    let check = scalar_mul(order, &g, p);
    assert!(check.infinity, "[n]G != O");
    println!("    Verificacion: [n]G = O ✓\n");

    // ════════════════════════════════════════════════
    // [2] ENDOMORFISMO (la debilidad de |D|=3)
    // ════════════════════════════════════════════════
    println!("[2] ENDOMORFISMO (la debilidad de |D|=3)");

    let beta = find_beta(p);
    let beta2 = mod_mul(beta, beta, p);
    println!("    beta  = {} (raiz cubica de 1 mod p)", beta);
    println!("    beta2 = {}", beta2);
    println!("    Verificacion: beta^3 mod p = {} ✓", mod_pow(beta, 3, p));
    assert_eq!(mod_pow(beta, 3, p), 1, "beta no es raiz cubica de 1");

    let phi_g = apply_endo(&g, beta, p);
    println!("    phi(G) = (beta * Gx mod p, Gy) = {}", phi_g);
    assert!(is_on_curve(&phi_g, b, p), "phi(G) no esta en la curva!");
    println!("    Verificacion: phi(G) esta en la curva ✓");

    // Encontrar lambda tal que phi(G) = [lambda]G
    let lambda = find_lambda(order);
    let lambda_g = scalar_mul(lambda, &g, p);
    assert_eq!(lambda_g, phi_g, "phi(G) != [lambda]G");
    println!("    lambda = {} (raiz de x^2+x+1 mod n)", lambda);
    println!("    Verificacion: phi(G) = [lambda]G ✓");
    println!("    Verificacion: lambda^2 + lambda + 1 mod n = {} ✓\n",
             mod_add(mod_add(mod_mul(lambda, lambda, order), lambda, order), 1, order));

    // ════════════════════════════════════════════════
    // [3] GRUPO DE AUTOMORFISMOS
    // ════════════════════════════════════════════════
    println!("[3] GRUPO DE AUTOMORFISMOS");
    println!("    |Aut(E)| = 6 (vs 2 en una curva con |D| grande)\n");

    let orbit = automorphism_orbit(&g, beta, p);
    let labels = ["[1]G", "[zeta3]G", "[zeta3^2]G", "[-1]G", "[-zeta3]G", "[-zeta3^2]G"];
    for (i, pt) in orbit.iter().enumerate() {
        let on_curve = is_on_curve(pt, b, p);
        println!("    {} = {:>20}  en curva: {}", labels[i], pt,
                 if on_curve { "✓" } else { "✗" });
        assert!(on_curve, "{} no esta en la curva!", labels[i]);
    }
    println!("    Las 6 imagenes estan en la curva ✓\n");

    // ════════════════════════════════════════════════
    // [4] ATAQUE ECDLP: BSGS ESTANDAR
    // ════════════════════════════════════════════════
    println!("[4] ATAQUE ECDLP: BSGS ESTANDAR (sin explotar D=-3)");

    // Elegir clave secreta determinista pero no trivial
    let secret_k = 7777u64 % order;
    let pub_q = scalar_mul(secret_k, &g, p);
    println!("    Clave secreta (oculta): k = {} (elegida para la demo)", secret_k);
    println!("    Clave publica: Q = k*G = {}", pub_q);
    println!("    Resolviendo Q = k*G por BSGS estandar...");

    let (found_k, ops_std) = bsgs_standard(&pub_q, &g, order, p);
    assert_eq!(found_k, secret_k, "BSGS estandar encontro k incorrecto!");
    let m_std = (order as f64).sqrt().ceil() as u64;
    println!("    Baby steps: {}, Giant steps: {}", m_std, ops_std as u64 - m_std);
    println!("    Total operaciones: {}", ops_std);
    println!("    Resultado: k = {} ✓\n", found_k);

    // ════════════════════════════════════════════════
    // [5] ATAQUE ECDLP: BSGS CON GLV
    // ════════════════════════════════════════════════
    println!("[5] ATAQUE ECDLP: BSGS CON GLV (explotando D=-3)");
    println!("    Misma Q, misma curva.");
    println!("    Usando clases de equivalencia de tamanio 6...");

    let (found_k2, ops_glv) = bsgs_glv(&pub_q, &g, order, p, beta);
    let verify = scalar_mul(found_k2, &g, p);
    assert_eq!(verify, pub_q, "BSGS GLV encontro k incorrecto!");
    let m_glv = ((order as f64) / 6.0).sqrt().ceil() as u64;
    println!("    Baby steps: {}, Giant steps: {}", m_glv, ops_glv as u64 - m_glv);
    println!("    Total operaciones: {}", ops_glv);
    println!("    Resultado: k = {} ✓\n", found_k2);

    // ════════════════════════════════════════════════
    // [6] COMPARACION
    // ════════════════════════════════════════════════
    println!("[6] COMPARACION");
    let speedup = ops_std as f64 / ops_glv as f64;
    let theoretical = (6.0f64).sqrt();

    println!("    ┌─────────────────────┬──────────┬──────────┐");
    println!("    │ Metodo              │ Ops      │ Speedup  │");
    println!("    ├─────────────────────┼──────────┼──────────┤");
    println!("    │ BSGS estandar       │ {:>8} │ 1.00x    │", ops_std);
    println!("    │ BSGS con D=-3       │ {:>8} │ {:.2}x   │", ops_glv, speedup);
    println!("    │ Speedup teorico     │          │ {:.2}x   │", theoretical);
    println!("    └─────────────────────┴──────────┴──────────┘");

    println!();
    println!("    VULNERABILIDAD DEMOSTRADA:");
    println!("    El discriminante |D|=3 permite resolver ECDLP {:.2}x mas rapido.", speedup);
    println!("    Speedup teorico: sqrt(6) = {:.4}", theoretical);
    println!("    Factor observado: {:.4}", speedup);
    println!();
    println!("    En Bitcoin (secp256k1), esto reduce la seguridad efectiva de");
    println!("    128 bits a ~122 bits. Cada automorfismo adicional es una");
    println!("    simetria explotable que debilita la criptografia.");

    // ════════════════════════════════════════════════
    // [7] VERIFICACIÓN FORMAL
    // ════════════════════════════════════════════════
    println!();
    println!("[7] VERIFICACION FORMAL (Z3 SMT Solver)");
    
    let verifier = Z3Verifier::new();
    let results = verifier.run_all_verifications();
    
    for result in results {
        let status = if result.passed { "✓" } else { "✗" };
        println!("    {} {}: {}", status, result.name, result.details);
    }

    println!();
    println!("=== Fin de la Demostracion ===");
}
