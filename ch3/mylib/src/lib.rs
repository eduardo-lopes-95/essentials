pub fn greet(name: &str){
    println!("Hi mighty {}, what brings you here?", name);
}

pub fn gree_both(name1: &str, name2: &str){
    greet(name1);
    greet(name2);
}

pub fn increment_power(power :i32) -> i32{
    println!("My power is going to increase:");
    power + 1
}

pub fn double (num:i32) -> i32{
    num * 2
}

#[cfg(test)]
mod tests {
    
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn arithmetic() {
        let power = 5;
        assert_eq!(power, 6);
    }

    #[test]
    #[ignore]
    fn foo() {
        assert!(2 + 3 == 6);
    }

    #[test]
    #[should_panic(expected = "assertion failed")]
    fn failing_test() {
        assert!(6 == 2 + 3);
    }

    #[test]
    fn double_test_pass() {
        assert_eq!(double(2), 4);
    }
}
