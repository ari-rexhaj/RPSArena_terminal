use rand::prelude::*;
use std::io::{stdin,stdout,Write};
use std::ops::Add;
use std::{process::Command,thread,time};
use colored::*;

#[derive(Debug,Copy,Clone,PartialEq)]
enum Team {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug,Copy,Clone,PartialEq)]
struct Bot {
    x: f32,
    y: f32,
    team: Team
}

impl Bot {
    fn map_pos(&self) -> (u32,u32) {
        (self.x.round() as u32,self.y.round() as u32)
    }

    fn chase(&self) -> Team {
        match self.team{
            Team::Rock => Team::Scissors,
            Team::Paper => Team::Rock,
            Team::Scissors => Team::Paper
        }
    }
    
    fn distance(&self,bot2:Bot) -> (f32,f32,f32) {
    
        let dist_x = bot2.x-self.x;
        let dist_y = bot2.y-self.y;
        let dist_points = f32::sqrt((dist_x)*(dist_x)+(dist_y)*(dist_y));
    
        return (dist_points,dist_x,dist_y)
    }
}

fn generate_bots(map:(f32,f32),bot_amount:i32) -> Vec<Bot> {
    let mut rng = rand::thread_rng();
    let mut bot_list = vec![];

    for _ in 0..bot_amount {
        let xpos = rng.gen_range(0.0..map.0);
        let ypos = rng.gen_range(0.0..map.1);
        let mut bot_team = Team::Rock;
        match rng.gen_range(0..3) {
            0 => bot_team = Team::Rock,
            1 => bot_team = Team::Paper,
            2 => bot_team = Team::Scissors,
            _ => println!("how could dis happen? - heavy tf2")
        }

        let temp_bot = Bot{
            x: xpos,
            y: ypos,
            team: bot_team
        };

        bot_list.push(temp_bot)
    }
    return bot_list
}


