#[derive(Debug)]
struct Alien{
    name: &'static str,
    health: u32,
    damage: u32,
}

#[derive(Debug)]
struct Zombie{
    name: &'static str,
    health: u32,
    damage: u32,
}

trait Monster{
    fn ctr(hlt: u32, dam:u32) -> Self;
    fn attack(&self);
    fn noise(&self) -> &'static str;
    fn attack_with_sound(&self){
        println!("The monster attacks by making an awkward sound {}", self.noise())
    }
}

impl Monster for Alien{
    fn attack(&self){
        println!("I attack! Your health lowers with {} damage points", self.damage);
    }
    
    fn ctr(hlt: u32, dam:u32) -> Alien{
        Alien {name: "Walker", health:hlt, damage: dam}
    }

    fn noise(&self) -> &'static str{
        "Ahhhhhhh"
    }
    
    fn attack_with_sound(&self){
        println!("asdalsdkmslkdmas");
    }
}

impl Monster for Zombie{
    fn attack(&self){
        println!("I bite you! Your health lowers with {} damage points", 2 * self.damage);
    }

    fn ctr(hlt: u32, dam:u32) -> Zombie{
        Zombie {name: "Walker", health:hlt, damage: dam}
    }

    fn noise(&self) -> &'static str{
        "Ahhhhhhh"
    }
    
    fn attack_with_sound(&self){
        println!("asdalsdkmslkdmas");
    }

}

fn main(){
    let zb = Zombie{name:"Nemesis", health:80, damage:100};
    let al = Alien {name:"Chubaca", health:50, damage:70};

    zb.attack_with_sound();
    al.attack_with_sound();
}