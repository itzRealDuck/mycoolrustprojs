#[warn(non_snake_case)]
extern crate rand;
use core::panic;

use rand::Rng;
use std::io;
use std::ops::Deref;

struct Number {
    number: i32,
}

impl Number {
    fn new(number: i32) -> Number {
        Number { number: number }
    }
}

impl Deref for Number {
    type Target = i32;

    fn deref(&self) -> &<Self as Deref>::Target {
        &self.number
    }
}

fn DrawBoard(spaces: &mut [char; 9]) {
    println!("     |     |     " );
    println!(" {}   |  {}  |    {}",spaces[0],spaces[1],spaces[2]);
    println!("_____|_____|_______");
    println!("     |     |        ");
    println!(" {}   |  {}  |    {}",spaces[3],spaces[4],spaces[5]);
    println!("_____|_____|_______");
    println!("     |     |        ");
    println!(" {}   |  {}  |    {}",spaces[6],spaces[7],spaces[8]);
    println!("_____|_____|_______");
    
    
    
}
fn playermove(spaces: &mut [char; 9], player:  char) {
    let mut input = String::new();
    let mut number: usize;

    println!("Welcome To Tic Tac Toe");
    println!("**********************");
    println!("enter number for what square you like");

    io::stdin().read_line(&mut input).unwrap();

    number = input.trim().parse().expect(" invalid diget");



    if spaces[number] == ' ' {
        spaces[number] = player;
    }
    if number < 0 || number > 8 {
        panic!("Out of Bounds and this is not from compiler but from me bozo you are a really bad bozo skill issue");
    }
}
fn computermove(spaces:  &mut [char; 9], computer:  char) {
    let mut rng = rand::thread_rng();

    let mut number: Number;

    while true {
        number = Number::new(rng.gen_range(0..9));

        let mut num2 = *number;

        

        if spaces[num2 as usize] == ' ' {
            spaces[num2 as usize] = computer;
            break;
        }
    }
}
fn calculatewinner(spaces:  &mut [char; 9], computer: char, player: char) -> bool {


match (spaces,player) { 

(spaces,player) if spaces[0] != ' ' && spaces[0] == spaces[1] && spaces[1] == spaces[2] => if spaces[0] == player {println!("You won") } else { println!("You lost bozo L")} 
(spaces,player) if spaces[3] != ' ' && spaces[3] == spaces[4] && spaces[4] == spaces[5] => if spaces[3] == player {println!("You won")} else {println!("You lost L")}
(spaces,player) if spaces[6] != ' ' && spaces[6] == spaces[7] && spaces[7] == spaces[8] => if spaces[6] == player {println!("You won")} else { println!("You lost L")}
(spaces,player) if spaces[0] != ' ' && spaces[0] == spaces[4] && spaces[4] == spaces[8] => if spaces[0] == player {println!("You won")} else {println!("You lost L bozo")}
(spaces,player) if spaces[2] != ' ' && spaces[2] == spaces[4] && spaces[4] == spaces[6] =>  if spaces[2] == player {println!("You won")} else {println!("You lost L bozo")}
(spaces,player) if spaces[0] != ' ' && spaces[0] == spaces[3] && spaces[3] == spaces[6] =>  if spaces[0] == player {println!("You won")} else {println!("You lost L bozo")}
(spaces,player) if spaces[1] != ' ' && spaces[1] == spaces[4] && spaces[4] == spaces[7] => if spaces[1] == player {println!("You won")} else {println!("You lost L bozo")}
(spaces,player) if spaces[2] != ' ' && spaces[2] == spaces[5] && spaces[5] == spaces[8] => if spaces[2] == player {println!("You won")} else {println!("You lost L bozo")}
_ => {return false}

}


    true
}
fn checktie(spaces: &mut [char; 9]) -> bool {
    for i in 0..9 {
        if spaces[i] == ' ' {
            return false;
        }
    }
    true
}

fn main() {
    let mut spaces = [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '];
    let  player = 'O';
    let  computer = 'X';
    let mut running = true;

    DrawBoard(&mut spaces);

    while running {
        playermove(&mut spaces,  player);

        DrawBoard(&mut spaces);

        if calculatewinner(&mut spaces, computer, player) {
            running = false;
            break;
        }

        computermove(&mut spaces,  computer);

        DrawBoard(&mut spaces);

        if calculatewinner(&mut spaces, computer, player) {
            running = false;

            break;
        } else if checktie(&mut spaces) {
            running = false;
            break;
        }
    }
}
