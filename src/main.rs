use anyhow::Result;
use std::fs::OpenOptions;
use std::fs::{File};
use std::io::{Write, Read, Seek};
use std::io::{Error, ErrorKind};
// const fn - !SPECIAL! function that can be computed during compile time 

fn increase(file: &mut File) -> Result<u64>{
    // read out the value
    let mut buffer = [0; 8];
    // check if file even contains any info
    file.read_exact(&mut buffer).unwrap_or_default();
    // cast bytes + increase
    let mut res = u64::from_be_bytes(buffer);
    res += 1;
    // write
    file.rewind()?;
    file.write_all(&res.to_be_bytes())?;
    // return
    Ok(res)
}

fn main() -> Result<()> {
    let mut file = OpenOptions::new().write(true).read(true).create(true).open("coins.txt")?;
    let current_coins = increase(&mut file);
    // try to set coins to 0 if no jar was yet created
    // None is not Option<T> in this implementation, I can not just write None
    // increase
    println!("Currently {current_coins:?} coins in the jar");
    Ok(())
}
