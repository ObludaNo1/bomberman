use crate::{assets::TILESET_TILE_SIZE, tileset_enum};

tileset_enum!(
    PowerUp,
    TILESET_TILE_SIZE,
    (222, 205),
    Range => (1, 135),
    BombCount => (18, 135),
    Negative => (35, 135),
    ExtraLife => (52, 135),
    Hook => (69, 135),
    BombKick => (86, 135),
    Detonator => (103, 135),
    Turbo => (120, 135),
    LineBomb => (137, 135),
    DoubleBomb => (154, 135),
    Max => (171, 135),
);
