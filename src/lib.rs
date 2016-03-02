//! Zircon - Entity Component System (ECS) implementation in Rust

pub mod comp;
pub mod syst;

use comp::*;
use syst::*;

use std::collections::BTreeSet;
use std::default::Default;

/// Entity.
pub type EntId = u64;

/// The main struct of an ECS, `State` contains all systems, components and entities of the system.
pub struct State<Cs, Ss> {
    pub comps: Cs,
    pub systs: Ss,
    ents: BTreeSet<EntId>,
    current: EntId,
}

impl<Cs, Ss> State<Cs, Ss>
    where Cs: Comps,
          Ss: Systs<Comps = Cs>
{
    /// Creates a new empty `State` with the specified components and systems managers.
    ///
    /// The new `State` contains no entities, and the next alive entity will have ID #1.
    /// Entity #0 is considered invalid, using it may cause panics.
    pub fn new(comps: Cs, systs: Ss) -> State<Cs, Ss> {
        State {
            comps: comps,
            systs: systs,
            ents: BTreeSet::new(),
            current: 0,
        }
    }

    /// Creates a new unique entity and returns its ID.
    ///
    /// The IDs are not recycled. That means that if entity B is created later than entity A,
    /// B will always have a larger ID, even if A by that time is dead.
    pub fn new_ent(&mut self) -> EntId {
        self.current += 1;
        self.ents.insert(self.current);
        self.current
    }

    /// Returns `true` if the entity `e` exists and is alive, and `false` otherwise.
    pub fn is_alive(&mut self, e: EntId) -> bool {
        self.ents.contains(&e)
    }

    /// Removes the entity `e` and all of its components.
    pub fn delete_ent(&mut self, e: EntId) {
        self.ents.remove(&e);
    }

    /// Update the state.
    pub fn update(&mut self, data: &Ss::UpdateData) {
        self.systs.update(&mut self.comps, data);
    }
}

impl<Cs, Ss> Default for State<Cs, Ss>
    where Cs: Comps + Default,
          Ss: Systs<Comps = Cs> + Default
{
    fn default() -> State<Cs, Ss> {
        State::new(Cs::default(), Ss::default())
    }
}
