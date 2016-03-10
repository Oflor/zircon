use EntId;
use {Comp, Comps, Diff};

use std::collections::{HashMap, BTreeMap};
use std::any::TypeId;
use std::mem::{transmute, size_of};

mod utils {
    use std::slice::{from_raw_parts, from_raw_parts_mut};
    use std::mem::{transmute, size_of, uninitialized};
    
    pub fn to_slice<T: Sized>(c: &T) -> &[u8] {
        unsafe { from_raw_parts(transmute(c), size_of::<T>()) }
    }
    
    pub fn to_slice_mut<T: Sized>(c: &mut T) -> &mut [u8] {
        unsafe { from_raw_parts_mut(transmute(c), size_of::<T>()) }
    }
    
    pub fn to_boxed_slice<T: Sized>(c: T) -> Box<[u8]> {
        let mut v: Vec<u8> = Vec::with_capacity(size_of::<T>());
        v.extend_from_slice(to_slice(&c));
        v.into_boxed_slice()
    }
    
    pub fn from_boxed_slice<T: Sized>(c: Box<[u8]>) -> T {
        assert_eq!(c.len(), size_of::<T>());
        let mut val: T = unsafe { uninitialized() };
        {
            let r = to_slice_mut(&mut val);
            for i in 0..size_of::<T>() {
                r[i] = c[i]
            }
        }
        val
    }
}

enum DiffEnum {
    Insert(TypeId, EntId, Box<[u8]>),
    Remove(TypeId, EntId),
    Replace(TypeId, EntId, usize, Box<[u8]>)
}

use self::DiffEnum::*;

pub struct BasicCompsDiff(DiffEnum);

impl Diff<BasicComps> for BasicCompsDiff {
    fn insert<T: Comp>(comps: &BasicComps, e: EntId, comp: T) -> Self {
        BasicCompsDiff(Insert(TypeId::of::<T>(), e, utils::to_boxed_slice(comp)))
    }
    
    fn pop<T: Comp>(comps: &BasicComps, e: EntId) -> Self {
        BasicCompsDiff(Remove(TypeId::of::<T>(), e))
    }
    
    fn replace<T: Comp>(comps: &BasicComps, e: EntId, idx: usize, comp: T) -> Self {
        BasicCompsDiff(Replace(TypeId::of::<T>(), e, idx, utils::to_boxed_slice(comp)))
    }
}

#[derive(Default)]
pub struct BasicComps {
    comps: HashMap<TypeId, (BTreeMap<EntId, Vec<u8>>, usize)>,
    sizes: HashMap<TypeId, usize>
}

impl BasicComps {
    fn register_comp_dyn(&mut self, t: TypeId, s: usize) -> Result<(), ()> {
        self.comps.insert(t, (BTreeMap::new(), s));
        Ok(())
    }
    
    fn get_dyn(&self, t: TypeId, e: EntId, idx: usize)  -> Option<&u8> {
        if let Some(tuple) = self.comps.get(&t) {
            if let Some(vec) = tuple.0.get(&e) {
                return vec.get(idx * tuple.1)
            }
        }
        None
    }
    
    fn replace_dyn(&mut self, t: TypeId, e: EntId, idx: usize, comp: &[u8]) -> Option<()> {
        if let Some(tuple) = self.comps.get_mut(&t) {
            if let Some(vec) = tuple.0.get_mut(&e) {
                let size = tuple.1;
                assert!(comp.len() == size);
                if (idx + 1) * size - 1 < vec.len() {
                    for i in 0..size { // TODO: memcpy
                        vec[(idx * size) + i] = comp[i];
                    }
                    return Some(());
                }
            }
        }
        None
    }
    
    fn insert_dyn(&mut self, t: TypeId, e: EntId, comp: &[u8]) -> usize {
        if let Some(tuple) = self.comps.get_mut(&t) {
            let size = tuple.1;
            assert!(comp.len() == size);
            if let Some(vec) = tuple.0.get_mut(&e) {
                vec.extend_from_slice(comp);
                return vec.len() / size - 1;
            }
            let mut vec = Vec::<u8>::with_capacity(tuple.1);
            vec.extend_from_slice(comp);
            tuple.0.insert(e, vec);
            0
        } else {
            panic!("Unregistered type of component")
        }
    }
    
    fn pop_dyn(&mut self, t: TypeId, e: EntId) -> Option<Box<[u8]>> {
        if let Some(tuple) = self.comps.get_mut(&t) {
            if let Some(vec) = tuple.0.get_mut(&e) {
                let len = vec.len();
                let vec2 = vec.split_off(len - tuple.1);
                return Some(vec2.into_boxed_slice());
            }
        }
        None
    }
}

impl Comps for BasicComps {
    type RegData = ();
    type RegError = ();
    type Diff = BasicCompsDiff;
    fn register_comp<T: Comp>(&mut self, _: &()) -> Result<(), ()> {
        self.register_comp_dyn(TypeId::of::<T>(), size_of::<T>())
    }

    fn get<T: Comp>(&self, e: EntId, idx: usize) -> Option<&T> {
        unsafe { transmute(self.get_dyn(TypeId::of::<T>(), e, idx)) }
    }

    fn replace<T: Comp>(&mut self, e: EntId, idx: usize, comp: T) -> Option<()> {
        self.replace_dyn(TypeId::of::<T>(), e, idx, utils::to_slice(&comp))
    }

    fn insert<T: Comp>(&mut self, e: EntId, comp: T) -> usize {
        self.insert_dyn(TypeId::of::<T>(), e, utils::to_slice(&comp))
    }

    fn pop<T: Comp>(&mut self, e: EntId) -> Option<T> {
        self.pop_dyn(TypeId::of::<T>(), e).map(|e| utils::from_boxed_slice::<T>(e))
    }

    fn remove_all<T: Comp>(&mut self, e: EntId) {
        if let Some(tuple) = self.comps.get_mut(&TypeId::of::<T>()) {
            tuple.0.remove(&e);
        }
    }

    fn len<T: Comp>(&self, e: EntId) -> usize {
        if let Some(tuple) = self.comps.get(&TypeId::of::<T>()) {
            if let Some(vec) = tuple.0.get(&e) {
                return vec.len() / tuple.1;
            } 
        }
        0
    }
    
    fn commit<D: IntoIterator<Item=BasicCompsDiff>>(&mut self, diffs: D) {
        for diff in diffs {
            match diff {
                BasicCompsDiff(Insert(t, e, comp)) => {},
                BasicCompsDiff(Remove(t, e)) => {},
                BasicCompsDiff(Replace(t, e, idx, comp)) => {}
            }
        }
    }
}