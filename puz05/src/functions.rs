use std::convert::TryInto;

pub struct Vent{
    p1: (i32, i32),
    p2: (i32, i32),
}

impl Vent{
    pub fn new(data: &str) -> Self {
        let items = data.split(" -> ").collect::<Vec<&str>>();
        let xy1 = items[0].split(',').collect::<Vec<&str>>();
        let xy2 = items[1].split(',').collect::<Vec<&str>>();
        Self { p1: (xy1[0].parse().unwrap(), xy1[1].parse().unwrap()),
               p2: (xy2[0].parse().unwrap(), xy2[1].parse().unwrap()),
        }
    }

    pub fn is_horizontal(&mut self) -> bool {
        self.p1.0 == self.p2.0
    }

    pub fn is_vertical(&mut self) -> bool {
        self.p1.1 == self.p2.1
    }

    pub fn get_dots(&mut self) -> Vec<(i32, i32)>{
        let mut resultado = Vec::new();
        let incx = self.p2.0 - self.p1.0;
        let incy = self.p2.1 - self.p1.1;
        let inc = if incx.abs() > incy.abs() {incx.abs()} else {incy.abs()};
        let ax = match incx.cmp(&0){
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Equal => 0
        };
        let ay = match incy.cmp(&0){
            std::cmp::Ordering::Less => -1,
            std::cmp::Ordering::Greater => 1,
            std::cmp::Ordering::Equal => 0
        };
        // println!("ax: {}, ay: {}", ax, ay);
        // println!("incx: {}, incy: {}, inc: {}", incx, incy, inc);
        for i in 0..(inc + 1){
            let x = self.p1.0 + (ax * i);
            let y = self.p1.1 + (ay * i);
            resultado.push((x, y));
        }
        resultado
    }
    pub fn print(&self){
        println!("{},{} -> {},{}", self.p1.0, self.p1.1, self.p2.0, self.p2.1);
    }
}

pub fn part1(data: &str) -> i32{
    let total = 1000;
    let mut vents: Vec<Vent> = Vec::new();
    for line in data.lines(){
        let vent = Vent::new(line);
        vent.print();
        vents.push(vent);
    }
    let mut campo = vec![vec![0usize; total]; total];
    for mut vent in vents {
        if vent.is_horizontal() || vent.is_vertical(){
            // println!("====");
            // vent.print();
            for dot in vent.get_dots(){
                // print!("({},{}) ", dot.0, dot.1);
                let x:usize = dot.0.try_into().unwrap();
                let y:usize = dot.1.try_into().unwrap();
                campo[x][y] += 1;
            }
        }
    }
    let mut contador = 0;
    for y in 0..total {
        //print!("{} -> ", y);
        for x in 0..total {
            //print!("{} ", campo[x][y]);
            if campo[x][y] >= 2{
                contador += 1;
            }
        }
        println!();

    }
    contador
}

pub fn part2(data: &str) -> i32 {
    let total = 1000;
    let mut vents: Vec<Vent> = Vec::new();
    for line in data.lines(){
        let vent = Vent::new(line);
        vent.print();
        vents.push(vent);
    }
    let mut campo = vec![vec![0usize; total]; total];
    for mut vent in vents {
        // println!("====");
        // vent.print();
        for dot in vent.get_dots(){
            // print!("({},{}) ", dot.0, dot.1);
            let x:usize = dot.0.try_into().unwrap();
            let y:usize = dot.1.try_into().unwrap();
            campo[x][y] += 1;
        }
    }
    let mut contador = 0;
    for y in 0..total {
        //print!("{} -> ", y);
        for x in 0..total {
            //print!("{} ", campo[x][y]);
            if campo[x][y] >= 2{
                contador += 1;
            }
        }
        println!();

    }
    contador
}

