use libpaillier::{
    unknown_order::BigNumber,
    *
};
use num_bigint::{BigInt, ToBigInt, Sign};
use num_traits::ToPrimitive;

#[test]
fn test() {
    let res = DecryptionKey::random();
    let sk = res.unwrap();
    let pk = EncryptionKey::from(&sk);
    let m1 = BigNumber::from(7);
    let m2 = BigNumber::from(6);

    let res1 = pk.encrypt(&m1.to_bytes(), None);

    let (c1, _) = res1.unwrap();
    
    // Multiply the ciphertext
    let res = pk.mul(&c1, &m2);
    assert!(res.is_some());
    let c2 = res.unwrap();
    let res = sk.decrypt(&c2);
    assert!(res.is_some());
    let bytes = res.unwrap();
    let m3 = BigNumber::from_slice(bytes.as_slice());
    // Prove homomorphic properties worked
    assert_eq!(m3, BigNumber::from(42));
}