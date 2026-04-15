

use bevy::{asset::LoadedFolder, image::ImageSampler, prelude::*};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
enum Appstatus {
    #[default] // 标记默认状态是 Setup
    Setup,
    Finished,
}

pub fn myself_run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<Appstatus>()
        .add_systems(OnEnter(Appstatus::Setup), load_folder)  // onEnter 是一次性执行;相当于刚进 Appstatus::Setup 就执行
        .add_systems(Update, check_load_folder.run_if(in_state(Appstatus::Setup))) //这个可以多次执行； 这个是监测状态是 Appstatus::Setup 就执行
        .add_systems(OnEnter(Appstatus::Finished), set_up)
        .run();
}


#[derive(Resource, Default)] //Resource 是资源，资源是全局唯一的数据，整个应用只有一个实例; Default :可以创建默认实例
struct RpgSpriteFolder(Handle<LoadedFolder>); // Handle ： 资源的引用权柄，不是实际数据；  LoaderFolder:表示已加载的文件夹资源



fn load_folder(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(
        RpgSpriteFolder(
            asset_server.load_folder("Angle")
        )
    );
}

fn check_load_folder(
    mut next_status: ResMut<NextState<Appstatus>>, //获取系统状态, NextState bevy内置资源，专门用来管理状态切换
    folder: Res<RpgSpriteFolder>, // 获取资源权柄
    mut events: MessageReader<AssetEvent<LoadedFolder>>, // 获取事件信号， 目录加载资源监听
) {
    for event in events.read() {
        if event.is_loaded_with_dependencies(&folder.0) {  //检查 指定资源 加载的依赖是否都完成了
            next_status.set(Appstatus::Finished);
        }
    }
}

fn set_up(
    mut commands: Commands,
    folder: Res<RpgSpriteFolder>,  //目录资源权柄
    asset_server: Res<AssetServer>,  // 资源管理
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>, //纹理图集
    load_folders: Res<Assets<LoadedFolder>>, // 已加载的文件夹资源集合
    mut textures: ResMut<Assets<Image>>,
) {
    let loaded_folder = load_folders.get(&folder.0).unwrap(); //从已经加载的文件夹资源中获取我们的资源

    commands.spawn(Camera2d);
    
    let configrations = [
    /** 标题                 是否填充数据            平滑或像素                     x轴坐标 **/
    ("Linear, No Padding",  None,                   Some(ImageSampler::linear()),  0.0 ),
    // ("Nearest, No Padding", None,                   Some(ImageSampler::nearest()),  300.0 ),
    // ("Linear, Padding",     Some(UVec2::new(6, 6)), Some(ImageSampler::linear()),  -300.0  ),
    // ("Nearest, Padding",    Some(UVec2::new(6, 6)), Some(ImageSampler::nearest()), 300.0 ),
    ];

    for (lable, padding, sample, x) in configrations {
        let (texture_atlas_layout, texture_atlas_sources, texture) = create_texture_atlas(
            loaded_folder,  //目录资源
            padding,   //是否边缘填充
            sample, //采样方式平滑还是像素
            &mut textures, //图片资源

        );

        

        commands.spawn((
           Sprite::from_image(texture.clone()),
           Transform {
                translation: Vec3::new(x, 0.0, 0.0),
                scale: Vec3::splat(0.9),
                ..default()
           },
        ));

        create_label(&mut commands, (x, -150.0, 0.0), lable, &asset_server);

    }
}

//纹理图集生成
fn create_texture_atlas(
    folder: &LoadedFolder,  //已加载的文件夹引用
    padding: Option<UVec2>, //填充设置
    sample: Option<ImageSampler>, //采样设置
    texture: &mut ResMut<Assets<Image>> //图片资源存储
) -> (TextureAtlasLayout, TextureAtlasSources, Handle<Image>) // 图集布局（如何切割大图） 源图片到图集位置的映射  最终生成的纹理图集句柄
{
    let mut texture_atlas_builder = TextureAtlasBuilder::default(); // 创建默认的纹理图集构建器，相当于准备了一个空画布，开始拼图

    if let Some(pad) = padding {
        texture_atlas_builder.padding(pad); //设置间隔
    }

    for handle in folder.handles.iter() { //便利文件夹中每个图片的句柄
        let id = handle.id().typed_unchecked::<Image>(); //获取图片资源的 ID  handle.id() 返回通用ID，typed_unchecked 转换为具体类型

        if let Some(texture) = texture.get(id) { //从图片资源存储集中获取指定的图片数据
            texture_atlas_builder.add_texture(Some(id), texture); //将图片添加到图集构建器中
        }
    }

        //图集布局(每个子图的位置和尺寸)  源图片到图集位置的映射关系   生成的纹理图集图片数据
    let (layout, sources, mut image) = texture_atlas_builder.build().unwrap(); //texture_atlas_builder.build() 执行拼图算法，生成最终的大图

    if let Some(sampler) = sample {
        image.sampler = sampler; //增加采样
    }

    let texture_handle = texture.add(image);  //将生成的图片资源添加到资源管理器


    (layout, sources, texture_handle)

}

fn create_label(
    commands: & mut Commands,
    translation: (f32, f32, f32),
    text: &str,
    assert_server: &Res<AssetServer>,
) {
    let font = assert_server.load("fonts/STKAITI.TTF");

    let text_style = TextFont {
        font: font.clone(),
        font_size: 22.0,
        ..default()
    };

    commands.spawn((
       Text2d::new(text),
       text_style,
       TextLayout::new_with_justify(Justify::Center),
       Transform {
            translation: Vec3::new(translation.0, translation.1, translation.2),
            ..default()
       } 
    ));
}
