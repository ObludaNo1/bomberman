use crate::{assets::TILESET_TILE_SIZE, tileset_enum};

tileset_enum!(
    Map,
    TILESET_TILE_SIZE,
    (255, 434),
    Floor => (86, 417),
    Wall => (1, 417),
    WallFade1 => (18, 417),
    WallFade2 => (35, 417),
    WallFade3 => (52, 417),
    WallFade4 => (69, 417),
    IndestructibleWall => (120, 417),
);
