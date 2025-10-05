#![allow(dead_code)]

pub trait PairTrait: Copy + Clone {}
// pair struct
#[derive(Debug, Copy, Clone)]
pub struct Pair<T: PairTrait, U: PairTrait> {
	pub x: T,
	pub y: U,
}

impl <T: PairTrait> Pair <T, T> {
	pub fn swap(&self) -> Pair<T, T> {
		Pair { x: self.y, y: self.x }
	}
}

impl <T: PairTrait, U: PairTrait> Pair <T, U> {
	pub fn new(x: T, y: U) -> Pair<T, U> {
		Pair { x, y }
	}
}
