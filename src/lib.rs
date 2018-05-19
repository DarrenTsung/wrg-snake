#![feature(proc_macro, extern_prelude, wasm_custom_section, wasm_import_module)]

#[macro_use] extern crate lazy_static;

extern crate circular_queue;
extern crate wasm_bindgen;
extern crate wasm_rgame_ui;
extern crate wasm_rgame;
extern crate wrg_2d;

use wasm_bindgen::prelude::*;

pub mod wrg_bootstrap;
pub use wrg_bootstrap::*;

pub mod application;
pub use application::*;

#[wasm_bindgen]
extern {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f64;
}

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
