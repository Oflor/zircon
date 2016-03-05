extern crate zircon;

use zircon::*;

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

#[derive(Default, Debug)]
struct MyComps {
    comps: HashMap<TypeId, BTreeMap<EntId, Vec<Vec2f>>>,
}

impl Comps for MyComps {
    type RegData = ();
    type RegError = ();
    fn register_comp<T: Comp>(&mut self, _: &()) -> Result<(), ()> {
        self.comps.insert(TypeId::of::<T>(), BTreeMap::new());
        Ok(())
    }

    fn get<T: Comp>(&self, e: EntId, idx: usize) -> Option<&T> {
        if let Some(btm) = self.comps.get(&TypeId::of::<T>()) {
            if let Some(vec) = btm.get(&e) {
                unsafe { transmute::<_, &Vec<T>>(vec) }.get(idx)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn get_mut<T: Comp>(&mut self, e: EntId, idx: usize) -> Option<&mut T> {
        if let Some(btm) = self.comps.get_mut(&TypeId::of::<T>()) {
            if let Some(vec) = btm.get_mut(&e) {
                unsafe { transmute::<_, &mut Vec<T>>(vec) }.get_mut(idx)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn insert<T: Comp>(&mut self, e: EntId, comp: T) -> usize {
        if let Some(btm) = self.comps.get_mut(&TypeId::of::<T>()) {
            if let Some(vec) = btm.get_mut(&e) {
                let mut vec = unsafe { transmute::<_, &mut Vec<T>>(vec) };
                vec.push(comp);
                return vec.len() - 1;
            }
            let mut vec = Vec::<T>::with_capacity(1);
            vec.push(comp);
            btm.insert(e, unsafe { transmute(vec) });
            0
        } else {
            panic!("Unregistered type of component")
        }
    }

    fn remove<T: Comp>(&mut self, e: EntId, idx: usize) -> Option<T> {
        if let Some(btm) = self.comps.get_mut(&TypeId::of::<T>()) {
            if let Some(vec) = btm.get_mut(&e) {
                let vec = unsafe { transmute::<_, &mut Vec<T>>(vec) };
                if idx < vec.len() {
                    Some(vec.remove(idx))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }

    fn remove_all<T: Comp>(&mut self, e: EntId) {
        if let Some(btm) = self.comps.get_mut(&TypeId::of::<T>()) {
            btm.remove(&e);
        }
    }

    fn len<T: Comp>(&self, e: EntId) -> usize {
        if let Some(btm) = self.comps.get(&TypeId::of::<T>()) {
            if let Some(vec) = btm.get(&e) {
                unsafe { transmute::<_, &Vec<T>>(vec) }.len()
            } else {
                0
            }
        } else {
            0
        }
    }
}

#[derive(Default)]
struct MySysts;

impl Updater for MySysts {
    type UpdateData = ();
    type Comps = MyComps;
    fn update(&mut self, _: &mut Ents, comps: &mut MyComps, _: &()) {
        if let Some(btm) = comps.comps.get_mut(&TypeId::of::<Vec2f>()) {
            for (_, vec) in btm {
                if vec.len() < 2 {
                    continue;
                }
                vec[0] = vec[0] + vec[1];
            }
        }
    }
}

fn main() {
    let mut w = State::<MyComps, MySysts>::default();
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
    println!("{:?}", w.comps);
    w.update(&());
    println!("{:?}", w.comps);
}
