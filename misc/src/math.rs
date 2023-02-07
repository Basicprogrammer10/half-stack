/// Calculate the greatest common denomanator of `a` and `b`
pub fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
