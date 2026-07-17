use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarkerBase {
    IndestructibleWall,
    BasicWall,
    Floor,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BonusType {
    Range,
    BombCount,
    Negative,
    ExtraLife,
    Hook,
    BombKick,
    Detonator,
    Turbo,
    LineBomb,
    DoubleBomb,
    Max,
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapTileMarker {
    base: MarkerBase,
    has_bomb: bool,
    has_explosion: bool,
    has_exit: bool,
    bonus: Option<BonusType>,
}

impl MapTileMarker {
    pub fn new(base: MarkerBase) -> Self {
        MapTileMarker {
            base,
            has_bomb: false,
            has_explosion: false,
            has_exit: false,
            bonus: None,
        }
    }

    pub fn floor() -> Self {
        MapTileMarker::new(MarkerBase::Floor)
    }

    pub fn basic_wall() -> Self {
        MapTileMarker::new(MarkerBase::BasicWall)
    }

    pub fn indestructible_wall() -> Self {
        MapTileMarker::new(MarkerBase::IndestructibleWall)
    }

    pub fn is_floor(&self) -> bool {
        self.base == MarkerBase::Floor
    }

    pub fn is_basic_wall(&self) -> bool {
        self.base == MarkerBase::BasicWall
    }

    pub fn remove_wall(&mut self) -> &mut Self {
        if self.base == MarkerBase::BasicWall {
            self.base = MarkerBase::Floor;
        }
        self
    }

    pub fn with_exit(mut self) -> Self {
        self.has_exit = true;
        self
    }

    pub fn with_bonus(mut self, bonus: BonusType) -> Self {
        self.bonus = Some(bonus);
        self
    }

    pub fn clear_bonus(&mut self) -> &mut Self {
        self.bonus = None;
        self
    }

    pub fn is_walkable(&self) -> bool {
        self.base == MarkerBase::Floor && !self.has_bomb
    }

    pub fn is_obstacle(&self) -> bool {
        !self.is_walkable()
    }

    pub fn is_ai_walkable(&self) -> bool {
        self.is_walkable() && !self.has_explosion
    }

    pub fn tile_base(&self) -> MarkerBase {
        self.base
    }

    pub fn has_bomb(&self) -> bool {
        self.has_bomb
    }

    pub fn set_bomb(&mut self, bomb: bool) -> &mut Self {
        if bomb {
            if self.base == MarkerBase::Floor {
                self.has_bomb = true;
            } else {
                // Bombs can only be placed on floor tiles.
            }
        } else {
            self.has_bomb = false;
        }
        self
    }

    pub fn has_explosion(&self) -> bool {
        self.has_explosion
    }

    pub fn set_explosion(&mut self, explosion: bool) -> &mut Self {
        if explosion {
            if self.base != MarkerBase::IndestructibleWall {
                self.has_explosion = true;
            } else {
                // Do nothing, indestructible walls cannot have explosions
            }
        } else {
            self.has_explosion = false;
        }
        self
    }

    // pub fn has_exit(&self) -> bool {
    //     self.has_exit
    // }
}
