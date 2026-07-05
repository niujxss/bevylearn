use crate::comp_data::*;
use bevy::color::palettes::basic::*;
use bevy::prelude::*;

// ============ Marker Components ============

#[derive(Component)]
pub struct UpgradeRoot;

#[derive(Component)]
pub struct UpgradeLevelText;

#[derive(Component)]
pub struct UpgradeExpText;

#[derive(Component)]
pub struct UpgradeAtkText;

#[derive(Component)]
pub struct UpgradeMaterialList;

#[derive(Component)]
pub struct UpgradeDoButton;

#[derive(Component)]
pub struct UpgradeBackButton;

#[derive(Component)]
pub struct UpgradeResultText;

// ============ UI 创建 ============

pub fn create_upgrade_ui(mut commands: Commands, asset: Res<AssetServer>) {
    let font: Handle<Font> = asset.load("fonts/STKAITI.TTF");

    // 背景半透明覆盖
    commands.spawn((
        UpgradeRoot,
        DespawnOnExit(Appstatus::Upgrade),
        Name::new("UpgradeRoot"),
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
                    height: percent(12.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                children![(
                    Text::new("🛠 升级车间"),
                    TextFont { font: font.clone(), font_size: 36.0, ..default() },
                    TextColor(Color::srgb(1.0, 0.85, 0.3)),
                )],
            ),
            // ===== 信息面板 =====
            (
                Node {
                    width: percent(70.0),
                    height: percent(50.0),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceEvenly,
                    border: UiRect::all(px(2.0)),
                    padding: UiRect::all(px(16.0)),
                    row_gap: px(8.0),
                    ..default()
                },
                BorderColor::all(Color::srgb(0.3, 0.3, 0.5)),
                BorderRadius::all(px(12.0)),
                BackgroundColor(Color::srgba(0.06, 0.06, 0.14, 0.9)),
                children![
                    // 当前等级
                    (
                        UpgradeLevelText,
                        Text::new("等级：Lv.1 → Lv.2"),
                        TextFont { font: font.clone(), font_size: 28.0, ..default() },
                        TextColor(Color::srgb(0.3, 0.85, 1.0)),
                    ),
                    // 经验条
                    (
                        UpgradeExpText,
                        Text::new("经验：0 / 10"),
                        TextFont { font: font.clone(), font_size: 20.0, ..default() },
                        TextColor(Color::srgb(0.6, 0.9, 0.6)),
                    ),
                    // 攻击加成
                    (
                        UpgradeAtkText,
                        Text::new("攻击加成：+0%  →  +10%"),
                        TextFont { font: font.clone(), font_size: 18.0, ..default() },
                        TextColor(Color::srgb(0.9, 0.6, 0.3)),
                    ),
                    // 材料列表
                    (
                        UpgradeMaterialList,
                        Text::new("所需材料："),
                        TextFont { font: font.clone(), font_size: 18.0, ..default() },
                        TextColor(Color::srgb(0.8, 0.8, 0.8)),
                    ),
                    // 状态 / 结果文字
                    (
                        UpgradeResultText,
                        Text::new(""),
                        TextFont { font: font.clone(), font_size: 18.0, ..default() },
                        TextColor(Color::srgb(0.9, 0.9, 0.3)),
                    ),
                ],
            ),
            // ===== 按钮行 =====
            (
                Node {
                    width: percent(60.0),
                    height: percent(15.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: px(40.0),
                    ..default()
                },
                children![
                    // 升级按钮
                    (
                        Button,
                        UpgradeDoButton,
                        Node {
                            width: px(160.0),
                            height: px(60.0),
                            border: UiRect::all(px(3.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BorderColor::all(Color::srgb(0.2, 0.7, 0.2)),
                        BorderRadius::all(px(8.0)),
                        BackgroundColor(Color::srgb(0.08, 0.35, 0.08)),
                        children![(
                            Text::new("⬆ 升级"),
                            TextFont { font: font.clone(), font_size: 26.0, ..default() },
                            TextColor(Color::srgb(0.6, 1.0, 0.6)),
                        )],
                    ),
                    // 返回按钮
                    (
                        Button,
                        UpgradeBackButton,
                        Node {
                            width: px(160.0),
                            height: px(60.0),
                            border: UiRect::all(px(3.0)),
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

// ============ 每帧更新升级面板信息 ============

pub fn update_upgrade_display(
    player_level: Res<PlayerLevel>,
    upgrade_db: Res<UpgradeDataBase>,
    backpack: Option<Res<Backpack>>,
    warehouse: Option<Res<Warehouse>>,
    mut text_params: ParamSet<(
        Query<&mut Text, With<UpgradeLevelText>>,
        Query<&mut Text, With<UpgradeExpText>>,
        Query<&mut Text, With<UpgradeAtkText>>,
        Query<&mut Text, With<UpgradeMaterialList>>,
        Query<&mut Text, With<UpgradeResultText>>,
    )>,
) {
    let next_lv = player_level.level + 1;
    let mut result = String::new();

    // 等级文字
    if let Ok(mut t) = text_params.p0().single_mut() {
        if next_lv > upgrade_db.max_level {
            t.0 = format!("等级：Lv.{}（已满级）", player_level.level);
        } else {
            t.0 = format!("等级：Lv.{}  →  Lv.{}", player_level.level, next_lv);
        }
    }

    // 攻击加成文字
    if let Ok(mut t) = text_params.p2().single_mut() {
        let curr_bonus = (player_level.attack_multiplier() - 1.0) * 100.0;
        let next_bonus = curr_bonus + 10.0;
        if next_lv > upgrade_db.max_level {
            t.0 = format!("攻击加成：{:+.0}%（已满级）", curr_bonus);
        } else {
            t.0 = format!("攻击加成：{:+.0}%  →  {:+.0}%", curr_bonus, next_bonus);
        }
    }

    // 查找本级的升级配置
    if let Some(cfg) = upgrade_db.for_level(next_lv) {
        // 经验文字
        if let Ok(mut t) = text_params.p1().single_mut() {
            t.0 = format!("经验：{} / {}（需战斗获得）", player_level.exp, cfg.exp_needed);
        }

        // 材料列表，同时检查背包+仓库
        let empty_vec = Vec::new();
        let bp = backpack.as_ref().map(|b| &b.items).unwrap_or(&empty_vec);
        let wh = warehouse.as_ref().map(|w| &w.items).unwrap_or(&empty_vec);

        let mut mat_lines = String::from("所需材料：\n");
        let mut all_mat_ok = true;
        let mut all_exp_ok = false;

        for (item, need_qty) in &cfg.cost {
            let bp_qty = bp.iter()
                .find(|s| s.item_type == *item)
                .map(|s| s.quantity)
                .unwrap_or(0);
            let wh_qty = wh.iter()
                .find(|s| s.item_type == *item)
                .map(|s| s.quantity)
                .unwrap_or(0);
            let total = bp_qty + wh_qty;
            let mark = if total >= *need_qty { "✅" } else { "❌" };
            mat_lines += &format!("  {} {}：背包{} + 仓库{} , 共{} / 需要{}\n", mark, item.name(), bp_qty, wh_qty, (bp_qty + wh_qty),need_qty);
            if total < *need_qty {
                all_mat_ok = false;
            }
        }

        if player_level.exp >= cfg.exp_needed {
            all_exp_ok = true;
        }

        if let Ok(mut t) = text_params.p3().single_mut() {
            t.0 = mat_lines;
        }

        // 结果状态
        if next_lv > upgrade_db.max_level {
            result = "🎉 已达最高等级！".to_string();
        } else if all_mat_ok && all_exp_ok {
            result = "✅ 所有条件满足，可以升级！".to_string();
        } else if !all_mat_ok && !all_exp_ok {
            result = "⚠️ 经验不足，且材料不足！".to_string();
        } else if !all_mat_ok {
            result = "⚠️ 材料不足！先去战斗收集吧".to_string();
        } else {
            result = "⚠️ 经验不足！先出击战斗吧".to_string();
        }
    } else {
        // 没有找到配置（已满级或数据缺失）
        if let Ok(mut t) = text_params.p1().single_mut() {
            t.0 = format!("经验：{}（已达最高等级）", player_level.exp);
        }
        if let Ok(mut t) = text_params.p3().single_mut() {
            t.0 = "无需更多材料，已满级！".to_string();
        }
        result = "🎉 你已满级！".to_string();
    }

    if let Ok(mut t) = text_params.p4().single_mut() {
        t.0 = result;
    }
}

// ============ 升级按钮 ============

pub fn check_upgrade_button(
    mut player_level: ResMut<PlayerLevel>,
    upgrade_db: Res<UpgradeDataBase>,
    mut backpack: ResMut<Backpack>,
    mut warehouse: ResMut<Warehouse>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<UpgradeDoButton>),
    >,
    mut result_text: Query<&mut Text, With<UpgradeResultText>>,
) {
    if let Ok((inter, mut bg)) = interaction_query.single_mut() {
        match inter {
            Interaction::Pressed => {
                bg.0 = Color::srgb(0.15, 0.6, 0.15);

                let next_lv = player_level.level + 1;
                if next_lv > upgrade_db.max_level {
                    if let Ok(mut t) = result_text.single_mut() {
                        t.0 = "已达最高等级，无法继续升级！".to_string();
                    }
                    return;
                }

                let cfg = match upgrade_db.for_level(next_lv) {
                    Some(c) => c,
                    None => {
                        if let Ok(mut t) = result_text.single_mut() {
                            t.0 = "配置缺失，无法升级！".to_string();
                        }
                        return;
                    }
                };

                // 检查经验
                if player_level.exp < cfg.exp_needed {
                    if let Ok(mut t) = result_text.single_mut() {
                        t.0 = format!("经验不足！需要 {}，当前 {}", cfg.exp_needed, player_level.exp);
                    }
                    return;
                }

                // 检查材料：背包 + 仓库
                for (item, need_qty) in &cfg.cost {
                    let bp_qty = backpack.items.iter()
                        .find(|s| s.item_type == *item)
                        .map(|s| s.quantity)
                        .unwrap_or(0);
                    let wh_qty = warehouse.items.iter()
                        .find(|s| s.item_type == *item)
                        .map(|s| s.quantity)
                        .unwrap_or(0);
                    if bp_qty + wh_qty < *need_qty {
                        if let Ok(mut t) = result_text.single_mut() {
                            t.0 = format!("材料不足！{} 需要 {}，背包+仓库共 {}", item.name(), need_qty, bp_qty + wh_qty);
                        }
                        return;
                    }
                }

                // ✅ 全部满足，执行升级
                // 消耗经验
                player_level.exp -= cfg.exp_needed;

                // 消耗材料：先扣背包，再扣仓库
                for (item, need_qty) in &cfg.cost {
                    let mut remaining = *need_qty;
                    let bp_qty = backpack.items.iter()
                        .find(|s| s.item_type == *item)
                        .map(|s| s.quantity)
                        .unwrap_or(0);
                    let take_from_bp = bp_qty.min(remaining);
                    if take_from_bp > 0 {
                        let _ = backpack.remove(*item, take_from_bp);
                        remaining -= take_from_bp;
                    }
                    if remaining > 0 {
                        let _ = warehouse.take(*item, remaining);
                    }
                }

                player_level.level += 1;

                if let Ok(mut t) = result_text.single_mut() {
                    if next_lv >= upgrade_db.max_level {
                        t.0 = format!("🎉 满级！Lv.{}，攻击加成 {:.0}%", player_level.level, (player_level.attack_multiplier() - 1.0) * 100.0);
                    } else {
                        t.0 = format!(
                            "✅ 升级成功！Lv.{} → Lv.{}，攻击加成 {:.0}%，继续挑战下一级！",
                            player_level.level - 1,
                            player_level.level,
                            (player_level.attack_multiplier() - 1.0) * 100.0
                        );
                    }
                }
            }
            Interaction::Hovered => {
                bg.0 = Color::srgb(0.12, 0.5, 0.12);
            }
            Interaction::None => {
                bg.0 = Color::srgb(0.08, 0.35, 0.08);
            }
        }
    }
}

// ============ 返回按钮 ============

pub fn check_upgrade_back_button(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<UpgradeBackButton>),
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
