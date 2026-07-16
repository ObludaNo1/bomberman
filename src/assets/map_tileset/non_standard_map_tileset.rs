use bevy::prelude::*;

use crate::tileset_enum;

tileset_enum!(
    MapGate,
    UVec2::new(32, 26),
    (255, 424),
    Closed => (145, 63),
    Open => (186, 63),
);
