#![allow(non_snake_case)]

//#![allow(unused_assignments)]

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
    println!("Hello, world!");
    
    fn update_screen(bot_list: &Vec<Position>) {
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
        println!()
    }
    let bot1 = Position{x:16,y:4};
    let bot2 = Position{x:4,y:2};
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


