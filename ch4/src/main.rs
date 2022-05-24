use std::str::FromStr;

fn main() {
    // let nome1 = "Merlin";
    // let nome2: &str = "Gandalf";
    // let nome3: &'static str = "Aragorn"; //string literal, armazenada diretamente no executável do programa, ou seja, uma constante global

    // let mut str1 = String::new();

    // let mut str3 = nome1.to_string();

    // let str4 = &str3;

    // let str5 = &nome1[0..=2];

    // str1.push_str(nome1);
    // str1.push_str(nome2);
    // str1.push_str(nome3);

    // let mut str6 = String::new();
    // str6.push_str("Eduardo");
    // str6.push('s');
    // println!("{}", str6);
    
    // let str7 = str6.replace("Eduardo", "Damari");
    // println!("{}", str7);

    // let str8 = str6 + &str7;
    // println!("{}", str8);

    // println!("{}", "Ru\nst");
    // println!("{}", r"Ru\nst");
    // println!("{}", r#"Ru\nst"#);
    
    println!("{:?}", f64::from_str("3.6"));
    let number: f64 = f64::from_str("3.7").unwrap();
    println!("{}", number);
}
