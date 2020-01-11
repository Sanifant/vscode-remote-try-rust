/*--------------------------------------------------------------------------------------------------------------
 * Copyright (c) Microsoft Corporation. All rights reserved.
 * Licensed under the MIT License. See https://go.microsoft.com/fwlink/?linkid=2090316 for license information.
 *-------------------------------------------------------------------------------------------------------------*/

fn main() {
    let name = "VS Code Remote - Containers";
    println!("Hello, {}!", name);
    fizzbuzz_to(100);
}

fn fizzbuzz_to(n: u32) {
    for n in 1..n {
        fizzbuzz(n);
    }
}

fn fizzbuzz(n:u32) -> () {
    if is_dividable(n, 15) {
        println!("fizz-buzz");
    } else if is_dividable(n, 3) {
        println!("fizz");
    } else if is_dividable(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

///test comment
fn is_dividable(lhs:u32, rhs:u32)->bool{
    if rhs == 0{
        panic!("Divide by zero is not allowed");
    }
    
    lhs % rhs == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dividable_true(){
        assert_eq!(is_dividable(2, 2), true);
    }

    #[test]
    fn test_dividable_false(){
        assert_eq!(is_dividable(2, 3), false);
    }

    #[test]
    #[should_panic]
    fn test_dividable_zero(){
        is_dividable(2, 0);
    }
}