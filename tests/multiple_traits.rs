use enum_dispatch::enum_dispatch;
use core::convert::TryInto;

#[enum_dispatch]
trait Trait1 {
    fn describe(&self) -> char;
}

impl Trait1 for A {
    fn describe(&self) -> char { 'A' }
}

impl Trait1 for B {
    fn describe(&self) -> char { 'B' }
}

impl Trait1 for C {
    fn describe(&self) -> char { 'C' }
}

#[enum_dispatch]
trait Trait2 {
    fn as_string(&self) -> String;
}

impl Trait2 for A {
    fn as_string(&self) -> String { "A".to_string() }
}

impl Trait2 for B {
    fn as_string(&self) -> String { " B ".to_string() }
}

impl Trait2 for C {
    fn as_string(&self) -> String { self.describe().to_string() }
}

pub struct A;
pub struct B;
pub struct C;

#[enum_dispatch(Trait1)]
#[enum_dispatch(Trait2)]
pub enum Traited {
    A,
    B,
    C,
}

#[test]
fn main() {
    let a = Traited::from(A);
    let b = Traited::from(B);
    let c = Traited::from(C);

    assert_eq!(a.describe(), 'A');
    assert_eq!(b.describe(), 'B');
    assert_eq!(c.describe(), 'C');

    assert_eq!(a.as_string(), "A".to_string());
    assert_eq!(b.as_string(), " B ".to_string());
    assert_eq!(c.as_string(), "C".to_string());

    let b_from_c: Result<B, _> = c.try_into();
    assert!(b_from_c.is_err());
    assert_eq!(b_from_c.err().unwrap(), "Tried to convert variant C to B");

    let a_from_a: Result<A, _> = a.try_into();
    assert!(a_from_a.is_ok());
}
