# D3-Symmetry

> Demostracion del ataque GLV sobre una curva toy con discriminante |D|=3

Este repositorio es una **prueba de concepto (PoC)** que demuestra como el discriminante CM bajo ($|D|=3$) de la curva secp256k1 de Bitcoin crea simetrias explotables que aceleran la resolucion del problema del logaritmo discreto (ECDLP).

## Que demuestra

1. **Endomorfismo eficiente**: En una curva $y^2 = x^3 + 7$ con $|D|=3$, existe una constante $\beta$ ($\beta^3 \equiv 1 \bmod p$) tal que $\phi(x,y) = (\beta x, y)$ es un automorfismo.

2. **Grupo de automorfismos 3x mas grande**: $|\text{Aut}(E)| = 6$ en lugar de los 2 habituales. Cada automorfismo extra es una simetria explotable.

3. **ECDLP mas rapido**: Un atacante usando BSGS con las 6 simetrias resuelve el logaritmo discreto $\sqrt{6} \approx 2.45$ veces mas rapido que con BSGS estandar.


**Cero dependencias externas** - todo con aritmetica u64 y la biblioteca estandar de Rust.

## Ejecucion

```bash
cargo build

cargo run
```

## Requisitos: Z3 SMT Solver

Este laboratorio utiliza **Z3** para certificar que la simetría $|D|=3$ es una obligación lógica y no un error de implementación. A diferencia de un test normal, Z3 usa **lógica simbólica** y `BitVectors` para demostrar la invarianza de la curva.

Este laboratorio utiliza **Z3** para certificar que la simetría $|D|=3$ es una obligación lógica y no un error de implementación. A diferencia de un test normal, Z3 usa **lógica simbólica** y `BitVectors` para demostrar la invarianza de la curva de forma formal.

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


La demo ejecuta en milisegundos y muestra:
- Parametros de la curva toy (p=10477, analogia directa con Bitcoin)
- Verificacion del endomorfismo y la constante beta
- Las 6 imagenes bajo el grupo de automorfismos
- Resolucion del ECDLP por BSGS estandar
- Resolucion del ECDLP por BSGS con GLV (explotando D=-3)
- Tabla comparativa de operaciones y speedup

## El argumento central

```
secp256k1:  |D| = 3          → 6 automorfismos → ECDLP sqrt(6)x mas rapido
Curve25519: |D| ~ 2^253      → 2 automorfismos → sin atajos estructurales
```

SafeCurves recomienda $|D| > 2^{100}$. Bitcoin usa $|D| = 3$.

## Referencias

1. [La Fragilidad del Discriminante |D|=3](docs/Fragilidad_Discriminante.md) - Analisis matematico.
2. Gallant, Lambert, Vanstone (2001). *Faster Point Multiplication on Elliptic Curves with Efficient Endomorphisms*.
3. [SafeCurves](https://safecurves.cr.yp.to/) - Criterios para curvas elipticas seguras.

## Disclaimer

A ver, siendo realistas, este proyecto es únciamente una PoC tonta sobre ""vulnerabilidades"" (Lo pongo en duda¿?) algebráicas ya conocidas en la comunidad de criptografía, el margen de seguridad es muy grande, secp256k1 es eficiente sobre hardware (al ser una curva de Koblitz (ASICs)), esta propiedad es la que permite a la red validar miles de transaccioens por segundo, de la seguridad "teórica" a la "práctica" sigue habiendo un abismo enorme