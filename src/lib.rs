#![feature(nll)]
extern crate rand;
extern crate core;
extern crate ordered_float;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[cfg(test)]
mod tests;
mod design;
mod logic;
mod player;
mod state;
mod utils;
mod foundation;
mod objects;

use std::ffi::CString;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::boxed::Box;
use foundation::GameState;
use utils::ipoint::IPoint;

#[no_mangle]
pub fn press_key(pimpl: *mut GameState, key: *mut c_char) {
    // unsafe = grab raw pointers
    let mut game: Box<GameState> = unsafe { Box::from_raw(pimpl) };
    let input: &str = unsafe { CStr::from_ptr(key) }.to_str().unwrap();

    game.process_key(input);

    // unsafe - drop pointer without disposing resources
    Box::into_raw(game);
}

#[no_mangle]
pub fn get_view(pimpl: *mut GameState) -> *mut c_char {
    // unsafe = grab raw pointer
    let mut game: Box<GameState> = unsafe { Box::from_raw(pimpl) };

    let result = game.get_view();

    // unsafe - drop pointers without disposing resources
    Box::into_raw(game);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub fn get_messages(pimpl: *mut GameState, count: i32) -> *mut c_char {
    // unsafe = grab raw pointer
    let mut game: Box<GameState> = unsafe { Box::from_raw(pimpl) };

    let result = game.get_messages(count);

    // unsafe - drop pointers without disposing resources
    Box::into_raw(game);
    CString::new(result).unwrap().into_raw()
}

#[no_mangle]
pub fn create_world() -> *mut GameState {
    // unsafe
    // create World, get pointer to it
    // drop pointer without disposing resources
    Box::into_raw(Box::new(GameState::new(IPoint{x: 60, y: 40})))
}

#[no_mangle]
pub fn destroy_world(pimpl: *mut GameState) {
    unsafe {
        // grab raw pointer and dispose resources
        Box::from_raw(pimpl);
    };
}

#[no_mangle]
pub fn deallocate(raw_string: *mut c_char) {
    unsafe {
        // grab raw pointer and dispose resources
        CString::from_raw(raw_string);
    }
}

#[no_mangle]
pub fn allocate(length: usize) -> *mut c_char {
    // unsafe
    // create cstring of given size and forget it, so resources are not disposed
    CString::new(" ".repeat(length)).unwrap().into_raw()
}

pub fn print(string: String) {
    unsafe {
        print_raw(CString::new(string).unwrap().into_raw())
    }
}

extern {
    fn print_raw(raw_string: *mut c_char);
}