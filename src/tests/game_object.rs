use std::arch::asm;

#[repr(C)]
pub struct VecF3
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
pub struct GameObject
{
    pub health: f32,
    pub xp: u32,
    pub game_time: f64,
    pub position: VecF3,
}

#[unsafe(no_mangle)]
pub static GLOBAL_GAME_OBJECT: GameObject = GameObject
{
    health: 58.2f32,
    xp: 58271,
    game_time: 94715235.165f64,
    position: VecF3
    {
        x: 1.1,
        y: 2.2,
        z: 3.3
    },
};

#[unsafe(no_mangle)]
pub fn read_game_object_health() -> f32
{
    let mut x: f32;
    unsafe
    {
        asm!(
        "/*  */",
        "test rax,rax",
        "mov rcx, [rax]",

        in("rax") &GLOBAL_GAME_OBJECT,
        out("rcx") x);
    }    
    return x;
}