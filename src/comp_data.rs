use bevy::prelude::*;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use rand::prelude::*;

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
    #[default]
    Menu,
    Game,
    Vollage,
    WorldMap,
    SearchEnemy,
    War,
    Upgrade,
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

// ============ 升级经验/材料配置 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeLevelConfig {
    /// 目标等级（例如 level: 2 表示从 Lv.1→Lv.2 的升级配置）
    pub level: u32,
    /// 需要积累的经验值
    pub exp_needed: u32,
    /// 需要消耗的材料列表
    pub cost: Vec<(ItemType, u32)>,
}

#[derive(Resource)]
pub struct UpgradeDataBase {
    pub configs: Vec<UpgradeLevelConfig>,
    pub max_level: u32,
}

impl UpgradeDataBase {
    pub fn load() -> Result<UpgradeDataBase> {
        let config_path = "configs/Upgrade.ron";
        let config_str = std::fs::read_to_string(config_path).unwrap();
        let raw: RawUpgradeData = ron::from_str(&config_str).unwrap();
        Ok(Self {
            configs: raw.levels,
            max_level: raw.max_level,
        })
    }

    /// 查找某个等级的升级配置
    pub fn for_level(&self, level: u32) -> Option<&UpgradeLevelConfig> {
        self.configs.iter().find(|c| c.level == level)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RawUpgradeData {
    levels: Vec<UpgradeLevelConfig>,
    max_level: u32,
}

// ============ 等级 / 升级系统 ============

#[derive(Resource, Debug, Clone)]
pub struct PlayerLevel {
    pub level: u32,
    pub exp: u32,
}

impl PlayerLevel {
    pub fn new() -> Self {
        Self { level: 1, exp: 0 }
    }

    /// 当前等级的攻击加成倍率（每级 +10%，等级 1 为 1.0x）
    pub fn attack_multiplier(&self) -> f32 {
        1.0 + (self.level as f32 - 1.0) * 0.1
    }

    /// 积累经验值（仅累加，不自动升级）
    pub fn gain_exp(&mut self, amount: u32) {
        self.exp += amount;
    }
}

// ============ 物品 & 背包系统 ============

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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

    /// 用于 UI 显示的图标 emoji
    pub fn icon(&self) -> &'static str {
        match self {
            ItemType::ScrapIron => "⚙️",
            ItemType::Leather => "🧤",
            ItemType::CopperWire => "🔌",
            ItemType::DryBattery => "🔋",
            ItemType::HighStrengthSpring => "🌀",
            ItemType::BearPaw => "🐾",
        }
    }

    /// 物品品质色 (0-金色, 1-蓝, 2-绿, 3-白)
    pub fn rarity_color(&self) -> Color {
        match self {
            ItemType::BearPaw => Color::srgb(1.0, 0.84, 0.0),          // 金色
            ItemType::HighStrengthSpring => Color::srgb(0.3, 0.6, 1.0), // 蓝色
            ItemType::DryBattery => Color::srgb(0.3, 0.9, 0.4),         // 绿色
            ItemType::CopperWire => Color::srgb(0.4, 0.8, 0.8),         // 青色
            ItemType::Leather => Color::srgb(0.8, 0.6, 0.4),            // 棕色
            ItemType::ScrapIron => Color::srgb(0.7, 0.7, 0.7),          // 灰色
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
pub const BACKPACK_MAX_SLOTS: usize = 30;

impl Backpack {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// 添加物品。每个数量单位占1格，总格子数不能超过上限。
    pub fn add(&mut self, item_type: ItemType, quantity: u32) -> Result<(), String> {
        let total: u32 = self.items.iter().map(|s| s.quantity).sum();
        if total + quantity > BACKPACK_MAX_SLOTS as u32 {
            return Err(format!("背包已满（{}/{}）！", total, BACKPACK_MAX_SLOTS));
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

    /// 背包当前物品总数量（每个数量单位占1格）
    pub fn used_slots(&self) -> usize {
        self.items.iter().map(|s| s.quantity as usize).sum()
    }

    /// 判断是否有至少1个空位
    pub fn has_space_for(&self, _item_type: ItemType) -> bool {
        let total: u32 = self.items.iter().map(|s| s.quantity).sum();
        (total as usize) < BACKPACK_MAX_SLOTS
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

// ============ 敌人配置系统 ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LootEntryConfig {
    pub item: ItemType,
    pub quantity: u32,
    pub probability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnemyConfig {
    pub name: String,
    pub hp: i32,
    pub damage: i32,
    pub weight: u32,
    pub image_path: Option<String>,
    pub loot: Vec<LootEntryConfig>,
}

#[derive(Resource)]
pub struct EnemyDataBase {
    pub configs: Vec<EnemyConfig>,
}

impl EnemyDataBase {
    pub fn load() -> Result<EnemyDataBase> {
        let config_path = "configs/Enemies.ron";
        let config_str = std::fs::read_to_string(config_path).unwrap();
        let configs: Vec<EnemyConfig> = ron::from_str(&config_str).unwrap();
        Ok(Self { configs })
    }

    pub fn pick_random(&self) -> &EnemyConfig {
        let total_weight: u32 = self.configs.iter().map(|c| c.weight).sum();
        if total_weight == 0 {
            return &self.configs[0];
        }
        let mut rng = thread_rng();
        let roll = rng.gen_range(0..total_weight);
        let mut cumulative = 0;
        for config in self.configs.iter() {
            cumulative += config.weight;
            if roll < cumulative {
                return config;
            }
        }
        &self.configs[self.configs.len() - 1]
    }

    /// 按名称查找敌人配置
    pub fn find_by_name(&self, name: &str) -> Option<&EnemyConfig> {
        self.configs.iter().find(|c| c.name == name)
    }
}