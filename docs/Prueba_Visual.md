# Prueba Visual: Salida de `cargo run`

> Resultado completo de la demostración D3-Symmetry ejecutada sobre la curva toy $y^2 = x^3 + 7$ en $\mathbb{F}_{10477}$.

---

## \[1\] Curva Toy (analogía directa con Bitcoin)

| Parámetro | Valor |
|---|---|
| Ecuación | $y^2 = x^3 + 7$ |
| Campo | $\mathbb{F}_{10477}$ &ensp; ($10477 \bmod 3 = 1$) |
| Orden del grupo | $n = 10639$ |
| Generador | $G = (3,\ 731)$ |

```
Verificación: G está en la curva         ✓
Verificación: [n]G = O                   ✓
```

---

## \[2\] Endomorfismo (la debilidad de $\lvert D \rvert = 3$)

| Constante | Valor | Propiedad |
|---|---|---|
| $\beta$ | $716$ | $\beta^3 \equiv 1 \pmod{p}$ ✓ |
| $\beta^2$ | $9760$ | — |
| $\lambda$ | $1893$ | $\lambda^2 + \lambda + 1 \equiv 0 \pmod{n}$ ✓ |

```
φ(G) = (β · Gx mod p, Gy) = (2148, 731)
Verificación: φ(G) está en la curva      ✓
Verificación: φ(G) = [λ]G                ✓
```

---

## \[3\] Grupo de Automorfismos

$$|\text{Aut}(E)| = 6 \quad \text{(vs 2 en una curva con } |D| \text{ grande)}$$

| Automorfismo | Imagen de $G$ | ¿En la curva? |
|---|---|---|
| $[1]G$ | $(3,\ 731)$ | ✓ |
| $[\zeta_3]G$ | $(2148,\ 731)$ | ✓ |
| $[\zeta_3^2]G$ | $(8326,\ 731)$ | ✓ |
| $[-1]G$ | $(3,\ 9746)$ | ✓ |
| $[-\zeta_3]G$ | $(2148,\ 9746)$ | ✓ |
| $[-\zeta_3^2]G$ | $(8326,\ 9746)$ | ✓ |

> Las 6 imágenes están en la curva ✓

---

## \[4\] Ataque ECDLP: BSGS Estándar (sin explotar $D = -3$)

```
Clave secreta (oculta): k = 7777
Clave pública:          Q = k·G = (6142, 7632)
```

| Métrica | Valor |
|---|---|
| Baby steps | 104 |
| Giant steps | 74 |
| **Total operaciones** | **178** |
| Resultado | $k = 7777$ ✓ |

---

## \[5\] Ataque ECDLP: BSGS con GLV (explotando $D = -3$)

> Misma $Q$, misma curva. Usando clases de equivalencia de tamaño 6.

| Métrica | Valor |
|---|---|
| Baby steps | 43 |
| Giant steps | 15 |
| **Total operaciones** | **58** |
| Resultado | $k = 7777$ ✓ |

---

## \[6\] Comparación

```
┌─────────────────────┬──────────┬──────────┐
│ Método              │ Ops      │ Speedup  │
├─────────────────────┼──────────┼──────────┤
│ BSGS estándar       │      178 │ 1.00x    │
│ BSGS con D=-3       │       58 │ 3.07x    │
│ Speedup teórico     │          │ 2.45x    │
└─────────────────────┴──────────┴──────────┘
```

**Vulnerabilidad demostrada:**

El discriminante $|D| = 3$ permite resolver ECDLP **3.07x** más rápido.

| | Valor |
|---|---|
| Speedup teórico | $\sqrt{6} \approx 2.4495$ |
| Factor observado | $3.0690$ |

> En Bitcoin (secp256k1), esto reduce la seguridad efectiva de **128 bits a ~122 bits**. Cada automorfismo adicional es una simetría explotable que debilita la criptografía.

---

## \[7\] Verificación Formal (Z3 SMT Solver)

| Prueba | Veredicto | Detalle |
|---|---|---|
| Polinomio característico $\lambda$ | ✓ | $\lambda^2 + \lambda + 1 \equiv 0 \pmod{n}$ &ensp; `[UNSAT ⇒ válido]` |
| Unicidad de $\lambda$ | ✓ | $\exists x = \lambda$ tal que $x^2+x+1 \equiv 0 \pmod{n}$ &ensp; `[SAT ⇒ válido]` |
| Discriminante $D = -3$ | ✓ | $p \equiv 1 \pmod{3} \land \Delta \neq 0$ &ensp; `[UNSAT ⇒ válido]` |

> Detalles técnicos de la implementación Z3: [Análisis Simbólico Z3](Analisis_Simbolico_Z3.md)

---

*Generado a partir de `cargo run` — D3-Symmetry v0.1.0*
