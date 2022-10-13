// This file is part of the mem-rs distribution (https://github.com/FrankvdStam/mem-rs).
// Copyright (c) 2022 Frank van der Stam.
// https://github.com/FrankvdStam/mem-rs/blob/main/LICENSE
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

use std::thread::sleep;
use std::time::Duration;
use mem_rs::prelude::*;

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
            self.ai_timer = self.process.scan_rel("AI Timer", "48 8b 0d ? ? ? ? 48 85 c9 74 0e 48 83 c1 28", 3, 7, vec![0])?;
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