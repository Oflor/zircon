extern crate zircon;

use zircon::{EntId, State};
use zircon::comp::*;
use zircon::syst::*;

use std::ops::Add;
use std::collections::{HashMap, BTreeMap};
use std::any::TypeId;
use std::mem::transmute;

#[derive(Debug)]
struct Vec2f(f32, f32);

impl Add<Vec2f> for Vec2f {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec2f(self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Default)]
struct MyComps {
    comps: HashMap<TypeId, BTreeMap<EntId, Vec<u8>>>,
}

impl Comps for MyComps {
    type RegData = ();
    type RegError = ();
    fn register_comp<T: Comp>(&mut self, _: ()) -> Result<(), ()> {
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
}

#[derive(Default)]
struct MySysts;

impl Systs for MySysts {
    type Comps = MyComps;
}

fn main() {
    let mut state = State::<MyComps, MySysts>::default();
    state.comps.register_comp::<Vec2f>(()).unwrap();
}
