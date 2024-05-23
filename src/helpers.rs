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
use windows::core::{PCSTR, PCWSTR};

/// Naive linear search for a needle in a haystack with wildcards
pub fn scan(haystack: &[u8], needle: &[Option<u8>]) -> Option<usize>
{
    if haystack.len() == 0
    {
        return None;
    }

    for i in 0..haystack.len() - needle.len()
    {
        let mut found = true;
        for j in 0..needle.len()
        {
            if let Some(byte) = needle[j]
            {
                if byte != haystack[i + j]
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

/// Converts a string of hex characters into a byte pattern with wildcards.
/// ? is the character used for wildcards.
/// Hex characters don't have to be prefixed with 0x
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

/// Retrieve only the filename portion from a filepath.
pub fn get_file_name_from_string(str: &String) -> String
{
    return String::from(Path::new(&str).file_name().unwrap().to_str().unwrap());
}

/// Win32 memes. Use with caution.
pub fn vec_u16_to_u8(vec_u16: &Vec<u16>) -> Vec<u8>
{
    return unsafe { vec_u16.align_to::<u8>().1.to_vec() };
}

/// Win32 memes. Use with caution.
pub fn w32str_to_string(w32str: &Vec<u16>) -> String
{
    return w32str.iter().map(|&v| (v & 0xFF) as u8).take_while(|&c| c != 0).map(|c| c as char).collect();
}

/// Win32 memes. Use with caution.
pub fn get_w32str_from_str(str: &str) -> Vec<u16>
{
    return str.encode_utf16().collect();
}

/// Win32 memes. Use with caution.
pub fn get_pcwstr_from_str(str: &str) -> PCWSTR
{
    let vec: Vec<u16> = str.encode_utf16().collect();
    return PCWSTR(vec.as_ptr());
}

/// Win32 memes. Use with caution.
pub fn get_pcstr_from_str(str: &str) -> PCSTR
{
    return PCSTR(str.as_ptr());
}