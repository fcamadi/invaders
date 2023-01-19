// This is the frame module

use crate::{NUM_COLS, NUM_ROWS};

// We are using a type alias because the frame is a vector of vectors
// of borrowed static string slices
pub type Frame = Vec<Vec<& 'static str>>;

// function to create a Frame
pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(NUM_COLS);
        for _ in 0..NUM_COLS {
            let mut col = Vec::with_capacity(NUM_ROWS);
            for _ in 0..NUM_ROWS {
                col.push(" ");  //default content
            }
            cols.push(col);
        }
        cols
}

// everything that we want to see is going to need to be able to draw itself into the frame:
pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}

