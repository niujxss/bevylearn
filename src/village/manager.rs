use bevy::prelude::*;
use std::collections::HashMap;

// 村庄类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VillageType {
    Starting,       // 起始村庄
    Merchant,       // 商业村庄
    Military,       // 军事据点
    Ruined,         // 废墟村庄
    Hidden,         // 隐藏村庄
    Research,       // 研究前哨
}


//村庄数据
#[derive(Debug, Clone)]
pub struct VillageData {
    pub id : String,           //唯一标识
    pub name : String,         // 村庄名称
    pub village_type : VillageType,   // 村庄类型
    pub description : String, // 描述
    pub position: (f32 , f32),  //位置
    pub unlocked : bool , // 是否已经解锁

    // 设施
    pub has_shop: bool, //是否有商店
    pub has_garage : bool, // 是否有车库
    pub has_bar : bool, // 是否有酒吧
    pub has_clinic : bool, // 是否有诊所
    pub has_inn : bool, // 是否有旅馆

    // 资源
    pub population : u32 , // 人口
    pub defense : u32 , // 防御等级
    pub supplies : u32 , // 物资存量
}


// 村庄管理器资源
#[derive(Resource, Default)]
pub struct VillageManager {
    pub villages : HashMap<String , VillageData>, // 所有村庄
    pub current_village : Option<String> , // 当前所在村庄
    pub discovered_villages : Vec<String>, // 已发现的村庄
}