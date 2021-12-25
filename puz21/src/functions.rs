use std::{convert::TryInto};

pub fn part1(position1: u32, position2: u32) -> u32{
    let mut player1 = Player::new("1", position1);
    let mut player2 = Player::new("2", position2);
    let mut die_value = 1;
    let mut contador = 0;
    println!("=== Posiciones iniciales: {}, {}", position1, position2);
    loop{
        die_value = player1.play(die_value);
        contador += 3;
        if player1.score >= 1000{
            break;
        }
        die_value = player2.play(die_value);
        contador += 3;
        if player2.score >= 1000{
            break;
        }
        if contador > 1000{
            println!("=== Lanzamientos: {} ===", contador);
            player1.print();
            player2.print();
        }
    }
    player1.print();
    player2.print();
    let mut value = 0;
    if player1.score > player2.score{
        println!("El ganador fue: {}", player1.name);
        value = player2.score * contador;
    }else{
        println!("El ganador fue: {}", player2.name);
        value = player1.score * contador;
    }
    println!("El dado fue lanzado {} veces", contador);
    println!("Valor final: {}", value);
    value
}

pub fn part2(data: &str) -> usize{
    0
}

struct Player{
    name: String,
    position: u32,
    score: u32
}

impl Player{
    fn new(name: &str, position: u32) -> Player{
        Self{name: name.to_string(), position, score: 0}
    }
    fn play(&mut self, die_value: u32) -> u32{
        self.position = (3 * (die_value + 1) + self.position) % 10;
        if self.position == 0{
            self.position = 10;
        }
        self.score += self.position;
        die_value + 3
    }
    fn print(&self){
        println!("Name: {}. Position: {}, Score: {}", self.name, self.position,
                 self.score);
    }
}


#[test]
fn complete_sample(){
    let value = part1(4, 8);
    assert_eq!(value, 739785);
}
