//! Implementations of the Comps and Updater traits.

pub mod basic;

use {EntId, Ents, Comps, Updater};

pub struct Chain<A, B>(pub A, pub B);

impl<A, B> Updater for Chain<A, B>
    where A: Updater,
          B: Updater<UpdateData=A::UpdateData, Comps=A::Comps>
{
    type UpdateData = A::UpdateData;
    type Comps = A::Comps;
    fn update(&mut self, ents: &mut Ents, comps: &mut Self::Comps, data: &Self::UpdateData) {
        self.0.update(ents, comps, data);
        self.1.update(ents, comps, data);
    }
}

pub trait System {
    type UpdateData;
    type Comps: Comps;
    fn process(&mut self, e: EntId, comps: &mut Self::Comps, data: &Self::UpdateData);
}

impl<D, Cs: Comps, S> Updater for S
    where S: System<UpdateData=D, Comps=Cs>
{
    type UpdateData = D;
    type Comps = Cs;
    fn update(&mut self, ents: &mut Ents, comps: &mut Cs, data: &D) {
        for &e in ents.iter() {
            self.process(e, comps, data)
        }
    }
}