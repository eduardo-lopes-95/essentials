struct Alien{
    name: &'static str,
    health: u32,
    damage: u32,
}

struct AlienBuilder{
    name: &'static str,
    health: u32,
    damage: u32,
}

impl AlienBuilder{
    
    //construtor
    fn bld() -> Self{
        AlienBuilder {name: "Walker", health: 100, damage: 20}
    }

    fn name(&mut self, n: &'static str) -> &mut AlienBuilder{
        self.name = n;
        self
    }

    fn health(&mut self, h: u32) -> &mut AlienBuilder{
        self.health = h;
        self
    }
    
    fn damage(&mut self, d: u32) -> &mut AlienBuilder{
        self.damage = d;
        self
    }
    
    fn finish(&self) -> Alien{
        Alien {name: self.name, health: self.health, damage: self.damage}
    }


}

//implementa no Alien (struct) a função new que retorna um Alien
impl Alien{
    
    //construtor
   fn ctr(s: &'static str, mut h: u32, d: u32) -> Self {
       if h > 100 { h = 100 };
       Alien {name: s, health: h, damage: d}
   }

   pub fn default() -> Self {
       Alien::ctr("Walker", 100, 10)
   }

   pub fn give_health(h: u32) -> Self {
       Alien::ctr("Zombie", h, 5)
   }

    //função estática
    // fn new(mut h:u32, d:u32) -> Alien{
    //     if h > 100 {h = 100;}
    //     Alien {health: h, damage: d}
    // }
    
    //função estática
    fn foo() -> () {
        print!("Ola")
    }

    //função estática
    fn warn() -> &'static str {
        "Saia do planeta"
    }

    //método real
        //são invocados após a instanciação
        //sempre tem &self como um parametro
    fn attack(&self){
        println!("I attack {}", self.damage)
    }
    
    //não há métodos sobrecarregados (mesmo nome, parâmetros diferentes) em Rust
    fn attack_and_suffer(&mut self, damage_from_other: u32){
        println!("I attack!");
        self.health -= damage_from_other;
    }
}

fn main() {
    // let str1 = "abc";
    // println!("{}", str1.len()); jeito orientado a objetos

    // let mut bork = Alien{health:100, damage:5};
    // bork.attack();

    //Associated function
    // println!("{}", Alien::warn());

    // let al0 = Alien::ctr("Genesys", 100, 100);
    // let al1 = Alien{name: "Bork", health:100, damage: 5}; //chamando diretamente o struct
    // let al2 = Alien::ctr("Berserk", 150, 15); // ::static function, não chama a instancia
    // let al3 = Alien::default(); // ::static function, não chama a instancia
    // let al4 = Alien::give_health(75); // ::static function, não chama a instancia

    let al1 = AlienBuilder::bld()
                            .name("Bork")
                                .health(80)
                                    .damage(20)
                                        .finish();
    println!("Name = {}", al1.name);
    
}