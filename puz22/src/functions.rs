use log::max_level;
use regex::Regex;
use multiarray::*;
use std::{convert::TryInto, ops::Mul};

pub fn part1(data: &str) -> u64{
    let re = Regex::new(r"^([^\s]*)\s*x=([-]{0,1}\d*)..([-]{0,1}\d*),y=([-]{0,1}\d*)..([-]{0,1}\d*),z=([-]{0,1}\d*)..([-]{0,1}\d*)$").unwrap();
    let mut region = Array3D::new([101, 101, 101], false);
    for line in data.lines(){
        operate(&mut region, line, &re, -50, 50);
    }
    let value = get_ons(&region);
    println!("Number of cubes: {}", &value);
    value
}

pub fn part2(data: &str) -> i64{
    let mut cubes: Vec<Cube> = Vec::new();
    for line in data.lines(){
        let new_cube = Cube::from(line);
        if cubes.is_empty() {
            cubes.push(new_cube);
        }else{
            let mut to_add: Vec<Cube> = Vec::new();
            for cube in &cubes{
                let resultado = intersect(cube, &new_cube);
                if let Some(icube) = resultado{
                    to_add.push(icube);
                }
            }
            if new_cube.get_on(){
                print!("Adding: ");
                new_cube.print();
                cubes.push(new_cube);
            }
            for cube in to_add{
                cubes.push(cube);
            }
        }
    }
    let mut resultado = 0;
    for cube in cubes{
        cube.print();
        let mut valor:i64 = cube.calcule().try_into().unwrap();
        if !cube.get_on(){
            valor = - valor;
        }
        resultado += valor;
    }
    resultado
}

struct Cube{
    on: bool,
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32)
}

impl Cube{
    fn new(on: bool, x: (i32, i32), y: (i32, i32), z: (i32, i32)) -> Cube{
        Self {on, x, y, z}
    }

    fn clone(&self) -> Cube{
        Cube {on: self.on, x: self.x, y: self.y, z: self.z}
    }

    fn from(instruction: &str) -> Cube{
        let re = Regex::new(r"^([^\s]*)\s*x=([-]{0,1}\d*)..([-]{0,1}\d*),y=([-]{0,1}\d*)..([-]{0,1}\d*),z=([-]{0,1}\d*)..([-]{0,1}\d*)$").unwrap();
        let capture = re.captures(instruction).unwrap();
        let onoff = &capture[1] == "on";
        let xi:i32 = capture[2].parse().unwrap();
        let xf:i32 = capture[3].parse().unwrap();
        let yi:i32 = capture[4].parse().unwrap();
        let yf:i32 = capture[5].parse().unwrap();
        let zi:i32 = capture[6].parse().unwrap();
        let zf:i32 = capture[7].parse().unwrap();
        Cube::new(onoff, (xi, xf), (yi, yf), (zi, zf))
    }

    fn calcule(&self) -> u64{
        let deltax: u64 = (self.x.1 - self.x.0 + 1).abs().try_into().unwrap();
        let deltay: u64 = (self.y.1 - self.y.0 + 1).abs().try_into().unwrap();
        let deltaz: u64 = (self.z.1 - self.z.0 + 1).abs().try_into().unwrap();
        deltax * deltay * deltaz
    }
    fn get_on(&self) -> bool{
        self.on
    }
    fn print(&self){
        println!("{} ({}, {}), ({},{}), ({},{})", self.on, self.x.0, self.x.1,
            self.y.0, self.y.1, self.z.0, self.z.1);
    }
}
fn union(cube1: &Cube, cube2: &Cube) -> Vec<Cube>{
    let mut resultado: Vec<Cube> = vec![cube1.clone(), cube2.clone()];
    let intersect_cube = intersect(cube1, cube2);
    if let Some(mut cube) = intersect_cube{
        cube.on = false;
        resultado.push(cube);
    }
    resultado
}

