static MAX_HEALTH: i32 = 100;
static GAME_NAME: &str = "Monster Attack";

pub fn foo() -> u32{
    let a = 1;
    let b = 2;
    //a + b //return a + b;
    return a + b;
}

fn main() {
    // let _a = 11usize;
    // let _b = 10isize;
    // let _c = 10i8;
    // let _d = 3.14f32;
    // let _e = 3.14f32;
    // let f = 10_u8;
    // let _x = ();
    // println!("{}", _a);

    //shadowing / code block
    // let mut outer1 = 42u8;
    // {
    //     //code block
    //     let inner = 3.14f32;
    //     println!("{}", inner);
    //     let outer2 = 99u8;
    //     println!("outer {}", outer2);
    //     let borrow = outer1;
    // }
    // println!("inner {}", inner);
    // println!("outer {}", outer1);

    //to_string
    // let p1 = "Rob";
    // let p2 = "Jane";
    // //let p12 = p1 + p2; não é possível
    // let _p12 = p1.to_string() + p2;
    // let p3 = format!("{} {}", p1, p2);
    // println!("{}", p3);

    // //casting
    // let points = 10i32;
    // let mut saved_points: u32 = 0;
    // //saved_points = points; mismatched types -> incompativeis
    // saved_points = points as u32; 
    
    //expression
    // let n1 = {
    //     let a = 2;
    //     let b = 5;
    //     a + b
    // };
    // println!("{}", n1);
    // let _n2 = {
    //     let a = 2;
    //     let b = 5;
    //     a + b;
    // };

    // let game = "Space";
    // let game2 = &game;
    // println!("{:p}", game); //{:p} p = pointer formating
    // // println!("{}", *game2);
    // // println!("{}", game2); 
    // dbg!("{}", *game2);
    // dbg!("{}", game2);

    let x = Box::new(5_i32);
    dbg!("{}", x);
}
