mod variables;
mod functions;
mod branches;
mod loops;
mod ownership;
mod references;

use crate::variables::variables;
use crate::functions::functions;
use crate::branches::branches;
use crate::loops::loops;
use crate::ownership::ownership;
use crate::references::references;

fn main() {
    //variables();
    //functions();
    //branches();
    //loops();
    //ownership();
    references();
}
