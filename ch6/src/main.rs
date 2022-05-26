struct Alien{
    health: u32,
    damage: u32,
}
//implementa no Alien (struct) a função new que retorna um Alien
impl Alien{
    
    //função estática
    fn new(mut h:u32, d:u32) -> Alien{
        if h > 100 {h = 100;}
        Alien {health: h, damage: d}
    }
    
    //função estática
    fn foo() -> () {
        print!("Ola")
    }

    fn warn() -> &'static str {
        "Saia do planeta"
    }

    //método real
    fn attack(&self){
        println!("I attack {}", self.damage)
    }
    
    //não há métodos sobrecarregados (mesmo nome porém parâmetros diferentes) em Rust
    fn attack_and_suffer(&mut self){
        self.health -= 10;
    }
}

fn main() {
    // let str1 = "abc";
    // println!("{}", str1.len()); jeito orientado a objetos

    let mut bork = Alien{health:100, damage:5};
    bork.attack();

    // println!("{}", Alien::warn());


    
}
