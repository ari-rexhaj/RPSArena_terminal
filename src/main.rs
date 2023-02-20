#![allow(non_snake_case)]

use std::{thread, time, vec, io::{stdin,stdout,Write}};
use rand::Rng;
//constants
const SCREENSIZE_X: i32 = 350; 
const SCREENSIZE_Y: i32 = 90; 
const SPEED: f64 = 1.0; 

#[derive(Debug,Copy,Clone,PartialEq)]
enum Team {
    Misc = 0,
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug,Copy,Clone,PartialEq)]
struct Properties {
    x: i32,
    y: i32,
    team: Team
}

fn main() {
    let delay = time::Duration::from_millis(200);
    let mut bot_list = vec![];
    let mut rng = rand::thread_rng();

    println!("Hello, world!");
    fn movement(bot_list: &[Properties]) -> Vec<Properties> {
        let mut bot = vec![];
        let mut real_bot = vec![];
        let mut botx: f64;
        let mut boty: f64;

        let mut closest_bot: &Properties = &bot_list[0];
        for bot1 in bot_list.iter() {
            let ax: f64 = bot1.x as f64;
            let ay: f64 = bot1.y as f64;

            let mut closest_dis:f64 = 1000.0;
            for bot2 in bot_list.iter() {  //this method for finding the closest bot is not pretty but i just want to make a proof of concept for now and maybe change it later
                let mut distance = ((bot1.x-bot2.x) as f64).powf(0.5)+((bot1.y-bot2.y) as f64).powf(0.5);
                if distance < 0.0 {distance *= -1.0}
                if bot1 != bot2 && distance < closest_dis {
                    closest_bot = bot2;
                    closest_dis = distance;
                    println!("new closest found: {:?}",closest_bot);
                }
            }
            
            let bx = closest_bot.x as f64;
            let by = closest_bot.y as f64;

            let vector_x = bx-ax;
            let vector_y = by-ay;
            let vec = vec![(vector_x/((vector_x*vector_x+vector_y*vector_y).powf(0.5))),(vector_y/((vector_x*vector_x+vector_y*vector_y).powf(0.5)))];

            botx = ax+vec[0]*SPEED;
            boty = ay+vec[1]*SPEED;

            real_bot.append(&mut vec![Properties{x:botx.round() as i32,y:boty.round() as i32,team:Team::Misc}]);
            bot.append(&mut vec![Properties{x:botx.round() as i32,y:boty.round() as i32,team:Team::Misc}]);
        }

            update_screen(&bot);
            return real_bot.to_vec()
    }
    
    fn update_screen(bot_list: &Vec<Properties>) {
        //this is where the board updates from bot Propertiess

        print!("{}[2J", 27 as char);
        print!(".");
        for _ in 0..SCREENSIZE_X {
            print!("_")
        }
        
        let mut bot_found: bool;
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
            }
        }
        println!();
    }
    
    
    let mut auto: bool = false;           //is bool for when auto is activated
    let mut running = true;
    let mut auto_check: bool = true;     //is for checking if auto was turned off
    
    //Terminal rendering
    println!("{:?}",bot_list);
    while running == true {
        if auto != auto_check {
            for _ in 0..5 {
                let bot = Properties{x:rng.gen_range(1..SCREENSIZE_X),y:rng.gen_range(1..SCREENSIZE_Y),team:Team::Misc};
                bot_list.append(&mut vec![bot]);
            }
        }

        let new_list = movement(&bot_list);
        bot_list = new_list;
        println!("new: {:?}",&bot_list);
        
        if auto == false {
            let mut s=String::new();
            print!("command:  ");
            let _=stdout().flush();
            stdin().read_line(&mut s).expect("Did not enter a correct string");
            if let Some('\n')=s.chars().next_back() {
                s.pop();
            }
            if let Some('\r')=s.chars().next_back() {
                s.pop();
            }
            if s == "end" { running = false } else if s == "auto" { auto = true }
        }
        auto_check = auto.clone();
        thread::sleep(delay);
    }
}


