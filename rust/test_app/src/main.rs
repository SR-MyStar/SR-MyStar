trait TT {
    fn m(&self) {}
}

struct A;
impl TT for A {}

struct B;
impl TT for B {}

fn r1<T: TT>(p1: T, p2: T) {}
fn r2(p1: impl TT, p2: impl TT) {}

fn main() {
    r1(A {}, B {}); // compilation error
    r2(A {}, B {}); // ok
}
