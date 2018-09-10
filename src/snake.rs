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


#[derive(Copy, Clone, PartialEq)]

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

#[derive(Debug, Clone)]
struct Block{
    x: i32,
    y:i32,
}

//
pub struct Snake{
    direction: Direction,  // The current direction the snake is in
    body: LinkedList<Block>,// body of a snake will be a LinkedList of blocks
    tail: Option<Block>, // A tail will be an actual value, when we eat an apple
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
            tail: None,

        }

    }

    pub fn draw(&self, con: &Context, g: &mut G2d){
        for block in &self.body{
            draw_block (SNAKE_COLOR, block.x, block.y, con, g);
        }
    }


    pub fn head_position(&self) -> (i32, i32){
        let head_block = self.body.front().unwrap();
        (head_block.x, head_block.y)
    }

    pub fn move_forward(&mut self, dir:Option<Direction>){
        match dir{
            Some(d) =>self.direction = d,
            None => (),


        }

        let (last_x, last_y ): (i32, i32) = self.head_position();

        let new_block = match self.direction {
            Direction::Up =>Block{
                x: last_x,
                y: last_y -1,
            },

            Direction::Down =>Block{
                x: last_x,
                y: last_y +1,
            },

            Direction::Left =>Block{
                x: last_x-1,
                y: last_y ,
            },
            Direction::Right =>Block{
                x: last_x +1,
                y: last_y ,
            },



        };

        self.body.push_front(new_block);
        let removed_block = self.body.pop_back().unwrap();

        self.tail = Some(removed_block);


    }

    pub fn head_direction(&self) -> Direction{
        self.direction
    }

    pub fn next_head(&self, dir : Option<Direction>) -> (i32, i32){
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.direction;

        match dir{

            Some(d) => moving_dir = d,
            None  => {}
        }

        match moding_dir {
            Direction::Up => (head_x, head_y -1),
            Direction::Down => (head_x, head_y +1),
            Direction::Left => (head_x -1, head_y),
            Direction::Up => (head_x +1, head_y),
        }

    }


    /*
    Tail doesn't get rendered unless we eat an apple.
    So when we eat the apple, the restore_tail method will be called.
    It takes a reference of itself as a parameter ( Mutable because the size changes and reference because main method doesn't
    need to own it.


    */
    pub fn restore_tail(&mut self){
        let blk = self.tail.clone().unwrap();
        self.body.push_back(blk)


    }

    pub fn overlap_tail(&self, x:i32, y:i32) -> bool {
        let mut ch = 0;
        for block in &self.body{ // Iterate through our snake body and check to see if x = block.x and y = block.y
            if x == block.x && y == block.y    { // Check if the snake overlapping with it's body, return true or false accordingly.
                    return true;
                }
            ch += 1;

            if ch == self.body.len()-1{
                break;
            }
        }

        return false;
    }
}