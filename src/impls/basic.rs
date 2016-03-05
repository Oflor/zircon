use EntId;
use {Comp, Comps};

use std::collections::{HashMap, BTreeMap};
use std::any::TypeId;
use std::mem::transmute;

#[derive(Default)]
pub struct BasicComps {
    comps: HashMap<TypeId, BTreeMap<EntId, Vec<u8>>>,
}

impl Comps for BasicComps {
    type RegData = ();
    type RegError = ();
    fn register_comp<T: Comp>(&mut self, _: &()) -> Result<(), ()> {
        self.comps.insert(TypeId::of::<T>(), BTreeMap::new());
        Ok(())
    }

    fn get<T: Comp>(&self, e: EntId, idx: usize) -> Option<&T> {
        if let Some(btm) = self.comps.get(&TypeId::of::<T>()) {
            if let Some(vec) = btm.get(&e) {
                return unsafe { transmute::<_, &Vec<T>>(vec) }.get(idx)
            }
        }
        None
    }

    fn get_mut<T: Comp>(&mut self, e: EntId, idx: usize) -> Option<&mut T> {
        if let Some(btm) = self.comps.get_mut(&TypeId::of::<T>()) {
            if let Some(vec) = btm.get_mut(&e) {
                return unsafe { transmute::<_, &mut Vec<T>>(vec) }.get_mut(idx)
            }
        }
        None
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
                    return Some(vec.remove(idx))
                }
            }
        }
        None
    }

    fn remove_all<T: Comp>(&mut self, e: EntId) {
        if let Some(btm) = self.comps.get_mut(&TypeId::of::<T>()) {
            btm.remove(&e);
        }
    }

    fn len<T: Comp>(&self, e: EntId) -> usize {
        if let Some(btm) = self.comps.get(&TypeId::of::<T>()) {
            if let Some(vec) = btm.get(&e) {
                return unsafe { transmute::<_, &Vec<T>>(vec) }.len()
            } 
        }
        0
    }
}