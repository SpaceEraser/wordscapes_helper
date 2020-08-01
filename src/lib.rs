#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::prelude::*;
pub use wordscapes_helper::*;

mod wordscapes_helper;
#[wasm_bindgen]
pub struct WordscapesHelperWrapper(WordscapesHelper);

#[wasm_bindgen]
impl WordscapesHelperWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(WordscapesHelper::default())
    }

    pub fn lookup(&self, s: &str) -> String {
        self.0.lookup(s).join("\n")
    }

    pub fn lookup_filter(&self, s: &str, filter: &str) -> String {
        self.0.lookup_filter(s, filter).join("\n")
    }
}

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}
