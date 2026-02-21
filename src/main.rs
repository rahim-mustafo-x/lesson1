fn main() {
    //integer-i8,i16,i32,i64
    //unsigned integer-u8,u16,u32,u64
    let x: i64 = i64::MAX;
    println!("The value of x is: {}", x);
    //floating point-f32,f64
    let pi: f64 = 3.14;
    println!("The value of pi is: {}", pi);
    //boolean-bool
    let is_rust_fun: bool = true;
    println!("Is Rust fun? {}", is_rust_fun);
    println!("Is Rust fun? {} {}", is_rust_fun, pi);
    let letter:char = 'A';
    println!("The value of letter is: {}", letter);


    //compound types
    //tuple, array, slice, strings(slice string)

    //array
    //; <-- semi colon is used to separate the type and the length of the array
    // in rust we have to specify the type of the elements in the array and the length of the array
    //you can do this by using the syntax [type; length]
    let numbers:[i32; 5] = [1,2,3,4,5];
    //its only for fixed size arrays, if you want to use dynamic size arrays you can use vectors
    println!("The value of numbers is: {:?}", numbers);
    println!("The value of numbers[0] is: {}", numbers[0]);
    println!("The value of numbers[1] is: {}", numbers[1]);
    println!("The value of numbers[2] is: {}", numbers[2]);
    println!("The value of numbers[3] is: {}", numbers[3]);
    println!("The value of numbers[4] is: {}", numbers[4]);
    //you can get the elements by doing this ^


    //we use texts in rust by using the string slice type &str or the String type
    let name:&str  = "Rust";
    println!("The value of name is: {}", name);


    //tuple
    let human = ("John", 30, true);
    let human2:(String , i32, bool) = ("John".to_string(), 30, true);
    println!("The value of human is: {:?}", human);
    println!("The value of human2 is: {:?}", human2);
    //both are good but the second one is more flexible because we can change the values of the tuple, while the first one is immutable
    let mixed_tuple = (1, "Hello", 3.14, true, [1,2,3]);
    println!("The value of mixed_tuple is: {:?}", mixed_tuple);


    //--------------------------------
    //slices
    let numbers2:&[i32] = &[1,2,3,4,5];
    println!("The value of numbers2 is: {:?}", numbers2);


    //string vs string slice
    //the strings are rulable 
    //strings [growable, mutable, awned string type]

    let string_slice: &str = "Hello, world!";
    let string_object: String = String::from("Hello, world!");
    println!("The value of string_slice is: {}", string_slice);
    println!("The value of string_object is: {}", string_object);
    //definied wrom speciul nofchic types
    //the difference between &str and String is that the String is free and does whathever it wants and on the other hand you can borrow text with &str its mainly used on the arguments of functions like here
    //on the hellotext function
    println!("The value of string_object is: {}", hellotext("Mustafo"));
}
//you can choose specific value that returns using -> String | <-this returns string when the operation is done
fn  hellotext(text: &str)-> String {
    text.to_string()
}