use bevy::prelude::*;
use crate::village::VillageType;
// 村庄标记组件

#[derive(Component)]
pub struct DialogueEvent {
    pub npc_id: String,
    pub dialogue_id: String,
}

#[derive(Component)]
pub struct village {
    pub id : String,
    pub village_type : VillageType,
}

//建筑组件
#[derive(Component)]
pub struct Building {
    pub buildingtype : BuildingType,
    pub is_enterable : bool,
    pub interactable : bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildingType {
    Shop,       // 商店
    Garage,     // 车库
    Bar,        // 酒吧
    Clinic,     // 诊所
    Inn,        // 旅馆
    TownHall,   // 镇公所
    House,      // 民居
    Warehouse,  // 仓库
}


// NPC 组件
#[derive(Component)]
pub struct NPC {
    pub id : String,
    pub name : String,
    pub npc_type : NPCType,
    pub dialogue_id : String, // 对话树
    pub shop_id : Option<String>, // 商店ID
    pub quest_id : Option<String>, //任务ID
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NPCType {
    Villager,   // 村民
    Merchant,   // 商人
    Mechanic,   // 机械师
    Doctor,     // 医生
    Bartender,  // 酒保
    Guard,      // 守卫
    Mayor,      // 镇长
    QuestGiver, // 任务给予者
}

