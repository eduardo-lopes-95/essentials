use std::clone::Clone;

fn main() {
    let mag = MagicNumber { value: 42 };
    let mag2 = mag; //valor 42 foi movido
    let mag3 = mag; //valor não pode ser usado pois foi movido
    let mag4 = mag.clone();

    println!("address mag {:p}", &mag);   //address mag 0xbd7ddbf738
    println!("address mag2 {:p}", &mag2); //address mag2 0xbd7ddbf740
    println!("address mag3 {:p}", &mag3); //address mag3 0xbd7ddbf748   
    println!("address mag4 {:?}", &mag4); //address mag3 0xbd7ddbf748   
    //são copias distintas pois tem diferentes endereços de memória
}
#[derive(Debug, Copy, Clone)]
struct MagicNumber {
    value: u64,
}

// impl Copy for MagicNumber {}
// impl Clone for MagicNumber{
//     fn clone(&self) -> MagicNumber {
//         *self
//     }
// }
