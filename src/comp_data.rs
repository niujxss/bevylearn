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
    War,
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

// ============ 物品 & 背包系统 ============

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemType {
    ScrapIron,            // 废铁
    Leather,              // 皮革
    CopperWire,           // 铜线
    DryBattery,           // 干电池
    HighStrengthSpring,   // 高强度弹簧
    BearPaw,              // 熊掌
}

impl ItemType {
    pub fn name(&self) -> &'static str {
        match self {
            ItemType::ScrapIron => "废铁",
            ItemType::Leather => "皮革",
            ItemType::CopperWire => "铜线",
            ItemType::DryBattery => "干电池",
            ItemType::HighStrengthSpring => "高强度弹簧",
            ItemType::BearPaw => "熊掌",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ItemStack {
    pub item_type: ItemType,
    pub quantity: u32,
}

#[derive(Resource, Debug, Clone)]
pub struct Backpack {
    pub items: Vec<ItemStack>,
}

impl Default for Backpack {
    fn default() -> Self {
        Self::new()
    }
}

/// 背包最大格数（每行3格，共5行）
pub const BACKPACK_MAX_SLOTS: usize = 15;

impl Backpack {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// 添加物品。如果物品已存在则直接增加数量（不占新格）；
    /// 如果是新物品且背包已满则返回错误。
    pub fn add(&mut self, item_type: ItemType, quantity: u32) -> Result<(), String> {
        // 如果该物品类型已存在，不占新格
        let exists = self.items.iter().any(|s| s.item_type == item_type);
        if !exists && self.items.len() >= BACKPACK_MAX_SLOTS {
            return Err(format!("背包已满，无法携带更多物品！"));
        }
        for stack in self.items.iter_mut() {
            if stack.item_type == item_type {
                stack.quantity += quantity;
                return Ok(());
            }
        }
        self.items.push(ItemStack { item_type, quantity });
        Ok(())
    }

    /// 移除指定数量的物品，数量不足或不存在时返回错误。
    pub fn remove(&mut self, item_type: ItemType, quantity: u32) -> Result<(), String> {
        for i in 0..self.items.len() {
            if self.items[i].item_type == item_type {
                if self.items[i].quantity < quantity {
                    return Err(format!("{}不足，无法取出", item_type.name()));
                }
                self.items[i].quantity -= quantity;
                if self.items[i].quantity == 0 {
                    self.items.remove(i);
                }
                return Ok(());
            }
        }
        Err(format!("背包中没有{}", item_type.name()))
    }

    /// 背包当前占用的格数
    pub fn used_slots(&self) -> usize {
        self.items.len()
    }

    /// 判断是否有空位容纳新物品类型
    pub fn has_space_for(&self, item_type: ItemType) -> bool {
        self.items.iter().any(|s| s.item_type == item_type)
            || self.items.len() < BACKPACK_MAX_SLOTS
    }

    /// 文本摘要 - 3列网格格式
    pub fn summary(&self) -> String {
        if self.items.is_empty() {
            return "背包空空如也……".to_string();
        }
        let mut grid = Vec::new();
        let mut row = Vec::new();
        for s in self.items.iter() {
            row.push(format!("{:8}×{}", s.item_type.name(), s.quantity));
            if row.len() == 3 {
                grid.push(row.join("    "));
                row.clear();
            }
        }
        if !row.is_empty() {
            grid.push(row.join("    "));
        }
        let header = format!("📦 背包 ({}/{})\n", self.used_slots(), BACKPACK_MAX_SLOTS);
        header + &grid.join("\n")
    }
}

// ============ 仓库系统 ============

#[derive(Resource, Debug, Clone)]
pub struct Warehouse {
    pub items: Vec<ItemStack>,
}

impl Default for Warehouse {
    fn default() -> Self {
        Self::new()
    }
}

impl Warehouse {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// 存入物品（无限容量）
    pub fn store(&mut self, item_type: ItemType, quantity: u32) {
        for stack in self.items.iter_mut() {
            if stack.item_type == item_type {
                stack.quantity += quantity;
                return;
            }
        }
        self.items.push(ItemStack { item_type, quantity });
    }

    /// 取出物品，数量不足时返回错误
    pub fn take(&mut self, item_type: ItemType, quantity: u32) -> Result<(), String> {
        for i in 0..self.items.len() {
            if self.items[i].item_type == item_type {
                if self.items[i].quantity < quantity {
                    return Err(format!("仓库中{}不足", item_type.name()));
                }
                self.items[i].quantity -= quantity;
                if self.items[i].quantity == 0 {
                    self.items.remove(i);
                }
                return Ok(());
            }
        }
        Err(format!("仓库中没有{}", item_type.name()))
    }

    /// 文本摘要
    pub fn summary(&self) -> String {
        if self.items.is_empty() {
            return "仓库空空如也……".to_string();
        }
        let mut lines: Vec<String> = self.items
            .iter()
            .map(|s| format!("{} × {}", s.item_type.name(), s.quantity))
            .collect();
        lines.join("\n")
    }
}