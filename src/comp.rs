//! Components and component manager traits.
//!
//!

use EntId;

use std::any::Any;

/// A trait for defining components.
pub trait Comp: Any {}

impl<T> Comp for T where T: Any
{}

/// A manager of components.
///
/// `Manager`'s purpose is to store and return components in association with their owning entities.
/// Each of the trait's methods reflects on component's type.
/// If we want to get the second `Vec2` component of an entity `e`, we do this:
///
/// ```
/// comps.get::<Vec2>(e, 1);
/// ```
///
/// Note, that like everything in Rust, entity's components are zero-indexed.
pub trait Comps {
    /// A type of additional data to be used when registring a new type of component.
    /// Could be used to specify the style of storing the components of that type in memory.
    type RegData;
    /// A type of error to be returned when failing to register a type of component.
    type RegError;

    /// Register a new type of components, allowing the manager to store them.
    fn register_comp<T: Comp>(&mut self, Self::RegData) -> Result<(), Self::RegError>;

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
}
