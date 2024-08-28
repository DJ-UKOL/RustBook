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
mod strings;
mod hash_maps;
mod errors;
mod generics;
mod traits;
mod aggregator;
mod lifetimes;
mod closures;

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
use crate::strings::strings;
use crate::hash_maps::hash_maps;
use crate::errors::errors;
use crate::generics::generics;
use crate::traits::traits;
use crate::lifetimes::lifetimes;
use crate::closures::closures;

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
    //vectors();
    //strings();
    //hash_maps();
    //errors();
    //generics();
    //traits();
    //lifetimes();
    closures();
}
