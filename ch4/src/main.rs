use std::str::FromStr;

fn main() {
    // let nome1 = "Merlin";
    // let nome2: &str = "Gandalf";
    // let nome3: &'static str = "Aragorn"; //string literal, armazenada diretamente no executável do programa, ou seja, uma constante global

    // let mut str1 = String::new();

    // let mut str3 = nome1.to_string();

    // let str4 = &str3;

    // let str5 = &nome1[0..=2];

    // str1.push_str(nome1);
    // str1.push_str(nome2);
    // str1.push_str(nome3);

    // let mut str6 = String::new();
    // str6.push_str("Eduardo");
    // str6.push('s');
    // println!("{}", str6);
    
    // let str7 = str6.replace("Eduardo", "Damari");
    // println!("{}", str7);

    // let str8 = str6 + &str7;
    // println!("{}", str8);

    // println!("{}", "Ru\nst");
    // println!("{}", r"Ru\nst");
    // println!("{}", r#"Ru\nst"#);
    
    // println!("{:?}", f64::from_str("3.6"));
    // let number: f64 = f64::from_str("3.7").unwrap();
    // println!("{}", number);

    // let mut empty: [i32; 10] = [0; 10];
    // println!("{:?}", empty);

    // let mut numbers: Vec<i32> = Vec::new();
    // let mut magic_numbers = vec!(7i32, 42, 47, 45, 54);
    // println!("{:?}", numbers);
    // println!("{:?}", magic_numbers);
    
    // let rgvec: Vec<i32> = (0..7).collect();
    // println!("{:?}", rgvec);
    
    // let location = "Middle-Earth";
    // let part = &location[7..12];
    // println!("{}", part);//Earth

    // let magico = "Merlin";
    // let mut chars: Vec<char> = magico.chars().collect();
    // chars.sort();
    // for c in chars.iter() {
    //     print!("{} ", c);
    // }
    
    // let v: Vec<&str> = "o magico de oz".split(" ").collect();
    // print!("{:?} ", v); //["o", "magico", "de", "oz"] 
    
    // let v: Vec<&str> = "abc1def2ghi".split(|c:char| c.is_numeric()).collect();
    // print!("{:?} ", v); //["abc", "def", "ghi"] 
 
    // let mut strength = 78;
    // let triples = |n| {3 * n};
    // strength = again(triples, strength);
    // println!("{}", strength);

    // let m: i32 = 42;
    // let print_add_move = move |s:i32| {
    //     println!("m is {}", m);
    //     m + s
    // };

    // let mut rng = 0..=7;
    // println!("> {:?}", rng.next());
    // println!("> {:?}", rng.next());
    // for elem in rng {
    //     print!("{} - ", elem);
    // }

    // let aliens = ["A", "B", "C", "D", "E"];
    // for alien in aliens.iter().rev(){
    //     println!("{}", alien);
    // }

    let mut _range = 0..100;
    // let range_vec: Vec<i32> = range.collect();
    // print!("{:?} ", range_vec);
    //A função collect() percorre todo o range, "copia" todos os elementos e colocar dentro de um container
    
    // let encontrar_42 = range.find(|n| *n >= 42);
    // print!("{:?} ", encontrar_42.unwrap());
    
    // let mut range = 0..100;
    // let range_even = range.filter(|n| *n % 2 == 0).collect::<Vec<i32>>();
    // print!("{:?} ", range_even);
    
    // let mut range_even_pow = range.filter(|n| n % 2 == 0)
    // .map(|n| n * n * n)
    // .collect::<Vec<i32>>();
    // print!("{:?} ", range_even_pow);
    
    // let sum = (0..10).fold(0, |sum, n| sum + n);
    // print!("{} ", sum);

    // struct Point<T>{
    //     x: T,
    //     y: T,
    // }

    // //let point1 = Point{x:42, y:12.2}; er - tipos diferentes
    // //let point2 = Point{x:42, y:12};   ok - tipos iguais
    // //são de tipos iguais <T> porém qualquer (i32, f32)
    
    // struct Coordination<T, U>{
    //     x: T,
    //     y: U
    // }
    
    // let coor1 = Coordination{x:42, y:12.2}; ok - tipos diferentes
    // let coor2 = Coordination{x:42, y:12};   ok - tipos iguais
    // //x é um tipo <T> qualquer
    // //y é um tipo <U> qualquer

    generic("Ola");
    // foo("Oi", 25);
}

// pub fn again< F: Fn(i32) -> i32> (f: F, s:i32) -> i32 { f(f(s))}
// //Higher order function = função que recebe uma outra função como parâmetro
// //F: Fn(i32) -> i32, F é um tipo função (Fn) que recebe i32 como parâmetro, por fim, F retorna um i32
use std::fmt::Display;
use std::ops::Add;

pub fn generic<T: Display>(var:T){
    println!("{}", var);
}

