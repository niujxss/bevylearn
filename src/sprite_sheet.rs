//! Renders an animated sprite by loading all animation frames from a single image (a sprite sheet)
//! into a texture atlas, and changing the displayed image periodically.

use bevy::prelude::*;
use bevy::image::ImageSampler;

pub fn self_run() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest())) // prevents blurry sprites
        .add_systems(Startup, setup)// 在启动时调用 setup 系统，只调用一次
        .add_systems(Update, animate_sprite)  // 在每一帧更新时调用 animate_sprite 系统
        .run();
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&AnimationIndices, &mut AnimationTimer, &mut Sprite)>, //获取 AnimationIndices 的引用；获取 AnimationTimer的可变引用；获取 Sprite的可变引用；这里有获取可变引用；所以形参需要 mut 类型
) {
    for (indices, mut timer, mut sprite) in &mut query {  //这里因为代码要修改 timer 和 sprite，所以 query 需要是可变引用
        timer.tick(time.delta()); // 更新定时器的时间；

        if timer.just_finished() //如果定时器触发了
            && let Some(atlas) = &mut sprite.texture_atlas
        {
            atlas.index = if atlas.index == indices.last {  //更新图片编号
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>, // AssetServer 资源服务器；用于加载和管理游戏资源（纹理、音频、模型等）
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>, //TextureAtlasLayout 纹理图集布局；它定义了如何从一张大图中切分出各个子图（精灵帧）
    mut textures: ResMut<Assets<Image>>,
) {
    // let texture = asset_server.load("2.png"); //加载纹理资源，这里是一个包含多个动画帧的精灵表
    // let layout = TextureAtlasLayout::from_grid(  //根据网格（grid）创建一个纹理布局，指定如何从大图中切分出子图
    //     UVec2::new(448,798),  // 每个子图的尺寸
    //     3, // 列数
    //     1,    // 行数
    //     None, //子图间的间距
    //     None  //从左上角的偏移
    // );

    let texture = asset_server.load("sprites/nun_walk.png"); //加载纹理资源，这里是一个包含多个动画帧的精灵表
    let layout = TextureAtlasLayout::from_grid(  //根据网格（grid）创建一个纹理布局，指定如何从大图中切分出子图
        UVec2::new(80,100),  // 每个子图的尺寸
        4, // 列数
        2,    // 行数
        Some(UVec2::new(2, 2)), //子图间的间距
        None  //从左上角的偏移
    );


    let texture_atlas_layout = texture_atlas_layouts.add(layout);  //返回一个句柄，用于后续引用该布局
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 0, last: 7 };

    if let Some(image) = textures.get_mut(&texture) {
        // 选择适合您游戏风格的采样方式：
        // image.sampler = ImageSampler::nearest();  // 像素风格（推荐用于像素游戏）
        // 或者：
        image.sampler = ImageSampler::linear();  // 平滑风格
    }


    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_atlas_image( // 从纹理图集中创建一个精灵
            texture, // 纹理资源句柄
            TextureAtlas { // 纹理图集组件
                layout: texture_atlas_layout, // 使用之前创建的布局
                index: animation_indices.first, // 初始显示的子图索引
            },
        ),
        Transform::from_scale(Vec3::splat(1.2)), // 缩放精灵
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)), //timer 定时器； TimerMode::Repeating 表示定时器会重复触发； 0.1 秒间隔
    ));
}
