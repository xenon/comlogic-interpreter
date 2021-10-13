#[cfg(target_arch = "wasm32")]
cfg_if::cfg_if! {
    if #[cfg(target_arch = "wasm32", feature = "wee_alloc")] {
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

pub mod clterm;
pub mod term;
#[cfg(target_arch = "wasm32")]
pub mod wasm;
