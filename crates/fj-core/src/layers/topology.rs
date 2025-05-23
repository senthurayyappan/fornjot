//! Layer infrastructure for [`Topology`]

use crate::{
    geometry::Geometry,
    topology::{AboutToBeStored, AnyObject, Topology},
    validation::Validation,
};

use super::{Command, Event, Layer, validation::ValidateObject};

impl Layer<Topology> {
    /// Insert an object into the stores
    ///
    /// Passes any events produced to the validation layer.
    pub fn insert(
        &mut self,
        object: AnyObject<AboutToBeStored>,
        geometry: &mut Layer<Geometry>,
        validation: &mut Layer<Validation>,
    ) {
        let mut events = Vec::new();
        self.process_command_and_capture_events(
            InsertObject { object },
            &mut events,
        );

        for event in events {
            validation.process_command(ValidateObject {
                object: event.object.into(),
                geometry,
            });
        }
    }
}

/// Insert an object into the stores
#[derive(Clone, Debug)]
pub struct InsertObject {
    /// The object to insert
    pub object: AnyObject<AboutToBeStored>,
}

impl Command<Topology> for InsertObject {
    type Result = ();
    type Event = InsertObject;

    fn decide(self, _: &Topology, events: &mut Vec<Self::Event>) {
        events.push(self);
    }
}

impl Event<Topology> for InsertObject {
    fn evolve(self, state: &mut Topology) {
        self.object.clone().insert(state);
    }
}
