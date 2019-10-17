use mcl::*;
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

        let mut z = unsafe { <$t>::uninit() };
        let mut w = unsafe { <$t>::uninit() };

        let a = 256;
        let b = 8;
        x.set_int(a);
        y.set_int(b);
        <$t>::add(&mut z, &x, &y);
        w.set_int(a + b);
        assert_eq!(z, w);

        <$t>::sub(&mut z, &x, &y);
        w.set_int(a - b);
        assert_eq!(z, w);

        <$t>::mul(&mut z, &x, &y);
        w.set_int(a * b);
        assert_eq!(z, w);

        <$t>::div(&mut z, &x, &y);
        w.set_int(a / b);
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
        #[allow(non_snake_case)]
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
        <$t>::add(&mut P2, &$P, &$P);
        assert_eq!(P2, P1);
        let mut y: Fr = Fr::from_int(1);
        <$t>::mul(&mut P2, &$P, &y);
        assert_eq!(P2, $P);
        y.set_int(2);
        <$t>::mul(&mut P2, &$P, &y);
        assert_eq!(P2, P1);
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

#[test]
#[allow(non_snake_case)]
fn test() {
    assert_eq!(mem::size_of::<Fr>(), 32);
    assert_eq!(mem::size_of::<Fp>(), 48);
    assert_eq!(mem::size_of::<Fp2>(), 48 * 2);
    assert_eq!(mem::size_of::<G1>(), 48 * 3);
    assert_eq!(mem::size_of::<G2>(), 48 * 2 * 3);
    assert_eq!(mem::size_of::<GT>(), 48 * 12);
    assert!(init(CurveType::BLS12_381));
    assert_eq!(get_fp_serialized_size(), 48);
    assert_eq!(get_g1_serialized_size(), 48);
    assert_eq!(get_g2_serialized_size(), 48 * 2);
    assert_eq!(get_gt_serialized_size(), 48 * 12);
    assert_eq!(get_fr_serialized_size(), 32);

    // Fp
    assert_eq!(get_field_order(), "4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787");
    // Fr
    assert_eq!(
        get_curve_order(),
        "52435875175126190479447740508185965837690552500527637822603658699938581184513"
    );

    field_test! {Fr};
    field_test! {Fp};

    let P = G1::from_str("1 3685416753713387016781088315183077757961620795782546409894578378688607592378376318836054947676345821548104185464507 1339506544944476473020471379941921221584933875938349620426543736416511423956333506472724655353366534992391756441569", 10).unwrap();
    let Q = G2::from_str("1 352701069587466618187139116011060144890029952792775240219908644239793785735715026873347600343865175952761926303160 3059144344244213709971259814753781636986470325476647558659373206291635324768958432433509563104347017837885763365758 1985150602287291935568054521177171638300868978215655730859378665066344726373823718423869104263333984641494340347905 927553665492332455747201965776037880757740193453592970025027978793976877002675564980949289727957565575433344219582", 10).unwrap();

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
}
