use mcl_rust::*;
use std::mem;

macro_rules! field_test {
    ($t:ty) => {{
        let mut x = <$t>::zero();
        assert!(x.is_valid());
        assert!(x.is_zero());
        assert!(!x.is_one());
        x.set_int(1);
        assert!(!x.is_zero());
        assert!(x.is_one());
        let mut y = <$t>::from_int(1);
        assert!(y.is_valid());
        assert_eq!(x, y);
        y.set_int(2);
        assert!(x != y);
        x.set_str("65535", 10);
        y.set_str("ffff", 16);
        assert!(x.is_valid());
        assert_eq!(x, y);
        x.set_int(123);
        assert!(x.is_odd());
        x.set_int(124);
        assert!(!x.is_odd());
        assert!(!x.is_negative());
        x.set_int(-124);
        assert!(x.is_negative());
        x.set_int(5);
        y.set_int(2);
        assert_eq!(x.cmp(&y), 1);
        assert_eq!(x.cmp(&x), 0);
        assert_eq!(y.cmp(&x), -1);
        y.set_int(-2); // unsigned large number
        assert_eq!(x.cmp(&y), -1);
        assert_eq!(y.cmp(&x), 1);

        let mut z = unsafe { <$t>::uninit() };
        let mut w = unsafe { <$t>::uninit() };

        let a = 256;
        let b = 8;
        x.set_int(a);
        y.set_int(b);
        <$t>::add(&mut z, &x, &y);
        w.set_int(a + b);
        assert_eq!(z, w);
        assert_eq!(w, (&x + &y));
        z = x.clone();
        z += &y;
        assert_eq!(z, w);

        <$t>::sub(&mut z, &x, &y);
        w.set_int(a - b);
        assert_eq!(z, w);
        assert_eq!(w, (&x - &y));
        z = x.clone();
        z -= &y;
        assert_eq!(z, w);

        <$t>::mul(&mut z, &x, &y);
        w.set_int(a * b);
        assert_eq!(z, w);
        assert_eq!(w, (&x * &y));
        z = x.clone();
        z *= &y;
        assert_eq!(z, w);

        <$t>::div(&mut z, &x, &y);
        w.set_int(a / b);
        assert_eq!(z, w);
        assert_eq!(z, (&x / &y));
        z = x.clone();
        z /= &y;
        assert_eq!(z, w);

        assert!(x.set_little_endian_mod(&[1, 2, 3, 4, 5]));
        assert_eq!(x.get_str(16), "504030201");
        <$t>::sqr(&mut y, &x);
        <$t>::mul(&mut z, &x, &x);
        assert_eq!(y, z);

        assert!(<$t>::square_root(&mut w, &y));
        if w != x {
            <$t>::neg(&mut z, &w);
            assert_eq!(x, z);
        }
    }};
}

macro_rules! ec_test {
    ($t:ty, $f:ty, $P:expr) => {
        assert!($P.is_valid());
        assert!(!$P.is_zero());
        let mut P1 = <$t>::zero();
        assert!(P1.is_zero());
        assert_ne!(P1, $P);
        <$t>::neg(&mut P1, &$P);
        let mut x: $f = unsafe { <$f>::uninit() };
        <$f>::neg(&mut x, &P1.y);
        assert_eq!(&x, &$P.y);

        <$t>::dbl(&mut P1, &$P);
        let mut P2: $t = unsafe { <$t>::uninit() };
        let mut P3: $t = unsafe { <$t>::uninit() };
        <$t>::add(&mut P2, &$P, &$P);
        assert_eq!(P2, P1);
        <$t>::add(&mut P3, &P2, &$P);
        assert_eq!(P3, (&P2 + &$P));
        assert_eq!(P2, (&P3 - &$P));
        let mut y: Fr = Fr::from_int(1);
        <$t>::mul(&mut P2, &$P, &y);
        assert_eq!(P2, $P);
        y.set_int(2);
        <$t>::mul(&mut P2, &$P, &y);
        assert_eq!(P2, P1);
        y.set_int(3);
        <$t>::mul(&mut P2, &$P, &y);
        assert_eq!(P2, P3);
        P2 = P1.clone();
        P2 += &$P;
        assert_eq!(P2, P3);

        P2 -= &$P;
        assert_eq!(P2, P1);
        P1.set_hash_of(b"abcd");
        assert!(P1.is_valid());

        // mul_vec test
        let tbl = [0, 1, 2, 3, 4, 15, 16, 50, 300];
        for n in tbl {
            let mut xs: Vec<$t> = Vec::new();
            let mut ys: Vec<Fr> = Vec::new();
            xs.resize_with(n, Default::default);
            ys.resize_with(n, Default::default);
            let mut y = <Fr>::zero();
            for i in 0..n {
                ys[i].set_by_csprng();
                <$t>::mul(&mut xs[i], &$P, &ys[i]);
                let mut yy = unsafe { Fr::uninit() };
                <Fr>::sqr(&mut yy, &ys[i]);
                y += &yy;
            }
            let mut g1 = unsafe { <$t>::uninit() };
            let mut g2 = unsafe { <$t>::uninit() };
            <$t>::mul_vec(&mut g1, &xs, &ys);
            <$t>::mul(&mut g2, &$P, &y);
            assert_eq!(g1.get_str(16), g2.get_str(16));
        }
    };
}

