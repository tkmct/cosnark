# Cosnark

## What is Cosnark?

#[Collaborative zk snarks](https://eprint.iacr.org/2021/1530.pdf) proving system levaraging existing toolchains such as circom and halo2 as much as possible.

## What to achieve?

- Collaboratively generate zk snarks proof
- Compatibility with existing toolchains (circom or halo2)

### How to make compatible with Circom?

Circom is a domain-specific language to define arithmetic circuits for zk snarks. Circom compiler generates R1CS files. In order to use circom toolchain in cosnark, proving system of cosnark needs to read the R1CS file provided by circom.

### How to make compatible with Halo2?

Arithmetisation

- [ ] Addition
- [ ] Multiplication
- [ ] Custom gates
- [ ] Lookup table
