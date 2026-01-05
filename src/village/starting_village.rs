use bevy::prelude::*;
use crate::components::village::*;
use crate::village::*;

// 创建起始村庄
pub fn crate_starting_village(
    mut commands : Commands,
    mut village_manager : ResMut<VillageManager>,
) {
    // 创建村庄数据
    let village_id = "village_001".to_string();
    let village_data = VillageData {
        id : village_id.clone(),
        name : "希望村".to_string(),
        village_type : VillageType::Starting,
        description : "人类最后的希望".to_string(),
        position : (0.0 , 0.0),
        unlocked : true,

        has_shop : true,
        has_garage : true,
        has_bar : true,
        has_clinic : true,
        has_inn : true,
        population : 80,
        defense : 30,
        supplies : 60,
    };

    // 添加到管理器
    village_manager.villages.insert(village_id.clone() ,village_data);
    village_manager.current_village = Some(village_id.clone());
    village_manager.discovered_villages.push(village_id.clone());

    //创建村庄实体
    let village_entry = commands.spawn(
        (
            //Sprite::default(),
            Transform::from_xyz(0.0, 0.0, 0.0),
            village {
                id : village_id.clone(),
                village_type : VillageType::Starting,
            },
            Name::new("希望村"),
        )

    ).id();

    //地面
    commands.spawn(
    (
            Sprite {
                color: Color::srgb(0.3, 0.5, 0.3),
                custom_size: Some(Vec2::new(1000.0, 1000.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 1.0),
            Name::new("村庄地面"),
        )
    );

    //建筑
    create_buildings(&mut commands, village_entry);

    create_npc(&mut commands, village_entry);
}

fn create_buildings(commands : &mut Commands, parent : Entity) {
    let buildings = [
        (BuildingType::TownHall, "镇公所", -200.0, 50.0, Color::srgb(0.6, 0.4, 0.2)),
        (BuildingType::Shop, "杂货店", 150.0, 100.0, Color::srgb(0.9, 0.8, 0.1)),
        (BuildingType::Bar, "酒吧", 200.0, -100.0, Color::srgb(0.7, 0.2, 0.2)),
        (BuildingType::Clinic, "诊所", -150.0, -80.0, Color::srgb(0.9, 0.9, 0.9)),
    ];

    for (buildtype, name, x, y, color) in buildings {
        commands.spawn(
    (

                Sprite {
                        color,
                        custom_size: Some(Vec2::new(100.0, 100.0)),
                        ..default()
                },
                
                Transform::from_xyz(x, y, 2.0),
                Building {
                    buildingtype : buildtype,
                    is_enterable : true,
                    interactable : true,
                },
                Name::new(name),
            )
        );

    }
}

fn create_npc(commands : &mut Commands, parent : Entity) {
    let npcs = [
        ("npc_001", "老村长", NPCType::Mayor, -200.0, 0.0, Color::srgb(0.1, 0.1, 0.8), "mayor_intro", None, Some("find_ancient_tank")),
        ("npc_002", "老陈", NPCType::Merchant, 170.0, 100.0, Color::srgb(0.1, 0.8, 0.1), "merchant_greeting", Some("shop_001"), None),
        ("npc_003", "铁锤", NPCType::Bartender, 220.0, -100.0, Color::srgb(0.8, 0.5, 0.1), "bartender_gossip", None, Some("clear_rats")),
    ];
    
    for (id, name, npc_type, x, y, color, dialogue_id, shop_id, quest_id) in npcs {
        commands.spawn((

            Sprite {
                color,
                custom_size: Some(Vec2::new(20.0, 30.0)),
                ..default()
            },
            Transform::from_xyz(x, y, 3.0),
            NPC {
                id: id.to_string(),
                name: name.to_string(),
                npc_type,
                dialogue_id: dialogue_id.to_string(),
                shop_id: shop_id.map(|s| s.to_string()),
                quest_id: quest_id.map(|q| q.to_string()),
            },
            Name::new(name),
            println!("set {} success",name),
        ));
    }
}