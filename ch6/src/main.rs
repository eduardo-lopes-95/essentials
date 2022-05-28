use std::f32::consts;
// <'a> = é uma variáve lifetime dentro do escopo da função foo, leia "lifetime a"
// i: &'a i32 = parametro i é uma referencia a uma i32 com lifetime a

fn foo<'a, 'b>(i: &'a i32, j: &'b i32 ) -> i32 {
    *i + *j
}

//Lifetime elision
fn goo(i: &i32, j: &i32 ) -> i32 {
    i + j
}

//functions
fn sqroot(r:f32) -> Result<f32, String>{
    if r < 0.0 {
        return Err("Number cannot be negative!".to_string());
    }
    Ok(f32::sqrt(r))
}

//generic function
// fn sqrootT<T:Float>(r:T) -> Result<T, String> where T:Float{
//     if r < num::zero() {
//         return Err("Number cannot be negative!".to_string());
//     }
//     Ok(Float::sqrt(r))
// }

fn main(){

}