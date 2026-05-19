use std::arch::asm;
use std::mem;

#[macro_export]
macro_rules! field_offset {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

#[repr(C, packed)]
pub struct VecF3
{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C, packed)]
pub struct StructWithAllTypes
{
    pub i8: i8,
    pub i32: i32,
    pub i64: i64,

    pub u8: u8,
    pub u32: u32,
    pub u64: u64,

    pub f32: f32,
    pub f64: f64,

    pub bool: bool,
}

impl StructWithAllTypes
{
    pub const OFFSET_I8: usize = mem::offset_of!(StructWithAllTypes, i8);
    pub const OFFSET_I32: usize = mem::offset_of!(StructWithAllTypes, i32);
    pub const OFFSET_I64: usize = mem::offset_of!(StructWithAllTypes, i64);
    pub const OFFSET_U8: usize = mem::offset_of!(StructWithAllTypes, u8);
    pub const OFFSET_U32: usize = mem::offset_of!(StructWithAllTypes, u32);
    pub const OFFSET_U64: usize = mem::offset_of!(StructWithAllTypes, u64);
    pub const OFFSET_F32: usize = mem::offset_of!(StructWithAllTypes, f32);
    pub const OFFSET_F64: usize = mem::offset_of!(StructWithAllTypes, f64);
    pub const OFFSET_BOOL: usize = mem::offset_of!(StructWithAllTypes, bool);
}

#[repr(C, packed)]
pub struct GameObject
{
    pub health: f32,
    pub xp: u32,
    pub game_time: f64,
    pub position: VecF3,
    pub struct_with_all_types: StructWithAllTypes,
}

impl GameObject
{
    pub const OFFSET_HEALTH: usize = mem::offset_of!(GameObject, health);
    pub const OFFSET_XP: usize = mem::offset_of!(GameObject, xp);
    pub const OFFSET_GAME_TIME: usize = mem::offset_of!(GameObject, game_time);
    pub const OFFSET_STRUCT_WITH_ALL_TYPES: usize = mem::offset_of!(GameObject, struct_with_all_types);
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
    struct_with_all_types: StructWithAllTypes
    {
        i8: 1,
        i32: 2,
        i64: 3,
        u8: 4,
        u32: 5,
        u64: 6,
        f32: 7.0,
        f64: 8.0,
        bool: true,
    }
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