use std::rc::Rc;

#[derive(Debug)]
struct Alien {
    name: String,
    n_tentacles: u8,
}

#[derive(Debug)]
struct Tentacle {
    poison: u8,
    owner: Rc<Alien>,
}

fn main() {
    let d = Alien {
        name: "Dharkalen".to_string(),
        n_tentacles: 7,
    };
    
    let d_master = Rc::new(d);
    for i in 0u8..d_master.n_tentacles {
        let t = Tentacle {
            poison: i * 3,
            owner: d_master.clone(),
        };
        println!("{:?}", t);
    }
}
