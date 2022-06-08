struct Alien {
    planet: String,
    n_tentacles: u32,
}

fn main() {
    /*
    let a = 12;
    let mut b = &a; //b pega emprestado 12 de a
    let c = &a; //c pega emprestado o valor representado por a
    println!("a: {}", a);
    println!("b: {}", b); //aponta para a referência 0xaf8f6ffac4
    println!("c: {}", c); //aponta para a referência 0xaf8f6ffac4

    let a = 12;
    let mut b = &a; //referência mutavel
    let c = &a; //referência não mutavel
    println!("b:{} e c:{}", b, c); // 12 e 12
    b = &23;
    println!("b:{} e c:{}", b, c); // 23 e 12
    */

    /* teste
    let mut a = 12;
    let b = &a;
    let c = &mut a;
    *c += 1;
    dbg!(c);
    dbg!(a);
     */

    // let mut a = 12;
    // let b = &mut a;
    // println!("b = {}", b);
    // *b += 1;
    // println!("b: {}", b);
    // println!("a: {}", a);

    //borrowing immutable
    /*
    let a = 12;
    let b = &a;
    let c = &a;
    println!("a: {}, b: {} e c: {}", a,b,c);
     */

    //borrowing mutable
    // let mut a = 12;
    // println!("{}", a);
    // let b = &mut a;
    // *b += 1;
    // println!("{}", b);
    // println!("{}", a);

    let mut klaatu = Alien{planet:"Venus".to_string(),
    n_tentacles:15 };

    let k12 = klaatu; //a titularidade do conteúdo de klaatu foi movida para k12
    println!("{}", klaatu.planet); //value borrowed here after move

    let k12 = &klaatu;
    println!("{}", klaatu.planet);
}