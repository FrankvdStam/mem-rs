use std::thread;
use std::arch::asm;
use std::time::Duration;

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

fn main()
{
    println!("Test binary started");

    let x = read_game_object_health();
    println!("x: {}", x);

    loop{
        thread::sleep(Duration::from_millis(1000));
    }
}

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