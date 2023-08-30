use rand::prelude::*;
use std::io::{stdin,stdout,Write};

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
}


fn main() {
    let mut rng = rand::thread_rng();
    let mut game = true;

    let map = (160.0,45.0);                                                 //dimensions of map
    let mut bot_list: Vec<Bot> = vec![];

    for _ in 0..4 {                                                                     // generates bots
        let xpos = rng.gen_range(0.0..map.0);
        let ypos = rng.gen_range(0.0..map.1);
        let mut bot_team = Team::Rock;
        match rng.gen_range(0..3) {
            0 => bot_team = Team::Rock,
            1 => bot_team = Team::Paper,
            2 => bot_team = Team::Scissors,
            _ => println!("Some shit that should never happen happened at bot generation")
        }

        let temp_bot = Bot{
            x: xpos,
            y: ypos,
            team: bot_team
        };

        bot_list.push(temp_bot)
    }

    while game == true {
        for bot in &bot_list {
            println!("{:?}",bot.map_pos());
        }

        generate_map(bot_list.clone(),map);

        let mut input=String::new();
        print!("input: ");
        let _=stdout().flush();
        stdin().read_line(&mut input).expect("Did not enter a correct string");

        if let Some('\n')=input.chars().next_back() {input.pop();}
        if let Some('\r')=input.chars().next_back() {input.pop();}

        if input == "exit" {game = false}

        bot_list = next_turn(bot_list);

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
    let mut closest_bot:Bot;

    for bot1 in new_bot_list.clone() {  //saves bot 1, the bot that will be moved
        closest_dist = (1000.0,0.0,0.0);
        closest_bot = bot1;
        for bot2 in old_bot_list.clone() {  //saves bot 2, this bot will cycle through all the bots and calculate the distance between them to figure out where bot 1 should move (towards the closest bot in bot list)
            
            if (bot1 != bot2) && (chase(bot1.team) == bot2.team) {
                dist = distance(bot1, bot2);
                
                if dist.0 < closest_dist.0 {
                    closest_dist = dist.clone();
                    closest_bot = bot2;
                    if dist.0 < 1.0 {        
                        // if bot2 is less than 1 distance away, remove from new_bot_list and replace with copy of self with new team
                    }
                }
            }
            //else {continue;}
        }
        let mut new_bot1 = bot1.clone();
        
        new_bot1.x = bot1.x + closest_dist.1/closest_dist.0;  //code for moving x
        new_bot1.y = bot1.y + closest_dist.2/closest_dist.0;  //code for moving y
        
        if new_bot1.map_pos() != closest_bot.map_pos() {      // seems theres a bug where 2 bots on the same team can still move on top of eachother, oh well!
            new_bot_list.remove(0);
            new_bot_list.push(new_bot1);
        }
        else {
            new_bot_list.remove(0);
            new_bot_list.push(bot1)
        }

    }
    return new_bot_list
}

fn chase(own_team:Team) -> Team {
    match own_team{
        Team::Rock => Team::Scissors,
        Team::Paper => Team::Rock,
        Team::Scissors => Team::Paper
    }
}

fn distance(bot1: Bot,bot2:Bot) -> (f32,f32,f32) {

    let dist_x = bot2.x-bot1.x;
    let dist_y = bot2.y-bot1.y;
    let dist_points = f32::sqrt((dist_x)*(dist_x)+(dist_y)*(dist_y));

    return (dist_points,dist_x,dist_y)
}