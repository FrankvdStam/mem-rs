# mem-rs
[![Crates.io](https://img.shields.io/crates/v/mem-rs)](https://crates.io/crates/mem-rs) 
[![Crates.io](https://img.shields.io/crates/d/mem-rs)](https://crates.io/crates/mem-rs) 
[![build](https://github.com/FrankvdStam/mem-rs/actions/workflows/build.yml/badge.svg)](https://github.com/FrankvdStam/mem-rs/actions/workflows/build.yml)  
Memory manipulation library for running processes, mainly aimed at games.

## Features
- Process abstraction
- Reading and writing to process memory
- Pattern scanning in the process's main module
- Pointer abstraction and offset resolving
- Reading and writing relatively from a pointer
- dll injection

## example usage

```rust
struct Ds1
{
    process: Process,
    game_data_man: Pointer,
    ai_timer: Pointer,
}

impl Ds1
{
    pub fn new() -> Self
    {
        Ds1
        {
            process: Process::new("DarkSoulsRemastered.exe"),
            game_data_man: Pointer::default(),
            ai_timer: Pointer::default(),
        }
    }

    pub fn get_in_game_time_milliseconds(&self) -> u32
    {
        return self.game_data_man.read_u32_rel(Some(0xa4));
    }

    pub fn get_ai_timer(&self) -> f32
    {
        return self.ai_timer.read_f32_rel(Some(0x24));
    }

    pub fn refresh(&mut self) -> Result<(), String>
    {
        if !self.process.is_attached()
        {
            self.process.refresh()?;
            self.game_data_man = self.process.scan_rel("GameDataMan", "48 8b 05 ? ? ? ? 48 8b 50 10 48 89 54 24 60", 3, 7, vec![0])?;
            self.ai_timer = self.process.scan_rel("GameDataMan", "48 8b 0d ? ? ? ? 48 85 c9 74 0e 48 83 c1 28", 3, 7, vec![0])?;
        }
        else
        {
            self.process.refresh()?;
        }
        Ok(())
    }
}

fn main()
{
    let mut ds1 = Ds1::new();

    loop
    {
        match ds1.refresh()
        {
            Ok(()) => {}
            Err(e) => println!("{}", e)
        }

        println!("igt: {}", ds1.get_in_game_time_milliseconds());
        println!("ai: {}", ds1.get_ai_timer());
        sleep(Duration::from_secs(1));
    }
}
```
