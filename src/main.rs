use mega_math::bignum::BigNum;

fn main() {
    println!("=== Mega Math Demo: Running Pepin's test ===");
    let mut n = BigNum::new(3);
    for i in 0..10 {
        n.square();
        println!("{i}");
    }
    println!("{:?}", n);
}
