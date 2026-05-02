use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Component)]
pub struct EnumUi;

#[derive(Component)]
pub struct StartGameUi;

#[derive(Component)]
pub struct StopGameUi;
#[derive(Component)]
pub struct Player {
    pub health: i32,
    pub max_health: i32,
    pub second_gun: SecondGun,
    pub engine: Engine,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CannonType {
    CANNON_LEVEL1,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CannonConfig {
    pub name: String,
    pub description: String,
    pub max_ammo: u32,
    pub damage: u32,
    pub weight: f32,
}

#[derive(Resource)]
pub struct CannonDataBase {
    pub configs: HashMap<CannonType, CannonConfig>,
}


#[derive(Component)]
pub struct Cannon {
    pub name: String,
    pub description: String,
    pub max_ammo: u32,
    pub damage: u32,
    pub weight: f32,
    pub current_ammo: u32,
    pub available: bool,
    pub cannon_type: CannonType,

}




pub enum SecondGun {
    NONE,
    LEVEL1(String, String, f32, i32),
    LEVEL2(String, String, f32, i32),
}

pub enum Engine {
    NONE,
    LEVEL1(String, String, f32),
    LEVEL2(String, String, f32),
}



#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum Appstatus {
    #[default] // 标记默认状态是 Setup
    Menu,
    Game,
    Vollage,
    WorldMap,
}

impl SecondGun {
    pub fn get_name(&self) -> &str {
        match self {
            SecondGun::NONE => "无",
            SecondGun::LEVEL1(name, _, _, _) => name,
            SecondGun::LEVEL2(name, _, _, _) => name,
        }
    }
    pub fn get_message(&self) -> &str {
        match self {
            SecondGun::NONE => "无",
            SecondGun::LEVEL1(_, message, _, _) => message,
            SecondGun::LEVEL2(_, message, _, _) => message,
        }
    }

    pub fn get_zhongliang(&self) -> f32 {
        match self {
            SecondGun::NONE => 0.0,
            SecondGun::LEVEL1(_, _, zhongliang, _) => *zhongliang,
            SecondGun::LEVEL2(_, _, zhongliang, _) => *zhongliang,
        }
    }

    pub fn get_shanghai(&self) -> i32 {
        match self {
            SecondGun::NONE => 0,
            SecondGun::LEVEL1(_, _, _, shanghai) => *shanghai,
            SecondGun::LEVEL2(_, _, _, shanghai) => *shanghai,
        }
    }
}


impl Engine {
    pub fn get_name(&self) -> &str {
        match self {
            Engine::NONE => "无",
            Engine::LEVEL1(name, _, _) => name,
            Engine::LEVEL2(name, _, _) => name,
        }
    }

    pub fn get_message(&self) -> &str {
        match self {
            Engine::NONE => "无",
            Engine::LEVEL1(_, message, _) => message,
            Engine::LEVEL2(_, message, _) => message,
        }
    }

    pub fn get_zhongliang(&self) -> f32 {
        match self {
            Engine::NONE => 0.0,
            Engine::LEVEL1(_, _, zhongliang) => *zhongliang,
            Engine::LEVEL2(_, _, zhongliang) => *zhongliang,
        }
    }
}



impl Cannon {
    pub fn new(cannon_type: CannonType, cannon_config: &CannonConfig) -> Self {
        Cannon {
            available: true,
            name: cannon_config.name.clone(),
            description: cannon_config.description.clone(),
            max_ammo: cannon_config.max_ammo,
            damage: cannon_config.damage,
            weight: cannon_config.weight,
            current_ammo: cannon_config.max_ammo,
            cannon_type: cannon_type,
        }
    }

    pub fn remove(&mut self) {
        self.available = false;
        self.name.clear();
        self.description.clear();
        self.max_ammo = 0;
        self.damage = 0;
        self.weight = 0.0;
        self.current_ammo = 0;
    }

    pub fn update(&mut self,cannon_type: CannonType, cannon_config: &CannonConfig) {
        self.available = false;
        self.name = cannon_config.name.clone();
        self.description = cannon_config.description.clone();
        self.max_ammo = cannon_config.max_ammo;
        self.damage = cannon_config.damage;
        self.weight = cannon_config.weight;
        self.current_ammo = cannon_config.max_ammo;
        self.cannon_type = cannon_type;
    }
}


impl CannonDataBase {
    pub fn load() -> Result<CannonDataBase>{
        let config_path = "configs/Cannons.ron";

        let config_str = std::fs::read_to_string(config_path).unwrap();

        let configs : HashMap<CannonType, CannonConfig> = ron::from_str(&config_str).unwrap();

        Ok(
            Self {
                configs
            }
        )

    }

    pub fn get(&self, cannon_type: CannonType) -> Option<&CannonConfig> {
        self.configs.get(&cannon_type)
    }
}