/// Operaciones de curva eliptica y^2 = x^3 + b sobre F_p.

use super::field::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: u64,
    pub y: u64,
    pub infinity: bool,
}

impl Point {
    pub fn infinity() -> Self {
        Point { x: 0, y: 0, infinity: true }
    }

    pub fn new(x: u64, y: u64) -> Self {
        Point { x, y, infinity: false }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if self.infinity {
            write!(f, "O (punto en el infinito)")
        } else {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
}

/// Verifica si P esta en la curva y^2 = x^3 + b (mod p).
pub fn is_on_curve(p_point: &Point, b: u64, p: u64) -> bool {
    if p_point.infinity {
        return true;
    }
    let y2 = mod_mul(p_point.y, p_point.y, p);
    let x3 = mod_pow(p_point.x, 3, p);
    let rhs = mod_add(x3, b, p);
    y2 == rhs
}

/// Suma de puntos P + Q en y^2 = x^3 + b (mod p).
pub fn point_add(p1: &Point, p2: &Point, p: u64) -> Point {
    if p1.infinity {
        return *p2;
    }
    if p2.infinity {
        return *p1;
    }
    if p1.x == p2.x {
        if p1.y == p2.y && p1.y != 0 {
            return point_double(p1, p);
        }
        // P + (-P) = O, o ambos tienen y=0
        return Point::infinity();
    }

    let dy = mod_sub(p2.y, p1.y, p);
    let dx = mod_sub(p2.x, p1.x, p);
    let slope = mod_mul(dy, mod_inv(dx, p), p);

    let x3 = mod_sub(mod_sub(mod_mul(slope, slope, p), p1.x, p), p2.x, p);
    let y3 = mod_sub(mod_mul(slope, mod_sub(p1.x, x3, p), p), p1.y, p);

    Point::new(x3, y3)
}

/// Duplicacion de punto 2P en y^2 = x^3 + b (mod p).
/// Para y^2 = x^3 + b, el coeficiente a = 0.
pub fn point_double(pt: &Point, p: u64) -> Point {
    if pt.infinity || pt.y == 0 {
        return Point::infinity();
    }

    // slope = 3x^2 / (2y)  (a=0 para nuestra curva)
    let x2 = mod_mul(pt.x, pt.x, p);
    let num = mod_mul(3, x2, p);
    let den = mod_mul(2, pt.y, p);
    let slope = mod_mul(num, mod_inv(den, p), p);

    let x3 = mod_sub(mod_mul(slope, slope, p), mod_mul(2, pt.x, p), p);
    let y3 = mod_sub(mod_mul(slope, mod_sub(pt.x, x3, p), p), pt.y, p);

    Point::new(x3, y3)
}

/// Multiplicacion escalar k*P usando double-and-add.
pub fn scalar_mul(k: u64, pt: &Point, p: u64) -> Point {
    if k == 0 || pt.infinity {
        return Point::infinity();
    }
    let mut result = Point::infinity();
    let mut base = *pt;
    let mut k = k;

    while k > 0 {
        if k & 1 == 1 {
            result = point_add(&result, &base, p);
        }
        base = point_double(&base, p);
        k >>= 1;
    }
    result
}

/// Negacion de un punto: -P = (x, p-y).
pub fn point_neg(pt: &Point, p: u64) -> Point {
    if pt.infinity {
        return Point::infinity();
    }
    Point::new(pt.x, mod_sub(0, pt.y, p))
}

/// Cuenta el numero de puntos en la curva y^2 = x^3 + b sobre F_p (fuerza bruta).
/// Retorna n = #E(F_p) incluyendo el punto en el infinito.
pub fn count_points(b: u64, p: u64) -> u64 {
    let mut count = 1u64; // punto en el infinito
    for x in 0..p {
        let rhs = mod_add(mod_pow(x, 3, p), b, p);
        if rhs == 0 {
            count += 1; // y = 0
        } else {
            let ls = legendre(rhs, p);
            if ls == 1 {
                count += 2; // dos raices
            }
        }
    }
    count
}

/// Encuentra un punto generador de orden `order` en la curva y^2 = x^3 + b.
pub fn find_generator(b: u64, p: u64, order: u64) -> Point {
    for x in 1..p {
        let rhs = mod_add(mod_pow(x, 3, p), b, p);
        if let Some(y) = mod_sqrt(rhs, p) {
            if y == 0 {
                continue;
            }
            let pt = Point::new(x, y);
            // Verificar que tiene orden completo
            let check = scalar_mul(order, &pt, p);
            if check.infinity {
                // Verificar que no tiene un suborden trivial
                // Chequeamos factores primos del orden
                let mut is_generator = true;
                let mut n = order;
                let mut f = 2u64;
                while f * f <= n {
                    if n % f == 0 {
                        if scalar_mul(order / f, &pt, p).infinity {
                            is_generator = false;
                            break;
                        }
                        while n % f == 0 {
                            n /= f;
                        }
                    }
                    f += 1;
                }
                if is_generator && n > 1 {
                    if scalar_mul(order / n, &pt, p).infinity {
                        is_generator = false;
                    }
                }
                if is_generator {
                    return pt;
                }
            }
        }
    }
    panic!("No se encontro generador");
}
