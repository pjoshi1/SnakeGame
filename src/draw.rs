


use piston_window::{rectangle, Context, G2d};



use piston_window::types::Color;



// Our blocks will scale up 30 pixels

const BLOCK_SIZE: f64  = 30.0;

// Takes in a game coordinate and cast it to f64 and multiple by BLOCK_SIZE
// pub means makes public to entire program



pub fn to_coord(game_coord: i32) -> f64{
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32{
    to_coord(game_coord) as u32
}


pub fn draw_block(color:Color, x:i32, y:i32, con: &Context, g: &mut G2d){

    let gui_x = to_coord(x);
    let gui_y  = to_coord(y);


    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,  // context transform
        g, // graphics buffer
    );

}


pub fn draw_rectangle (color: Color, x: i32,  y: i32, width: i32, height: i32, con: &Context,  g: &mut G2d,){
    let x = to_coord(x);
    let y  = to_coord(y);

    rectangle(
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE + (height as f64),


        ],
        con.transform,
        g,
    );

}