#![feature(convert)]

use std::slice;
use std::ffi::OsString;

fn main(){
    let hello = "hello";
    let space = " ";
    let world = "world";

    let bytes = unsafe{
        slice::from_raw_parts(hello.as_ptr(), 11)
    };

    let newhello = OsString::from_bytes(bytes);
    println!("{:?}", newhello);


}
