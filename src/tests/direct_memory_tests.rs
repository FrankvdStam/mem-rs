use crate::prelude::*;
use crate::tests::game_object::read_game_object_health;

pub fn setup_process() -> Process
{
    read_game_object_health();

    let self_name = Process::get_current_process_name().expect("Failed to get process name");
    let mut process = Process::new_with_memory_type(self_name.as_str(), MemoryType::Direct);
    process.refresh().expect("failed to refresh process");
    process
}


#[test]
pub fn hoster()
{

    let mut mock_process = setup_process();
    let game_object_pointer = mock_process.scan_rel("GameObject", "48 8d 05 ? ? ? ? 48 85 c0 48 8b 08", 3, 7, Vec::new()).expect("failed to scan for GameObject");
    let health = game_object_pointer.read_f32_rel(Some(0x0));
    let game_time = game_object_pointer.read_f64_rel(Some(0x8));

    assert_eq!(58.2f32, health);
    assert_eq!(94715235.165f64, game_time);
}