macro_rules! serialize_test {
    ($t:ty, $x:expr) => {
        let buf = $x.serialize();
        let mut y: $t = unsafe { <$t>::uninit() };
        assert!(y.deserialize(&buf));
        assert_eq!($x, y);
    };
}

macro_rules! str_test {
    ($t:ty, $x:expr) => {
        for base in [10, 16] {
            let s = $x.get_str(base);
            let mut y: $t = unsafe { <$t>::uninit() };
            assert!(y.set_str(&s, base));
            assert_eq!($x, y);
        }
    };
}

#[allow(non_snake_case)]
fn testCurve(curve: CurveType) {
    assert_eq!(mem::size_of::<Fr>(), 32);
    assert_eq!(mem::size_of::<Fp>(), 48);
    assert_eq!(mem::size_of::<Fp2>(), 48 * 2);
    assert_eq!(mem::size_of::<G1>(), 48 * 3);
    assert_eq!(mem::size_of::<G2>(), 48 * 2 * 3);
    assert_eq!(mem::size_of::<GT>(), 48 * 12);
    assert!(init(curve));
    let b = match curve {
        CurveType::BN254 => 32,
        _ => 48,
    };
    assert_eq!(get_fp_serialized_size(), b);
    assert_eq!(get_g1_serialized_size(), b);
    assert_eq!(get_g2_serialized_size(), b * 2);
    assert_eq!(get_gt_serialized_size(), b * 12);
    assert_eq!(get_fr_serialized_size(), 32);

    field_test! {Fr};
    field_test! {Fp};

    let mut P = G1::zero();
    let mut Q = G2::zero();
    P.set_hash_of(b"abc");
    Q.set_hash_of(b"abc");

    match curve {
        CurveType::BN254 => {
            // Fp
            assert_eq!(
                get_field_order(),
                "16798108731015832284940804142231733909889187121439069848933715426072753864723"
            );
            // Fr
            assert_eq!(
                get_curve_order(),
                "16798108731015832284940804142231733909759579603404752749028378864165570215949"
            );
        }
        CurveType::BLS12_381 => {
            // Fp
            assert_eq!(get_field_order(), "4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787");
            // Fr
            assert_eq!(
                get_curve_order(),
                "52435875175126190479447740508185965837690552500527637822603658699938581184513"
            );
        }
        CurveType::BLS12_377 => {
            // Fp
            assert_eq!(get_field_order(), "258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177");
            // Fr
            assert_eq!(
                get_curve_order(),
                "8444461749428370424248824938781546531375899335154063827935233455917409239041"
            );
        }
        _ => {
            //			panic!("not supported curve");
        }
    }

    ec_test! {G1, Fp, P};
    ec_test! {G2, Fp2, Q};

    let x = Fr::from_int(3);
    let y = Fp::from_int(-1);
    let mut e = unsafe { GT::uninit() };
    pairing(&mut e, &P, &Q);
    serialize_test! {Fr, x};
    serialize_test! {Fp, y};
    serialize_test! {G1, P};
    serialize_test! {G2, Q};
    serialize_test! {GT, e};
    serialize_test! {Fp2, Q.x};

    str_test! {Fr,x};
    str_test! {Fp, y};
    str_test! {G1, P};
    str_test! {G2, Q};
    str_test! {GT, e};
}

#[test]
fn test_all() {
    testCurve(CurveType::BN254);
    testCurve(CurveType::BLS12_381);
    testCurve(CurveType::BLS12_377);
}
