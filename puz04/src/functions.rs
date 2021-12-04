use std::convert::{TryFrom, TryInto};

pub struct Carton{
    lineas: Vec<Vec<i32>>
}

impl Carton{
    pub fn new() -> Self {
        Self { lineas: Vec::new() }
    }

    pub fn push(&mut self, linea: Vec<i32>){
        self.lineas.push(linea);
    }

    pub fn mark(&mut self, number: i32) -> bool{
        let mut marked = false;
        for i in 0..self.lineas.len(){
            for j in 0..self.lineas[i].len(){
                if self.lineas[i][j] == number{
                    self.lineas[i][j] = -1;
                    marked = true;
                }
            }
        }
        marked
    }

    pub fn check(&mut self) -> bool{
        for i in 0..5{
            let mut suma_fila = 0;
            let mut suma_columna = 0;
            for j in 0..5{
                suma_fila += self.lineas[i][j];
                suma_columna += self.lineas[j][i];
            }
            println!("Sumas. Fila: {}, Columna: {}", suma_fila, suma_columna);
            if suma_fila == -5 || suma_columna == -5 {
                return true;
            }
        }
        false
    }

    pub fn resultado(&mut self) -> i32{
        let mut suma = 0;
        for i in 0..5 {
            for j in 0..5 {
                if self.lineas[i][j] != -1{
                    suma += self.lineas[i][j];
                }
            }
        }
        suma
    }


    pub fn print(&mut self){
        for i in 0..self.lineas.len(){
            print_linea(&self.lineas[i]);
        }
    }
}

pub fn part1(data: &str) -> i32{
    let mut resultado = 0;
    let numbers = read_numbers(data);
    let mut cartones = read_cartones(data);
    'outer: for number in numbers{
        println!("==== Numero {} ====", number);
        for i in 0..cartones.len(){
            println!("==== Carton {} ====", i);
            cartones[i].mark(number);
            cartones[i].print();
            if cartones[i].check(){
                println!("======= alto =========");
                resultado = cartones[i].resultado() * number;
                break 'outer;
            }
        }
    }
    resultado
}

pub fn part2(data: &str) -> i32 {
    let numbers = read_numbers(data);
    let mut cartones = read_cartones(data);
    let mut cartones_marcados = 0;
    for number in numbers{
        println!("==== Numero {} ====", number);
        for i in 0..cartones.len(){
            if !cartones[i].check(){
                cartones[i].mark(number);
                if cartones[i].check(){
                    println!("==== Carton {} ====", i);
                    cartones[1].print();
                    cartones_marcados += 1;
                    if cartones_marcados == cartones.len(){
                        return cartones[i].resultado() * number;
                    }
                }
            }
        }
    }
    -1
}

fn print_linea(vector: &Vec<i32>){
    for item in vector.iter(){
        print!("{} ", item);
    }
    println!();
}

pub fn read_numbers(data: &str) -> Vec<i32>{
    let first_line = data.lines().next().unwrap();
    let text_numbers = first_line.split(',');
    let mut result: Vec<i32> = Vec::new();
    for item in text_numbers{
        result.push(item.parse().unwrap())
    }
    print_linea(&result);
    result
}

pub fn read_cartones(data: &str) -> Vec<Carton>{
    let mut resultado: Vec<Carton> = Vec::new();
    for i in (2..data.lines().count()).step_by(6){
        let mut carton: Carton = Carton::new();
        for nol in 0..5{
            let mut linea: Vec<i32> = Vec::new();
            let items = data.lines().nth(i + nol).unwrap().split(' ');
            for item in items{
                if !item.is_empty() && item != " "{
                    linea.push(item.parse().unwrap())
                }
            }
            print!("{} -> ", i);
            print_linea(&linea);
            carton.push(linea);
        }
        resultado.push(carton);
    }
    resultado
}
