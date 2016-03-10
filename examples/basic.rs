extern crate zircon;
extern crate chrono;
use chrono::duration::Duration;

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
    let updater = |e: EntId, _: &Ents, comps: &BasicComps, _: &()| {
        let mut pos_new;
        let mut vec: Vec<<BasicComps as Comps>::Diff> = Vec::with_capacity(1);
        if let (Some(&pos), Some(&vel)) = (comps.get::<Vec2f>(e, 0), comps.get::<Vec2f>(e, 1)) {
            pos_new = pos.clone() + vel.clone();
        } else {
            return vec;
        }
        vec.push(<BasicComps as Comps>::Diff::replace::<Vec2f>(comps, e, 0, pos_new));
        vec
    };
    let mut w = State::new(BasicComps::default(), updater);
    w.comps.register_comp::<Vec2f>(&()).unwrap();
    //Creating entities
    for i in 0..65536 {
        let e = w.new_ent();
        w.comps.insert(e,
                       Vec2f(0.125 * i as f32, 0.25 * i as f32));
        w.comps.insert(e,
                       Vec2f(0.25 * i as f32, 0.125 * i as f32));
    }
    //Updating and printing the state
    //print(&w);
    for _ in 0..16 {
        println!("Took: {} ms", Duration::span(|| w.update(&())).num_milliseconds());
    }
    //print(&w);
}

fn print<D, U: Updater<BasicComps, D>>(w: &State<BasicComps, U, D>) {
    for &e in w.ents.iter() {
        println!("Entity #{}: ", e);
        for i in 0..w.comps.len::<Vec2f>(e) {
            println!("Comp #{}: {:?}", i, w.comps.get::<Vec2f>(e, i).unwrap());
        }
    }
}