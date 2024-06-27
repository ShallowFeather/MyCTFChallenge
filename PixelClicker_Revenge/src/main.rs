#![deny(clippy::all)]
#![forbid(unsafe_code)]


use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use std::collections::HashMap;

const WIDTH: u32 = 400;
const HEIGHT: u32 = 400;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct World {
    map: HashMap<(u32, u32), bool>,
}

fn main() -> Result<(), Error> {
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(1920, 1200, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };
    let mut world = World::new();

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            world.draw(pixels.frame_mut());
            if let Err(err) = pixels.render() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Space) {

                println!("Check");
                world.check();
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }
            
            let check1 = input.mouse_pressed(0);
            let check2 = input.mouse_held(0);
            // `pos` contains the mouse position
            if check1 || check2 {
                if let Some(pos) = input.mouse() {
                    world.update((pos.0) as u32, (pos.1) as u32);
                    window.request_redraw();
                }
            }            
        }
    });
}



impl World {
    /// Create a new `World` instance that can draw a moving box.
    fn new() -> Self {
        let map: HashMap<(u32, u32), bool> = HashMap::new();

        Self {
            map,
        }
    }

    fn check(&self) {
        let c = [192, 100, 132, 109, 171, 307, 337, 341, 218, 95, 275, 343, 268, 60, 285, 351, 316, 42, 118, 84, 129, 61, 247, 105, 46, 298, 124, 296, 295, 44, 53, 52, 84, 67, 53, 28, 36, 98, 42, 98, 78, 330, 255, 55, 250, 349, 45, 296, 154, 58, 29, 52, 70, 325, 291, 107, 296, 24, 294, 99, 271, 355, 88, 99, 315, 105, 231, 48, 206, 98, 303, 52, 86, 318, 118, 95, 27, 52, 230, 32, 74, 313, 157, 309, 250, 340, 127, 46, 288, 325, 155, 300, 10, 52, 313, 90, 226, 341, 69, 334, 155, 36, 333, 321, 74, 334, 128, 62, 152, 26, 97, 33, 348, 28, 13, 108, 53, 63, 94, 51, 285, 111, 139, 282, 41, 314, 252, 349, 229, 56, 95, 84, 325, 44, 246, 116, 127, 48, 216, 85, 96, 56, 59, 305, 36, 60, 123, 41, 308, 335, 234, 108, 218, 94, 39, 303, 287, 102, 163, 69, 68, 108, 348, 29, 122, 319, 13, 43, 44, 298, 272, 345, 331, 320, 156, 291, 62, 326, 349, 27, 50, 68, 59, 312, 295, 30, 269, 108, 91, 111, 353, 90, 236, 85, 348, 122, 265, 90, 50, 272, 59, 28, 48, 280, 392, 57, 85, 310, 16, 52, 335, 322, 250, 343, 334, 351, 79, 329, 18, 32, 315, 86, 196, 45, 348, 42, 227, 101, 45, 315, 202, 103, 9, 53, 48, 279, 53, 39, 269, 113, 315, 55, 52, 68, 180, 63, 153, 60, 226, 60, 9, 118, 132, 111, 282, 333, 53, 37, 199, 109, 296, 115, 141, 285, 231, 41, 328, 91, 81, 312, 181, 52, 11, 97, 317, 86, 350, 95, 93, 93, 182, 52, 348, 44, 361, 48, 311, 111, 285, 350, 260, 336, 347, 107, 9, 59, 258, 337, 23, 52, 58, 100, 90, 47, 9, 61, 234, 115, 117, 41, 35, 91, 308, 341, 348, 57, 256, 113, 360, 48, 109, 106, 153, 34, 115, 26, 295, 53, 179, 112, 156, 41, 286, 104, 225, 340, 249, 337, 135, 313, 212, 47, 53, 49, 285, 114, 201, 105, 302, 334, 49, 273, 121, 84, 271, 358, 269, 90, 116, 25, 164, 287, 28, 36, 330, 342, 337, 344, 96, 54, 316, 47, 243, 338, 31, 48, 180, 59, 47, 28, 126, 32, 136, 102, 75, 324, 380, 58, 234, 349, 337, 346, 348, 32, 25, 52, 302, 339, 278, 90, 271, 100, 363, 48, 135, 314, 302, 340, 360, 41, 379, 58, 326, 119, 247, 104, 38, 306, 269, 114, 273, 344, 217, 61, 241, 57, 320, 109, 313, 87, 295, 38, 315, 54, 36, 59, 62, 28, 74, 324, 134, 320, 351, 23, 116, 112, 214, 60, 197, 46, 158, 20, 213, 87, 126, 288, 279, 90, 328, 341, 271, 60, 239, 338, 335, 335, 323, 323, 43, 315, 198, 49, 77, 108, 25, 26, 236, 96, 180, 54, 124, 292, 337, 323, 34, 53, 328, 352, 123, 66, 122, 320, 264, 90, 268, 48, 32, 50, 179, 108, 71, 103, 149, 33, 254, 100, 292, 107, 349, 98, 37, 310, 230, 54, 27, 35, 129, 55, 122, 317, 335, 350, 29, 41, 214, 85, 67, 334, 245, 337, 165, 69, 255, 51, 326, 58, 67, 308, 135, 282, 155, 288, 37, 86, 114, 95, 89, 97, 284, 324, 126, 299, 243, 349, 231, 45, 306, 335, 222, 62, 282, 342, 37, 311, 271, 96, 295, 29, 285, 108, 321, 111, 335, 340, 323, 113, 157, 40, 150, 28, 362, 49, 96, 58, 88, 67, 249, 100, 299, 54, 316, 105, 224, 27, 90, 321, 267, 50, 261, 54, 227, 27, 267, 49, 89, 310, 272, 357, 24, 23, 111, 63, 246, 337, 313, 333, 270, 104, 271, 351, 285, 347, 68, 104, 285, 106, 308, 338, 235, 103, 277, 115, 9, 116, 295, 45, 163, 309, 150, 27, 385, 58, 215, 62, 56, 108, 259, 58, 118, 112, 295, 49, 247, 54, 264, 52, 126, 42, 126, 38, 153, 307, 326, 45, 353, 87, 254, 50, 208, 94, 308, 39, 199, 55, 345, 125, 70, 298, 58, 28, 63, 70, 155, 22, 122, 107, 193, 101, 77, 331, 155, 297, 254, 48, 166, 309, 332, 58, 214, 38, 358, 48, 153, 45, 36, 89, 88, 29, 199, 62, 164, 69, 239, 56, 28, 52, 276, 116, 47, 68, 10, 102, 56, 93, 216, 60, 49, 274, 120, 84, 114, 26, 125, 291, 316, 45, 362, 48, 150, 321, 285, 105, 316, 46, 124, 320, 218, 106, 189, 95, 315, 103, 116, 67, 55, 68, 325, 321, 247, 103, 14, 99, 53, 65, 225, 343, 378, 59, 120, 97, 80, 327, 271, 342, 189, 46, 321, 326, 42, 303, 22, 52, 269, 110, 164, 309, 334, 336, 127, 320, 323, 86, 87, 45, 152, 115, 334, 322, 297, 103, 54, 314, 119, 25, 281, 333, 144, 100, 308, 41, 41, 305, 91, 47, 50, 297, 83, 323, 129, 303, 37, 108, 144, 101, 214, 37, 9, 55, 40, 98, 316, 48, 302, 336, 295, 106, 250, 346, 57, 105, 87, 101, 328, 93, 304, 50, 215, 83, 285, 348, 229, 30, 214, 58, 79, 323, 96, 60, 9, 114, 212, 54, 43, 298, 45, 68, 85, 94, 122, 106, 160, 69, 250, 342, 43, 97, 244, 56, 230, 30, 218, 92, 271, 356, 348, 36, 316, 50, 364, 57, 9, 109, 141, 283, 225, 27, 353, 92, 10, 100, 328, 350, 326, 318, 347, 114, 255, 53, 349, 105, 273, 91, 306, 47, 333, 58, 95, 29, 129, 52, 56, 68, 135, 315, 150, 319, 90, 66, 234, 339, 119, 40, 76, 331, 325, 86, 60, 70, 123, 320, 252, 100, 122, 67, 84, 33, 83, 37, 348, 46, 51, 28, 40, 306, 346, 112, 131, 320, 311, 105, 96, 29, 116, 41, 325, 117, 352, 101, 67, 310, 227, 59, 251, 49, 127, 49, 242, 57, 121, 67, 179, 45, 295, 115, 225, 349, 152, 67, 231, 103, 251, 116, 239, 349, 276, 90, 184, 87, 9, 57, 91, 94, 273, 357, 255, 110, 35, 57, 112, 112, 323, 44, 81, 67, 348, 106, 216, 64, 267, 61, 20, 105, 318, 47, 157, 69, 158, 69, 194, 45, 155, 302, 307, 345, 336, 334, 221, 27, 278, 333, 295, 51, 12, 52, 60, 92, 235, 85, 154, 305, 310, 118, 85, 31, 129, 53, 96, 93, 229, 103, 87, 308, 212, 48, 359, 35, 365, 58, 118, 67, 301, 54, 340, 123, 125, 41, 316, 100, 187, 47, 60, 28, 285, 353, 214, 86, 295, 34, 38, 310, 234, 111, 90, 303, 179, 99, 270, 103, 80, 313, 348, 116, 49, 315, 68, 102, 53, 41, 127, 286, 126, 30, 273, 333, 322, 111, 289, 99, 53, 44, 87, 103, 73, 102, 133, 103, 327, 45, 358, 33, 324, 115, 179, 104, 234, 103, 178, 45, 272, 60, 49, 68, 12, 101, 11, 48, 59, 311, 195, 107, 149, 30, 241, 338, 133, 101, 50, 28, 86, 309, 69, 305, 41, 89, 123, 88, 88, 321, 9, 112, 353, 91, 218, 28, 347, 98, 352, 48, 34, 94, 102, 108, 70, 301, 295, 52, 127, 302, 33, 96, 9, 62, 186, 89, 286, 103, 118, 25, 107, 310, 259, 337, 295, 39, 271, 90, 276, 60, 271, 353, 332, 321, 86, 317, 333, 351, 77, 64, 48, 285, 134, 282, 190, 96, 46, 291, 387, 58, 346, 125, 346, 110, 87, 314, 63, 326, 347, 113, 315, 87, 79, 66, 239, 53, 231, 94, 57, 93, 311, 116, 152, 64, 61, 328, 163, 115, 124, 298, 69, 104, 24, 22, 229, 98, 215, 33, 61, 91, 226, 103, 268, 318, 90, 304, 252, 116, 355, 27, 199, 60, 77, 109, 148, 288, 126, 33, 348, 50, 350, 24, 156, 290, 322, 336, 47, 315, 73, 334, 271, 321, 277, 343, 236, 101, 105, 294, 153, 49, 352, 23, 47, 298, 378, 58, 293, 107, 198, 50, 68, 312, 180, 56, 336, 348, 179, 115, 248, 52, 126, 43, 194, 103, 199, 51, 165, 286, 271, 347, 9, 122, 359, 38, 121, 40, 203, 101, 25, 28, 118, 40, 319, 108, 126, 63, 120, 112, 152, 45, 20, 29, 10, 49, 159, 309, 99, 111, 250, 344, 23, 22, 150, 26, 148, 31, 132, 110, 29, 40, 243, 57, 61, 327, 91, 65, 78, 65, 230, 103, 34, 55, 179, 111, 90, 301, 87, 94, 217, 88, 309, 345, 43, 68, 288, 108, 271, 357, 10, 106, 306, 345, 269, 117, 353, 85, 357, 48, 218, 103, 123, 25, 122, 40, 331, 336, 207, 96, 117, 112, 218, 111, 68, 101, 350, 81, 348, 80, 373, 60, 38, 86, 180, 55, 215, 34, 260, 56, 359, 48, 61, 70, 190, 97, 58, 302, 155, 55, 134, 310, 307, 335, 131, 306, 323, 318, 94, 52, 213, 39, 268, 61, 179, 106, 120, 25, 103, 307, 72, 102, 212, 42, 154, 42, 255, 57, 10, 63, 76, 100, 240, 103, 341, 125, 316, 49, 322, 318, 121, 25, 153, 59, 248, 107, 167, 115, 310, 119, 322, 112, 37, 314, 85, 30, 61, 28, 285, 344, 231, 44, 278, 357, 296, 99, 48, 278, 279, 343, 295, 54, 95, 93, 364, 56, 86, 30, 315, 100, 289, 108, 86, 316, 287, 108, 77, 63, 349, 81, 350, 25, 69, 105, 68, 100, 253, 100, 39, 97, 154, 288, 303, 345, 321, 98, 80, 328, 84, 321, 155, 52, 247, 116, 53, 64, 223, 340, 180, 52, 34, 93, 131, 283, 181, 89, 295, 32, 28, 39, 96, 61, 255, 54, 15, 108, 151, 317, 77, 102, 26, 31, 212, 49, 42, 68, 129, 59, 248, 349, 16, 108, 87, 110, 280, 333, 183, 87, 305, 345, 329, 319, 110, 109, 261, 90, 61, 331, 205, 100, 262, 90, 39, 308, 157, 115, 43, 301, 255, 56, 279, 114, 37, 62, 217, 64, 83, 43, 212, 50, 348, 30, 101, 110, 318, 106, 246, 349, 301, 346, 357, 30, 121, 110, 237, 103, 40, 93, 77, 106, 126, 35, 267, 90, 219, 116, 95, 63, 298, 115, 228, 349, 230, 31, 134, 311, 353, 23, 214, 59, 352, 102, 225, 342, 58, 301, 316, 41, 13, 52, 326, 44, 331, 58, 354, 48, 247, 106, 16, 99, 35, 58, 234, 114, 355, 26, 34, 56, 313, 94, 161, 19, 271, 97, 327, 95, 238, 338, 129, 285, 155, 49, 87, 109, 108, 106, 313, 101, 17, 36, 308, 337, 351, 95, 127, 300, 228, 28, 348, 124, 288, 100, 48, 68, 358, 31, 231, 36, 249, 51, 188, 92, 295, 46, 128, 50, 352, 83, 32, 49, 111, 28, 195, 45, 307, 45, 145, 101, 199, 52, 218, 114, 350, 82, 54, 297, 44, 315, 179, 105, 80, 323, 304, 335, 389, 57, 9, 113, 348, 27, 155, 54, 324, 336, 117, 40, 122, 41, 239, 50, 168, 309, 95, 53, 273, 88, 184, 49, 157, 288, 391, 57, 239, 55, 129, 57, 222, 27, 77, 101, 346, 111, 68, 106, 124, 299, 280, 60, 54, 93, 125, 320, 157, 36, 47, 287, 313, 88, 36, 88, 130, 304, 274, 344, 301, 344, 30, 46, 317, 99, 115, 67, 349, 106, 64, 326, 303, 53, 77, 105, 271, 95, 87, 108, 348, 54, 347, 80, 280, 343, 239, 103, 224, 62, 30, 44, 84, 32, 160, 115, 311, 107, 133, 104, 204, 101, 91, 93, 48, 276, 14, 108, 269, 111, 156, 292, 126, 39, 321, 46, 48, 277, 89, 304, 131, 305, 156, 289, 20, 108, 351, 104, 46, 315, 126, 40, 295, 31, 274, 60, 139, 283, 257, 100, 167, 285, 256, 111, 236, 339, 53, 66, 325, 318, 265, 318, 152, 33, 250, 108, 93, 111, 346, 48, 272, 90, 38, 304, 295, 42, 308, 35, 297, 101, 319, 98, 40, 300, 69, 303, 250, 337, 126, 41, 77, 313, 226, 27, 77, 104, 16, 38, 352, 104, 66, 325, 151, 46, 124, 293, 200, 106, 156, 288, 271, 343, 46, 293, 24, 21, 97, 111, 61, 329, 36, 61, 307, 43, 313, 84, 121, 98, 237, 349, 328, 344, 270, 90, 19, 101, 59, 310, 296, 105, 92, 29, 307, 42, 273, 90, 218, 91, 122, 318, 53, 68, 170, 308, 154, 24, 249, 116, 155, 35, 179, 97, 349, 96, 127, 62, 151, 48, 231, 38, 48, 283, 26, 33, 349, 48, 77, 103, 248, 102, 124, 297, 295, 40, 211, 90, 284, 333, 328, 342, 83, 44, 313, 100, 40, 299, 192, 101, 188, 47, 142, 100, 44, 299, 226, 61, 348, 31, 180, 58, 285, 113, 83, 311, 165, 309, 308, 342, 155, 41, 96, 30, 225, 345, 120, 40, 312, 103, 286, 108, 90, 94, 312, 96, 160, 288, 337, 345, 239, 46, 116, 95, 348, 41, 39, 94, 113, 27, 246, 115, 351, 94, 218, 115, 58, 312, 295, 50, 238, 103, 88, 46, 202, 104, 72, 313, 155, 309, 337, 58, 19, 99, 346, 108, 375, 59, 291, 99, 286, 102, 42, 315, 161, 309, 121, 41, 81, 326, 46, 290, 28, 40, 351, 82, 253, 115, 166, 285, 122, 86, 169, 308, 89, 66, 309, 32, 82, 325, 358, 34, 251, 100, 191, 97, 349, 25, 276, 343, 59, 304, 49, 272, 248, 337, 350, 96, 269, 60, 320, 46, 361, 44, 11, 52, 90, 29, 125, 65, 348, 121, 86, 308, 69, 304, 250, 116, 277, 322, 213, 56, 93, 86, 292, 115, 253, 110, 212, 45, 236, 99, 41, 299, 123, 91, 154, 49, 228, 57, 118, 94, 97, 31, 217, 29, 231, 37, 365, 57, 311, 117, 102, 109, 326, 118, 126, 45, 89, 321, 274, 90, 275, 90, 297, 115, 133, 105, 271, 319, 123, 89, 33, 94, 54, 28, 30, 52, 19, 108, 132, 108, 216, 84, 295, 105, 311, 113, 39, 311, 234, 106, 250, 347, 323, 114, 182, 51, 309, 28, 328, 95, 353, 93, 250, 339, 77, 107, 231, 39, 306, 49, 18, 108, 47, 289, 260, 90, 242, 338, 180, 66, 302, 337, 259, 56, 41, 88, 124, 294, 135, 319, 309, 30, 198, 110, 52, 28, 85, 67, 138, 282, 220, 27, 273, 87, 130, 284, 228, 58, 335, 349, 267, 318, 212, 51, 311, 106, 132, 306, 263, 53, 180, 65, 387, 57, 53, 60, 308, 37, 183, 51, 150, 33, 290, 108, 38, 102, 213, 54, 115, 112, 12, 45, 254, 51, 360, 39, 123, 87, 388, 57, 93, 50, 32, 98, 209, 93, 274, 83, 107, 309, 117, 95, 152, 313, 231, 349, 154, 68, 281, 323, 303, 337, 25, 27, 57, 28, 10, 105, 92, 89, 115, 25, 45, 298, 129, 60, 83, 38, 180, 60, 362, 51, 53, 33, 9, 121, 347, 48, 151, 288, 228, 340, 282, 323, 287, 114, 269, 109, 53, 94, 271, 346, 285, 342, 336, 347, 218, 64, 139, 100, 75, 333, 269, 112, 290, 99, 65, 325, 216, 82, 34, 57, 308, 40, 261, 334, 60, 91, 40, 301, 243, 337, 255, 58, 63, 28, 82, 324, 344, 125, 328, 89, 271, 333, 352, 103, 14, 42, 11, 98, 121, 94, 274, 86, 102, 307, 336, 340, 121, 97, 26, 52, 331, 352, 119, 112, 104, 308, 83, 39, 230, 53, 328, 94, 228, 100, 314, 100, 275, 333, 38, 313, 275, 321, 119, 84, 271, 348, 322, 335, 376, 59, 92, 90, 33, 98, 35, 98, 321, 45, 24, 25, 348, 48, 273, 117, 278, 115, 129, 302, 284, 354, 328, 336, 54, 30, 160, 19, 37, 104, 122, 105, 85, 309, 153, 24, 327, 58, 102, 294, 285, 354, 185, 48, 37, 308, 230, 55, 135, 318, 212, 43, 331, 341, 186, 48, 359, 36, 255, 100, 225, 61, 308, 336, 125, 289, 105, 309, 180, 53, 192, 45, 17, 34, 87, 104, 46, 68, 129, 54, 278, 322, 88, 307, 42, 302, 68, 107, 125, 42, 276, 322, 265, 330, 354, 24, 347, 97, 330, 320, 83, 36, 158, 38, 67, 107, 348, 118, 151, 33, 72, 324, 61, 332, 53, 42, 182, 88, 311, 109, 11, 47, 40, 91, 232, 93, 37, 105, 308, 38, 285, 343, 31, 52, 313, 86, 230, 33, 310, 117, 311, 114, 231, 50, 285, 352, 360, 40, 149, 288, 239, 49, 16, 37, 37, 313, 333, 340, 355, 48, 133, 100, 53, 48, 88, 101, 271, 117, 263, 332, 44, 97, 215, 63, 96, 111, 225, 348, 334, 352, 227, 28, 20, 52, 179, 47, 316, 52, 261, 55, 55, 110, 155, 299, 279, 60, 63, 334, 126, 37, 348, 33, 36, 109, 161, 287, 97, 30, 361, 45, 122, 108, 89, 111, 57, 106, 94, 84, 313, 92, 346, 109, 122, 102, 59, 303, 297, 54, 78, 313, 213, 40, 354, 23, 35, 90, 113, 112, 77, 324, 21, 26, 83, 34, 80, 67, 179, 46, 70, 297, 53, 36, 123, 92, 134, 104, 13, 44, 222, 341, 69, 313, 135, 316, 276, 333, 302, 54, 55, 109, 152, 25, 39, 309, 169, 115, 57, 313, 283, 324, 179, 100, 153, 308, 155, 51, 272, 92, 96, 62, 124, 295, 316, 43, 311, 112, 70, 296, 79, 313, 9, 119, 335, 336, 148, 30, 85, 322, 149, 29, 231, 42, 223, 62, 199, 108, 217, 31, 57, 104, 18, 33, 155, 57, 37, 106, 285, 112, 347, 125, 75, 313, 326, 320, 227, 349, 271, 323, 179, 102, 270, 60, 179, 101, 157, 21, 297, 99, 32, 52, 217, 86, 254, 49, 125, 28, 122, 109, 353, 86, 234, 113, 47, 297, 289, 115, 94, 29, 102, 306, 172, 307, 348, 39, 149, 321, 363, 54, 218, 108, 352, 84, 53, 54, 303, 336, 67, 108, 295, 41, 363, 55, 53, 61, 182, 87, 133, 309, 9, 58, 180, 62, 87, 105, 156, 36, 255, 349, 126, 29, 159, 19, 231, 34, 393, 57, 83, 33, 132, 107, 275, 117, 349, 99, 188, 93, 17, 35, 196, 109, 317, 49, 273, 60, 50, 314, 85, 319, 18, 52, 198, 48, 53, 35, 327, 86, 19, 52, 141, 284, 58, 102, 18, 99, 151, 318, 153, 62, 122, 85, 41, 315, 135, 103, 59, 92, 67, 109, 257, 58, 150, 115, 59, 307, 271, 344, 213, 57, 329, 336, 157, 37, 212, 88, 300, 347, 255, 111, 179, 49, 39, 98, 239, 54, 309, 31, 314, 82, 104, 294, 153, 44, 224, 61, 136, 282, 336, 58, 53, 32, 122, 104, 312, 97, 218, 97, 324, 97, 349, 100, 362, 50, 382, 58, 292, 99, 328, 346, 53, 314, 133, 308, 313, 102, 247, 53, 195, 105, 251, 50, 231, 339, 319, 47, 322, 44, 115, 66, 291, 115, 348, 98, 20, 107, 252, 110, 239, 51, 230, 97, 108, 310, 70, 313, 328, 348, 112, 28, 295, 99, 322, 45, 227, 340, 319, 86, 33, 52, 32, 51, 59, 68, 126, 36, 212, 53, 296, 25, 83, 67, 151, 47, 154, 115, 301, 342, 37, 87, 154, 59, 218, 113, 9, 63, 86, 67, 134, 105, 70, 299, 117, 84, 315, 332, 179, 103, 53, 56, 218, 110, 90, 306, 271, 352, 220, 62, 326, 86, 216, 61, 59, 306, 256, 349, 152, 34, 267, 327, 155, 298, 179, 51, 39, 312, 68, 325, 133, 99, 89, 93, 180, 116, 218, 100, 288, 101, 296, 54, 256, 114, 89, 46, 87, 106, 351, 48, 129, 56, 235, 104, 37, 307, 40, 313, 165, 115, 24, 52, 53, 34, 250, 100, 297, 102, 328, 343, 126, 320, 68, 105, 44, 68, 12, 46, 231, 49, 316, 51, 150, 48, 180, 57, 348, 96, 250, 51, 318, 48, 350, 48, 327, 120, 225, 346, 180, 64, 55, 314, 302, 341, 295, 37, 328, 58, 190, 45, 91, 92, 240, 56, 12, 108, 53, 31, 180, 93, 348, 40, 92, 87, 216, 32, 119, 95, 148, 33, 251, 108, 122, 25, 37, 309, 329, 58, 10, 51, 88, 313, 276, 357, 332, 352, 55, 297, 123, 90, 29, 43, 20, 103, 253, 349, 280, 357, 269, 115, 115, 95, 20, 102, 55, 28, 158, 37, 82, 311, 295, 48, 256, 100, 322, 86, 199, 54, 252, 48, 161, 69, 75, 100, 155, 303, 27, 34, 37, 63, 30, 45, 307, 44, 296, 103, 334, 58, 269, 106, 67, 309, 118, 41, 298, 54, 180, 90, 49, 28, 309, 25, 130, 320, 42, 304, 348, 55, 199, 57, 285, 107, 304, 51, 231, 46, 15, 99, 324, 44, 362, 52, 93, 64, 316, 86, 273, 320, 68, 307, 96, 57, 348, 37, 120, 96, 51, 297, 341, 124, 248, 116, 283, 333, 155, 69, 191, 45, 284, 342, 152, 311, 238, 46, 212, 46, 287, 325, 143, 100, 59, 70, 348, 123, 240, 57, 119, 94, 71, 313, 88, 111, 353, 84, 169, 309, 196, 108, 72, 334, 56, 314, 272, 117, 112, 110, 296, 26, 53, 58, 124, 41, 153, 288, 122, 100, 218, 62, 48, 28, 253, 48, 90, 93, 150, 288, 91, 29, 112, 64, 125, 27, 77, 110, 9, 56, 113, 65, 162, 69, 235, 87, 85, 320, 147, 288, 155, 53, 132, 106, 76, 313, 353, 89, 152, 314, 328, 92, 90, 46, 64, 334, 9, 60, 277, 60, 154, 43, 59, 309, 9, 108, 119, 41, 132, 282, 285, 109, 103, 294, 353, 48, 324, 86, 128, 320, 14, 43, 88, 97, 336, 322, 314, 103, 336, 352, 249, 101, 70, 334, 67, 311, 127, 45, 348, 35, 102, 110, 335, 352, 285, 346, 9, 110, 180, 61, 185, 88, 269, 107, 271, 359, 38, 99, 133, 98, 122, 99, 87, 315, 196, 110, 124, 26, 92, 93, 318, 107, 53, 297, 52, 314, 93, 85, 38, 312, 88, 306, 281, 357, 33, 95, 162, 19, 179, 113, 316, 330, 88, 311, 231, 52, 288, 115, 197, 110, 126, 31, 137, 102, 278, 60, 234, 88, 326, 96, 84, 322, 73, 101, 210, 92, 336, 323, 314, 333, 157, 39, 313, 91, 293, 115, 384, 58, 334, 340, 309, 27, 225, 347, 271, 349, 263, 52, 120, 94, 348, 38, 318, 328, 270, 105, 198, 47, 348, 117, 163, 287, 22, 24, 15, 52, 15, 39, 348, 125, 17, 99, 96, 63, 154, 35, 156, 294, 306, 48, 259, 57, 293, 99, 191, 98, 277, 90, 26, 32, 45, 294, 58, 99, 48, 282, 234, 110, 58, 92, 213, 55, 274, 343, 383, 58, 82, 67, 25, 29, 39, 302, 62, 70, 189, 94, 275, 84, 135, 320, 362, 47, 171, 308, 179, 95, 89, 305, 372, 60, 119, 67, 100, 111, 92, 111, 247, 107, 44, 297, 86, 45, 348, 52, 124, 66, 133, 307, 9, 120, 48, 286, 117, 67, 254, 115, 285, 110, 134, 312, 265, 51, 9, 54, 12, 100, 122, 93, 56, 28, 126, 34, 219, 62, 308, 345, 233, 91, 90, 308, 248, 101, 32, 97, 158, 309, 356, 48, 133, 97, 356, 28, 40, 314, 306, 46, 17, 108, 35, 92, 49, 297, 327, 319, 121, 111, 140, 283, 328, 351, 90, 111, 92, 48, 390, 57, 9, 123, 140, 100, 228, 29, 179, 116, 155, 56, 130, 303, 179, 109, 337, 343, 168, 115, 324, 116, 285, 324, 279, 357, 56, 299, 122, 101, 236, 97, 302, 335, 219, 27, 40, 308, 187, 91, 333, 352, 227, 103, 82, 322, 53, 45, 280, 323, 315, 51, 256, 58, 311, 118, 25, 30, 353, 88, 63, 333, 335, 58, 129, 58, 10, 101, 120, 41, 275, 60, 328, 347, 20, 30, 315, 56, 51, 68, 282, 324, 215, 82, 199, 61, 68, 103, 89, 96, 258, 58, 21, 52, 255, 115, 83, 42, 149, 322, 33, 53, 86, 94, 141, 100, 156, 21, 315, 53, 223, 27, 312, 99, 342, 125, 90, 302, 301, 343, 272, 93, 57, 103, 109, 107, 297, 100, 138, 100, 126, 300, 281, 342, 352, 94, 348, 43, 199, 58, 214, 35, 218, 104, 351, 83, 328, 319, 312, 101, 351, 100, 22, 23, 42, 299, 179, 48, 152, 65, 274, 85, 274, 82, 31, 47, 127, 287, 244, 337, 87, 29, 194, 104, 330, 58, 308, 340, 236, 94, 53, 30, 295, 35, 269, 116, 53, 67, 87, 107, 13, 99, 39, 310, 328, 88, 152, 310, 153, 63, 20, 104, 66, 334, 218, 99, 305, 49, 94, 93, 62, 327, 126, 64, 180, 94, 151, 315, 57, 68, 269, 325, 239, 47, 271, 102, 61, 330, 309, 29, 127, 47, 48, 284, 348, 120, 312, 95, 86, 322, 111, 29, 256, 112, ];
        
        //print!("[");
        let mut i = 0;
        for (pos, _) in &self.map {
            if(c[i] == pos.0 && c[i + 1] == pos.1) {
                i += 2;
                continue;
            }
            //print!("{}, {}, ", pos.0, pos.1);
            println!("FAIL");
            break;
        }
        //print!("]");
        //println!();
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self, x: u32, y: u32) {
        //println!("Mouse clicked at: {} {}", x, y);
        let value = self.map.get(&(x, y)).copied().unwrap_or(false);
        //println!("{}", value);
        if value == false {
            //println!("{} {}", x, y);
            self.map.insert((x, y), true);
        }
    }

    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x: u32 = i as u32 % WIDTH as u32;
            let y: u32 = i as u32 / WIDTH as u32;
            
            let mut rgba = [0xff, 0xff, 0xff, 0xff];
            let value = self.map.get(&(x, y)).copied().unwrap_or(false);

            if value == true {
                //println!("{} {}", x, y);
                rgba = [0, 0, 0, 0];
            }
      

            pixel.copy_from_slice(&rgba);
        }
    }
}