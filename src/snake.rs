use std::collections::LinkedList;
use piston_window::{Context, G2d};
use piston_window::types::Color;


use draw:: draw_block;

/*First Item: Red Element
Second Item: Green Element
Third Item: Blue Element
Fourth Item: Opacity

Since we wanted to have green snake, second paramater is 0.80.

We can change the color of the snake accordingly .
More on this later.
*/

const SNAKE_COLOR: Color = [0.00, 0.80, 0.00, 1.0];



pub enum Direction{
    Up,
    Down,
    Left,

    Right,
}

// implement a method for our enum here.

impl Direction{
    pub fn opposite(&self) -> Direction{
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,


        }
    }

}


// create a struct for our Block Type

struct Block{
    x: i32,
    y:i32,
}

//
pub struct Snake{
    direction: Direction,  // The current direction the snake is in
    body: LinkedList<Block>,// body of a snake will be a LinkedList of blocks
    tai: Option<Block>, // A tail will be an actual value, when we eat an apple
}

impl Snake{

    pub fn new(x: i32, y: i32) -> Snake {

        // Create a mutable body of typep LinkedList <Block>
        // Since it changes it's size, it's going to be mutable

        let mut body: LinkedList<Block> = LinkedList::new();


        /*
        We create a snake with 3 blocks.
        As it can be seen below, three blocks are created for the snake.

        */
        body.push_back(Block{
            x: x+2,
            y,
        });

        body.push_back(Block{x: x+1, y,});

        body.push_back(Block{x, y,});


        /*
        We also have Snake struct called and direction is left, and there is no tail initially.

        */

        Snake{
            direction: Direction::Left;
            body,
            tail:None,

        }

    }
}