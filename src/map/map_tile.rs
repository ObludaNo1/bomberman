use std::time::Duration;

use bevy::prelude::*;

use crate::world_entities::BonusType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BaseTile {
    Floor,
    BasicWall,
    BreakingWall(Timer),
    IndestructibleWall,
}

impl BaseTile {
    pub fn is_walkable(&self) -> bool {
        match self {
            BaseTile::Floor => true,
            BaseTile::BasicWall => false,
            BaseTile::BreakingWall(_) => false,
            BaseTile::IndestructibleWall => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BombTile {
    pub timer: Timer,
    pub range: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExplosionTile(pub Timer);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BombOrExplosionTile {
    Bomb(BombTile),
    Explosion(ExplosionTile),
}

impl BombOrExplosionTile {
    pub fn is_walkable(&self) -> bool {
        match self {
            BombOrExplosionTile::Bomb(_) => false,
            BombOrExplosionTile::Explosion(_) => true,
        }
    }

    pub fn is_explosion(&self) -> bool {
        match self {
            BombOrExplosionTile::Bomb(_) => false,
            BombOrExplosionTile::Explosion(_) => true,
        }
    }

    pub fn is_bomb(&self) -> bool {
        match self {
            BombOrExplosionTile::Bomb(_) => true,
            BombOrExplosionTile::Explosion(_) => false,
        }
    }

    pub fn bomb(&self) -> Option<&BombTile> {
        match self {
            BombOrExplosionTile::Bomb(bomb) => Some(bomb),
            BombOrExplosionTile::Explosion(_) => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialTile {
    OpenExit,
    ClosedExit,
    Bonus(BonusType),
}

impl SpecialTile {
    pub fn bonus(&self) -> Option<BonusType> {
        match self {
            SpecialTile::Bonus(bonus) => Some(*bonus),
            _ => None,
        }
    }

    pub fn is_open_exit(&self) -> bool {
        matches!(self, SpecialTile::OpenExit)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapTile {
    base_type: BaseTile,
    special: Option<SpecialTile>,
    bomb_or_explosion: Option<BombOrExplosionTile>,
}

impl MapTile {
    pub fn new(base_type: BaseTile, special: Option<SpecialTile>) -> Self {
        MapTile {
            base_type,
            special,
            bomb_or_explosion: None,
        }
    }

    pub fn base_type(&self) -> &BaseTile {
        &self.base_type
    }

    pub fn bomb_or_explosion(&self) -> Option<&BombOrExplosionTile> {
        self.bomb_or_explosion.as_ref()
    }

    pub fn special(&self) -> Option<&SpecialTile> {
        self.special.as_ref()
    }

    pub fn is_walkable(&self) -> bool {
        self.base_type.is_walkable()
            && self
                .bomb_or_explosion
                .as_ref()
                .map(|v| v.is_walkable())
                .unwrap_or(true)
    }

    pub fn is_ai_walkable(&self) -> bool {
        self.base_type.is_walkable() && self.bomb_or_explosion.is_none()
    }

    pub fn tick(&mut self, delta: Duration) {
        if let BaseTile::BreakingWall(timer) = &mut self.base_type {
            timer.tick(delta);
        }
        if let Some(bomb_or_explosion) = self.bomb_or_explosion.as_mut() {
            match bomb_or_explosion {
                BombOrExplosionTile::Bomb(bomb) => bomb.timer.tick(delta),
                BombOrExplosionTile::Explosion(explosion) => explosion.0.tick(delta),
            };
        }
    }

    /// Converts expired breaking walls to floors and clears explosions that have finished.
    /// Bombs have to be handled by different system since they can chain explode and have to be
    /// handled in a more complex way.
    pub fn convert_expired_entities(&mut self) {
        if let BaseTile::BreakingWall(timer) = &self.base_type
            && timer.is_finished()
        {
            self.base_type = BaseTile::Floor;
        }
        if let Some(BombOrExplosionTile::Explosion(timer)) = &self.bomb_or_explosion
            && timer.0.is_finished()
        {
            self.bomb_or_explosion = None;
        }
    }

    pub fn remove_bonus(&mut self) -> Option<BonusType> {
        if let Some(SpecialTile::Bonus(bonus)) = self.special {
            self.special = None;
            Some(bonus)
        } else {
            None
        }
    }

    pub fn try_add_bomb(&mut self, bomb_tile: BombTile) -> bool {
        match self.bomb_or_explosion {
            // We cannot add a bomb if a bomb already exists there
            Some(BombOrExplosionTile::Bomb(_)) => false,
            // TODO how to handle adding to already exploding tile??? For not a character trying to
            // place a bomb there should die before he can stand on this tile. But later there might
            // be a bomb kicking ability and a bomb may appear on top of an explosion.
            Some(BombOrExplosionTile::Explosion(_)) => false,
            None => {
                self.bomb_or_explosion = Some(BombOrExplosionTile::Bomb(bomb_tile));
                true
            }
        }
    }

    pub fn set_explosion(&mut self, explosion_tile: ExplosionTile) {
        self.bomb_or_explosion = Some(BombOrExplosionTile::Explosion(explosion_tile));
    }

    pub fn break_wall(&mut self, timer: Timer) {
        if self.base_type == BaseTile::BasicWall {
            self.base_type = BaseTile::BreakingWall(timer);
        }
    }

    pub fn open_exit(&mut self) {
        if self.special == Some(SpecialTile::ClosedExit) {
            self.special = Some(SpecialTile::OpenExit);
        }
    }
}
