//! Implementations of the Comps and Updater traits.

pub mod basic;

use {EntId, Ents, Comps, Updater};

pub struct Chain<A, B>(pub A, pub B);

impl<A, B, Cs: Comps, D> Updater<Cs, D> for Chain<A, B>
    where A: Updater<Cs, D>,
          B: Updater<Cs, D>
{
    fn update(&mut self, ents: &mut Ents, comps: &mut Cs, data: &D) {
        self.0.update(ents, comps, data);
        self.1.update(ents, comps, data);
    }
}

impl<Cs: Comps, D, F> Updater<Cs, D> for F
    where F: FnMut(EntId, &mut Cs, &D)
{
    fn update(&mut self, ents: &mut Ents, comps: &mut Cs, data: &D) {
        for &e in ents.iter() {
            self(e, comps, data)
        }
    }
}

impl<Cs: Comps, D> Updater<Cs, D> for () {
    fn update(&mut self, ents: &mut Ents, comps: &mut Cs, data: &D) {
        
    }
}