fn intersect(cube1: &Cube, cube2: &Cube) -> Option<Cube>{
    // Caso 1 not intersect
    if cube1.x.0 > cube2.x.1 || cube2.x.0 > cube1.x.1 ||
            cube1.y.0 > cube2.y.1 || cube2.y.0 > cube1.y.1 ||
            cube1.z.0 > cube2.z.1 || cube2.z.0 > cube1.z.1{
        return None;
    }
    let mut icube = Cube::new(true, (0, 0), (0, 0), (0, 0));
    icube.x.0 = if cube2.x.0 >= cube1.x.0 {cube2.x.0} else {cube1.x.0};
    icube.x.1 = if cube2.x.1 <= cube1.x.1 {cube2.x.1} else {cube1.x.1};
    icube.y.0 = if cube2.y.0 >= cube1.y.0 {cube2.y.0} else {cube1.y.0};
    icube.y.1 = if cube2.y.1 <= cube1.y.1 {cube2.y.1} else {cube1.y.1};
    icube.z.0 = if cube2.z.0 >= cube1.z.0 {cube2.z.0} else {cube1.z.0};
    icube.z.1 = if cube2.z.1 <= cube1.z.1 {cube2.z.1} else {cube1.z.1};
    icube.on = if cube1.on == cube2.on {!cube2.on} else {cube2.on};
    Some(icube)
}

fn test_regex(){
    let re = Regex::new(r"^([^\s]*)\s*x=(\d*)..(\d*),y=(\d*)..(\d*),z=(\d*)..(\d*)$").unwrap();
    let sample = "on x=10511..25773,y=70842..83381,z=7646..40114";
    let capture = re.captures(sample).unwrap();
    let onoff = &capture[1];
    assert_eq!(onoff, "on");
}
#[test]
fn third_part_of_test(){
    let data = "on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507";
    let mut cubes: Vec<Cube> = Vec::new();
    for line in data.lines(){
        let new_cube = Cube::from(line);
        if cubes.is_empty() {
            cubes.push(new_cube);
        }else{
            let mut to_add: Vec<Cube> = Vec::new();
            for cube in &cubes{
                let resultado = intersect(cube, &new_cube);
                if let Some(icube) = resultado{
                    to_add.push(icube);
                }
            }
            if new_cube.get_on(){
                print!("Adding: ");
                new_cube.print();
                cubes.push(new_cube);
            }
            for cube in to_add{
                cubes.push(cube);
            }
        }
    }
    let mut resultado = 0;
    for cube in cubes{
        cube.print();
        let mut valor:i64 = cube.calcule().try_into().unwrap();
        if !cube.get_on(){
            valor = - valor;
        }
        resultado += valor;
    }
    assert_eq!(39, resultado);
}
fn second_part_of_test(){
    let data = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";
    let mut cubes: Vec<Cube> = Vec::new();
    for line in data.lines(){
        let new_cube = Cube::from(line);
        if cubes.is_empty() {
            cubes.push(new_cube);
        }else{
            let mut to_add: Vec<Cube> = Vec::new();
            for cube in &cubes{
                let resultado = intersect(cube, &new_cube);
                if let Some(icube) = resultado{
                    to_add.push(icube);
                }
            }
            if new_cube.get_on(){
                print!("Adding: ");
                new_cube.print();
                cubes.push(new_cube);
            }
            for cube in to_add{
                cubes.push(cube);
            }
        }
    }
    let mut resultado = 0;
    for cube in cubes{
        cube.print();
        let mut valor:i64 = cube.calcule().try_into().unwrap();
        if !cube.get_on(){
            valor = - valor;
        }
        resultado += valor;
    }
    assert_eq!(39, resultado);
}

