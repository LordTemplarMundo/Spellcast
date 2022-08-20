use std::collections::VecDeque;

const GESTURE_NAMES: [&str; 6] = ["fingers","palm","snap","wave","digit","clap"];

#[derive(Debug)]
enum Gesture { // Individual hands make gestures.
    Fingers,
    Palm,
    Snap,
    Wave,
    Digit,
    Clap,
    Empty,
}

impl From<char> for Gesture {
    fn from(key: char) -> Self {
        match key.to_string().to_lowercase().as_ref() {
            "f" => Gesture::Fingers,
            "p" => Gesture::Palm,
            "s" => Gesture::Snap,
            "w" => Gesture::Wave,
            "d" => Gesture::Digit,
            "c" => Gesture::Clap,
            "e" => Gesture::Empty,
            _ => Gesture::Empty
        }
    }
}

impl From<&str> for Gesture {
    fn from(name: &str) -> Self {
        if GESTURE_NAMES.iter().any(|&g| g == name) || name.len() == 1 {
            Gesture::from(name.chars().next().unwrap_or('e'))
        } else {
            Gesture::Empty
        }
    }
}

type Phrase = (Gesture,Gesture);

struct Player {
    name: String,
    max_health: u8,
    current_health: u8,
    gesture_buffer: std::collections::VecDeque<Phrase>
}

fn main() {
    println!("╔══════════════════╗");
    println!("╠S─P─E─L─L─C─A─S─T!╣");
    println!("╚══════════════════╝");
    println!("(q on its own in any prompt to quit)");
    let mut player_one_name: String = "".to_string();   
    let mut name_set: bool = false;
    println!("\n\rPlease name your spellcaster:");
    while !name_set {
        player_one_name = "".to_string();
        std::io::stdin().read_line(&mut player_one_name).unwrap();
        player_one_name = player_one_name.replace(&['\n','\r'], "");
        match player_one_name.as_str() {
            "" => {
                player_one_name = String::from("Mysterious Mage");
                name_set = true;
            },
            s => match s.parse::<i128>() {
                Ok(_) => {
                    println!("That isn't any name I've ever heard of!");
                    name_set = false;
                },
                Err(_) => {
                    name_set = true;
                }
            }
        };
    };
    println!("\n\rWelcome, {}!\n\r",player_one_name);
    let mut player_one: Player = Player {
        name: player_one_name,
        max_health: 20,
        current_health: 20,
        gesture_buffer: VecDeque::new()
    };
    let mut turn_number = 1;
    loop {
        println!("\n\rTurn {} begins:",turn_number);
        let mut line = String::new();
        println!("{}: {}/{} >",player_one.name, player_one.current_health,player_one.max_health);
        std::io::stdin().read_line(&mut line).unwrap();
        line = line.replace(&['\n','\r'], "");
        if line == "q" {
                println!("Quitting.");
                break;
            }
        let mut parts:Vec<&str> = line.split(',').filter(|p| p != &"").map(|p| p.trim()).rev().collect();
        let input_phrase: Phrase = {
            match parts.pop() {
                Some(g) => {match parts.pop() {
                    Some(t) => (Gesture::from(g),Gesture::from(t)),
                    None => (Gesture::from(g),Gesture::from("Empty"))
                }
                },
                None => {
                    (Gesture::from("Empty"),Gesture::from("Empty"))
                },
            }
        };
        println!("Phrase entered: {:?}",input_phrase);
        player_one.gesture_buffer.push_front(input_phrase);
        println!("History: {:?}",player_one.gesture_buffer);
        turn_number += 1;
    }
}