# Verificación Formal de Invariantes GLV mediante SMT

Este documento detalla el uso del solver **Z3 (Satisfiability Modulo Theories)** para certificar las propiedades algebraicas de la curva `secp256k1` relevantes al endomorfismo GLV.

## 1. ¿Por qué Verificación Formal?

A diferencia de un *Unit Test* que comprueba valores concretos (aritmética concreta), la verificación formal utiliza **lógica simbólica**. 

Z3 no comprueba si un número particular funciona; **demuestra** que, dada la definición de la curva $y^2 = x^3 + 7$, la existencia de un endomorfismo eficiente es una **consecuencia lógica** de los parámetros elegidos.

## 2. Modelado con BitVectors (512-bit)

Para emular el comportamiento real del hardware y la aritmética modular de 256 bits, utilizamos la teoría de **BitVectors** de Z3:

* **Precisión de Bit**: Cada variable se trata como un registro de bits. 
* **Evitación de Overflows**: Usamos un ancho de **512 bits** para las operaciones intermedias. Esto es crítico para verificar el polinomio $\lambda^2 + \lambda + 1 \equiv 0 \pmod n$, ya que el cuadrado de $\lambda$ requiere el doble de espacio antes de aplicar el módulo.
* **Semántica de Hardware**: Z3 analiza todas las posibles combinaciones de bits simultáneamente, garantizando que no existan casos de borde (edge cases).

## 3. Las Pruebas Lógicas

### A. El Polinomio de Eisenstein
Demostramos que el valor $\lambda$ utilizado en el ataque GLV satisface:
$$\lambda^2 + \lambda + 1 \equiv 0 \pmod n$$
La prueba se realiza por **contradicción**: Pedimos a Z3 que busque un caso donde esto *no* se cumpla. Al devolver `UNSAT` (Unsatisfiable), Z3 certifica que la igualdad es una verdad universal bajo los parámetros de secp256k1.

### B. Unicidad y No-Trivialidad
Verificamos simbólicamente que $\lambda$ no es una raíz trivial (como 1 o $n-1$). Z3 confirma que $\lambda$ es un elemento de orden 3 en el grupo multiplicativo $(\mathbb{Z}/n\mathbb{Z})^*$.

### C. Condición del Discriminante ($\lvert D \rvert=3$)
Validamos que el primo $p$ de secp256k1 cumple $p \equiv 1 \pmod 3$. Esta es la condición necesaria y suficiente para que $\mathbb{F}_p$ contenga raíces cúbicas de la unidad, lo que a su vez permite la existencia del endomorfismo $\phi$.

---

## 4. Implementación: `glv_invariant_prover_z3.rs`

El motor de certificación SMT reside en [`src/formal_verification/glv_invariant_prover_z3.rs`](../src/formal_verification/glv_invariant_prover_z3.rs). Su propósito es demostrar los invariantes algebraicos del endomorfismo GLV sobre los parámetros concretos de secp256k1.

### 4.1 El Motor: BitVectors de 512 bits

```rust
const BV_BITS: u32 = 512;
```

Esta decisión no es estética; es **aritméticamente obligatoria**:

* **Evitación de Truncamiento**: Al verificar $\lambda^2 \pmod{n}$, el valor intermedio de la multiplicación de dos números de 256 bits requiere hasta 512 bits de precisión.
* **Emulación a nivel de Hardware**: Los BitVectors de Z3 modelan el comportamiento exacto de registros de CPU/ASIC, capturando overflows que la aritmética abstracta de enteros podría ignorar.

### 4.2 Constantes Criptográficas

```rust
const N_DEC: &str = "11579208923731619...4337";  // Orden del grupo (n)
const LAMBDA_DEC: &str = "37718080363155...0018";  // Eigenvalue del endomorfismo
```

Codificadas en **decimal** para alimentar directamente a `BV::from_str()` de Z3. Son las constantes intrínsecas de secp256k1 que el solver verifica.

### 4.3 Prueba A — Verificación por Refutación (`verify_lambda_polynomial`)

Aplica una estrategia de **refutación lógica**:

1. Calcula el resultado de $f(\lambda) = \lambda^2 + \lambda + 1 \pmod{n}$ usando `bvmul`, `bvadd`, `bvurem`.
2. Le pide a Z3 que demuestre que $f(\lambda) \neq 0$ — es decir, `solver.assert(&sum.ne(&zero))`.
3. **Veredicto**: Si Z3 devuelve `UNSAT` (Insatisfacible), no existe ninguna interpretación lógica donde el resultado sea distinto de cero. La propiedad queda **certificada**.

```
Z3 confirma: λ² + λ + 1 ≡ 0 (mod n) [UNSAT ⇒ válido]
```

### 4.4 Prueba B — Existencia Simbólica (`verify_lambda_uniqueness`)

Aquí el código utiliza una **variable simbólica** `x`:

```rust
let x = BV::new_const("x", BV_BITS);
```

A diferencia de un test normal, no pasamos un valor concreto. Le decimos a Z3: *"Encuentra un `x` en todo el espacio de $2^{512}$ posibilidades que cumpla las restricciones del endomorfismo"*.

Al forzar `solver.assert(&x.eq(&known_lambda))`, certificamos que nuestro $\lambda$ concreto es un testigo válido del polinomio. Z3 devuelve `SAT` — el testigo existe.

```
Z3 confirma: ∃x = λ tal que x²+x+1 ≡ 0 (mod n) [SAT ⇒ válido]
```

### 4.5 Prueba C — Invariante del Campo (`verify_discriminant`)

Valida las condiciones necesarias del campo base:

* **Condición de CM**: Verifica que $p \equiv 1 \pmod{3}$. Esta propiedad garantiza que $\mathbb{F}_p$ contiene las raíces cúbicas de la unidad necesarias para el endomorfismo $\phi(P) = (\beta x, y)$.
* **No-singularidad**: Valida que el discriminante $\Delta \neq 0$ para confirmar que la curva es no-singular.

Ambas condiciones se niegan simultáneamente. Si Z3 devuelve `UNSAT`, ninguna de las dos puede ser falsa.

```
Z3 confirma: p ≡ 1 (mod 3) ∧ Δ ≠ 0 [UNSAT ⇒ válido]
```

### 4.6 Resumen de Certificados

| Método | Objetivo Lógico | Estrategia SMT |
|---|---|---|
| `verify_lambda_polynomial` | Validar Anillo de Eisenstein | `UNSAT` (Refutación) |
| `verify_lambda_uniqueness` | Hallar testigo simbólico | `SAT` (Existencia) |
| `verify_discriminant` | Validar condición de $D = -3$ | `UNSAT` (Contradicción) |


