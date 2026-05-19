use std::thread;
use std::arch::asm;
use std::time::Duration;

#[path = "../../../src/tests/game_object.rs"]
mod game_object;

fn main()
{
    println!("Test binary started");

    let x = game_object::read_game_object_health();
    println!("x: {}", x);

    loop{
        thread::sleep(Duration::from_millis(1000));
    }
}