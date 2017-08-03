// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]
#![deny(unreachable_code)]
#![feature(never_type)]

fn a() {
    loop { return; }
    println!("I am dead.");
}

fn b() {
    loop {
        break;
    }
    println!("I am not dead.");
}

fn c() {
    loop { return; }
    println!("I am dead.");
}

fn d() {
    'outer: loop { loop { break 'outer; } }
    println!("I am not dead.");
}

fn e() {
    loop { 'middle: loop { loop { break 'middle; } } }
    println!("I am dead.");
}

fn main() { }