fn main() {
    clear_terminal_screen();
    println!("{}","\nwelcome to RPSArena!!!\nmade by urs truly the awsome aARi rexxhaj!!!!\n\ntype help for commands");

    let mut game = true;

    let mut map = (100.0,40.0);//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - dimensions of map
    let mut bot_amount = 200;
    let mut bot_list: Vec<Bot> = generate_bots(map,bot_amount);
 
    let mut speed = 10; 
    let mut auto = 0;
    let mut update = false;

    while game == true {        
        let mut rock_count:u16 = 0;
        let mut paper_count:u16 = 0;
        let mut scissors_count:u16 = 0;
        
        for bot in &bot_list {
            
            match bot.team {
                Team::Rock => rock_count += 1,
                Team::Paper => paper_count += 1,
                Team::Scissors => scissors_count += 1,
            }
        }
        
        if update == true {
            generate_map(bot_list.clone(),map);
            println!("map x: {0} | map y: {1}",map.0,map.1);
            println!("{0}: {1} | {2}: {3} | {4}: {5} | bots: {6} \n","rock".bright_red(),rock_count,"paper".bright_green(),paper_count,"scissors".bright_blue(),scissors_count,(rock_count+paper_count+scissors_count));
        }
        else {
            update = true
        }
        
        if auto > 0 {   // aka if auto is not 0 but written so even if the program somehow jumped from 1 to -1, the code would still stop
            println!("auto turns left: {}",auto);
            auto -= 1 
        }
        else { 
            let mut input=String::new();
            print!(">");
            let _=stdout().flush();
            
            stdin().read_line(&mut input).expect("Did not enter a correct string");
            
            if let Some('\n')=input.chars().next_back() {input.pop();}
            if let Some('\r')=input.chars().next_back() {input.pop();}
            
            if input == "exit" {game = false}
            else if input.contains("delay")  {          // sets the delay
                for _ in 0..6 {
                    input.remove(0);
                }
                speed = input.parse().unwrap()
            }
            
            else if input.contains("autoplay") {        // turns on autoplay
                for _ in 0..9 {
                    input.remove(0);
                }
                auto = input.parse().unwrap()
            }

            else if input.contains("reset") {           //respawns bot
                bot_list = generate_bots(map,bot_amount)
            }

            else if input.contains("mapx") {            //changes mapx value and resets
                for _ in 0..5 {
                    input.remove(0);
                }
                map.0 = input.parse().unwrap();
                bot_list = generate_bots(map,bot_amount)

            }

            else if input.contains("mapy") {            // changes mapy value and resets
                for _ in 0..5 {
                    input.remove(0);
                }
                map.1 = input.parse().unwrap();
                bot_list = generate_bots(map,bot_amount)
            }

            else if input.contains("bot_amount") {            // changes mapy value and resets
                for _ in 0..11 {
                    input.remove(0);
                }
                bot_amount = input.parse().unwrap();
                bot_list = generate_bots(map,bot_amount)
            }

            else if input.contains("help") {            // changes mapy value and resets
                update = false;
                println!("\ncommands:\n
    delay int       changes how long the loop waits until next update
    autoplay int    automatically plays <n> amount of turns
    reset           respawns bots
    mapx int        changes mapx value and respawns bots
    mapy int        changes mapy value and respawns bots
    bot_amount int  changes amount of bots and respawns bots
    exit            stops the script\n
    these commands are very case sensetive and one mistake may throw exception\n")
            }

        }
        bot_list = next_turn(bot_list);
        thread::sleep(time::Duration::from_millis(speed));
    }
}

fn generate_map(bot_list: Vec<Bot>,map:(f32,f32)) {
    let mut bot_found;
    let mut string_map:String = "".to_string();

    for y in (0..(map.1+1.0) as u32).rev() {        //Generates Y lane, this is reversed because print pushed old prints on top of self, so to counter this and make the bottom left corner == (0,0), we reverse the loop
        for x in 0..(map.0+1.0) as u32 {            //Generates X lane, technically Y lane is never generated but is made automatically because only X lanes are made and stacked on top of eachother on the terminal
            bot_found = false;                           //This is to prevent printing 2 bots on the same position, since doing this will add an X. 
            for bot in &bot_list{
                if bot.map_pos() == (x,y) && !bot_found {
                    bot_found = true;
                    match bot.team {
                        Team::Rock => string_map = string_map.add("\x1b[101mR\x1b[0m"),
                        Team::Paper => string_map = string_map.add("\x1b[102mP\x1b[0m"),
                        Team::Scissors =>  string_map = string_map.add("\x1b[104mS\x1b[0m")
                    }
                }
            }
            if !bot_found { string_map = string_map.add(".")}
            //println!("{:?}x {:?}y",x,y)
        }
        string_map = string_map.add("\n")
    }
    clear_terminal_screen();
    println!("{}",string_map)
}

fn next_turn(old_bot_list: Vec<Bot>) -> Vec<Bot> {

    let mut new_bot_list = old_bot_list.clone();

    let mut dist:(f32,f32,f32); //saves the distance, x component of distance and y component of distance
    let mut closest_dist:(f32,f32,f32);

    for bot1 in old_bot_list {  //saves bot 1, the bot that will be moved
        closest_dist = (1000.0,0.0,0.0);
        for bot2 in new_bot_list.iter_mut() {  //saves bot 2, this bot will cycle through all the bots and calculate the distance between them to figure out where bot 1 should move (towards the closest bot in bot list)
            
            if (bot1 != *bot2) && (bot1.chase() == bot2.team) {
                dist = bot1.distance(*bot2);
                
                if dist.0 < closest_dist.0 {
                    closest_dist = dist.clone();
                    if dist.0 < f32::sqrt(3.0) {
                        bot2.team = bot1.team
                    }
                }
            }
        }

        let mut new_bot1 = bot1.clone();
        
        new_bot1.x = bot1.x + closest_dist.1/closest_dist.0;  //code for moving x
        new_bot1.y = bot1.y + closest_dist.2/closest_dist.0;  //code for moving y
        
        new_bot_list.remove(0);
        new_bot_list.push(new_bot1);
    }
    return new_bot_list
}

pub fn clear_terminal_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/c", "cls"])
            .spawn()
            .expect("cls command failed to start")
            .wait()
            .expect("failed to wait");
    } else {
        Command::new("clear")
            .spawn()
            .expect("clear command failed to start")
            .wait()
            .expect("failed to wait");
    };
}