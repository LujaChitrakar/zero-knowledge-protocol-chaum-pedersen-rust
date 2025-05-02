use std::result;

use num_bigint::BigUint;
use zkp_chaum_pedersen::ZKP;

#[test]
fn test_example() {
    // two generators g and h
    let g = BigUint::from(4u32);
    let h = BigUint::from(9u32);

    // prime number p and order q
    let p = BigUint::from(23u32);
    let q = BigUint::from(11u32);

    // secret x and random form prover k
    let x = BigUint::from(6u32);
    let k = BigUint::from(7u32);
    let zkp = ZKP {
        p,
        q,
        g: g.clone(),
        h: h.clone(),
    };

    // challenge from verifier c
    let c = BigUint::from(4u32);

    // exponentiate
    let y1 = ZKP::exponentiate(&zkp, &g, &x);
    let y2 = ZKP::exponentiate(&zkp, &h, &x);

    let r1 = ZKP::exponentiate(&zkp, &g, &k);
    let r2 = ZKP::exponentiate(&zkp, &h, &k);

    let s = ZKP::solve(&zkp, &k, &c, &x);

    let result = ZKP::verify(&zkp, &r1, &r2, &y1, &y2, &s, &c);

    assert_eq!(y1, BigUint::from(2u32));
    assert_eq!(y2, BigUint::from(3u32));

    assert_eq!(r1, BigUint::from(8u32));
    assert_eq!(r2, BigUint::from(4u32));

    assert_eq!(s, BigUint::from(5u32));

    assert_eq!(result, true);
}

#[test]
fn test_example_with_random_numbers() {
    // two generators g and h
    let g = BigUint::from(4u32);
    let h = BigUint::from(9u32);

    // prime number p and order q
    let p = BigUint::from(23u32);
    let q = BigUint::from(11u32);

    // secret x and random form prover k
    let x = BigUint::from(6u32);
    let k = ZKP::generate_random_less_than(&q);

    let zkp = ZKP {
        p,
        q: q.clone(),
        g: g.clone(),
        h: h.clone(),
    };

    // challenge from verifier c
    let c = ZKP::generate_random_less_than(&q);

    // exponentiate
    let y1 = ZKP::exponentiate(&zkp, &g, &x);
    let y2 = ZKP::exponentiate(&zkp, &h, &x);

    let r1 = ZKP::exponentiate(&zkp, &g, &k);
    let r2 = ZKP::exponentiate(&zkp, &h, &k);

    let s = ZKP::solve(&zkp, &k, &c, &x);

    let result = ZKP::verify(&zkp, &r1, &r2, &y1, &y2, &s, &c);

    assert_eq!(y1, BigUint::from(2u32));
    assert_eq!(y2, BigUint::from(3u32));

    // assert_eq!(r1,BigUint::from(8u32));
    // assert_eq!(r2,BigUint::from(4u32));

    // assert_eq!(s,BigUint::from(5u32));

    assert_eq!(result, true);
}
