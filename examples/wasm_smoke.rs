// Deterministic smoke test for wasm targets (no CSPRNG).
// Exercises Fr arithmetic, pairing bilinearity, and the mul_vec malloc path.
use mcl_rust::*;

fn main() {
    assert!(init(CurveType::BLS12_381));

    // Fr arithmetic
    let mut a = Fr::zero();
    a.set_int(3);
    let mut b = Fr::zero();
    b.set_int(5);
    let mut c = Fr::zero();
    Fr::add(&mut c, &a, &b);
    assert_eq!(c.get_str(10), "8");

    // points via hash-and-map
    let mut p = unsafe { G1::uninit() };
    assert!(p.set_hash_of("abc".as_bytes()));
    let mut q = unsafe { G2::uninit() };
    assert!(q.set_hash_of("abc".as_bytes()));

    // e(a P, b Q) == e(P, Q)^(a b)
    let mut ap = unsafe { G1::uninit() };
    let mut bq = unsafe { G2::uninit() };
    G1::mul(&mut ap, &p, &a);
    G2::mul(&mut bq, &q, &b);

    let mut e1 = unsafe { GT::uninit() };
    pairing(&mut e1, &ap, &bq);

    let mut e2 = unsafe { GT::uninit() };
    pairing(&mut e2, &p, &q);
    let mut ab = Fr::zero();
    Fr::mul(&mut ab, &a, &b);
    let mut e2pow = unsafe { GT::uninit() };
    GT::pow(&mut e2pow, &e2, &ab);
    assert!(e1 == e2pow, "pairing bilinearity failed");

    // mul_vec with large n exercises the only malloc/free users in mcl.
    let n = 200usize;
    let mut xs: Vec<G1> = Vec::new();
    let mut ys: Vec<Fr> = Vec::new();
    for i in 0..n {
        xs.push(p.clone());
        let mut y = Fr::zero();
        y.set_int(i as i32 + 1);
        ys.push(y);
    }
    let mut g = unsafe { G1::uninit() };
    G1::mul_vec(&mut g, &xs, &ys);
    // sum_{i=1}^{n} i = n(n+1)/2
    let mut s = Fr::zero();
    s.set_int(n as i32 * (n as i32 + 1) / 2);
    let mut g2 = unsafe { G1::uninit() };
    G1::mul(&mut g2, &p, &s);
    assert!(g == g2, "mul_vec mismatch");

    println!("ok");
}
