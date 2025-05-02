use std::result;

use num_bigint::BigUint;
use zkp_chaum_pedersen::{exponentiate, solve, verify};

#[test]
fn test_example(){
    // two generators g and h
    let g=BigUint::from(4u32);
    let h=BigUint::from(9u32);

    // prime number p and order q
    let p=BigUint::from(23u32);
    let q=BigUint::from(11u32);

    // secret x and random form prover k    
    let x=BigUint::from(6u32);
    let k=BigUint::from(7u32);

    // challenge from verifier c
    let c=BigUint::from(4u32);

    // exponentiate
    let y1=exponentiate(&g, &x, &p);
    let y2=exponentiate(&h, &x, &p);

    let r1=exponentiate(&g, &k, &p);
    let r2=exponentiate(&h, &k, &p);
    
    let s=solve(&k, &c, &x, &q);

    let result=verify(&r1, &r2, &g,&h, &y1, &y2, &s, &c, &p);

    assert_eq!(y1,BigUint::from(2u32));
    assert_eq!(y2,BigUint::from(3u32));

    assert_eq!(r1,BigUint::from(8u32));
    assert_eq!(r2,BigUint::from(4u32));

    assert_eq!(s,BigUint::from(5u32));

    assert_eq!(result,true);
}