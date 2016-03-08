//! Zircon - Entity Component System (ECS) implementation in Rust

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
pub trait Comps {
    /// A type of additional data to be used when registring a new type of components.
    /// Could be used to specify the style of storing the components of that type in memory.
    type RegData;
    /// A type of error to be returned when failing to register a type of components.
    type RegError;

    /// Register a new type of components, allowing the manager to store them.
    fn register_comp<T: Comp>(&mut self, &Self::RegData) -> Result<(), Self::RegError>;

    /// Get a reference to the `idx`th component `T` of the entity `e`.
    /// Returns `None` if that component does not exist.
    fn get<T: Comp>(&self, e: EntId, idx: usize) -> Option<&T>;

    /// Get a mutable reference to the `idx`th component `T` of the entity `e`.
    /// Returns `None` if that component does not exist.
    fn get_mut<T: Comp>(&mut self, e: EntId, idx: usize) -> Option<&mut T>;

    /// Get a reference to the first component `T` of the entity `e`.
    /// Returns `None` if that component does not exist.
    fn get_first<T: Comp>(&self, e: EntId) -> Option<&T> {
        Self::get::<T>(self, e, 0)
    }

    /// Get a mutable reference to the first component `T` of the entity `e`.
    /// Returns `None` if that component does not exist.
    fn get_first_mut<T: Comp>(&mut self, e: EntId) -> Option<&mut T> {
        Self::get_mut::<T>(self, e, 0)
    }
    /// Adds a new component `T` to the entity `e` and returns its index.
    fn insert<T: Comp>(&mut self, e: EntId, comp: T) -> usize;

    /// Removes the `idx`th component `T` of the entity `e` and returns it.
    /// Returns `None` if that component did not exist.
    fn remove<T: Comp>(&mut self, e: EntId, idx: usize) -> Option<T>;

    /// Removes all components `T` of the entity `e`.
    fn remove_all<T: Comp>(&mut self, e: EntId);

    /// Returns the number of components `T` of the entity `e`.
    fn len<T: Comp>(&self, e: EntId) -> usize;
}

/// State updater. In a classic ECS model, `Updater` is a manager of systems.
pub trait Updater<D, Cs: Comps> {
    /// Updates the world state.
    fn update(&mut self, ents: &mut Ents, comps: &mut Cs, data: &D);
}

use std::marker::PhantomData;

/// The main struct of an ECS, `State` stores the updater, components and entities of the system.
pub struct State<D, Cs, Upd> {
    _pd: PhantomData<D>,
    pub comps: Cs,
    pub updater: Upd,
    pub ents: Ents,
    current: EntId,
}

impl<D, Cs, Upd> State<D, Cs, Upd>
    where Cs: Comps,
          Upd: Updater<D, Cs>
{
    /// Creates a new empty `State` with the specified components and systems managers.
    ///
    /// The new `State` contains no entities, and the next alive entity will have ID #1.
    /// Entity #0 is considered invalid, using it may cause panics.
    pub fn new(comps: Cs, updater: Upd) -> State<D, Cs, Upd> {
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

impl<D, Cs, Upd> Default for State<D, Cs, Upd>
    where Cs: Comps + Default,
          Upd: Updater<D, Cs> + Default
{
    fn default() -> State<D, Cs, Upd> {
        State::new(Cs::default(), Upd::default())
    }
}
