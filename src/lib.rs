//! # Rawstdraw
//! 
//! Rust bindings for a platform-independant rendering (2D or basic 3D) library - [rawdraw](https://github.com/cntools/rawdraw) (also known as CNFG)

#[macro_use]
extern crate lazy_static;

mod externs;

use std::ffi::CString;
    
pub type Coordinate = u16;

pub struct Point {
    x: Coordinate,
    y: Coordinate,
}

pub struct Size {
    width: i32,
    height: i32,
}

pub struct Color(u8, u8, u8, u8);

pub fn set_background_color(color: Color) {
    unsafe {
        // super unsafe and silly
        let ptr: *const Color = &color;
        let value = *(ptr as *const u32);
        
        externs::CNFGBGColor = value;
    }
}

pub fn set_color(color: Color) {
    unsafe {
        // super unsafe and silly
        let ptr: *const Color = &color;
        let value = *(ptr as *const u32);

        externs::CNFGColor(value);
    }
}

pub trait App {    
    fn handle_key(&self, keycode: u32, went_down: bool);
    fn handle_button(&self, position: Point, button: u32, went_down: bool);
    fn handle_motion(&self, position: Point, mask: u32);
    fn handle_destroy(&self);
}

pub fn setup(window_title: &str, dimensions: Size, app: Box<dyn App + Sync>) -> Result<(), i32> {
    let title_c = CString::new(window_title).expect("Can't convert title to a C string");
        
    externs::RUNNING_APP.set(app).unwrap();
    unsafe {
        match externs::CNFGSetup(title_c.as_ptr(), dimensions.width as i32, dimensions.height as i32) {
            0 => Ok(()),
            code => Err(code)
        }
    }
}

pub mod window {
    use crate::{Size, Color, externs};
    use std::ffi::CString;
    
    pub fn set_icon(data: Vec<Color>, dimensions: Size) {
        unsafe {
            externs::CNFGSetWindowIconData(dimensions.width as i32, dimensions.height as i32, data.as_ptr() as *const u32)
        }
    }
    
    pub fn change_title(title: &str) {
        let title_c = CString::new(title).expect("Can't convert title to a C string");
        
        unsafe {
            externs::CNFGChangeWindowTitle(title_c.as_ptr());
        }
    }
}

pub mod primitives {
    use crate::{Point, externs};
    
    pub fn tack_pixel(position: Point) {
        unsafe {
            externs::CNFGTackPixel(
                position.x as i16, 
                position.y as i16
            );
        }
    }
    
    pub fn tack_segment(start: Point, end: Point) {
        unsafe {
            externs::CNFGTackSegment(
                start.x as i16, 
                start.y as i16, 
                end.x as i16, 
                end.y as i16
            );
        }
    }
    pub fn tack_rectangle(from: Point, to: Point) {
        unsafe {
            externs::CNFGTackRectangle(
                from.x as i16, 
                from.y as i16, 
                to.x as i16, 
                to.y as i16
            );
        }
    }
    pub fn tack_poly(points: Vec<Point>) {
        unsafe {
            externs::CNFGTackPoly(points.as_ptr() as *const externs::RDPoint, points.len() as i32);
        }
    }
}

pub mod frame {
    use crate::externs;
    
    pub fn clear() {
        unsafe {
            externs::CNFGClearFrame();
        }
    }
    pub fn swap_buffers() {
        unsafe {
            externs::CNFGSwapBuffers();
        }
    }
    
    pub fn handle_input() {
        unsafe {
            externs::CNFGHandleInput();
        }
    }
}

pub mod text {
    use crate::{Point, Size, externs};
    use std::ffi::CString;

    pub fn set_pen_position(position: Point) {
        unsafe {
            externs::CNFGPenX = position.x.into();
            externs::CNFGPenY = position.y.into();
        }
    }

    pub fn draw(text: &str, scale: u16) {
        let text_c = CString::new(text).expect("Can't convert text to a C string");
        
        unsafe {
            externs::CNFGDrawText(text_c.as_ptr(), scale as i16);
        }
    }
    
    pub fn get_extents(text: &str, scale: u16) -> Size {
        let mut width: i32 = 0;
        let mut height: i32 = 0;
        let text_c = CString::new(text).expect("Can't convert text to a C string");
        
        unsafe {
            externs::CNFGGetTextExtents(text_c.as_ptr(), &mut width, &mut height, scale.into());
        }
        
        Size { width: width.into(), height: height.into() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn run_example() {
        set_background_color(Color(0xff, 0xff, 0xff, 0xff));
        setup("Rawstdraw example", Size { width: 800, height: 600}, Box::new(ExampleApp())).expect("Failed to setup rawdraw!");
        
        loop {
            frame::handle_input();
            frame::clear();
            
            set_color(Color(0xff, 0x33, 0x33, 0x33));
            primitives::tack_rectangle(Point { x: 10, y: 10 }, Point { x: 600, y: 300 });
            
            frame::swap_buffers();
        }
    }
    
    struct ExampleApp();
    impl App for ExampleApp {
        fn handle_key(&self, keycode: u32, _went_down: bool) {
            if keycode == 65307 {
                std::process::exit(0);
            }
        }
        
        fn handle_button(&self, _position: Point, _button: u32, _went_down: bool) {}
        fn handle_motion(&self, _position: Point, _mask: u32) {}
        fn handle_destroy(&self) {}
    }
}
