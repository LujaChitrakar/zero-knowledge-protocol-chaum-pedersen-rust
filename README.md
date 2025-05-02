# Chaum-Pedersen Zero-Knowledge Protocol (Rust)

This Rust project implements the **Chaum-Pedersen zero-knowledge proof protocol**, which allows a prover to convince a verifier that two discrete logarithms are equal without revealing the actual secret.

## ✨ Features

- Implementation of the Chaum-Pedersen protocol in pure Rust
- Secure generation and verification of zero-knowledge proofs
- Based on the Discrete Logarithm Problem
- Modular and readable codebase

## 📖 What is Chaum-Pedersen?

The Chaum-Pedersen protocol proves the equality of discrete logarithms in two groups:

log_g(h) = log_g'(h')

That is, the prover can show knowledge of a secret `x` such that:

g^x = h and g'^x = h'

without revealing `x`. This is useful in privacy-preserving cryptographic systems such as anonymous credentials, secure voting, and confidential authentication.
