extern crate zircon;

use zircon::*;
use zircon::impls::*;
use zircon::impls::basic::*;

use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Vec2f(f32, f32);

impl Add<Vec2f> for Vec2f {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec2f(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn main() {
    let updater = |e: EntId, _: &Ents, comps: &mut BasicComps, _: &()| {
        let mut pos_new;
        if let (Some(&pos), Some(&vel)) = (comps.get::<Vec2f>(e, 0), comps.get::<Vec2f>(e, 1)) {
            pos_new = pos.clone() + vel.clone();
        } else {
            return;
        }
        comps.replace::<Vec2f>(e, 0, pos_new);
    };
    let mut w = State::new(BasicComps::default(), updater);
    w.comps.register_comp::<Vec2f>(&()).unwrap();
    //Creating entities
    for i in 0..4 {
        for j in 0..2 {
            let e = w.new_ent();
            w.comps.insert(e,
                           Vec2f(0.125 * (j + i * 6) as f32, 0.25 * (j + i * 6) as f32));
            w.comps.insert(e,
                           Vec2f(0.25 * (j + i * 6) as f32, 0.125 * (j + i * 6) as f32));
        }
        for j in 2..4 {
            let e = w.new_ent();
            w.comps.insert(e,
                           Vec2f(0.125 * (j + i * 6) as f32, 0.25 * (j + i * 6) as f32));
        }
        for _ in 4..6 {
            let _ = w.new_ent();
        }
    }
    //Updating and printing the state
    println!("===== Iteration #0 ======");
    print(&w);
    w.update(&());
    println!("===== Iteration #1 ======");
    print(&w);
}

fn print<D, U: Updater<BasicComps, D>>(w: &State<BasicComps, U, D>) {
    for &e in w.ents.iter() {
        println!("Entity #{}: ", e);
        for i in 0..w.comps.len::<Vec2f>(e) {
            println!("Comp #{}: {:?}", i, w.comps.get::<Vec2f>(e, i).unwrap());
        }
    }
}