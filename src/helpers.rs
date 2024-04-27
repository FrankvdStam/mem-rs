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

use std::path::Path;

pub fn scan(code: &[u8], pattern: &[Option<u8>]) -> Option<usize>
{
    if code.len() == 0
    {
        return None;
    }

    for i in 0..code.len() - pattern.len()
    {
        let mut found = true;
        for j in 0..pattern.len()
        {
            if let Some(byte) = pattern[j]
            {
                if byte != code[i + j]
                {
                    found = false;
                    break;
                }
            }
        }
        if found
        {
            return Some(i);
        }
    }
    return None;
}

pub fn to_pattern(str: &str) -> Vec<Option<u8>>
{
    let mut vec = Vec::new();
    for substr in str.split(" ")
    {
        if substr == "?"
        {
            vec.push(None);
        }
        else
        {
            vec.push(Some(u8::from_str_radix(substr, 16).expect("invalid hex string in pattern string")));
        }
    }
    return vec;
}

pub fn w32str_to_string(w32str: &Vec<u16>) -> String
{
    return w32str.iter().map(|&v| (v & 0xFF) as u8).take_while(|&c| c != 0).map(|c| c as char).collect();
}

pub fn get_file_name_from_string(str: &String) -> String
{
    return String::from(Path::new(&str).file_name().unwrap().to_str().unwrap());
}