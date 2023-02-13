#![allow(non_snake_case)]

#![allow(unused_assignments)]
use std::{thread, time, vec, io::{stdin,stdout,Write},};

use rand::Rng;
//constants
const SCREENSIZE_X: i32 = 160; 
const SCREENSIZE_Y: i32 = 45; 
const SPEED: f64 = 0.0; 

#[derive(Debug,Copy,Clone)]
struct Position {
    x: i32,
    y: i32,
}

fn main() {
    let delay = time::Duration::from_millis(250);
    let mut bot_list = vec![Position{x:0,y:0},Position{x:0,y:0}];
    let mut rng = rand::thread_rng();

    println!("Hello, world!");
    fn pathfinding(bot_list: &[Position]) -> Vec<Position> {
        let bot = bot_list;
        
            let ax = bot[0].x as f64;
            let ay = bot[0].y as f64;
            
            let bx = bot[1].x as f64;
            let by = bot[1].y as f64;

            let botx = 0.0;
            let boty = 0.0;

            let bot = vec![Position{x:(SPEED*ax+bx-ax).round() as i32,y:(SPEED*ay+by-ay).round() as i32},Position{x:bx.round() as i32,y:by.round() as i32}];
            update_screen(&bot);
            println!("Hello from pathfinding!!");
        return bot.to_vec();
    }
    
    fn update_screen(bot_list: &Vec<Position>) {
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
                for bot in bot_list.iter() {    //runs check for every bot in bot_list
                    if bot_found == true {break}
                    let posy = SCREENSIZE_Y - bot.y;
                    let posx = bot.x;
                    if posx == x && posy == y {
                        print!("@");
                        bot_found = true;
                    }
                }
                if !bot_found {print!(".")}
                bot_found = false;
            }
        }
        println!();
    }
    
    
    
    let mut auto: bool = true;           //is bool for when auto is activated
    let mut auto_check: bool = false;     //is for checking if auto was turned off
    
    //Terminal rendering
    println!("{:?}",bot_list);
    while auto == true {
        if auto != auto_check {
            let bot1 = Position{x:rng.gen_range(1..SCREENSIZE_X),y:rng.gen_range(1..SCREENSIZE_Y)};
            let bot2 = Position{x:rng.gen_range(1..SCREENSIZE_X),y:rng.gen_range(1..SCREENSIZE_Y)};
            bot_list = vec![bot1,bot2];
        }
        let new_list = pathfinding(&bot_list);
        bot_list = new_list;
        println!("new: {:?}",&bot_list);
        
        let mut s=String::new();
        print!("Please enter some text: ");
        let _=stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n')=s.chars().next_back() {
            s.pop();
        }
        if let Some('\r')=s.chars().next_back() {
            s.pop();
        }

        if s == "end" {
            auto = false
        }

        auto_check = auto.clone();
        thread::sleep(delay);
        //auto = false
    }
}


