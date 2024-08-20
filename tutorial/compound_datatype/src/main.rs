fn main() {
    let my_array = [1,2,3,4,5,5];
    println!("The sum of 3rd and 4th charater is {} ", my_array[3]+my_array[4]);

    // Define an array
    let numbers : [i32; 5];
    numbers = [0; 5]; // Repeat command
    //let index : usize = numbers.len();
    let index  = numbers.len();
    println!("Here we go {}", numbers[index-1]);
}
