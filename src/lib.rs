#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::prelude::*;
pub use wordscapes_lookup::*;

mod wordscapes_lookup;

#[wasm_bindgen]
pub struct WordscapesLookupWrapper(WordscapesLookup);

#[wasm_bindgen]
impl WordscapesLookupWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(WordscapesLookup::default())
    }

    pub fn lookup(&self, s: &str) -> String {
        self.0.lookup(s).join("\n")
    }
}

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}
