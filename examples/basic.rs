extern crate zircon;

use zircon::State;
use zircon::comp::*;
use zircon::syst::*;

use std::ops::Add;

#[derive(Debug)]
struct Vec2f(f32, f32);

impl Add<Vec2f> for Vec2f {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec2f(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn main() {}