fn seconf_part_test(){
    let data = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";
    let mut cubes: Vec<Cube> = Vec::new();
    for line in data.lines(){
        let cube = Cube::from(line);
        cubes.push(cube);
    }
    let cubo1 = &cubes[0];
    let cubo2 = &cubes[1];
    let cubo3 = &cubes[2];
    let cubo4 = &cubes[3];
    let cubo12 = intersect(cubo1, cubo2).unwrap();
    let cubo13 = intersect(cubo1, cubo3).unwrap();
    let cubo23 = intersect(cubo2, cubo3).unwrap();
    let cubo123 = intersect(&cubo12, cubo3).unwrap();
    /*
    let cubo14 = intersect(cubo1, cubo4).unwrap();
    //let cubo24 = intersect(cubo2, cubo4).unwrap();
    let cubo34 = intersect(cubo3, cubo4).unwrap();
    //let cubo124 = intersect(&cubo12, cubo4).unwrap();
    let cubo134 = intersect(&cubo13, cubo4).unwrap();
    //let cubo234 = intersect(&cubo23, cubo4).unwrap();
    let cubo1234 = intersect(&cubo123, cubo3).unwrap();

    //cubes[2].intersect(&cube1) + cubes[2].intersect(&cube2)
    //cucubes[2].intersect(&cube1) + cubes[2].intersect(&cube2)
    println!("Cubo1: {}", cubo1.calcule());
    println!("Cubo2: {}", cubo2.calcule());
    let mut resultado = cubo1.calcule() + cubo2.calcule() + cubo4.calcule()
        - cubo12.calcule() + (cubo13.calcule() + cubo23.calcule()) - cubo14.calcule() + cubo34.calcule()
        - cubo123.calcule() + cubo134.calcule()
        - cubo1234.calcule();
        */
    let mas = cubo1.calcule() + cubo2.calcule() - cubo12.calcule();
    let menos = cubo13.calcule() + cubo23.calcule() - cubo123.calcule();
    let resultado = mas - menos;

    assert_eq!(46, resultado);
}
fn first_test(){
    let data = "on x=10..12,y=10..12,z=10..12
on x=11..13,y=11..13,z=11..13
off x=9..11,y=9..11,z=9..11
on x=10..10,y=10..10,z=10..10";
    let re = Regex::new(r"^([^\s]*)\s*x=([-]{0,1}\d*)..([-]{0,1}\d*),y=([-]{0,1}\d*)..([-]{0,1}\d*),z=([-]{0,1}\d*)..([-]{0,1}\d*)$").unwrap();
    let mut region = Array3D::new([101, 101, 101], false);
    for line in data.lines(){
        operate(&mut region, line, &re, -50, 50);
        println!("Number of cubes: {}", get_ons(&region));
    }
    assert_eq!(39, get_ons(&region));
}

