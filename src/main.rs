mod variables;
mod functions;
mod branches;
mod loops;
mod ownership;
mod references;
mod dangling;
mod slices;
mod structs;
mod rectangles;
mod enums;
mod matches;
mod iflets;
mod vectors;

use crate::variables::variables;
use crate::functions::functions;
use crate::branches::branches;
use crate::loops::loops;
use crate::ownership::ownership;
use crate::references::references;
use crate::dangling::dangling;
use crate::slices::slices;
use crate::structs::structs;
use crate::rectangles::rectangles;
use crate::enums::enums;
use crate::matches::matches;
use crate::iflets::iflets;
use crate::vectors::vectors;

fn main() {
    //variables();
    //functions();
    //branches();
    //loops();
    //ownership();
    //references();
    //dangling();
    //slices();
    //structs();
    //rectangles();
    //enums();
    //matches();
    //iflets();
    vectors();
}
