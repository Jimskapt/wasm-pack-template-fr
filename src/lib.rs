mod utils;

use wasm_bindgen::prelude::*;

// Lorsque la fonctionnalité `wee_alloc` est activée, nous allons utiliser
// `wee_alloc` en tant qu'allocateur global.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn saluer() {
    alert("Salut, {{project-name}} !");
}