fn second_test(){
    let data = "on x=-5..47,y=-31..22,z=-19..33
on x=-44..5,y=-27..21,z=-14..35
on x=-49..-1,y=-11..42,z=-10..38
on x=-20..34,y=-40..6,z=-44..1
off x=26..39,y=40..50,z=-2..11
on x=-41..5,y=-41..6,z=-36..8
off x=-43..-33,y=-45..-28,z=7..25
on x=-33..15,y=-32..19,z=-34..11
off x=35..47,y=-46..-34,z=-11..5
on x=-14..36,y=-6..44,z=-16..29
on x=-57795..-6158,y=29564..72030,z=20435..90618
on x=36731..105352,y=-21140..28532,z=16094..90401
on x=30999..107136,y=-53464..15513,z=8553..71215
on x=13528..83982,y=-99403..-27377,z=-24141..23996
on x=-72682..-12347,y=18159..111354,z=7391..80950
on x=-1060..80757,y=-65301..-20884,z=-103788..-16709
on x=-83015..-9461,y=-72160..-8347,z=-81239..-26856
on x=-52752..22273,y=-49450..9096,z=54442..119054
on x=-29982..40483,y=-108474..-28371,z=-24328..38471
on x=-4958..62750,y=40422..118853,z=-7672..65583
on x=55694..108686,y=-43367..46958,z=-26781..48729
on x=-98497..-18186,y=-63569..3412,z=1232..88485
on x=-726..56291,y=-62629..13224,z=18033..85226
on x=-110886..-34664,y=-81338..-8658,z=8914..63723
on x=-55829..24974,y=-16897..54165,z=-121762..-28058
on x=-65152..-11147,y=22489..91432,z=-58782..1780
on x=-120100..-32970,y=-46592..27473,z=-11695..61039
on x=-18631..37533,y=-124565..-50804,z=-35667..28308
on x=-57817..18248,y=49321..117703,z=5745..55881
on x=14781..98692,y=-1341..70827,z=15753..70151
on x=-34419..55919,y=-19626..40991,z=39015..114138
on x=-60785..11593,y=-56135..2999,z=-95368..-26915
on x=-32178..58085,y=17647..101866,z=-91405..-8878
on x=-53655..12091,y=50097..105568,z=-75335..-4862
on x=-111166..-40997,y=-71714..2688,z=5609..50954
on x=-16602..70118,y=-98693..-44401,z=5197..76897
on x=16383..101554,y=4615..83635,z=-44907..18747
off x=-95822..-15171,y=-19987..48940,z=10804..104439
on x=-89813..-14614,y=16069..88491,z=-3297..45228
on x=41075..99376,y=-20427..49978,z=-52012..13762
on x=-21330..50085,y=-17944..62733,z=-112280..-30197
on x=-16478..35915,y=36008..118594,z=-7885..47086
off x=-98156..-27851,y=-49952..43171,z=-99005..-8456
off x=2032..69770,y=-71013..4824,z=7471..94418
on x=43670..120875,y=-42068..12382,z=-24787..38892
off x=37514..111226,y=-45862..25743,z=-16714..54663
off x=25699..97951,y=-30668..59918,z=-15349..69697
off x=-44271..17935,y=-9516..60759,z=49131..112598
on x=-61695..-5813,y=40978..94975,z=8655..80240
off x=-101086..-9439,y=-7088..67543,z=33935..83858
off x=18020..114017,y=-48931..32606,z=21474..89843
off x=-77139..10506,y=-89994..-18797,z=-80..59318
off x=8476..79288,y=-75520..11602,z=-96624..-24783
on x=-47488..-1262,y=24338..100707,z=16292..72967
off x=-84341..13987,y=2429..92914,z=-90671..-1318
off x=-37810..49457,y=-71013..-7894,z=-105357..-13188
off x=-27365..46395,y=31009..98017,z=15428..76570
off x=-70369..-16548,y=22648..78696,z=-1892..86821
on x=-53470..21291,y=-120233..-33476,z=-44150..38147
off x=-93533..-4276,y=-16170..68771,z=-104985..-24507";
    let re = Regex::new(r"^([^\s]*)\s*x=([-]{0,1}\d*)..([-]{0,1}\d*),y=([-]{0,1}\d*)..([-]{0,1}\d*),z=([-]{0,1}\d*)..([-]{0,1}\d*)$").unwrap();
    let mut region = Array3D::new([400000, 400000, 400000], false);
    for instruction in data.lines(){
        operate(&mut region, instruction, &re, -200000, 200000);
        //println!("Number of cubes: {}", get_ons(&region));
    }
    assert_eq!(590784, get_ons(&region));
}

fn operate(region: &mut MultiArray<bool, Dim3>, instruction: &str, re: &Regex, min: i32, max: i32){
    let capture = re.captures(instruction).unwrap();
    let onoff = &capture[1];
    let xi:i32 = capture[2].parse().unwrap();
    let xf:i32 = capture[3].parse().unwrap();
    let yi:i32 = capture[4].parse().unwrap();
    let yf:i32 = capture[5].parse().unwrap();
    let zi:i32 = capture[6].parse().unwrap();
    let zf:i32 = capture[7].parse().unwrap();
    println!("{},{} {},{} {},{}", xi, xf, yi, yf, zi, zf);
    if (xi < min && xf < min) || (xi > max && xf > max) ||
        (yi < min && yf < min) || (yi > max && yf > max) ||
        (zi < min && zf < min) || (zi > max && zf > max){
            return;
    }
    let xi = delimite(xi, min, max);
    let xf = delimite(xf, min, max);
    let yi = delimite(yi, min, max);
    let yf = delimite(yf, min, max);
    let zi = delimite(zi, min, max);
    let zf = delimite(zf, min, max);
    for x in xi..(xf + 1){
        for y in yi..(yf + 1){
            for z in zi..(zf + 1){
                region[[x, y, z]] = onoff == "on";
            }
        }
    }
}

fn get_ons(region: &MultiArray<bool, Dim3>) -> u64{
    let mut contador = 0;
    for x in 0..101{
        for y in 0..101{
            for z in 0..101{
                if region[[x, y, z]]{
                    contador += 1;
                }
            }
        }
    }
    contador
}

fn delimite(value: i32, min: i32, max: i32) -> usize{
    if value <= min{
        return 0;
    }else if value >= max{
        return (min.abs() + max).try_into().unwrap();
    }
    (min.abs() + value).try_into().unwrap()
}
