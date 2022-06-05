mod minesweeper;
mod rand;

use std::cell::RefCell;
use std::borrow::{Borrow, BorrowMut};
use wasm_bindgen::prelude::*;
use crate::minesweeper::Minesweeper;

thread_local! {
    static MINESWEEPER: RefCell<Minesweeper>  = RefCell::new(Minesweeper::new(10,10,9));
}


#[wasm_bindgen(js_name = getState)]
pub fn get_state() -> String {
    MINESWEEPER.with(|ms| ms.borrow().to_string())
}

#[wasm_bindgen(js_name = openField)]
pub fn open_field(x: usize, y: usize) {
    MINESWEEPER.with(|mut ms| {
        ms.borrow_mut().open((x, y));
    });
}

pub fn put_flag(x: usize, y: usize) {
    MINESWEEPER.with(|mut ms|{
        ms.borrow_mut().toggle_flag((x, y));
    })

}