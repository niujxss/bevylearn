use bevy::prelude::*;
use crate::comp_data::*;

// ============ UI Marker Components ============

#[derive(Component)]
pub struct EnemyHpBarFill;

#[derive(Component)]
pub struct PlayerHpBarFill;

#[derive(Component)]
pub struct EnemyHpText;

#[derive(Component)]
pub struct PlayerHpText;

#[derive(Component)]
pub struct PlayerAmmoText;

#[derive(Component)]
pub struct BattleLogText;

#[derive(Component)]
pub struct FireButton;

#[derive(Component)]
pub struct RetreatButton;

#[derive(Component)]
pub struct ResultReturnButton;

#[derive(Component)]
pub struct WarBg;

// ============ Enemy 组件 ============

#[derive(Component)]
pub struct Enemy {
    pub name: String,
    pub hp: i32,
    pub max_hp: i32,
    pub damage: i32,
}

// ============ 战斗状态资源 ============

#[derive(Resource)]
pub struct BattleLog {
    pub messages: Vec<String>,
}

#[derive(Resource)]
pub struct BattleOver {
    pub player_won: bool,
    pub over: bool,
}

// ============ 创建战斗 UI ============

fn roll_loot(config: &EnemyConfig) -> Vec<(ItemType, u32)> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let mut results = Vec::new();
    for (i, entry) in config.loot.iter().enumerate() {
        let seed = ((now.subsec_nanos() as u64).wrapping_add((i as u64) * 7919)) % 10000;
        let roll = seed as f32 / 10000.0;
        if roll < entry.probability {
            results.push((entry.item, entry.quantity));
        }
    }
    results
}


//文本创建
fn text_bundle( text: impl Into<String>, // impl 不用解释了； Into<String> 代表可以被转换成string 类型的 trait; 
                font: Handle<Font>,     // Handle<> bevy 的资源权柄；Handle<Font> 指的是字体资源
                font_size: f32,
                color: Color,
) -> impl Bundle {
    (
        Text::new(text.into()), // text.into 会自动将类型转换为String
        TextFont {
            font,
            font_size,
            ..default()
        },
        TextColor(color),
    )
}




const HP_TEXT_COLOR: Color = Color::srgb(1.0, 0.6, 0.6);
// HP 条
fn spawn_hp_bar(
    current: i32,
    max: i32,
    fill_color: Color,
    border_color: Color,
    bg_color: Color,
    font: Handle<Font>, 
    marker: impl Component,
    marker_text: impl Component,
) ->impl Bundle {
    let fill_percent = (current as f32 / max as f32) * 100.0;
    {
        (
            Node {
                width: percent(100.0),
                height: px(25.0),
                flex_direction: FlexDirection::Column,  // 子元素垂直排列，从上到下
                ..default()
            },
            children![
                //血条背景
                (
                    Node {
                        width: percent(100.0),
                        height: px(20.0),
                        border: UiRect::all(px(1.0)), //边框
                        ..default()
                    },
                    BorderColor:: all(border_color), //边框颜色
                    BackgroundColor(bg_color), //背景颜色
                    children![(
                        Node {
                            width: percent(fill_percent),
                            height: percent(100.0),
                            ..default()
                        },
                        BackgroundColor(fill_color),
                        marker,
                    )],
                ),

                //HP文字
                (
                    text_bundle(format!("HP: {}/{}", current, max), font, 14.0, HP_TEXT_COLOR),
                    marker_text,
                ),
            ],
        )
    }
}


