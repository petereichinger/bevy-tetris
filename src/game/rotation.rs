use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, Reflect)]
pub enum Rotation {
    #[default]
    R0,
    R90,
    R180,
    R270,
}

impl From<Rotation> for f32 {
    fn from(value: Rotation) -> Self {
        match value {
            Rotation::R0 => 0.0f32.to_radians(),
            Rotation::R90 => 90.0f32.to_radians(),
            Rotation::R180 => 180.0f32.to_radians(),
            Rotation::R270 => 270.0f32.to_radians(),
        }
    }
}
