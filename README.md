# D3-Symmetry

> Análisis del endomorfismo GLV y su impacto en ECDLP para curvas con discriminante CM $D=-3$

Este repositorio implementa una **prueba de concepto** que estudia cómo el discriminante CM bajo ($|D|=3$) de la curva secp256k1 genera estructura algebraica adicional (automorfismos de orden 6) que reduce la complejidad de los algoritmos de resolución del logaritmo discreto elíptico (ECDLP).

## Contenido

1. **Endomorfismo eficiente**: En una curva $y^2 = x^3 + 7$ con $D=-3$, existe $\beta$ ($\beta^3 \equiv 1 \bmod p$) tal que $\phi(x,y) = (\beta x, y)$ es un endomorfismo del grupo.

2. **Grupo de automorfismos**: $|\text{Aut}(E)| = 6$ frente a los 2 habituales en curvas con $|D|$ grande. Este factor se traduce en una reducción teórica de $\sqrt{6} \approx 2.45$ en la complejidad de ECDLP.

3. **Algoritmos implementados**:
   - BSGS estándar vs. BSGS con clases de equivalencia bajo $\text{Aut}(E)$
   - Pollard's $\rho$ estándar vs. Pollard's $\rho$ con Equivalence Class Search

4. **Verificación formal**: Certificación SMT (Z3) de los invariantes algebraicos de la curva.

## Ejecucion

```bash
cargo build

cargo run
```

## Requisitos

### Z3 SMT Solver

El módulo de verificación formal utiliza **Z3** para certificar mediante lógica simbólica y `BitVectors` de 512 bits que los invariantes algebraicos del endomorfismo se satisfacen.

### Instalación de dependencias

Para compilar este proyecto, necesitas el motor Z3 y las librerías de `clang` (requeridas por `bindgen` para generar los bindings de Rust):

* **Arch Linux**:
    ```bash
    sudo pacman -S z3 clang llvm
    ```
* **Ubuntu / Debian**:
    ```bash
    sudo apt install libz3-dev clang libclang-dev
    ```
* **macOS (Homebrew)**:
    ```bash
    brew install z3
    ```
* **Windows**: 
    Se recomienda usar `vcpkg install z3:x64-windows` o descargar los binarios oficiales y configurar la variable de entorno `LIBCLANG_PATH`.

> **Nota:** Si la compilación falla buscando `libclang`, asegúrate de que tu sistema esté actualizado para evitar conflictos entre las versiones de LLVM instaladas.


La ejecución muestra:
- Parámetros de la curva toy ($p=10477$, análoga a secp256k1)
- Verificación del endomorfismo $\phi$ y la constante $\beta$
- Las 6 imágenes bajo $\text{Aut}(E)$
- Resolución de ECDLP por BSGS estándar y con GLV
- Resolución de ECDLP por Pollard's $\rho$ estándar y con Equivalence Class Search
- Verificación formal Z3
- Tabla comparativa de operaciones, speedup y complejidad de memoria

## Contexto

```
secp256k1:  |D| = 3          → |Aut(E)| = 6  → factor sqrt(6) en ECDLP
Curve25519: |D| ~ 2^253      → |Aut(E)| = 2  → sin reducción adicional
```

SafeCurves recomienda $|D| > 2^{100}$. secp256k1 tiene $|D| = 3$.

## Referencias

1. [Fragilidad del Discriminante $|D|=3$](docs/Fragilidad_Discriminante.md) — Análisis matemático del discriminante CM y su relación con $\text{Aut}(E)$.
2. [Análisis Simbólico Z3](docs/Analisis_Simbolico_Z3.md) — Detalle técnico del módulo `glv_invariant_prover_z3.rs` y las pruebas SMT con BitVectors de 512 bits.
3. [Prueba Visual](docs/Prueba_Visual.md) — Salida completa de `cargo run` con tablas y verificaciones.
4. Gallant, Lambert, Vanstone (2001). *Faster Point Multiplication on Elliptic Curves with Efficient Endomorphisms*.
5. [SafeCurves](https://safecurves.cr.yp.to/) — Criterios para curvas elípticas seguras.

## Nota

Este proyecto estudia propiedades algebraicas conocidas de secp256k1. El factor $\sqrt{6}$ en ECDLP es una consecuencia teórica del tamaño de $\text{Aut}(E)$ y está documentado en la literatura (Gallant-Lambert-Vanstone, 2001). La distancia entre la reducción teórica (~6 bits) y un ataque práctico contra los 128 bits de seguridad efectiva sigue siendo enorme. secp256k1 fue elegida por su eficiencia aritmética (curva de Koblitz, $a=0$), lo que permite validación rápida de firmas en hardware.