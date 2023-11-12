#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
//Use linear algebra module

pub mod state;
use state::run;

fn main() {
    pollster::block_on(run());
}
