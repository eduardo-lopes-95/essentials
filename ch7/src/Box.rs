struct Alien{
    planet: String,
    n_tentacles: i32
}

fn main() {
    /*
    let mut a1 = Box::new(Alien{planet: "Mars".to_string(), n_tentacles:4});
    println!("a1 - tentacles: {}", a1.n_tentacles);

    let a2 = &mut a1;
    println!("a2 - planet: {}", a2.planet);
    a2.n_tentacles += 1;
    println!("a2 - tentacles: {}", a2.n_tentacles);
     */
    //After such a borrow the usual ownership rules as above hold:
    // a1 no longer has access, not even for reading
    
    let n = Box::new(42);
    
    /*
    let p = *n;
    println!("p: {}", p); //acessando diretamento o conteúdo 42
    
    let q = &*n; //(&n) - &Box<i32> = 0xa5f50ffb08, (&*n) - &i32 = 0x279137251a0
    println!("q: {:p}", q); //acessando a referência 
     */

     let mut m = n; //n = 42
     *m = 67; 
     //println!("{}", n); o valor foi movido
     println!("{}", m);
}