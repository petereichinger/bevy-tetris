use bevy::prelude::*;

#[derive(Debug, Event)]
pub struct CellEvent {
    pub position: IVec2,
    pub event_type: EventType,
}

#[derive(Debug)]
pub enum EventType {
    Added,
    // Removed,
}
