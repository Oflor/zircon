//! Zircon - Entity Component System (ECS) implementation in Rust

extern crate rayon;

pub mod impls;

use std::collections::BTreeSet;
use std::default::Default;
use std::any::Any;

/// An entity identifier.
pub type EntId = u64;

/// A set for storing entities.
pub type Ents = BTreeSet<EntId>;

/// A plain-old-data struct used as a component.
pub trait Comp: Any + Sized + Clone {}

impl<T> Comp for T where T: Any + Sized + Clone
{}

/// A manager of components.
///
/// Its purpose is to store and return components in association with their owning entities.
/// Each of the trait's methods reflects on component's type.
/// If we want to get the second `Vec2` component of an entity `e`, we do this:
///
/// ```
/// comps.get::<Vec2>(e, 1);
/// ```
///
/// Note, that like everything in Rust, entity's components are zero-indexed.
pub trait Comps: Sized {
    /// A type of additional data to be used when registring a new type of components.
    /// Could be used to specify the style of storing the components of that type in memory.
    type RegData;
    /// A type of error to be returned when failing to register a type of components.
    type RegError;
    /// Diff represents the changes in the components (e.g. component removal, insertion, or mutation)
    type Diff: Diff<Self>;

    /// Register a new type of components, allowing the manager to store them.
    fn register_comp<T: Comp>(&mut self, &Self::RegData) -> Result<(), Self::RegError>;

    /// Get a reference to the `idx`th component `T` of the entity `e`.
    /// Returns `None` if that component does not exist.
    fn get<T: Comp>(&self, e: EntId, idx: usize) -> Option<&T>;

    /// Replaces the `idx`th component `T` of the entity `e`.
    /// Returns `None` if that component does not exist.
    fn replace<T: Comp>(&mut self, e: EntId, idx: usize, comp: T) -> Option<()>;

    /// Get a reference to the first component `T` of the entity `e`.
    /// Returns `None` if that component does not exist.
    fn get_first<T: Comp>(&self, e: EntId) -> Option<&T> {
        Self::get::<T>(self, e, 0)
    }

    /// Replaces the first component `T` of the entity `e`.
    /// Returns `None` if that component does not exist.
    fn replace_first<T: Comp>(&mut self, e: EntId, comp: T) -> Option<()> {
        Self::replace::<T>(self, e, 0, comp)
    }
    
    /// Adds a new component `T` to the entity `e` and returns its index.
    fn insert<T: Comp>(&mut self, e: EntId, comp: T) -> usize;

    /// Removes the last component `T` of the entity `e` and returns it.
    /// Returns `None` if that entity contained no components of that type.
    fn pop<T: Comp>(&mut self, e: EntId) -> Option<T>;

    /// Removes all components `T` of the entity `e`.
    fn remove_all<T: Comp>(&mut self, e: EntId) {
        for _ in 0..self.len::<T>(e) {
            self.pop::<T>(e);
        }
    }

    /// Returns the number of components `T` of the entity `e`.
    fn len<T: Comp>(&self, e: EntId) -> usize;
    
    fn commit<D: IntoIterator<Item=Self::Diff>>(&mut self, D);
}

pub trait Diff<Cs: Comps>: Sized {
    fn insert<T: Comp>(comps: &Cs, e: EntId, comp: T) -> Self;
    fn pop<T: Comp>(comps: &Cs, e: EntId) -> Self;
    fn replace<T: Comp>(comps: &Cs, e: EntId, idx: usize, comp: T) -> Self;
}

/// State updater. In a classic ECS model, `Updater` is a manager of systems.
pub trait Updater<Cs: Comps, D> {
    /// Updates the world state.
    fn update(&mut self, ents: &mut Ents, comps: &mut Cs, data: &D);
}

use std::marker::PhantomData;

/// The main struct of an ECS, `State` stores the updater, components and entities of the system.
pub struct State<Cs, Upd, D> {
    _pd: PhantomData<D>,
    pub comps: Cs,
    pub updater: Upd,
    pub ents: Ents,
    current: EntId,
}

impl<Cs, Upd, D> State<Cs, Upd, D>
    where Cs: Comps,
          Upd: Updater<Cs, D>
{
    /// Creates a new empty `State` with the specified components and systems managers.
    ///
    /// The new `State` contains no entities, and the next alive entity will have ID #1.
    /// Entity #0 is considered invalid, using it may cause panics.
    pub fn new(comps: Cs, updater: Upd) -> State<Cs, Upd, D> {
        State {
            _pd: PhantomData,
            comps: comps,
            updater: updater,
            ents: Ents::new(),
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
    pub fn update(&mut self, data: &D) {
        self.updater.update(&mut self.ents, &mut self.comps, data);
    }
}

impl<Cs, Upd, D> Default for State<Cs, Upd, D>
    where Cs: Comps + Default,
          Upd: Updater<Cs, D> + Default
{
    fn default() -> State<Cs, Upd, D> {
        State::new(Cs::default(), Upd::default())
    }
}