//角色卡
fn spawn_character_card(
    image: Option<Handle<Image>>,
    emoji: &str,
    name: &str,
    hp: i32,
    max_hp: i32,
    hp_fill_color: Color,
    hp_border_color: Color,
    hp_bg_color: Color,
    border_color: Color,
    bg_color: Color,
    font: Handle<Font>,
    hp_fill_marker: impl Component,
    hp_text_marker: impl Component,
) -> impl Bundle {
    let name_color = if name == "你的战车" {
        Color::srgb(0.3, 0.85, 1.0)
    } else {
        Color::srgb(1.0, 0.4, 0.3)
    };
    (
        Node {
            width: percent(49.0),
            height: percent(100.0),
            flex_direction: FlexDirection::Column, // 子元素垂直
            align_items: AlignItems::Center, // 垂直居中
            justify_content: JustifyContent::SpaceBetween, // 水平
            //border: UiRect::all(px(2.0)), //边框
            row_gap: px(8.0), // 垂直间隔
            ..default()
        },
        BorderColor::all(border_color), //边框颜色
        BorderRadius::all(px(10.0)), // 边框圆角
        BackgroundColor(bg_color),

        children![
            // 名称
            (
                Node {
                    width: percent(100.0),
                    height: percent(10.0),
                    flex_direction: FlexDirection::Row, // 子元素水平排列
                    justify_content: JustifyContent::Center, // 水平居中
                    align_items: AlignItems::Center, // 垂直居中
                    column_gap: px(8.0), // 水平间隔
                    ..default()
                },
                children![
                    text_bundle(emoji, font.clone(), 22.0, Color::WHITE),
                    text_bundle(format!("【{}】", name), font.clone(), 24.0, name_color),
                ]
            ),
            // 图像或表情
            if let Some(img) = image {
                (
                    Node {
                        width: percent(60.0),
                        height: percent(60.0),
                        // flex_grow: 1.0,   // 填满卡片剩余高度
                        // flex_direction: FlexDirection::Column, // 子元素垂直排列
                        // align_items: AlignItems::Center, // 垂直居中
                        // justify_content: JustifyContent::Center, // 水平居中
                        // //border: UiRect::all(px(1.0)), //边框
                        ..default()
                    },
                    BorderColor::all(border_color),
                    BorderRadius::all(px(8.0)),
                    BackgroundColor(bg_color), // 稍微调亮
                    ImageNode::new(img).with_mode(NodeImageMode::Auto),
                   
                )
            } else
            {
                (
                    Node {
                        width: percent(60.0),
                        height: percent(60.0),
                        flex_grow: 1.0,   // 填满卡片剩余高度
                        flex_direction: FlexDirection::Column, // 子元素垂直排列
                        align_items: AlignItems::Center, // 垂直居中
                        justify_content: JustifyContent::Center, // 水平居中
                        border: UiRect::all(px(1.0)), //边框
                        ..default()
                    },
                    BorderColor::all(border_color),
                    BorderRadius::all(px(8.0)),
                    BackgroundColor(bg_color), // 稍微调亮
                    ImageNode::default(),
                )
            },

            //HP 条
            spawn_hp_bar(hp, max_hp, hp_fill_color, hp_border_color, hp_bg_color, font, hp_fill_marker, hp_text_marker),
        ]
    )

}



