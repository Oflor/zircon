extern crate zircon;

use zircon::*;
use zircon::impls::basic::*;

use std::ops::Add;
use std::collections::{HashMap, BTreeMap};
use std::any::TypeId;
use std::mem::transmute;

#[derive(Debug, Clone, Copy)]
struct Vec2f(f32, f32);

impl Add<Vec2f> for Vec2f {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec2f(self.0 + rhs.0, self.1 + rhs.1)
    }
}

trait System {
    fn update(&mut self, e: EntId, comps: &mut BasicComps);
}

struct Physics;

impl System for Physics {
    fn update(&mut self, e: EntId, comps: &mut BasicComps) {
        let mut pos;
        if let Some(vel) = comps.get::<Vec2f>(e, 1) {
            pos = vel.clone();
        } else {
            return;
        }
        if let Some(p) = comps.get_mut::<Vec2f>(e, 0) {
            pos = pos + *p;
            *p = pos;
        }
    }
}

#[derive(Default)]
struct MySysts {
    systs: Vec<Box<System>>
}

impl Updater for MySysts {
    type UpdateData = ();
    type Comps = BasicComps;
    fn update(&mut self, ents: &mut Ents, comps: &mut BasicComps, _: &()) {
        for s in &mut self.systs {
            for e in ents.iter() {
                s.update(*e, comps);
            }
        }
    }
}

fn main() {
    let mut w = State::<BasicComps, MySysts>::default();
    w.updater.systs.push(Box::new(Physics));
    w.comps.register_comp::<Vec2f>(&()).unwrap();
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
        for j in 4..6 {
            let e = w.new_ent();
        }
    }
    print(&w);
    w.update(&());
    print(&w);
}

fn print(w: &State<BasicComps, MySysts>) {
    for &e in w.ents.iter() {
        println!("Entity #{}: ", e);
        for i in 0..w.comps.len::<Vec2f>(e) {
            println!("Comp #{}: {:?}", i, w.comps.get::<Vec2f>(e, i).unwrap());
        }
    }
}