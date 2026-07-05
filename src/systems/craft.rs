use crate::comp_data::*;
use bevy::color::palettes::basic::*;
use bevy::prelude::*;

// ============ Marker Components ============

#[derive(Component)]
pub struct CraftRoot;

#[derive(Component)]
pub struct CraftRecipeList;

#[derive(Component)]
pub struct CraftRecipeButton(pub usize);

#[derive(Component)]
pub struct CraftDetailText;

#[derive(Component)]
pub struct CraftResultText;

#[derive(Component)]
pub struct CraftDoButton;

#[derive(Component)]
pub struct CraftBackButton;

#[derive(Resource)]
pub struct SelectedCraftRecipe(pub usize);

// ============ UI 创建 ============

pub fn create_craft_ui(mut commands: Commands, asset: Res<AssetServer>, item_db: Res<ItemDataBase>) {
    let font: Handle<Font> = asset.load("fonts/STKAITI.TTF");

    commands.insert_resource(SelectedCraftRecipe(0));

    // 根容器
    commands.spawn((
        CraftRoot,
        DespawnOnExit(Appstatus::Craft),
        Name::new("CraftRoot"),
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
                    Text::new("🔨 道具合成"),
                    TextFont { font: font.clone(), font_size: 36.0, ..default() },
                    TextColor(Color::srgb(0.3, 0.9, 0.4)),
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
                    // 左：合成列表
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
                        BorderColor::all(Color::srgb(0.15, 0.4, 0.15)),
                        BorderRadius::all(px(10.0)),
                        BackgroundColor(Color::srgba(0.06, 0.12, 0.06, 0.9)),
                        CraftRecipeList,
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
                            CraftDetailText,
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
                    CraftResultText,
                    Text::new(""),
                    TextFont { font: font.clone(), font_size: 20.0, ..default() },
                    TextColor(Color::srgb(0.3, 0.9, 0.3)),
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
                    // 合成按钮
                    (
                        Button,
                        CraftDoButton,
                        Node {
                            width: px(180.0),
                            height: px(60.0),
                            border: UiRect::all(px(3)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BorderColor::all(Color::srgb(0.1, 0.8, 0.3)),
                        BorderRadius::all(px(8.0)),
                        BackgroundColor(Color::srgb(0.05, 0.4, 0.15)),
                        children![(
                            Text::new("🔨 合成道具"),
                            TextFont { font: font.clone(), font_size: 26.0, ..default() },
                            TextColor(Color::srgb(0.3, 1.0, 0.4)),
                        )],
                    ),
                    // 返回按钮
                    (
                        Button,
                        CraftBackButton,
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

// ============ 每帧更新合成面板详情 ============

pub fn update_craft_display(
    selected: Res<SelectedCraftRecipe>,
    item_db: Res<ItemDataBase>,
    backpack: Option<Res<Backpack>>,
    warehouse: Option<Res<Warehouse>>,
    mut text_params: ParamSet<(
        Query<&mut Text, With<CraftDetailText>>,
        Query<&mut Text, With<CraftResultText>>,
    )>,
) {
    let idx = selected.0.min(item_db.items.len().saturating_sub(1));
    let recipe = &item_db.items[idx];

    let empty_vec = Vec::new();
    let bp = backpack.as_ref().map(|b| &b.items).unwrap_or(&empty_vec);
    let wh = warehouse.as_ref().map(|w| &w.items).unwrap_or(&empty_vec);

    // 构建详情文本
    let mut detail = format!("{}【{}】\n", recipe.icon, recipe.name);
    detail += &format!("{}\n\n", recipe.description);

    // 效果预览
    detail += "▶ 效果：";
    let effect_str = match recipe.effect {
        ItemEffect::Heal(val) => format!("恢复 {} 点装甲片\n", val),
        ItemEffect::Damage(val) => format!("造成 {} 点伤害\n", val),
    };
    detail += &effect_str;
    detail += "\n";

    // 材料检查
    detail += "🧱 需要材料：\n";
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
    let result = if !all_mat_ok {
        "❌ 材料不足！先去战斗收集材料吧".to_string()
    } else {
        "✅ 材料充足，可以合成！".to_string()
    };
    if let Ok(mut t) = text_params.p1().single_mut() {
        t.0 = result;
    }
}

// ============ 配方按钮点击：切换选中配方 ============

pub fn check_craft_recipe_buttons(
    mut selected: ResMut<SelectedCraftRecipe>,
    interaction_query: Query<
        (&Interaction, &CraftRecipeButton),
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

pub fn refresh_craft_recipes(
    mut commands: Commands,
    item_db: Res<ItemDataBase>,
    selected: Res<SelectedCraftRecipe>,
    parent_query: Query<Entity, With<CraftRecipeList>>,
    asset_server: Res<AssetServer>,
    existing: Query<Entity, With<CraftRecipeButton>>,
) {
    // 只运行一次：当配方列表还没有按钮时
    if !existing.is_empty() {
        return;
    }

    let font: Handle<Font> = asset_server.load("fonts/STKAITI.TTF");

    if let Ok(parent) = parent_query.single() {
        commands.entity(parent).despawn_children();

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
                    Text::new("📋 可合成道具"),
                    TextFont { font: font.clone(), font_size: 18.0, ..default() },
                    TextColor(Color::srgb(0.3, 0.9, 0.4)),
                )],
            ));

            for (i, item) in item_db.items.iter().enumerate() {
                let is_selected = i == selected.0;
                let bg = if is_selected {
                    Color::srgb(0.05, 0.25, 0.1)
                } else {
                    Color::srgb(0.04, 0.12, 0.06)
                };
                let border = if is_selected {
                    Color::srgb(0.1, 0.9, 0.3)
                } else {
                    Color::srgb(0.1, 0.3, 0.1)
                };

                list.spawn((
                    Button,
                    CraftRecipeButton(i),
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
                        Text::new(format!("{} {}", item.icon, item.name)),
                        TextFont { font: font.clone(), font_size: 20.0, ..default() },
                        TextColor(if is_selected {
                            Color::srgb(0.3, 1.0, 0.4)
                        } else {
                            Color::srgb(0.7, 0.9, 0.7)
                        }),
                    )],
                ));
            }
        });
    }
}

