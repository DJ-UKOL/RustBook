mod variables;
mod functions;
mod branches;
mod loops;
mod ownership;
mod references;
mod dangling;
mod slices;

use crate::variables::variables;
use crate::functions::functions;
use crate::branches::branches;
use crate::loops::loops;
use crate::ownership::ownership;
use crate::references::references;
use crate::dangling::dangling;
use crate::slices::slices;

fn main() {
    //variables();
    //functions();
    //branches();
    //loops();
    //ownership();
    //references();
    //dangling();
    slices();
}
