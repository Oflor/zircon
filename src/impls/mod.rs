//! Implementations of the Comps and Updater traits.

pub mod basic;

use super::*;

/// A struct for chained invokation of `Updater`s.
///
/// `Chain` could be used to unite different systems implementing `Updater` into one struct.
///
/// Because `Chain` itself implements `Updater`, it could contain another `Chain`:
///
/// ```
/// let updater = Chain(|_, _, _, _| { println!("System 1"); Vec::new() },
///               Chain(|e, _, _, _| { println!("Entity #{}", e); Vec::new() },
///                     |_, _, _, _| { println!("System 3"); Vec::new() }));
/// ```
/// This code won't actually compile, because the argument types need to be specified for closures
/// to become implementors of `Updater`.
pub struct Chain<A, B>(pub A, pub B);

impl<A, B, Cs: Comps, D> Updater<Cs, D> for Chain<A, B>
    where A: Updater<Cs, D>,
          B: Updater<Cs, D>
{
    #[inline(always)]
    fn update(&mut self, ents: &mut Ents, comps: &mut Cs, data: &D) {
        self.0.update(ents, comps, data);
        self.1.update(ents, comps, data);
    }
}

use rayon::prelude::*;
use rayon::par_iter::collect::collect_into;

impl<Cs: Comps + Sync, D: Sync, F: Sync, E: Sync + Send + IntoIterator<Item=Cs::Diff>> Updater<Cs, D> for F
    where F: Fn(EntId, &Ents, &Cs, &D) -> E
{
    fn update(&mut self, ents: &mut Ents, comps: &mut Cs, data: &D) {
        let mut vec = Vec::new();
        collect_into(ents.iter().collect::<Vec<&EntId>>()
            .par_iter().map(|&&e| self(e, ents, comps, data)), &mut vec);
        //vec = ents.iter().collect::<Vec<&EntId>>()
        //    .iter().map(|&&e| self(e, ents, comps, data)).collect::<Vec<_>>();
        for iter in vec {
            comps.commit(iter);
        }
    }
}

impl<Cs: Comps, D> Updater<Cs, D> for () {
    fn update(&mut self, _: &mut Ents, _: &mut Cs, _: &D) { }
}