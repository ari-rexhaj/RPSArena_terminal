use rand::prelude::*;
use std::io::{stdin,stdout,Write};
use std::{process::Command,thread,time};

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
    fn real_pos(&self) -> (f32,f32) {
        (self.x,self.y)
    }

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


fn main() {
    let mut rng = rand::thread_rng();
    let mut game = true;

    let map = (100.0,40.0);//- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - dimensions of map
    let mut bot_list: Vec<Bot> = vec![];


    let bot_amount = 200;// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - generates bots
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

    while game == true {
        clear_terminal_screen();
        
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
        
        println!("map x: {0} | map y: {1}",map.0,map.1);
        println!("rocks: {0} | papers: {1} | scissors: {2} | total: {3}",rock_count,paper_count,scissors_count,(rock_count+paper_count+scissors_count));
        
        //for bot in &bot_list {
        //    println!("{:?}",bot.map_pos())
        //}
        
        generate_map(bot_list.clone(),map);
        
        let mut input=String::new();
        print!("input: ");
        let _=stdout().flush();
        
        stdin().read_line(&mut input).expect("Did not enter a correct string");
        
        if let Some('\n')=input.chars().next_back() {input.pop();}
        if let Some('\r')=input.chars().next_back() {input.pop();}
        
        if input == "exit" {game = false}
        
        bot_list = next_turn(bot_list);
        
        thread::sleep(time::Duration::from_millis(30));
    }
}

fn generate_map(bot_list: Vec<Bot>,map:(f32,f32)) {
    let mut bot_found;

    for y in (0..(map.1+1.0) as u32).rev() {        //Generates Y lane, this is reversed because print pushed old prints on top of self, so to counter this and make the bottom left corner == (0,0), we reverse the loop
        for x in 0..(map.0+1.0) as u32 {            //Generates X lane, technically Y lane is never generated but is made automatically because only X lanes are made and stacked on top of eachother on the terminal
            bot_found = false;                           //This is to prevent printing 2 bots on the same position, since doing this will add an X. 
            for bot in &bot_list{
                if bot.map_pos() == (x,y) && !bot_found {
                    bot_found = true;
                    match bot.team {
                        Team::Rock => print!("R"),
                        Team::Paper => print!("P"),
                        Team::Scissors => print!("S")
                    }
                }
            }
            if !bot_found {print!(".")}
            //println!("{:?}x {:?}y",x,y)
        }
        println!()
    }
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