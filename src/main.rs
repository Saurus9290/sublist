#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(a: &[T], b: &[T]) -> Comparison {
    use Comparison::*;
    match (a.len(), b.len()) {
        (0, 0) => Equal,
        (0, _) => Sublist,
        (_, 0) => Superlist,
        (m, n) if m > n => if a.windows(n).any(|v| v == b) { Superlist } else { Unequal },
        (m, n) if m < n => if b.windows(m).any(|v| v == a) { Sublist } else { Unequal },
        (_, _) => if a == b { Equal } else { Unequal },
    }
}

fn main() {
    let a = [1, 2, 3];
    let b = [1, 2, 3, 4, 5];
    let c = [2, 3];
    let d = [4, 5, 6];

    println!("Sublist check for a and b: {:?}", sublist(&a, &b)); // Sublist
    println!("Sublist check for b and a: {:?}", sublist(&b, &a)); // Superlist
    println!("Sublist check for a and c: {:?}", sublist(&a, &c)); // Unequal
    println!("Sublist check for c and a: {:?}", sublist(&c, &a)); // Sublist
    println!("Sublist check for b and d: {:?}", sublist(&b, &d)); // Unequal
}
