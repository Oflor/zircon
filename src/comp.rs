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
    fn register_comp<T: Comp>(&mut self, Self::RegData) -> Result<(), Self::RegError>;
    fn get<T: Comp>(&self, e: EntId, idx: usize) -> Option<&T>;
    fn get_mut<T: Comp>(&mut self, e: EntId, idx: usize) -> Option<&mut T>;
    fn get_first<T: Comp>(&self, e: EntId) -> Option<&T> {
        Self::get::<T>(self, e, 0)
    }
    fn get_first_mut<T: Comp>(&mut self, e: EntId) -> Option<&mut T> {
        Self::get_mut::<T>(self, e, 0)
    }
    fn insert<T: Comp>(&mut self, e: EntId, comp: T) -> usize;
    fn remove<T: Comp>(&mut self, e: EntId, idx: usize) -> Option<T>;
    fn remove_all<T: Comp>(&mut self, e: EntId);
}
