## Notation

The elliptic curve equation is `E: y^2 = x^3 + b`.

- `Fp` : a finite field of prime order `p`, over which the curve is defined.
- `Fr` : a finite field of prime order `r`.
- `Fp2` : field extension over Fp of degree 2. Fp[i] / (i^2 + u).
- `Fp6` : field extension over Fp2 of degree 3. Fp2[v] / (v^3 - Xi) where Xi = i + xi_a.
- `Fp12` : field extension over Fp6 of degree 2. Fp6[w] / (w^2 - v).
- `G1` : the cyclic subgroup of E(Fp).
- `G2` : the cyclic subgroup of the inverse image of E'(Fp^2) under a twisting isomorphism from E' to E.
- `GT` : the cyclic subgroup of Fp12.
  - `G1`, `G2`, and `GT` have the order `r`.

The pairing e: G1 x G2 -> GT is the optimal ate pairing.

mcl treats `G1` and `G2` as additive groups and `GT` as a multiplicative group.

### Curve Parameters
r = |G1| = |G2| = |GT|

curveType   | b|u|xi_a| r and p |
------------|-|-|-|------------------|
BN254       | 2|1|1|r = 0x2523648240000001ba344d8000000007ff9f800000000010a10000000000000d <br> p = 0x2523648240000001ba344d80000000086121000000000013a700000000000013 |
BN_SNARK1|3|1|9|r = 0x30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001 <br> p = 0x30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47|
BLS12_381   | 4|1|1|r = 0x73eda753299d7d483339d80809a1d80553bda402fffe5bfeffffffff00000001 <br> p = 0x1a0111ea397fe69a4b1ba7b6434bacd764774b84f38512bf6730d2a0f6b0f6241eabfffeb153ffffb9feffffffffaaab |
BLS12_377|1|5|0|r = 0x12ab655e9a2ca55660b44d1e5c37b00159aa76fed00000010a11800000000001 <br> p = 0x1ae3a4617c510eac63b05c06ca1493b1a22d9f300f5138f1ef3622fba094800170b5d44300000008508c00000000001
BN381       | 2|1|1|r = 0x240026400f3d82b2e42de125b00158405b710818ac000007e0042f008e3e00000000001080046200000000000000000d <br> p = 0x240026400f3d82b2e42de125b00158405b710818ac00000840046200950400000000001380052e000000000000000013 |

## Thread safety
All functions except initialization and global setting changes are thread-safe.
