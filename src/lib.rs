#![feature(iterator_fold_self)]
#![feature(move_ref_pattern)]

// #[global_allocator]
// static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use wasm_bindgen::prelude::*;

pub use board_solver::*;
pub use filter::*;
pub use word::*;
pub use word_searcher::*;

mod board_solver;
mod filter;
mod word;
mod word_searcher;

#[wasm_bindgen]
pub struct WordSearcherWrapper(DAGSearcher);

#[wasm_bindgen]
impl WordSearcherWrapper {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self(DAGSearcher::default())
    }

    pub fn lookup(&self, s: &str) -> String {
        let mut words = self.0.lookup(s);
        words.sort_unstable_by(|a, b| {
            b.len()
                .cmp(&a.len())
                .then(b.frequency().cmp(&a.frequency()).then_with(|| a.cmp(&b)))
        });
        words.join("\n")
    }

    pub fn lookup_filter(&self, s: &str, filter: &str) -> String {
        let mut words = self.0.lookup_filter(s, filter);
        words.sort_unstable_by(|a, b| {
            b.len()
                .cmp(&a.len())
                .then(b.frequency().cmp(&a.frequency()).then_with(|| a.cmp(&b)))
        });
        words.join("\n")
    }
}

#[wasm_bindgen]
pub fn init() {
    console_error_panic_hook::set_once();
}
