use godot::{builtin::PackedArray, meta::PackedArrayElement};
use rand::Rng;

pub trait PickRandom<T: PackedArrayElement> {
    fn pick_random(&self) -> T;
}
impl<T: PackedArrayElement> PickRandom<T> for PackedArray<T> {
    fn pick_random(&self) -> T {
        let mut rng = rand::rng();
        let index = rng.random_range(0..self.len());

        self.get(index).unwrap()
    }
}
