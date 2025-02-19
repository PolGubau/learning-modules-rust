mod other_file;
mod some_folder;

fn traer_grua() {
    println!("Traer grua");
}

mod taller {
    pub mod reception {
        pub fn add_cita() {}
        pub fn llevar_a_taller() {
            super::super::traer_grua();
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

pub fn test_code() {
    garage_module::arreglar_coche();
}

// idiomatic way
use crate::taller::garaje::arreglar_coche;
pub fn test_code2() {
    arreglar_coche();
}
