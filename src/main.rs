#![allow(non_snake_case)]

#![allow(unused_assignments)]
use std::{thread, time};

use rand::Rng;
//constants
const SCREENSIZE_X: i32 = 160; 
const SCREENSIZE_Y: i32 = 45; 
const BOTAMOUNT: i32 = 2;

#[derive(Debug,Copy,Clone)]
struct Position {
    x: i32,
    y: i32,
}

fn main() {
    let mut rng = rand::thread_rng();

    println!("Hello, world!");
    fn pathfinding(bot_list: &Vec<Position>) {
        
        for i in 1..BOTAMOUNT {
            let ax = bot_list[i as usize].x;
            let ay = bot_list[i as usize].y;
            
            let bx = bot_list[1-(i as usize)].x;
            let by = bot_list[1-(i as usize)].y;
            

            let bot_list = vec![Position{x:0,y:0},Position{x:0,y:0}];
            update_screen(&bot_list);
            println!("Hello from pathfinding!!");
            println!("{:?}",(bx,by));
            println!("{:?}",(ax,ay));
            //vec = ((p2x-p1x),(p2y-p1y))
        }
        
        fn update_screen(bot_list: &Vec<Position>) {
            let delay = time::Duration::from_millis(500);
            //this is where the board updates from bot positions
            print!("{}[2J", 27 as char);
            print!(".");
            for _ in 0..SCREENSIZE_X {
                print!("_")
            }
            
            let mut bot_found: bool = false;
    
            for y in 0..SCREENSIZE_Y {
                println!("");
                //print!("{}",SCREENSIZE_Y-y);
                print!("|");
                for x in 0..SCREENSIZE_X {
                    bot_found = false;
                    for i in 0..BOTAMOUNT {
                        let bot = bot_list[i as usize];
                        let posy = SCREENSIZE_Y - bot.y;
                        let posx = bot.x;
                        if posx == x && posy == y {
                            print!("@");
                            bot_found = true;
                            break;
                        }
                    }
                    if !bot_found {
                        print!(".");
                    }
                }
            }
            println!();
            thread::sleep(delay);
        }
    }

   
    let mut bot_list = vec![Position{x:0,y:0},Position{x:0,y:0}];
    println!("{:?}",bot_list);
    

    let mut game_on: bool = true;
    let mut temp_game_on: bool = false;

    while game_on == true {
        if game_on != temp_game_on {
            let bot1 = Position{x:rng.gen_range(0..SCREENSIZE_X),y:rng.gen_range(0..SCREENSIZE_Y)};
            let bot2 = Position{x:rng.gen_range(0..SCREENSIZE_X),y:rng.gen_range(0..SCREENSIZE_Y)};
            bot_list = vec![bot1,bot2];
        }
        pathfinding(&bot_list);
        
        temp_game_on = game_on.clone();
    }
}


