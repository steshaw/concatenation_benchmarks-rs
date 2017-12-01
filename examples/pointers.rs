#![feature(convert)]

use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::slice;

fn main() {
    let hello = "hello";
    let _space = " ";
    let _world = "world";

    let bytes = unsafe { slice::from_raw_parts(hello.as_ptr(), 11) };

    let newhello = OsStr::from_bytes(bytes);
    println!("{:?}", newhello);


}
