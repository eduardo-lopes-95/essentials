fn main() {
    let n: i32 = 42;
    let m = &n;

    // println!("the address of n is {:p}", m);
    // println!("the value of n is {}", *m); //desferência ao ponteiro 
    // println!("the value of n is {}", m);
    let q = &42;
    println!("{}", square(q))
}

fn square(k: &i32) -> i32{
    k * k
}