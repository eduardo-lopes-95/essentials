fn main() {
    /*
    let n = 42u32;
    let n2 = n; // a copy of the value from n to n2
    println!("The value of n2 is {}, the same as n", n2);
    let p = life(n);
    println!("p is: {}", p); // p is: 42
    println!("{}", m); // error: unresolved name `m`.
    println!("{}", o); // error: unresolved name `o`.    
    */
}

//Struct
struct Magician{
    //name: &str, //sem 'static o compilador alerta que é necessário indicar o lifetime
    name: &'static str, //sem 'static o compilador alerta que é necessário indicar o lifetime
    power: u32
}

// struct MagicNumber{
//     magn: &u32,
//     magn2: &u32,
//     //não compila por falta de lifetime
// }

struct MagicNumbers<'a>{
    magn: &'a u32,
    magn2: &'a u32,
    //as propriedades tem o mesmo lifetime
}

//Funções
// fn return_magician<'a>() -> &'a Magician{
//     let mag = Magician{name: "Galdalf", power: 4625};
//     &mag //err: cannot return reference to local variable `mag`
//     //Referência inválida: dangling pointer
    
// }

fn life(m: u32) -> u32 {
    let o = m;
    o
}

//lifetime <'a>
fn transform<'a>(s: &'a str) { /* ... */ }

