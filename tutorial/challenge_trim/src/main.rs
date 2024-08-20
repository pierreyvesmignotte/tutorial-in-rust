fn main() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");
    println!("Passed test 1");
    
    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");
    println!("Passed test 2");

    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");   
    println!("Passed test 3");
    
    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");
    println!("Passed test 4");
    
    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");
    println!("Passed test 5");
    
    let test6 = "";
    assert_eq!(trim_spaces(test6), "");
    println!("Passed test 6");
    
    let test7 = " 🚀 ";
    assert_eq!(trim_spaces(test7), "🚀");
    println!("Passed test 7");
    println!("Tests passed!");
}
 
/* YOUR CODE GOES HERE */


fn trim_spaces (input : &str) -> &str {
    return trim_front_spaces( trim_back_spaces(input));
}

fn trim_front_spaces (input : &str) -> &str {
    //for (index, &item) in input.chars().iter().enumerate() {
    for (index, item) in input.chars().enumerate() {
        if item != ' ' {
            println!("string is {} and index {index}", &input[index..]);
            return &input[index..];
        }
    }
    &input[0..0]
}

fn trim_back_spaces (input : &str) -> &str {
    for (index, item) in input.chars().rev().enumerate() {
        if item != ' ' {
            let end = (input.len()-index);
            println!("string is {} and index {end}", &input[..end]);
            return &input[..end];
        }
    }
    &input[0..0]
}

fn trim_all_spaces (input : &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        if c!= ' ' {
            result.push (c);
        }
    }
    result
}