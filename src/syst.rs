use EntId;
use comp::Comps;

/// A manager of systems.
///
/// `Systs` manages updating the world state.
pub trait Systs {
    /// Data to send with the update method.
    type UpdateData;
    type Comps: Comps;
    /// Updates the world state.
    fn update(&mut self, comps: &mut Self::Comps, data: &Self::UpdateData);
}
