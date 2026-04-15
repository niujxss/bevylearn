use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, animation_sprite)
        .run();
}

fn setup(mut commands : Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    let texture = asset_server.load("tank_sheet_transparent2.png"); //从assets目录加载图片
    let layer   = TextureAtlasLayout::from_grid(UVec2::new(1168, 784), 8, 1, None, None);
    let handle  = texture_atlas_layouts.add(layer);

    let ani = AnimationIndices { first: 0, last: 7 };

    commands.spawn(Camera2d);

    // commands.spawn(
    //     Sprite::from_image(
    //         asset_server.load("photo2.png")  //从assets目录加载图片
    //     )
    // );

    commands.spawn(
        (
            Sprite::from_atlas_image(
                texture, 
                TextureAtlas { layout: handle, index: 0 }
            ),
            Transform::from_scale(Vec3::splat(0.3)),
            ani,
            AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        )
    );

    
}


fn animation_sprite(time : Res<Time>,
    mut query : Query<(
        &mut AnimationTimer,
        &AnimationIndices,
        &mut Sprite
    )>,) {

        for (mut timer, indices, mut sprite) in &mut query {
            timer.tick(time.delta());

            if timer.just_finished() && 
             let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = 
                if atlas.index == indices.last {
                    indices.first
                } else {
                    atlas.index + 1
                };
            }
        }
}


#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);