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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CannonType {
    CannonLevel1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SecgunType {
    SecGunLevel1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EngineType {
    EngineLevel1,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CannonConfig {
    pub name: String,
    pub description: String,
    pub max_ammo: u32,
    pub damage: u32,
    pub weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecgunConfig {
    pub name: String,
    pub description: String,
    pub damage: u32,
    pub weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfig {
    pub name: String,
    pub description: String,
    pub weight: f32,
}

// 主炮Map资源
#[derive(Resource)]
pub struct CannonDataBase {
    pub configs: HashMap<CannonType, CannonConfig>,
}

//副炮资源表
#[derive(Resource)]
pub struct SecgunDataBase {
    pub configs: HashMap<SecgunType, SecgunConfig>,
}

//引擎资源表
#[derive(Resource)]
pub struct EngineDataBase {
    pub configs: HashMap<EngineType, EngineConfig>,
}

//主炮组件
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

//副炮组件
#[derive(Component)]
pub struct SecGun {
    pub name: String,
    pub description: String,
    pub damage: u32,
    pub weight: f32,
    pub available: bool,
    pub secgun_type: SecgunType,

}

//引擎组件
#[derive(Component)]
pub struct Engine {
    pub name: String,
    pub description: String,
    pub weight: f32,
    pub available: bool,
    pub enginetype: EngineType,

}




#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum Appstatus {
    #[default] // 标记默认状态是 Setup
    Menu,
    Game,
    Vollage,
    WorldMap,
    SearchEnemy,
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


impl SecGun {
    pub fn new(secgun_type: SecgunType, secgun_config: &SecgunConfig) -> Self {
        SecGun {
            available: true,
            name: secgun_config.name.clone(),
            description: secgun_config.description.clone(),
            damage: secgun_config.damage,
            weight: secgun_config.weight,
            secgun_type: secgun_type,
        }
    }
}

impl Engine {
    pub fn new(engine_type: EngineType, engine_config: &EngineConfig) -> Self {
        Engine {
            available: true,
            name: engine_config.name.clone(),
            description: engine_config.description.clone(),
            weight: engine_config.weight,
            enginetype: engine_type,
        }
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


impl SecgunDataBase {
    pub fn load() -> Result<SecgunDataBase>{
        let config_path = "configs/SecGun.ron";

        let config_str = std::fs::read_to_string(config_path).unwrap();

        let configs : HashMap<SecgunType, SecgunConfig> = ron::from_str(&config_str).unwrap();

        Ok(
            Self {
                configs
            }
        )

    }

    pub fn get(&self, secgun_type: SecgunType) -> Option<&SecgunConfig> {
        self.configs.get(&secgun_type)
    }
}

impl EngineDataBase {
    pub fn load() -> Result<EngineDataBase>{
        let config_path = "configs/Engine.ron";

        let config_str = std::fs::read_to_string(config_path).unwrap();

        let configs : HashMap<EngineType, EngineConfig> = ron::from_str(&config_str).unwrap();

        Ok(
            Self {
                configs
            }
        )

    }

    pub fn get(&self, engine_type: EngineType) -> Option<&EngineConfig> {
        self.configs.get(&engine_type)
    }
}