fn create_button(font: Handle<Font>,
) ->impl Bundle {
    // ===== 底部区域 =====
    (
        Node {
            width: percent(100.0),
            height: percent(20.0),
            position_type: PositionType::Absolute,
            bottom: px(0),
            flex_direction: FlexDirection::Row,
            ..default()
        },
        BackgroundColor(Color::srgb(0.06, 0.06, 0.14)),
        BorderColor::all(Color::srgb(0.2, 0.2, 0.3)),
        children![
            // ===== 底部左侧：按钮 =====
            (
                Node {
                    width: percent(45),
                    height: percent(100),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: px(16.0),
                    padding: UiRect::all(px(10.0)),
                    ..default()
                },
                children![
                    // 开火按钮
                    (
                        Button,
                        Node {
                            width: px(80),
                            height: px(50),
                            border: UiRect::all(px(3)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BorderColor::all(Color::srgb(0.8, 0.2, 0.2)),
                        BorderRadius::all(px(8.0)),
                        BackgroundColor(Color::srgb(0.5, 0.08, 0.08)),
                        FireButton,
                        children![(
                            Text::new("开火"),
                            TextFont { font: font.clone(), font_size: 16.0, ..default() },
                            TextColor(Color::srgb(1.0, 0.8, 0.8)),
                        )],
                    ),
                    // 撤退按钮
                    (
                        Button,
                        Node {
                            width: px(80),
                            height: px(50),
                            border: UiRect::all(px(3)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BorderColor::all(Color::srgb(0.4, 0.4, 0.5)),
                        BorderRadius::all(px(8.0)),
                        BackgroundColor(Color::srgb(0.2, 0.2, 0.25)),
                        RetreatButton,
                        children![(
                            Text::new("撤退"),
                            TextFont { font: font.clone(), font_size: 16.0, ..default() },
                            TextColor(Color::srgb(0.8, 0.8, 0.9)),
                        )],
                    ),
                ],
            ),
            // ===== 底部右侧：战况 =====
            (
                Node {
                    width: percent(55),
                    height: percent(100),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    border: UiRect::left(px(1)),
                    padding: UiRect::all(px(10.0)),
                    ..default()
                },
                BorderColor::all(Color::srgb(0.2, 0.2, 0.3)),
                children![
                    (
                        Text::new("战况"),
                        TextFont { font: font.clone(), font_size: 20.0, ..default() },
                        TextColor(Color::srgb(0.8, 0.8, 0.5)),
                    ),
                    (
                        Text::new("战斗开始！"),
                        TextFont { font: font.clone(), font_size: 18.0, ..default() },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        BattleLogText,
                        TextLayout::new_with_justify(Justify::Center),
                    ),
                ],
            ),
        ],
    )
}

pub fn create_battle_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<(&Player, &Cannon)>,
    enemy_db: Res<EnemyDataBase>,
) {
    let (player, cannon) = player_query.single().unwrap();
    let template = enemy_db.pick_random().clone();
    let enemy_name = template.name.clone();
    let enemy_hp = template.hp;
    let enemy_damage = template.damage;

    // ===== 提前加载所有资源 =====
    let font: Handle<Font> = asset_server.load("fonts/STKAITI.TTF");
    let tank_img: Handle<Image> = asset_server.load("tank.png");
    let bg_img: Handle<Image> = asset_server.load("FeiTuHuangYuan_back.png");
    let enemy_img: Option<Handle<Image>> = template
        .image_path
        .clone()
        .map(|path| asset_server.load(path));

    // ===== 初始化战斗资源 =====
    commands.insert_resource(BattleLog { messages: Vec::new() });
    commands.insert_resource(BattleOver { player_won: false, over: false });

    // ===== 生成敌人实体 =====
    commands.spawn((
        Enemy { name: enemy_name.clone(), hp: enemy_hp, max_hp: enemy_hp, damage: enemy_damage },
        DespawnOnExit(Appstatus::War),
    ));

    commands.spawn((
    DespawnOnExit(Appstatus::War),
    Name::new("WarRoot"),
    WarBg,
    Node {
        width: percent(100.0),
        height: percent(100.0),
        flex_direction: FlexDirection::Column,
        ..default()
    },
    BackgroundColor(Color::srgb(0.05, 0.05, 0.12)),
    children![
        // 顶部背景
        (
            Node {
                width: percent(100.0),
                height: percent(100.0),
                position_type: PositionType::Absolute,
                top: px(0.0),
                ..default()
            },
            ImageNode::new(bg_img.clone()),
        ),
        // 对战主体区域
        (
            Node {
                width: percent(100.0),
                height: percent(40.0),
                flex_direction: FlexDirection::Row, // 子元素水平排列
                align_items: AlignItems::Center, // 子元素垂直居中
                justify_content: JustifyContent::SpaceBetween,
                //padding: UiRect::all(px(16.0)),
                //column_gap: px(20.0),
                ..default()
            },
            children![
                // 敌方卡片
                spawn_character_card(
                    enemy_img.clone(),
                    "👾",
                    &enemy_name,
                    enemy_hp,
                    enemy_hp,
                    Color::srgb(0.9, 0.15, 0.15),   // HP 填充色
                    Color::srgb(0.5, 0.2, 0.2),     // HP 边框色
                    Color::srgb(0.2, 0.05, 0.05),    // HP 背景色
                    Color::srgb(0.6, 0.15, 0.15),    // 卡片边框色
                    Color::srgb(0.08, 0.04, 0.10),  // 卡片背景色
                    font.clone(),
                    EnemyHpBarFill,
                    EnemyHpText,
                ),
                // 玩家卡片
                spawn_character_card(
                    Some(tank_img.clone()),
                    "⚔️",
                    "你的战车",
                    player.health,
                    player.max_health,
                    Color::srgb(0.15, 0.85, 0.15),
                    Color::srgb(0.2, 0.5, 0.2),
                    Color::srgb(0.05, 0.15, 0.05),
                    Color::srgb(0.15, 0.5, 0.7),
                    Color::srgb(0.04, 0.08, 0.12),
                    font.clone(),
                    PlayerHpBarFill,
                    PlayerHpText,
                ),
            ],
        ),
        create_button(font.clone(),),
    ],
    ));
        


    // 初始日志
    let mut log = BattleLog { messages: Vec::new() };
    log.add(format!("{} 出现了！", enemy_name));
    log.add("选择你的行动...".to_string());
    commands.insert_resource(log);
}

// ============ 每帧更新 UI 显示 ============

pub fn update_battle_display(
    player_query: Query<(&Player, &Cannon)>,
    enemy_query: Query<&Enemy>,
    log: Res<BattleLog>,
    mut node_params: ParamSet<(
        Query<&mut Node, With<EnemyHpBarFill>>,
        Query<&mut Node, With<PlayerHpBarFill>>,
    )>,
    mut text_params: ParamSet<(
        Query<&mut Text, With<EnemyHpText>>,
        Query<&mut Text, With<PlayerHpText>>,
        Query<&mut Text, With<PlayerAmmoText>>,
        Query<&mut Text, With<BattleLogText>>,
    )>,
) {
    // 更新敌方 HP 条
    if let Ok(enemy) = enemy_query.single() {
        if let Ok(mut node) = node_params.p0().single_mut() {
            let pct = (enemy.hp as f32 / enemy.max_hp as f32).max(0.0) * 100.0;
            node.width = Val::Percent(pct);
        }
        if let Ok(mut text) = text_params.p0().single_mut() {
            text.0 = format!("HP: {}/{}", enemy.hp.max(0), enemy.max_hp);
        }
    }

    // 更新玩家 HP 条和弹药
    if let Ok((player, cannon)) = player_query.single() {
        if let Ok(mut node) = node_params.p1().single_mut() {
            let pct = (player.health as f32 / player.max_health as f32).max(0.0) * 100.0;
            node.width = Val::Percent(pct);
        }
        if let Ok(mut text) = text_params.p1().single_mut() {
            text.0 = format!("HP: {}/{}", player.health.max(0), player.max_health);
        }
        if let Ok(mut text) = text_params.p2().single_mut() {
            text.0 = format!("【你的战车】弹药: {}/{}", cannon.current_ammo, cannon.max_ammo);
        }
    }

    // 更新战斗日志（显示最近4条）
    if let Ok(mut text) = text_params.p3().single_mut() {
        let start = log.messages.len().saturating_sub(4);
        text.0 = log.messages[start..].join("\n");
    }
}

// ============ 开火按钮 ============

pub fn check_fire_button(
    _commands: Commands,
    mut player_query: Query<(&mut Player, &mut Cannon)>,
    mut enemy_query: Query<&mut Enemy>,
    mut log: ResMut<BattleLog>,
    mut battle_over: ResMut<BattleOver>,
    player_level: Res<PlayerLevel>,
    mut fire_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<FireButton>),
    >,
) {
    if let Ok((inter, mut bg)) = fire_query.single_mut() {
        match inter {
            Interaction::Pressed => {
                bg.0 = Color::srgb(0.8, 0.2, 0.2);

                if battle_over.over {
                    log.add("战斗已经结束了！".to_string());
                    return;
                }

                let (mut player, mut cannon) = player_query.single_mut().unwrap();
                let mut enemy = enemy_query.single_mut().unwrap();

                if cannon.current_ammo == 0 {
                    log.add("炮弹用完了！无法开火！".to_string());
                    return;
                }

                let base_damage = cannon.damage;
                // 等级攻击加成：基础伤害 × 加成倍率
                let multiplier = player_level.attack_multiplier();
                let damage = (base_damage as f32 * multiplier).round() as u32;
                cannon.current_ammo -= 1;
                enemy.hp -= damage as i32;
                log.add(format!(
                    "开火！【{}】造成 {} 点伤害！(Lv.{} x{:.0}%加成)",
                    cannon.name, damage, player_level.level, (multiplier - 1.0) * 100.0
                ));

                if enemy.hp <= 0 {
                    enemy.hp = 0;
                    battle_over.over = true;
                    battle_over.player_won = true;
                    log.add(format!("【{}】被击败了！胜利！", enemy.name));
                    return;
                }

                let enemy_dmg = enemy.damage;
                player.health -= enemy_dmg;
                log.add(format!("【{}】反击，造成 {} 点伤害！", enemy.name, enemy_dmg));

                if player.health <= 0 {
                    player.health = 0;
                    battle_over.over = true;
                    battle_over.player_won = false;
                    log.add("你的战车被摧毁了……".to_string());
                }
            }
            Interaction::Hovered => {
                bg.0 = Color::srgb(0.8, 0.3, 0.3);
            }
            Interaction::None => {
                bg.0 = Color::srgb(0.6, 0.1, 0.1);
            }
        }
    }
}

// ============ 撤退按钮 ============

pub fn check_retreat_button(
    _commands: Commands,
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<RetreatButton>)>,
    mut next_status: ResMut<NextState<Appstatus>>,
    battle_over: Res<BattleOver>,
) {
    if let Ok((inter, mut bg)) = interaction_query.single_mut() {
        match inter {
            Interaction::Pressed => {
                bg.0 = Color::srgb(0.5, 0.5, 0.5);
                if !battle_over.over {
                    // 撤退视为回到大地图
                    next_status.set(Appstatus::WorldMap);
                } else {
                    next_status.set(Appstatus::WorldMap);
                }
            }
            Interaction::Hovered => {
                bg.0 = Color::srgb(0.4, 0.4, 0.4);
            }
            Interaction::None => {
                bg.0 = Color::srgb(0.3, 0.3, 0.3);
            }
        }
    }
}

// ============ 战斗结束后生成返回按钮 ============

pub fn check_battle_result(
    mut commands: Commands,
    battle_over: Res<BattleOver>,
    mut log: ResMut<BattleLog>,
    mut backpack: ResMut<Backpack>,
    mut player_level: ResMut<PlayerLevel>,
    warehouse: Option<Res<Warehouse>>,
    enemy_db: Res<EnemyDataBase>,
    enemy_query: Query<&Enemy>,
    fire_query: Query<Entity, With<FireButton>>,
    retreat_query: Query<Entity, With<RetreatButton>>,
    return_button_query: Query<Entity, With<ResultReturnButton>>,
    root_query: Query<Entity, With<WarBg>>,
    asset_server: Res<AssetServer>,
) {
    if !battle_over.is_changed() || !battle_over.over {
        return;
    }

    // 已经有返回按钮了就不重复生成
    if !return_button_query.is_empty() {
        return;
    }

    // 战斗胜利时发放战利品和经验
    if battle_over.player_won {
        if let Ok(enemy_entity) = enemy_query.single() {
            let enemy = enemy_entity;
            if let Some(template) = enemy_db.find_by_name(&enemy.name) {
                // 战利品掉落
                let loot = roll_loot(template);
                // 过滤掉玩家已经拥有的蓝图（背包或仓库中已有则不再掉落）
                let loot: Vec<(ItemType, u32)> = loot.into_iter()
                    .filter(|(item, _)| {
                        if *item == ItemType::V8EngineBlueprint {
                            let in_bp = backpack.items.iter()
                                .any(|s| s.item_type == ItemType::V8EngineBlueprint && s.quantity > 0);
                            let empty_wh = Vec::new();
                            let wh_items = warehouse.as_ref().map(|w| &w.items).unwrap_or(&empty_wh);
                            let in_wh = wh_items.iter()
                                .any(|s| s.item_type == ItemType::V8EngineBlueprint && s.quantity > 0);
                            !(in_bp || in_wh)
                        } else {
                            true
                        }
                    })
                    .collect();
                for (item, qty) in &loot {
                    log.add(format!("掉落：{} × {}", item.name(), qty));
                }
                for (item, qty) in &loot {
                    if let Err(_) = backpack.add(*item, *qty) {
                        log.add(format!("⚠️ 背包已满，{} 被丢弃了！", item.name()));
                    }
                }
                // 经验奖励（基于敌人权重和HP）
                let exp_gain = (template.weight * 10 + template.hp as u32 * 2) as u32;
                player_level.gain_exp(exp_gain);
                log.add(format!(
                    "✨ 获得 {} 经验值！（当前 {}）",
                    exp_gain,
                    player_level.exp,
                ));
            }
        }
    }

    // 删除开火按钮和撤退按钮
    if let Ok(entity) = fire_query.single() {
        commands.entity(entity).despawn();
    }
    if let Ok(entity) = retreat_query.single() {
        commands.entity(entity).despawn();
    }

    let font: Handle<Font> = asset_server.load("fonts/STKAITI.TTF");

    let button_text = if battle_over.player_won {
        "继续探索"
    } else {
        "返回基地"
    };

    // 把返回按钮作为 WarRoot 的子节点创建，确保布局正常
    if let Ok(root) = root_query.single() {
        commands.entity(root).with_children(|parent| {
            parent.spawn((
                DespawnOnExit(Appstatus::War),
                Name::new("ResultButton"),
                Node {
                    width: percent(90),
                    height: percent(15),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: px(40.0),
                    ..default()
                },
                children![
                    (
                        Button,
                        Node {
                            width: px(200),
                            height: px(55),
                            border: UiRect::all(px(3)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BorderColor::all(Color::WHITE),
                        BorderRadius::all(px(8.0)),
                        BackgroundColor(if battle_over.player_won {
                            Color::srgb(0.1, 0.6, 0.1)
                        } else {
                            Color::srgb(0.6, 0.1, 0.1)
                        }),
                        ResultReturnButton,
                        children![
                            (
                                Text::new(button_text),
                                TextFont { font, font_size: 28.0, ..default() },
                                TextColor(Color::WHITE),
                            ),
                        ],
                    ),
                ],
            ));
        });
    }
}

// ============ 结果按钮按下 ============

pub fn check_result_button(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<ResultReturnButton>)>,
    mut next_status: ResMut<NextState<Appstatus>>,
    battle_over: Res<BattleOver>,
) {
    for inter in interaction_query.iter() {
        if *inter == Interaction::Pressed {
            if battle_over.player_won {
                next_status.set(Appstatus::SearchEnemy);
            } else {
                next_status.set(Appstatus::Vollage);
            }
        }
    }
}

// ============ BattleLog 辅助方法 ============

impl BattleLog {
    pub fn add(&mut self, msg: String) {
        self.messages.push(msg);
        if self.messages.len() > 20 {
            self.messages.remove(0);
        }
    }
}
