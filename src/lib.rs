#![feature(proc_macro, extern_prelude, wasm_custom_section, wasm_import_module)]

extern crate circular_queue;
extern crate wasm_bindgen;
extern crate wasm_rgame_ui;
extern crate wasm_rgame;
extern crate wbg_rand;
extern crate wrg_2d;

use wasm_bindgen::prelude::*;

pub use wasm_rgame::bootstrap::*;

pub mod application;
pub use application::*;

#[wasm_bindgen]
#[allow(non_camel_case_types)]
pub struct wrg_snake_EntryPoint {}

#[wasm_bindgen]
impl wrg_snake_EntryPoint {
    pub fn init(application: &mut Application) {
        let spawner = application.as_spawner();

        let app_delegate = ApplicationDelegate::new(spawner);
        spawner.spawn_root(app_delegate);
    }
}
