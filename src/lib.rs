fn traer_grua() {
    println!("Traer grua");
}

mod taller {
    pub mod recepcion {
        fn add_cita() {}
        fn () {
            super::super::traer_grua();
        }
    }llevar_a_taller
    pub mod garaje {
        pub fn arreglar_coche() {}
    }
}
pub fn llevar_a_arreglar() {
    // create is like a / in the path
    // super is like a ../ in the path
    // self is like a ./ in the path
    crate::taller::garaje::arreglar_coche();
    // taller::garaje::arreglar_coche();
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
