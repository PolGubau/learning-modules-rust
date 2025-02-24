mod other_file;
mod some_folder;

fn get_crane() {
    println!("Traer crane");
}

mod taller {
    pub mod reception {
        pub fn add_cita() {}
        pub fn llevar_a_taller() {
            super::super::get_crane();
        }
    }
    pub mod garaje {
        pub fn arreglar_coche() {}
    }
}
pub fn llevar_a_arreglar() {
    // create is like a / in the path
    // super is like a ../ in the path
    // self is like a ./ in the path
    // crate::taller::garaje::arreglar_coche();
    taller::garaje::arreglar_coche();
    taller::reception::add_cita();
    taller::reception::llevar_a_taller();
    some_folder::some_files::example_function();
}

// using USE
// is like a import in python
use crate::taller::garaje as garage_module;
use some_folder::some_files as cool_module;

use rand::prelude::*;
pub fn test_code() {
    garage_module::arreglar_coche();
    cool_module::example_function();
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen();
    println!("Generated random number: {}", y);
}

// idiomatic way
use crate::taller::garaje::arreglar_coche;
pub fn test_code2() {
    arreglar_coche();
}
