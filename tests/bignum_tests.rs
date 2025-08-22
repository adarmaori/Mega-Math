use mega_math::bignum::BigNum;

#[test]
fn create_small_number() {
    let n = BigNum::new(13);
    assert_eq!(n.0, vec![13u64]);
}

#[test]
fn resize_number() {
    let mut n = BigNum::new(13);
    n.resize(3);
    assert_eq!(n.0, vec![13u64, 0u64, 0u64]);
}

#[test]
fn truncate_number() {
    let mut n = BigNum::new(13);
    n.resize(3);
    n.truncate();
    assert_eq!(n.0, vec![13u64]);
}

#[test]
fn add_numbers() {
    let mut n = BigNum::new(13);
    let m = BigNum(vec![13]);
    n.add(&m);
    assert_eq!(n.0, vec![26]);
}

#[test]
fn add_big_numbers() {
    let mut n = BigNum(vec![1, 2]);
    let m = BigNum(vec![3, 4]);
    n.add(&m);
    assert_eq!(n.0, vec![4, 6]);
}

#[test]
fn add_numbers_with_carry() {
    let mut n = BigNum(vec![500, 2000]);
    let m = BigNum(vec![0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF]);
    n.add(&m);
    assert_eq!(n.0, vec![499, 2000, 1]);
}

#[test]
fn compare_numbers() {
    let a = BigNum::new(1);
    let b = BigNum::new(2);
    let c = BigNum(vec![1, 2]);
    let d = BigNum(vec![1, 2]);
    let e = BigNum(vec![2, 1]);
    assert!(a < b);
    assert!(b > a);
    assert_eq!(c, d);
    assert!(c > e);
}

#[test]
fn subtract_numbers() {
    let mut a = BigNum::new(5);
    let b = BigNum::new(3);
    let _ = a.sub(&b);
    assert_eq!(a, BigNum::new(2));

    let mut a = BigNum(vec![2, 2, 2]);
    let b = BigNum(vec![1, 2, 2]);
    let _ = a.sub(&b);
    assert_eq!(a, BigNum::new(1));

    let mut a = BigNum(vec![1, 2, 3]);
    let b = BigNum(vec![2, 2, 1]);
    let _ = a.sub(&b);
    assert_eq!(a, BigNum(vec![0xFFFFFFFFFFFFFFFF, 0xFFFFFFFFFFFFFFFF, 1]));

    let mut a = BigNum(vec![1, 2, 3]);
    let b = BigNum(vec![1, 2, 3]);
    let _ = a.sub(&b);
    assert_eq!(a, BigNum(vec![0]));
}

#[test]
fn shift_left() {
    let mut a = BigNum::new(5);
    a.shift_left(64);
    assert_eq!(a, BigNum(vec![0, 5]));
}

#[test]
fn square_numbers() {
    let mut a = BigNum::new(5);
    a.square();
    assert_eq!(a, BigNum::new(25));
    a = BigNum(vec![2, 1]);
    a.square();
    assert_eq!(a, BigNum(vec![4, 4, 1]));
    a = BigNum(vec![1, 1, 1]);
    a.square();
    assert_eq!(a, BigNum(vec![1, 2, 3, 2, 1]));
}
