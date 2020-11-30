mod common;

use common::{compile_riscu, init};
use monster::disassemble::*;
use rayon::prelude::*;

#[test]
fn can_disassemble_risc_u_binary() {
    init();

    compile_riscu(None).1.for_each(|(source, object)| {
        let result = disassemble(object);

        assert!(
            result.is_ok(),
            "can disassemble object file of {}",
            source.to_str().unwrap()
        );
    });
}