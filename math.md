# Deriving formula of `rounded-div`

**Observations:**
1. `dividend / divisor` has the same sign as `dividend XOR divisor`.
2. `dividend % divisor` has the same sign as `dividend`.

> [!NOTE]
> Math does not have a notation for integer division so we'll use `floor(a/b)` (i.e. $\lfloor \frac a b \rfloor$) to mean integer division.

> [!NOTE]
> GitHub's rendering engine is unable to render percentage sign in latex in markdown correctly, so we'll use `rem(a, b)` to means getting the remainder of an integer division.

## Both `dividend` and `divisor` are non-negative

Let:
1. $a = dividend ≥ 0$
2. $b = divisor > 0$
3. $q = \lfloor \frac a b \rfloor = \lfloor \frac {dividend} {divisor} \rfloor ≥ 0$
4. $r = rem(a, b) = rem(dividend, divisor) = remainder ≥ 0$

We have:

$$
\frac {dividend} {divisor} = \frac a b =
q + \frac r b
$$

This value rounds up when:

$$
\frac r b ≥ 0.5
\implies \frac {2r} b ≥ 1
\implies 2 r ≥ b
$$

Replacing the variables, we have the following round up condition:

$$
2 \times remainder ≥ divisor
$$

## `dividend` is negative, `divisor` is not

Let:
1. $a = -dividend ≥ 0$
2. $b = divisor > 0$
3. $q = \lfloor \frac a b \rfloor = \lfloor \frac {-dividend} {divisor} \rfloor ≥ 0$
4. $r = rem(a, b) = rem(-dividend, divisor) = -remainder ≥ 0$ (observation 2)

We have:

$$
\frac {dividend} {divisor} = -\frac a b =
-q - \frac r b
$$

This value rounds down when:

$$
\frac r b ≥ 0.5
\implies 2 r ≥ b
$$

Replacing the variables, we have the following round down condition:

$$
2 \times (-remainder) ≥ divisor
$$

## `divisor` is negative, `dividend` is not

Let:
1. $a = dividend ≥ 0$
2. $b = -divisor > 0$
3. $q = \lfloor \frac a b \rfloor = \lfloor \frac {dividend} {-divisor} \rfloor ≥ 0$
4. $r = rem(a, b) = rem(dividend, -divisor) = remainder ≥ 0$ (observation 2)

We have:

$$
\frac {dividend} {divisor} = -\frac a b =
-q - \frac r b
$$

This value rounds down when:

$$
\frac r b ≥ 0.5
\implies 2 r ≥ b
$$

Replacing the variables, we have the following round down condition:

$$
2 \times remainder ≥ -divisor
$$

## Both `dividend` and `divisor` are negative

Let:
1. $a = -dividend ≥ 0$
2. $b = -divisor > 0$
3. $q = \lfloor \frac a b \rfloor = \lfloor \frac {dividend} {divisor} \rfloor ≥ 0$
4. $r = rem(a, b) = rem(-dividend, -divisor) = -remainder ≥ 0$ (observation 2)

We have:

$$
\frac {dividend} {divisor} = \frac a b =
q + \frac r b
$$

This value rounds up when:

$$
\frac r b ≥ 0.5
\implies 2 r ≥ b
$$

Replacing the variables, we have the following round up condition:

$$
2 \times (-remainder) ≥ -divisor
$$

Or,

$$
2 \times remainder ≤ divisor
$$
