fn play_with_types() {
    let x = 10;
    println!("My super variable x is {}", x);

    let mut z = 10;

    println!("My super variable z is {}", z);
    z += 1; 
    println!("My super variable z is {}", z);

    let y = z+x;
    println!("My super variable y is {}", y);

    let floating = 10.0/3.0;
    println!("My super variable 1/3 is {}", floating);
    println!("My super variable 1/3 (:010.5 format) is {:010.5}", floating);
    println!("My super variable 1/3 (:10.5 format) is {:10.5}", floating);
    println!("My super variable 1/3 (:010.15 format) is {:010.15}", floating);

    let floating2:f32 = 10.0/3.0;
    println!("My super variable 1/3 is {}", floating2);

    println!("\nMy x is {x} and y {y} and z {floating:02.5}");
}


fn play_with_character() {
    let finger = '\u{261D}';
    let a_letter = 'a';
    println!("My characeters are - {a_letter} and {finger}");
}


fn main() {

    play_with_types();


    // Call to a function
    play_with_character();
}
