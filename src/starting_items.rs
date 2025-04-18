use crate::pickup_meta::PickupType;

use serde::{Deserialize, Serialize};

fn default_true() -> bool {
    true
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StartingItems {
    #[serde(default = "default_true")]
    pub combat_visor: bool,
    #[serde(default = "default_true")]
    pub power_beam: bool,
    #[serde(default = "default_true")]
    pub scan_visor: bool,
    #[serde(default)]
    pub missiles: i32,
    #[serde(default)]
    pub energy_tanks: i8,
    #[serde(default)]
    pub power_bombs: i8,
    #[serde(default)]
    pub wave: bool,
    #[serde(default)]
    pub ice: bool,
    #[serde(default)]
    pub plasma: bool,
    #[serde(default)]
    pub charge: bool,
    #[serde(default)]
    pub morph_ball: bool,
    #[serde(default)]
    pub bombs: bool,
    #[serde(default)]
    pub spider_ball: bool,
    #[serde(default)]
    pub boost_ball: bool,
    #[serde(default)]
    pub power_suit: u32,
    #[serde(default)]
    pub varia_suit: bool,
    #[serde(default)]
    pub gravity_suit: bool,
    #[serde(default)]
    pub phazon_suit: bool,
    #[serde(default)]
    pub thermal_visor: bool,
    #[serde(default)]
    pub xray: bool,
    #[serde(default)]
    pub space_jump: bool,
    #[serde(default)]
    pub grapple: bool,
    #[serde(default)]
    pub super_missile: bool,
    #[serde(default)]
    pub wavebuster: bool,
    #[serde(default)]
    pub ice_spreader: bool,
    #[serde(default)]
    pub flamethrower: bool,
    #[serde(default)]
    pub unknown_item_1: u32,
    #[serde(default)]
    pub unlimited_missiles: bool,
    #[serde(default)]
    pub unlimited_power_bombs: bool,
    #[serde(default = "default_true")]
    pub missile_launcher: bool,
    #[serde(default = "default_true")]
    pub power_bomb_launcher: bool,
    #[serde(default)]
    pub spring_ball: bool,
}

impl StartingItems {
    pub fn from_u64(mut starting_items: u64) -> Self {
        let mut fetch_bits = move |bits: u8| {
            let ret = starting_items & ((1 << bits) - 1);
            starting_items >>= bits;
            ret as u8
        };

        StartingItems {
            power_beam: true,
            combat_visor: true,
            scan_visor: fetch_bits(1) == 1,
            missiles: fetch_bits(8) as i32,
            energy_tanks: fetch_bits(4) as i8,
            power_bombs: fetch_bits(4) as i8,
            wave: fetch_bits(1) == 1,
            ice: fetch_bits(1) == 1,
            plasma: fetch_bits(1) == 1,
            charge: fetch_bits(1) == 1,
            morph_ball: fetch_bits(1) == 1,
            bombs: fetch_bits(1) == 1,
            spider_ball: fetch_bits(1) == 1,
            boost_ball: fetch_bits(1) == 1,
            power_suit: 0,
            varia_suit: fetch_bits(1) == 1,
            gravity_suit: fetch_bits(1) == 1,
            phazon_suit: fetch_bits(1) == 1,
            thermal_visor: fetch_bits(1) == 1,
            xray: fetch_bits(1) == 1,
            space_jump: fetch_bits(1) == 1,
            grapple: fetch_bits(1) == 1,
            super_missile: fetch_bits(1) == 1,
            wavebuster: fetch_bits(1) == 1,
            ice_spreader: fetch_bits(1) == 1,
            flamethrower: fetch_bits(1) == 1,
            unknown_item_1: 0,
            unlimited_missiles: false,
            unlimited_power_bombs: false,
            missile_launcher: true,
            power_bomb_launcher: true,
            spring_ball: false,
        }
    }

    pub fn update_spawn_point(&self, spawn_point: &mut structs::SpawnPoint) {
        spawn_point.combat_visor = self.combat_visor as u32;
        spawn_point.power = self.power_beam as u32;
        spawn_point.scan_visor = self.scan_visor as u32;
        spawn_point.missiles = self.missiles as u32;
        spawn_point.energy_tanks = self.energy_tanks as u32;
        spawn_point.power_bombs = self.power_bombs as u32;
        spawn_point.wave = self.wave as u32;
        spawn_point.ice = self.ice as u32;
        spawn_point.plasma = self.plasma as u32;
        spawn_point.charge = self.charge as u32;
        spawn_point.morph_ball = self.morph_ball as u32;
        spawn_point.bombs = self.bombs as u32;
        spawn_point.spider_ball = self.spider_ball as u32;
        spawn_point.boost_ball = self.boost_ball as u32;
        spawn_point.power_suit = 0;
        spawn_point.varia_suit = self.varia_suit as u32;
        spawn_point.gravity_suit = self.gravity_suit as u32;
        spawn_point.phazon_suit = self.phazon_suit as u32;
        spawn_point.thermal_visor = self.thermal_visor as u32;
        spawn_point.xray = self.xray as u32;
        spawn_point.space_jump = self.space_jump as u32;
        spawn_point.grapple = self.grapple as u32;
        spawn_point.super_missile = self.super_missile as u32;
        spawn_point.wavebuster = self.wavebuster as u32;
        spawn_point.ice_spreader = self.ice_spreader as u32;
        spawn_point.flamethrower = self.flamethrower as u32;
        spawn_point.unknown_item_1 = self.unknown_item_1;
        let mut unknown_item_2 = 0;
        if self.unlimited_missiles {
            unknown_item_2 |= PickupType::UnlimitedMissiles.custom_item_value();
        }
        if self.unlimited_power_bombs {
            unknown_item_2 |= PickupType::UnlimitedPowerBombs.custom_item_value();
        }
        if self.missile_launcher {
            unknown_item_2 |= PickupType::MissileLauncher.custom_item_value();
        }
        if self.power_bomb_launcher {
            unknown_item_2 |= PickupType::PowerBombLauncher.custom_item_value();
        }
        if self.spring_ball {
            unknown_item_2 |= PickupType::SpringBall.custom_item_value();
        }
        spawn_point.unknown_item_2 = unknown_item_2 as u32;
    }

    /// Custom deserializataion function that accepts an int as well as the usual struct/object
    /// version
    pub fn custom_deserialize<'de, D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        pub enum Wrapper {
            Int(u64),
            Struct(StartingItems),
        }

        match <Wrapper as Deserialize>::deserialize(deserializer) {
            Ok(Wrapper::Struct(s)) => Ok(s),
            Ok(Wrapper::Int(i)) => Ok(StartingItems::from_u64(i)),
            Err(e) => Err(e),
        }
    }

    pub fn is_empty(&self) -> bool {
        !self.power_beam
            && !self.scan_visor
            && self.missiles == 0
            && self.energy_tanks == 0
            && self.power_bombs == 0
            && !self.wave
            && !self.ice
            && !self.plasma
            && !self.charge
            && !self.morph_ball
            && !self.bombs
            && !self.spider_ball
            && !self.boost_ball
            && !self.varia_suit
            && !self.gravity_suit
            && !self.phazon_suit
            && !self.thermal_visor
            && !self.xray
            && !self.space_jump
            && !self.grapple
            && !self.super_missile
            && !self.wavebuster
            && !self.ice_spreader
            && !self.flamethrower
            && !self.unlimited_missiles
            && !self.unlimited_power_bombs
            && !self.missile_launcher
            && !self.power_bomb_launcher
            && !self.spring_ball
    }
}

impl Default for StartingItems {
    fn default() -> Self {
        StartingItems {
            combat_visor: true,
            power_beam: true,
            scan_visor: true,
            missiles: 0,
            energy_tanks: 0,
            power_bombs: 0,
            wave: false,
            ice: false,
            plasma: false,
            charge: false,
            morph_ball: false,
            bombs: false,
            spider_ball: false,
            boost_ball: false,
            power_suit: 0,
            varia_suit: false,
            gravity_suit: false,
            phazon_suit: false,
            thermal_visor: false,
            xray: false,
            space_jump: false,
            grapple: false,
            super_missile: false,
            wavebuster: false,
            ice_spreader: false,
            flamethrower: false,
            unknown_item_1: 0,
            unlimited_missiles: false,
            unlimited_power_bombs: false,
            missile_launcher: true,
            power_bomb_launcher: true,
            spring_ball: false,
        }
    }
}
