fn main(){
    // let mut u:f64 = 3.14;
    // let v = &mut u;
    // *v = 3.15;
    // let u = 3.14f64;
    // let v = &u;
    // println!("{}", v);
    let mut m = 7;
    add_three_to_magic(&mut m);
    println!("{}", m);
}

fn add_three_to_magic(num: &mut i32){
    *num += 3;
}