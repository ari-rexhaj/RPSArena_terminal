#![allow(non_snake_case)]

use std::vec;

//Starting variables
const SCREENSIZE_X: i32 = 160; 
const SCREENSIZE_Y: i32 = 45; 
const BOTAMOUNT: i32 = 2;

#[derive(Debug,Copy,Clone)]
struct Position {
    x: i32,
    y: i32,
}

fn main() {
    println!("Hello, world!");
    
    fn update_screen(bot_list: &Vec<Position>) {
        print!("{}[2J", 27 as char);
        print!(".");
        for _ in 0..SCREENSIZE_X {
            print!("_")
        }
        
        
        for y in 0..SCREENSIZE_Y {
            println!("");
            //print!("{}",SCREENSIZE_Y-y);
            print!("|");
            for x in 0..SCREENSIZE_X {
                let mut dot_printed = false;
                
                for i in 0..BOTAMOUNT {
                    let bot = bot_list[i as usize];
                    let posx = bot.x;
                    let posy = SCREENSIZE_Y - bot.y;
                    if posx == x && posy == y {
                        print!("x");
                        break;
                    }
                
                    if !dot_printed {
                        print!(".");
                        dot_printed = true
                    }
                }
            }
        }
        println!()
    }
    let bot1 = Position{x:1,y:5};
    let bot2 = Position{x:2,y:4};
    let bot_list = vec![bot1,bot2];
    println!("{:?}",bot_list);
    
    let mut game_on: bool = true;
    while game_on == true {
        update_screen(&bot_list);
        game_on = false;
    }
    
    println!("{:?}",(bot1.x,bot1.y));
    println!("{:?}",(bot2.x,bot2.y));

}


