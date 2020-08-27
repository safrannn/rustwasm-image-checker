mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

const CHECKERBOARD_SIZE: usize = 20;
const OUTPUT_BUFFER_SIZE: usize = CHECKERBOARD_SIZE * CHECKERBOARD_SIZE * 4;
static mut OUTPUT_BUFFER: [u8; OUTPUT_BUFFER_SIZE] = [0; OUTPUT_BUFFER_SIZE];

#[wasm_bindgen]
pub fn get_buffer_pointer() -> *const u8 {
    let pointer: *const u8;
    unsafe {
        pointer = OUTPUT_BUFFER.as_ptr();
    }

    return pointer;
}

#[wasm_bindgen]
pub fn generate_checker_board(
    dark_red: u8,
    dark_green: u8,
    dark_blue: u8,
    light_red: u8,
    light_green: u8,
    light_blue: u8,
) {
    for y in 0..CHECKERBOARD_SIZE {
        for x in 0..CHECKERBOARD_SIZE {
            let mut is_dark_square: bool = true;

            if y % 2 == 0 {
                is_dark_square = false;
            }

            if x % 2 == 0 {
                is_dark_square = !is_dark_square;
            }

            let mut square_red: u8 = dark_red;
            let mut square_green: u8 = dark_green;
            let mut square_blue: u8 = dark_blue;
            if !is_dark_square {
                square_red = light_red;
                square_green = light_green;
                square_blue = light_blue;
            }

            let square_number: usize = y * CHECKERBOARD_SIZE + x;
            let square_rgba_index: usize = square_number * 4;

            unsafe {
                OUTPUT_BUFFER[square_rgba_index + 0] = square_red;
                OUTPUT_BUFFER[square_rgba_index + 1] = square_green;
                OUTPUT_BUFFER[square_rgba_index + 2] = square_blue;
                OUTPUT_BUFFER[square_rgba_index + 3] = 255;
            }
        }
    }
}
