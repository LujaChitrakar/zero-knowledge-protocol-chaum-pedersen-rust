use num_bigint::BigUint;
//          g^x mod p
// output = n^exp mod p
pub fn exponentiate(n: &BigUint, exp: &BigUint, p: &BigUint) -> BigUint {
    n.modpow(exp, p)
}

// output = s=(k-c*x) mod q
pub fn solve(k: &BigUint, c: &BigUint, x: &BigUint, q: &BigUint) -> BigUint {
    if *k >= c * x {
        (k - c * x).modpow(&BigUint::from(1u32), q)
        // (k-c*x)%q
    } else {
        q - (c * x - k).modpow(&BigUint::from(1u32), q)
        // q-((c*x-k)%p)
    }
}

// output = a=g^s * X^c mod p || b=h^s * b^c mod p
pub fn verify(
    a: &BigUint,
    b: &BigUint,
    g: &BigUint,
    h: &BigUint,
    y1: &BigUint,
    y2: &BigUint,
    s: &BigUint,
    c: &BigUint,
    p: &BigUint,
) -> bool {
    let cond1 = *a == g.modpow(s, p) * y1.modpow(c, p) % p;
    let cond2 = *b == h.modpow(s, p) * y2.modpow(c, p) % p;

    cond1 && cond2
}
