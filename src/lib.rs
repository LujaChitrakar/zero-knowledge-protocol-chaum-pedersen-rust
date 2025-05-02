use num_bigint::{BigUint, RandBigInt};

// struct
pub struct ZKP {
    pub p: BigUint,
    pub q: BigUint,
    pub g: BigUint,
    pub h: BigUint,
}

impl ZKP {
    //          g^x mod p
    // output = n^exp mod p
    pub fn exponentiate(&self, n: &BigUint, exp: &BigUint) -> BigUint {
        n.modpow(exp, &self.p)
    }

    // output = s=(k-c*x) mod q
    pub fn solve(&self, k: &BigUint, c: &BigUint, x: &BigUint) -> BigUint {
        if *k >= c * x {
            (k - c * x).modpow(&BigUint::from(1u32), &self.q)
            // (k-c*x)%q
        } else {
            &self.q - (c * x - k).modpow(&BigUint::from(1u32), &self.q)
            // q-((c*x-k)%p)
        }
    }

    // output = a=g^s * X^c mod p || b=h^s * b^c mod p
    pub fn verify(
        &self,
        a: &BigUint,
        b: &BigUint,
        y1: &BigUint,
        y2: &BigUint,
        s: &BigUint,
        c: &BigUint,
    ) -> bool {
        let cond1 = *a == &self.g.modpow(s, &self.p) * y1.modpow(c, &self.p) % &self.p;
        let cond2 = *b == &self.h.modpow(s, &self.p) * y2.modpow(c, &self.p) % &self.p;

        cond1 && cond2
    }

    // generate random number
    pub fn generate_random_less_than(bound: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng();
        rng.gen_biguint_below(bound)
    }
}
