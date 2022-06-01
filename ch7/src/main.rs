struct Alien{
    planet: String,
    n_tentacles: u32,
}
fn main(){
    let mut klaatu = Alien{planet:"Venus".to_string(),
        n_tentacles:15 };
    
    //let k12 = klaatu; //ownership saiu de klaatu para k12
    //println!("{}", klaatu.planet); //value borrowed here after move
    
    let k12 = &klaatu;
    println!("{}", klaatu.planet);
}