// ============ 执行合成按钮 ============

pub fn check_craft_do_button(
    mut commands: Commands,
    selected: Res<SelectedCraftRecipe>,
    item_db: Res<ItemDataBase>,
    mut backpack: ResMut<Backpack>,
    mut warehouse: ResMut<Warehouse>,
    player_query: Query<Entity, With<Player>>,
    mut result_text: Query<&mut Text, With<CraftResultText>>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<CraftDoButton>),
    >,
) {
    if let Ok((inter, mut bg)) = interaction_query.single_mut() {
        match inter {
            Interaction::Pressed => {
                bg.0 = Color::srgb(0.1, 0.6, 0.2);

                let idx = selected.0.min(item_db.items.len().saturating_sub(1));
                let recipe = &item_db.items[idx];
                let target_item = recipe.item_type;
                let target_name = recipe.name.clone();

                // 检查并消耗材料（先背包，再仓库）
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

                // ===== 所有条件满足，执行合成 =====
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

                // 产生道具
                if let Err(e) = backpack.add(target_item, 1) {
                    if let Ok(mut t) = result_text.single_mut() {
                        t.0 = format!("❌ 背包已满，无法合成！({})", e);
                    }
                    return;
                }

                if let Ok(mut t) = result_text.single_mut() {
                    t.0 = format!("✅ 合成成功！获得【{}】", target_name);
                }
            }
            Interaction::Hovered => {
                bg.0 = Color::srgb(0.1, 0.7, 0.3);
            }
            Interaction::None => {
                bg.0 = Color::srgb(0.05, 0.4, 0.15);
            }
        }
    }
}

// ============ 返回按钮 ============

pub fn check_craft_back_button(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<CraftBackButton>)>,
    mut next_status: ResMut<NextState<Appstatus>>,
) {
    if let Ok((inter, mut bg)) = interaction_query.single_mut() {
        match inter {
            Interaction::Pressed => {
                bg.0 = Color::srgb(0.3, 0.3, 0.3);
                next_status.set(Appstatus::Vollage);
            }
            Interaction::Hovered => {
                bg.0 = Color::srgb(0.3, 0.3, 0.4);
            }
            Interaction::None => {
                bg.0 = Color::srgb(0.15, 0.15, 0.2);
            }
        }
    }
}
