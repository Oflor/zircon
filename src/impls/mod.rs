//! Implementations of the Comps and Updater traits.

pub mod basic;

use {EntId, Ents, Comps, Updater};

pub struct Chain<A, B>(pub A, pub B);

impl<A, B, D, Cs: Comps> Updater<D, Cs> for Chain<A, B>
    where A: Updater<D, Cs>,
          B: Updater<D, Cs>
{
    fn update(&mut self, ents: &mut Ents, comps: &mut Cs, data: &D) {
        self.0.update(ents, comps, data);
        self.1.update(ents, comps, data);
    }
}

impl<D, Cs: Comps, F> Updater<D, Cs> for F
    where F: FnMut(EntId, &mut Cs, &D)
{
    fn update(&mut self, ents: &mut Ents, comps: &mut Cs, data: &D) {
        for &e in ents.iter() {
            self(e, comps, data)
        }
    }
}

impl<D, Cs: Comps> Updater<D, Cs> for () {
    fn update(&mut self, ents: &mut Ents, comps: &mut Cs, data: &D) {
        
    }
}