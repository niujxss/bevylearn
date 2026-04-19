use bevy::prelude::*;

#[derive(Component)]
pub struct EnumUi;

#[derive(Component)]
pub struct StartGameUi;

#[derive(Component)]
pub struct StopGameUi;
#[derive(Component)]
pub struct Player {
    pub health: i32,
    pub max_health: i32,
    pub main_gun: MainGun,
    pub second_gun: SecondGun,
    pub engine: Engine,
}

pub enum MainGun {
    NONE,
    LEVEL1(String, String, f32, i32),
    LEVEL2(String, String, f32, i32),
}


pub enum SecondGun {
    NONE,
    LEVEL1(String, String, f32, i32),
    LEVEL2(String, String, f32, i32),
}

pub enum Engine {
    NONE,
    LEVEL1(String, String, f32),
    LEVEL2(String, String, f32),
}



#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum Appstatus {
    #[default] // 标记默认状态是 Setup
    Menu,
    Vollage,
}

impl SecondGun {
    pub fn get_name(&self) -> &str {
        match self {
            SecondGun::NONE => "无",
            SecondGun::LEVEL1(name, _, _, _) => name,
            SecondGun::LEVEL2(name, _, _, _) => name,
        }
    }
    pub fn get_message(&self) -> &str {
        match self {
            SecondGun::NONE => "无",
            SecondGun::LEVEL1(_, message, _, _) => message,
            SecondGun::LEVEL2(_, message, _, _) => message,
        }
    }

    pub fn get_zhongliang(&self) -> f32 {
        match self {
            SecondGun::NONE => 0.0,
            SecondGun::LEVEL1(_, _, zhongliang, _) => *zhongliang,
            SecondGun::LEVEL2(_, _, zhongliang, _) => *zhongliang,
        }
    }

    pub fn get_shanghai(&self) -> i32 {
        match self {
            SecondGun::NONE => 0,
            SecondGun::LEVEL1(_, _, _, shanghai) => *shanghai,
            SecondGun::LEVEL2(_, _, _, shanghai) => *shanghai,
        }
    }
}

impl MainGun {
    pub fn get_name(&self) -> &str {
        match self {
            MainGun::NONE => "无",
            MainGun::LEVEL1(name, _, _, _) => name,
            MainGun::LEVEL2(name, _, _, _) => name,
        }
    }

    pub fn get_message(&self) -> &str {
        match self {
            MainGun::NONE => "无",
            MainGun::LEVEL1(_, message, _, _) => message,
            MainGun::LEVEL2(_, message, _, _) => message,
        }
    }

    pub fn get_zhongliang(&self) -> f32 {
        match self {
            MainGun::NONE => 0.0,
            MainGun::LEVEL1(_, _, zhongliang, _) => *zhongliang,
            MainGun::LEVEL2(_, _, zhongliang, _) => *zhongliang,
        }
    }

    pub fn get_shanghai(&self) -> i32 {
        match self {
            MainGun::NONE => 0,
            MainGun::LEVEL1(_, _, _, shanghai) => *shanghai,
            MainGun::LEVEL2(_, _, _, shanghai) => *shanghai,
        }
    }
}

impl Engine {
    pub fn get_name(&self) -> &str {
        match self {
            Engine::NONE => "无",
            Engine::LEVEL1(name, _, _) => name,
            Engine::LEVEL2(name, _, _) => name,
        }
    }

    pub fn get_message(&self) -> &str {
        match self {
            Engine::NONE => "无",
            Engine::LEVEL1(_, message, _) => message,
            Engine::LEVEL2(_, message, _) => message,
        }
    }

    pub fn get_zhongliang(&self) -> f32 {
        match self {
            Engine::NONE => 0.0,
            Engine::LEVEL1(_, _, zhongliang) => *zhongliang,
            Engine::LEVEL2(_, _, zhongliang) => *zhongliang,
        }
    }
}
