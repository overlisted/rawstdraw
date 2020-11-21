use std::os::raw::c_char;
use mut_static::MutStatic;

use crate::{App, Point};

#[allow(dead_code)]
pub struct RDPoint {
    x: i16,
    y: i16,
}

struct NoOpApp();

impl App for NoOpApp {
    fn handle_key(&self, _keycode: u32, _went_down: bool) {}
    fn handle_button(&self, _position: Point, _button: u32, _went_down: bool) {}
    fn handle_motion(&self, _position: Point, _mask: u32) {}
    fn handle_destroy(&self) {}
}

#[link(name="rawdraw", kind="static")]
extern "C" {
    pub static mut CNFGPenX: u32;
    pub static mut CNFGPenY: u32;
    pub static mut CNFGBGColor: u32;
    pub static CNFGLastColor: u32;

    pub fn CNFGColor(RGB: u32) -> u32;

    pub fn CNFGDrawText(text: *const c_char, scale: i16);
    pub fn CNFGGetTextExtents(text: *const c_char, w: *mut i32, h: *mut i32, textsize: i32);
    
    pub fn CNFGTackPixel(x: i16, y: i16);
    pub fn CNFGTackSegment(x1: i16, y1: i16, x2: i16, y2: i16);
    pub fn CNFGTackRectangle(x1: i16, y1: i16, x2: i16, y2: i16);
    pub fn CNFGTackPoly(points: *const RDPoint, verts: i32);
    
    pub fn CNFGSetup(WindowName: *const c_char, w: i32, h: i32) -> i32;
    
    pub fn CNFGHandleInput();
    pub fn CNFGClearFrame();
    pub fn CNFGSwapBuffers();

    pub fn CNFGChangeWindowTitle(windowtitle: *const c_char);
    pub fn CNFGSetWindowIconData(w: i32, h: i32, data: *const u32);
}

lazy_static! {
    pub static ref RUNNING_APP: MutStatic<Box<dyn App + Sync>> = MutStatic::new();
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn HandleKey(keycode: i32, bDown: i32) {
    RUNNING_APP.read().unwrap().handle_key(keycode as u32, bDown != 0);
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn HandleButton(x: i32, y: i32, button: i32, bDown: i32) {
    RUNNING_APP.read().unwrap().handle_button(
        Point { x: x as u16, y: y as u16 },
        button as u32,
        bDown != 0
    );
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn HandleMotion(x: i32, y: i32, mask: i32) {
    RUNNING_APP.read().unwrap().handle_motion(
        Point { x: x as u16, y: y as u16 },
        mask as u32,
    );
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn HandleDestroy() {
    RUNNING_APP.read().unwrap().handle_destroy();
}
