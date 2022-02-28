// main function is always the first code that runs in every executable Rust program
fn main(){
    // the ! makes it call a Rust macro instead of a function
    println!("Hello World");
}

// Compiling and Running are Separate Steps

// !!Compiling!!
// before running a Rust program you must compile it by entering rustc command with name of source file
// eg: $ rustc main.rs
// 2 files outputted from this: main    main.rs

// !!Running!!
// run the executable file like so:
// $ ./main

// rust is an ahead of time compiled language
// this means that you can compile a program and give the executable to someone else and they can run it
// without having Rust installed