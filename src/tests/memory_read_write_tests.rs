use std::mem;
use windows::Win32::System::Memory::PAGE_READWRITE;
use crate::prelude::*;
use crate::tests::game_object::{GameObject, StructWithAllTypes};
use crate::tests::MockProcess;

#[test]
pub fn direct_memory_read_write()
{
    let mut mock_process_direct = MockProcess::new(MemoryType::Direct);
    mock_process_direct.process.refresh().expect("failed to refresh");
    memory_read_write_asserts(&mut mock_process_direct);
}

#[test]
pub fn win32_memory_read_write()
{
    let mut mock_process_win32 = MockProcess::new(MemoryType::Win32Api);
    mock_process_win32.process.refresh().expect("failed to refresh");
    memory_read_write_asserts(&mut mock_process_win32);
}

pub fn memory_read_write_asserts(mock_process: &mut MockProcess)
{
    let game_object_pointer = mock_process.process.scan_rel("GameObject", "48 8d 05 ? ? ? ? 48 85 c0 48 8b 08", 3, 7, Vec::new()).expect("failed to scan for GameObject");
    let mut struct_with_all_types_pointer = mock_process.process.scan_rel("GameObject", "48 8d 05 ? ? ? ? 48 85 c0 48 8b 08", 3, 7, Vec::new()).expect("failed to scan for GameObject");
    let health = game_object_pointer.read_f32_rel(Some(GameObject::OFFSET_HEALTH));
    let game_time = game_object_pointer.read_f64_rel(Some(GameObject::OFFSET_GAME_TIME));

    assert_eq!(58.2f32, health);
    assert_eq!(94715235.165f64, game_time);

    assert_eq!(struct_with_all_types_pointer.read_i8_rel  (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_I8)), 1);
    assert_eq!(struct_with_all_types_pointer.read_i32_rel (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_I32)), 2);
    assert_eq!(struct_with_all_types_pointer.read_i64_rel (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_I64)), 3);

    assert_eq!(struct_with_all_types_pointer.read_u8_rel  (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_U8)), 4);
    assert_eq!(struct_with_all_types_pointer.read_u32_rel (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_U32)), 5);
    assert_eq!(struct_with_all_types_pointer.read_u64_rel (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_U64)), 6);

    assert_eq!(struct_with_all_types_pointer.read_f32_rel (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_F32)), 7.0f32);
    assert_eq!(struct_with_all_types_pointer.read_f64_rel (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_F64)), 8.0f64);

    assert_eq!(struct_with_all_types_pointer.read_bool_rel(Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_BOOL)), true);

    let base = struct_with_all_types_pointer.get_base_address();
    let size = mem::size_of::<StructWithAllTypes>();

    //make sure we have write access to the page. The rust compiler tends to put the data in a read-only page.
    let _ = mock_process.process.set_page_protection(base, size, PAGE_READWRITE);

    //write all values
    struct_with_all_types_pointer.write_i8_rel  (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_I8), 9);
    struct_with_all_types_pointer.write_i32_rel (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_I32), 10);
    struct_with_all_types_pointer.write_i64_rel (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_I64), 11);

    struct_with_all_types_pointer.write_u8_rel  (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_U8), 12);
    struct_with_all_types_pointer.write_u32_rel (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_U32), 13);
    struct_with_all_types_pointer.write_u64_rel (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_U64), 14);

    struct_with_all_types_pointer.write_f32_rel (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_F32), 15.0f32);
    struct_with_all_types_pointer.write_f64_rel (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_F64), 16.0f64);

    //now check if all values where written
    assert_eq!(struct_with_all_types_pointer.read_i8_rel  (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_I8)), 9);
    assert_eq!(struct_with_all_types_pointer.read_i32_rel (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_I32)), 10);
    assert_eq!(struct_with_all_types_pointer.read_i64_rel (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_I64)), 11);

    assert_eq!(struct_with_all_types_pointer.read_u8_rel  (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_U8)), 12);
    assert_eq!(struct_with_all_types_pointer.read_u32_rel (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_U32)), 13);
    assert_eq!(struct_with_all_types_pointer.read_u64_rel (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_U64)), 14);

    assert_eq!(struct_with_all_types_pointer.read_f32_rel (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_F32)), 15.0f32);
    assert_eq!(struct_with_all_types_pointer.read_f64_rel (Some(GameObject::OFFSET_STRUCT_WITH_ALL_TYPES + StructWithAllTypes::OFFSET_F64)), 16.0f64);
}
