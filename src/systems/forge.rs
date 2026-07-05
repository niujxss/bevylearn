use crate::comp_data::*;
use bevy::color::palettes::basic::*;
use bevy::prelude::*;

// ============ Marker Components ============

#[derive(Component)]
pub struct ForgeRoot;

#[derive(Component)]
pub struct ForgeTitle;

#[derive(Component)]
pub struct ForgeRecipeList;

#[derive(Component)]
pub struct ForgeRecipeButton(pub usize);

#[derive(Component)]
pub struct ForgeDetailText;

#[derive(Component)]
pub struct ForgeResultText;

#[derive(Component)]
pub struct ForgeDoButton;

#[derive(Component)]
pub struct ForgeBackButton;

// ============ UI 创建 ============

pub fn create_forge_ui(mut commands: Commands, asset: Res<AssetServer>, forge_db: Res<ForgeDataBase>) {
    let font: Handle<Font> = asset.load("fonts/STKAITI.TTF");
    let recipes = forge_db.recipes.clone();

    commands.insert_resource(SelectedForgeRecipe(0));

    // 根容器
    commands.spawn((
        ForgeRoot,
        DespawnOnExit(Appstatus::Forge),
        Name::new("ForgeRoot"),
        Node {
            width: percent(100.0),
            height: percent(100.0),
            position_type: PositionType::Absolute,
            top: px(0.0),
            left: px(0.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(Color::srgba(0.02, 0.02, 0.08, 0.95)),
        children![
            // ===== 标题 =====
            (
                Node {
                    width: percent(100.0),
                    height: percent(10.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![(
                    Text::new("🔧 锻造车间"),
                    TextFont { font: font.clone(), font_size: 36.0, ..default() },
                    TextColor(Color::srgb(1.0, 0.7, 0.2)),
                )],
            ),
            // ===== 主体内容：左列表 + 右详情 =====
            (
                Node {
                    width: percent(90.0),
                    height: percent(55.0),
                    flex_direction: FlexDirection::Row,
                    column_gap: px(16.0),
                    ..default()
                },
                children![
                    // 左：配方列表
                    (
                        Node {
                            width: percent(35.0),
                            height: percent(100.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(px(8.0)),
                            row_gap: px(8.0),
                            border: UiRect::all(px(2.0)),
                            ..default()
                        },
                        BorderColor::all(Color::srgb(0.4, 0.3, 0.15)),
                        BorderRadius::all(px(10.0)),
                        BackgroundColor(Color::srgba(0.08, 0.06, 0.12, 0.9)),
                        ForgeRecipeList,
                    ),
                    // 右：详情面板
                    (
                        Node {
                            width: percent(65.0),
                            height: percent(100.0),
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::FlexStart,
                            justify_content: JustifyContent::SpaceEvenly,
                            padding: UiRect::all(px(16.0)),
                            border: UiRect::all(px(2.0)),
                            row_gap: px(6.0),
                            ..default()
                        },
                        BorderColor::all(Color::srgb(0.3, 0.3, 0.5)),
                        BorderRadius::all(px(10.0)),
                        BackgroundColor(Color::srgba(0.06, 0.06, 0.14, 0.9)),
                        children![(
                            ForgeDetailText,
                            Text::new("选择左侧配方查看详情"),
                            TextFont { font: font.clone(), font_size: 20.0, ..default() },
                            TextColor(Color::srgb(0.8, 0.8, 0.8)),
                        )],
                    ),
                ],
            ),
            // ===== 状态文字 =====
            (
                Node {
                    width: percent(70.0),
                    height: percent(8.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![(
                    ForgeResultText,
                    Text::new(""),
                    TextFont { font: font.clone(), font_size: 20.0, ..default() },
                    TextColor(Color::srgb(0.9, 0.9, 0.3)),
                )],
            ),
            // ===== 按钮行 =====
            (
                Node {
                    width: percent(60.0),
                    height: percent(12.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: px(40.0),
                    ..default()
                },
                children![
                    // 锻造按钮
                    (
                        Button,
                        ForgeDoButton,
                        Node {
                            width: px(180.0),
                            height: px(60.0),
                            border: UiRect::all(px(3)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BorderColor::all(Color::srgb(0.8, 0.5, 0.1)),
                        BorderRadius::all(px(8.0)),
                        BackgroundColor(Color::srgb(0.4, 0.2, 0.05)),
                        children![(
                            Text::new("⚒ 执行锻造"),
                            TextFont { font: font.clone(), font_size: 26.0, ..default() },
                            TextColor(Color::srgb(1.0, 0.8, 0.3)),
                        )],
                    ),
                    // 返回按钮
                    (
                        Button,
                        ForgeBackButton,
                        Node {
                            width: px(160.0),
                            height: px(60.0),
                            border: UiRect::all(px(3)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BorderColor::all(Color::srgb(0.5, 0.5, 0.5)),
                        BorderRadius::all(px(8.0)),
                        BackgroundColor(Color::srgb(0.15, 0.15, 0.2)),
                        children![(
                            Text::new("🔙 返回"),
                            TextFont { font: font.clone(), font_size: 26.0, ..default() },
                            TextColor(Color::srgb(0.8, 0.8, 0.9)),
                        )],
                    ),
                ],
            ),
        ],
    ));
}

// ============ 每帧更新锻造面板详情 ============

pub fn update_forge_display(
    selected: Res<SelectedForgeRecipe>,
    forge_db: Res<ForgeDataBase>,
    backpack: Option<Res<Backpack>>,
    warehouse: Option<Res<Warehouse>>,
    player: Query<(&Player, &Engine)>,
    mut text_params: ParamSet<(
        Query<&mut Text, With<ForgeDetailText>>,
        Query<&mut Text, With<ForgeResultText>>,
    )>,
) {
    let idx = selected.0.min(forge_db.recipes.len().saturating_sub(1));
    let recipe = &forge_db.recipes[idx];

    let empty_vec = Vec::new();
    let bp = backpack.as_ref().map(|b| &b.items).unwrap_or(&empty_vec);
    let wh = warehouse.as_ref().map(|w| &w.items).unwrap_or(&empty_vec);

    // 检查当前部件的等级是否匹配 from_variant
    let current_part_ok = if let Ok((_, engine)) = player.single() {
        let from_variant = format!("{:?}", engine.enginetype);
        from_variant == recipe.from_variant
    } else {
        false
    };

    // 构建详情文本
    let mut detail = format!("【{}】\n", recipe.name);
    detail += &format!("{}\n\n", recipe.description);
    detail += &format!("▶ 升级路径: {} → {}\n", recipe.from_variant, recipe.to_variant);

    // 部件匹配检查
    detail += &format!(
        "▶ 当前部件匹配: {}\n\n",
        if current_part_ok { "✅ 可以锻造" } else { "❌ 部件等级不符或未装备" }
    );

    // 图纸检查
    detail += "📜 需要图纸：\n";
    let mut all_bp_ok = true;
    for (item, need_qty) in &recipe.blueprints {
        let bp_qty = bp.iter().find(|s| s.item_type == *item).map(|s| s.quantity).unwrap_or(0);
        let wh_qty = wh.iter().find(|s| s.item_type == *item).map(|s| s.quantity).unwrap_or(0);
        let total = bp_qty + wh_qty;
        let mark = if total >= *need_qty { "✅" } else { "❌" };
        detail += &format!("  {} {}：背包{} + 仓库{} / 需要{}\n", mark, item.name(), bp_qty, wh_qty, need_qty);
        if total < *need_qty {
            all_bp_ok = false;
        }
    }

    // 材料检查
    detail += "\n🧱 需要材料：\n";
    let mut all_mat_ok = true;
    for (item, need_qty) in &recipe.materials {
        let bp_qty = bp.iter().find(|s| s.item_type == *item).map(|s| s.quantity).unwrap_or(0);
        let wh_qty = wh.iter().find(|s| s.item_type == *item).map(|s| s.quantity).unwrap_or(0);
        let total = bp_qty + wh_qty;
        let mark = if total >= *need_qty { "✅" } else { "❌" };
        detail += &format!("  {} {}：背包{} + 仓库{} / 需要{}\n", mark, item.name(), bp_qty, wh_qty, need_qty);
        if total < *need_qty {
            all_mat_ok = false;
        }
    }

    if let Ok(mut t) = text_params.p0().single_mut() {
        t.0 = detail;
    }

    // 状态文字
    let result = if !current_part_ok {
        "⚠️ 当前装备的部件等级不符合锻造要求！".to_string()
    } else if !all_bp_ok {
        "❌ 图纸不足！需要先获得图纸".to_string()
    } else if !all_mat_ok {
        "❌ 材料不足！先去战斗收集材料吧".to_string()
    } else {
        "✅ 所有条件满足，可以锻造！".to_string()
    };
    if let Ok(mut t) = text_params.p1().single_mut() {
        t.0 = result;
    }
}

// ============ 配方按钮点击：切换选中配方 ============

pub fn check_forge_recipe_buttons(
    mut selected: ResMut<SelectedForgeRecipe>,
    interaction_query: Query<
        (&Interaction, &ForgeRecipeButton),
        Changed<Interaction>,
    >,
) {
    for (inter, button) in interaction_query.iter() {
        if *inter == Interaction::Pressed {
            selected.0 = button.0;
        }
    }
}

// ============ 配方列表刷新（在 OnEnter 后填充按钮） ============

pub fn refresh_forge_recipes(
    mut commands: Commands,
    forge_db: Res<ForgeDataBase>,
    selected: Res<SelectedForgeRecipe>,
    parent_query: Query<Entity, With<ForgeRecipeList>>,
    asset_server: Res<AssetServer>,
    existing: Query<Entity, With<ForgeRecipeButton>>,
) {
    // 只运行一次：当配方列表还没有按钮时
    if !existing.is_empty() {
        return;
    }

    let font: Handle<Font> = asset_server.load("fonts/STKAITI.TTF");

    if let Ok(parent) = parent_query.single() {
        // 先清空（理论上空的，但安全起见）
        commands.entity(parent).despawn_children();

        // 标题
        commands.entity(parent).with_children(|list| {
            list.spawn((
                Node {
                    width: percent(100.0),
                    height: percent(12.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![(
                    Text::new("📋 可用配方"),
                    TextFont { font: font.clone(), font_size: 18.0, ..default() },
                    TextColor(Color::srgb(0.9, 0.7, 0.3)),
                )],
            ));

            for (i, recipe) in forge_db.recipes.iter().enumerate() {
                let is_selected = i == selected.0;
                let bg = if is_selected {
                    Color::srgb(0.25, 0.15, 0.05)
                } else {
                    Color::srgb(0.12, 0.08, 0.04)
                };
                let border = if is_selected {
                    Color::srgb(0.9, 0.6, 0.1)
                } else {
                    Color::srgb(0.3, 0.2, 0.1)
                };

                list.spawn((
                    Button,
                    ForgeRecipeButton(i),
                    Node {
                        width: percent(90.0),
                        height: px(50.0),
                        border: UiRect::all(px(2)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor::all(border),
                    BorderRadius::all(px(6.0)),
                    BackgroundColor(bg),
                    children![(
                        Text::new(format!("{}", recipe.name)),
                        TextFont { font: font.clone(), font_size: 20.0, ..default() },
                        TextColor(if is_selected {
                            Color::srgb(1.0, 0.8, 0.3)
                        } else {
                            Color::srgb(0.8, 0.7, 0.5)
                        }),
                    )],
                ));
            }
        });
    }
}

// ============ 执行锻造按钮 ============

pub fn check_forge_do_button(
    mut commands: Commands,
    selected: Res<SelectedForgeRecipe>,
    forge_db: Res<ForgeDataBase>,
    engine_db: Res<EngineDataBase>,
    mut backpack: ResMut<Backpack>,
    mut warehouse: ResMut<Warehouse>,
    mut player_query: Query<(Entity, &mut Player, &mut Engine)>,
    mut result_text: Query<&mut Text, With<ForgeResultText>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ForgeDoButton>),
    >,
) {
    if let Ok((inter, mut bg)) = interaction_query.single_mut() {
        match inter {
            Interaction::Pressed => {
                bg.0 = Color::srgb(0.6, 0.3, 0.1);

                let idx = selected.0.min(forge_db.recipes.len().saturating_sub(1));
                let recipe = &forge_db.recipes[idx];

                // 检查部件匹配
                let (player_entity, mut player, mut engine) = match player_query.single_mut() {
                    Ok(p) => p,
                    Err(_) => {
                        if let Ok(mut t) = result_text.single_mut() {
                            t.0 = "❌ 无法获取玩家信息！".to_string();
                        }
                        return;
                    }
                };

                let current_variant = format!("{:?}", engine.enginetype);
                if current_variant != recipe.from_variant {
                    if let Ok(mut t) = result_text.single_mut() {
                        t.0 = format!("❌ 当前装备的引擎（{}）不符合锻造要求（需要{}）！", current_variant, recipe.from_variant);
                    }
                    return;
                }

                // 检查并消耗图纸（先背包，再仓库）
                for (item, need_qty) in &recipe.blueprints {
                    let bp_qty = backpack.items.iter()
                        .find(|s| s.item_type == *item).map(|s| s.quantity).unwrap_or(0);
                    let wh_qty = warehouse.items.iter()
                        .find(|s| s.item_type == *item).map(|s| s.quantity).unwrap_or(0);
                    if bp_qty + wh_qty < *need_qty {
                        if let Ok(mut t) = result_text.single_mut() {
                            t.0 = format!("❌ 图纸不足！需要{}×{}", item.name(), need_qty);
                        }
                        return;
                    }
                }

                // 检查并消耗材料
                for (item, need_qty) in &recipe.materials {
                    let bp_qty = backpack.items.iter()
                        .find(|s| s.item_type == *item).map(|s| s.quantity).unwrap_or(0);
                    let wh_qty = warehouse.items.iter()
                        .find(|s| s.item_type == *item).map(|s| s.quantity).unwrap_or(0);
                    if bp_qty + wh_qty < *need_qty {
                        if let Ok(mut t) = result_text.single_mut() {
                            t.0 = format!("❌ 材料不足！需要{}×{}", item.name(), need_qty);
                        }
                        return;
                    }
                }

                // ===== 所有条件满足，执行锻造 =====
                // 消耗图纸
                for (item, need_qty) in &recipe.blueprints {
                    let mut remaining = *need_qty;
                    let bp_qty = backpack.items.iter()
                        .find(|s| s.item_type == *item).map(|s| s.quantity).unwrap_or(0);
                    let take_from_bp = bp_qty.min(remaining);
                    if take_from_bp > 0 {
                        let _ = backpack.remove(*item, take_from_bp);
                        remaining -= take_from_bp;
                    }
                    if remaining > 0 {
                        let _ = warehouse.take(*item, remaining);
                    }
                }

                // 消耗材料
                for (item, need_qty) in &recipe.materials {
                    let mut remaining = *need_qty;
                    let bp_qty = backpack.items.iter()
                        .find(|s| s.item_type == *item).map(|s| s.quantity).unwrap_or(0);
                    let take_from_bp = bp_qty.min(remaining);
                    if take_from_bp > 0 {
                        let _ = backpack.remove(*item, take_from_bp);
                        remaining -= take_from_bp;
                    }
                    if remaining > 0 {
                        let _ = warehouse.take(*item, remaining);
                    }
                }

                // 根据配方类型更新装备
                match recipe.part_type {
                    ForgePartType::Engine => {
                        // 解析目标引擎类型
                        let target_type = match recipe.to_variant.as_str() {
                            "EngineLevel1" => EngineType::EngineLevel1,
                            "EngineLevel2" => EngineType::EngineLevel2,
                            _ => {
                                if let Ok(mut t) = result_text.single_mut() {
                                    t.0 = format!("❌ 未知引擎类型：{}", recipe.to_variant);
                                }
                                return;
                            }
                        };
                        if let Some(cfg) = engine_db.get(target_type) {
                            // 更新引擎组件，但保留 available 状态
                            engine.name = cfg.name.clone();
                            engine.description = cfg.description.clone();
                            engine.weight = cfg.weight;
                            engine.enginetype = target_type;

                            // 重新计算最大血量
                            // 需要临时查询 Cannon 和 SecGun
                            // 由于 query 冲突，使用 commands.queue
                            commands.queue(move |world: &mut World| {
                                let mut player_query = world.query::<(&Player, &Cannon, &SecGun, &mut Engine)>();
                                if let Ok((player, cannon, secgun, mut engine)) = player_query.single_mut(world) {
                                    let mut cannone_weight = 0.0;
                                    let mut secgun_weight = 0.0;
                                    if cannon.available { cannone_weight = cannon.weight; }
                                    if secgun.available { secgun_weight = secgun.weight; }
                                    let zhongliang = engine.weight - cannone_weight - secgun_weight;
                                    let max_health = (zhongliang * 100.0) as i32;
                                    
                                    // 更新 player 的 max_health
                                    let mut player_query2 = world.query::<&mut Player>();
                                    if let Ok(mut p) = player_query2.single_mut(world) {
                                        p.max_health = max_health;
                                    }
                                }
                            });

                            // 更新状态文字
                            if let Ok(mut t) = result_text.single_mut() {
                                t.0 = format!("✅ 锻造成功！引擎已升级为【{}】，载重能力大幅提升！", cfg.name);
                            }
                        } else {
                            if let Ok(mut t) = result_text.single_mut() {
                                t.0 = format!("❌ 未找到目标引擎配置：{:?}", target_type);
                            }
                        }
                    }
                    ForgePartType::Cannon | ForgePartType::SecGun => {
                        if let Ok(mut t) = result_text.single_mut() {
                            t.0 = "🔧 该部件锻造尚未实装，敬请期待！".to_string();
                        }
                    }
                }
            }
            Interaction::Hovered => {
                bg.0 = Color::srgb(0.55, 0.3, 0.1);
            }
            Interaction::None => {
                bg.0 = Color::srgb(0.4, 0.2, 0.05);
            }
        }
    }
}

// ============ 返回按钮 ============

pub fn check_forge_back_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ForgeBackButton>),
    >,
    mut next_status: ResMut<NextState<Appstatus>>,
) {
    if let Ok((inter, mut bg)) = interaction_query.single_mut() {
        match inter {
            Interaction::Pressed => {
                bg.0 = Color::srgb(0.3, 0.3, 0.4);
                next_status.set(Appstatus::Vollage);
            }
            Interaction::Hovered => {
                bg.0 = Color::srgb(0.25, 0.25, 0.35);
            }
            Interaction::None => {
                bg.0 = Color::srgb(0.15, 0.15, 0.2);
            }
        }
    }
}
