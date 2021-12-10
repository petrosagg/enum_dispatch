use enum_dispatch::enum_dispatch;

struct Nothing;
struct One(u8);
struct Array<const N: usize>([u8; N]);

impl<const N: usize> Trait<N> for Nothing {
    fn get_array(&self) -> [u8; N] {
        [0; N]
    }
}

impl<const N: usize> Trait<N> for One {
    fn get_array(&self) -> [u8; N] {
        [self.0; N]
    }
}

impl<const N: usize> Trait<N> for Array<N> {
    fn get_array(&self) -> [u8; N] {
        self.0
    }
}

#[enum_dispatch(Enum<3>)]
trait Trait<const N: usize> {
    fn get_array(&self) -> [u8; N];
}

#[enum_dispatch]
enum Enum<const N: usize> {
    Nothing,
    One,
    Array(Array<N>),
}

#[test]
fn main() {
    let three: Enum<3> = One(2).into();
    assert_eq!(three.get_array(), [2, 2, 2]);
}
