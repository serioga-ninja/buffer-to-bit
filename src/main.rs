use std::fs::File;
use std::io::Read;

extern crate lib;

fn main()
{
    let mut file=File::open("Cargo.toml").unwrap();
    let mut buf=[0u8;12];
    file.read(&mut buf).unwrap();
    
    let s = lib::do_this(buf);
    for i in &s {
    	println!("{:?}", i);
    }
}