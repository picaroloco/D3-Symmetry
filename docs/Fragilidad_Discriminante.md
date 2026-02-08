# La Fragilidad del Discriminante $|D|=3$

## 1. Definición de la Curva

La curva **secp256k1** de Bitcoin se define como:

$$y^2 = x^3 + 7 \quad \text{sobre } \mathbb{F}_p$$

donde $p = 2^{256} - 2^{32} - 977$.

El **discriminante del cuerpo de multiplicación compleja** (CM discriminant) de esta curva es:

$$D = -3$$

## 2. Por qué $|D|=3$ es un problema

### 2.1 Criterios de SafeCurves

Una curva elíptica segura bajo los criterios de [SafeCurves](https://safecurves.cr.yp.to/) requiere un $|D|$ grande, típicamente:

$$|D| > 2^{100}$$

Esto garantiza que la estructura algebraica del anillo de endomorfismos $\text{End}(E)$ sea lo suficientemente compleja como para resistir ataques estructurales. Bitcoin utiliza $|D| = 3$, un valor MUY BAJO.

| Curva | $\lvert D \rvert$ | Segura (SafeCurves) |
|-------|-------|---------------------|
| Curve25519 | $\approx 2^{253}$ | Sí |
| Ed448 | $\approx 2^{445}$ | Sí |
| **secp256k1** | **3** | **No** |

### 2.2 El Anillo de Endomorfismos

Cuando $D = -3$, el anillo de endomorfismos es:

$$\text{End}(E) \cong \mathbb{Z}[\zeta_3]$$

donde $\zeta_3$ es una raiz cubica primitiva de la unidad. Este anillo es **mucho mas rico** que el $\mathbb{Z}$ generico, lo que significa que la curva tiene **estructura algebraica adicional** que un atacante puede explotar.

## 3. El Endomorfismo Eficiente

### 3.1 Existencia de $\phi$

El discriminante $D = -3$ permite la existencia de un endomorfismo eficiente:

$$\phi: E \to E$$
$$\phi(P) = \lambda P$$

donde $\lambda$ es una raiz del polinomio caracteristico:

$$\lambda^2 + \lambda + 1 \equiv 0 \pmod{n}$$

siendo $n$ el orden del grupo.

### 3.2 La Constante $\beta$

En la practica, este endomorfismo se manifiesta como una constante $\beta$ con la propiedad:

$$\beta^3 \equiv 1 \pmod{p}$$

Para cualquier punto $P = (x, y)$ en la curva:

$$P' = (\beta \cdot x, \ y)$$

**tambien esta en la curva**. Esto se demuestra trivialmente:

$$(\beta x)^3 + 7 = \beta^3 x^3 + 7 = x^3 + 7 = y^2 \quad \checkmark$$

### 3.3 Valores Concretos

```
beta  = 0x7AE96A2B657C2AD8628D667F3BCD47C2A54A0104E8452093557697413B07023B
beta2 = 0x851695D49A83F8EF919BB86153CBCB16630FB68AED0A766A3EC693D68E6AFA40
```

Ambos satisfacen $\beta^3 \equiv 1 \pmod{p}$.

## 4. El Ataque GLV (Gallant-Lambert-Vanstone)

### 4.1 Descomposicion del Escalar

El metodo GLV explota el endomorfismo para **acelerar la multiplicacion escalar**. Dado un escalar $k$ de 256 bits, se descompone en:

$$k = k_1 + k_2 \cdot \lambda \pmod{n}$$

donde $k_1, k_2$ son de **128 bits cada uno**.

### 4.2 Implicacion en Seguridad

La multiplicacion escalar se reduce a:

$$kP = k_1 P + k_2 \phi(P)$$

Esto significa que en lugar de trabajar en un espacio de $2^{256}$ posibilidades, un atacante trabaja en un espacio de $2^{128} \times 2^{128}$, lo que permite:

1. **Multi-exponenciacion simultanea**: Dos multiplicaciones de 128 bits en paralelo.
2. **Reduccion del espacio de busqueda**: Algoritmos como Pollard's rho obtienen un factor constante de mejora.
3. **Optimizacion para ASICs**: La estructura predecible es ideal para hardware especializado.

### 4.3 Formalizacion de la Debilidad

Sea $E(\mathbb{F}_p)$ el grupo de puntos de la curva con orden $n$. El endomorfismo $\phi$ induce un automorfismo del grupo:

$$\text{Aut}(E) \supsetneq \{[1], [-1]\}$$

En una curva con $|D|$ grande, $\text{Aut}(E) = \{[1], [-1]\}$ (solo la identidad y la negacion). En secp256k1:

$$\text{Aut}(E) = \{[1], [\zeta_3], [\zeta_3^2], [-1], [-\zeta_3], [-\zeta_3^2]\}$$

El grupo de automorfismos es **6 veces mas grande** de lo necesario. Cada automorfismo adicional es una **simetria explotable**.

---

**Referencias:**
- Gallant, R., Lambert, R., Vanstone, S. (2001). *Faster Point Multiplication on Elliptic Curves with Efficient Endomorphisms*.
- Bernstein, D.J., Lange, T. *SafeCurves: choosing safe curves for elliptic-curve cryptography*. https://safecurves.cr.yp.to/
- Certicom Research. *SEC 2: Recommended Elliptic Curve Domain Parameters*. Version 2.0, 2010.
