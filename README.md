# ids721-rust-playground

# Mini Project
# Week 2 Demo
In this week, I learned how to install rust, and how to use rust to write a simple program.
Week 2 summary:
* Create a new github repository
* Setup github action
* Setup makefile
* Download rust
* Setup Cargo.toml to add dependencies.
* Use codespace to write code to print message with a cute animal


# Week 3 Demo
In this week I wirte a rust cli tool to find the most frequent number. 
* To build: cargo build
* To run cli: cargo run -- most-freq --name "1 4 2 1"

# Week 4 Demo
TextWrap is a powerful library for word wrapping, indenting, and dedenting strings. And this week I  wirte a rust cli tool to wrapping the user input 

For example:
cargo run -- text-wrap --name "If you need a small quick library that knows how to wrap text for command line utilities, take a look at this crate."

If you need

a small quick

library that

knows how to

wrap text for

command line

utilities,

take a look at

this crate.

# Week 5 Demo
A simple rust program to send HTTP Get request using reqwest and tokio

# Week 6 Demo
A Rust program that prints a random Chuck Norris joke to the console. This program uses the reqwest library to make an HTTP GET request to the Chuck Norris joke API, which returns a random joke in JSON format. It then deserializes the JSON response into a Joke struct and prints the joke to the console

# Week 7 Demo
A Rust program that generates a random password using the rand crate. It creates a new ThreadRng random number generator and uses the sample_iter() method to generate an iterator of random alphanumeric characters. It then uses the take() method to limit the iterator to 16 characters and the map() method to convert the characters to a String. 

Your new password is: 0KdLEID7a6HcFg50

# Week 8 Demo
A Rust program that calculates the factorial of a number using recursion. This program prompts the user to enter a positive integer, reads the input from the console, and calculates the factorial of the number.

Please enter a positive integer:
4
The factorial of 4 is 24


# Week 9 Demo
A Rust program that generates a QR code for a given input string and saves it to a file using the qrcode crate

Please enter the text to encode:
hello world!
QR code saved to qrcode.txt

```rust
███████   █   █   █   █   ███████
█     █  █   █   █   █    █     █
█ ███ █ █  █   █   █   █  █ ███ █
█ ███ █  █  ██  ██  ██  █ █ ███ █
█ ███ █  █  ██  ██  ██  █ █ ███ █
█     █   ███ ███ ███ ███ █     █
███████ █ █ █ █ █ █ █ █ █ ███████
        █  █   █   █   █         
███ █████   █   █   █   ███   █  
 ██  █ █ █ ███ ███ ███ ███  █ ███
█ █   █   ███ ███ ███ ███ █  █ ██
██ █    ███ ███ ███ ███ █   ██ █ 
  ███ ███ ██  ██  ██  ██  ███  █ 
   ██  █ ███  ██  ██  ██   █ █ ██
███ ███  █   █   █   █   █ ██ ███
 █   █   ███   █   █   █ ███   █ 
█  █ ██ █ █ █   █   █   ██  ██ █ 
█  ███ █  ████ ███ ███ ███  █  ██
     ██ █ ███ ███ ███ ███ █  █ ██
   █ █ █ █  ███ ███ ███ █   █    
     ███   █  ██  ██  ██  █ █  ██
  █    ██ ██  ██  ██  ██   █  ███
█   ███████  █   █   █   █ ███ ██
 ██  █  ██ █   █   █   █ ███   █ 
█ █ █ █ ███ █   █   █   ██████   
        ██ ███ ███ ███ ██   ██ ██
███████ █ ███ ███ ███ █ █ █ █  ██
█     █ ██  ███ ███ ███ █   █  █ 
█ ███ █ ██ █  ██  ██  ███████  █ 
█ ███ █   ██  ██  ██  ███ ███ ██ 
█ ███ █ █ █  █   █   █ █  █ █ █ █
█     █ █ ██   █   █     █ █   █ 
███████ █ █ █   █   █   █████  ██
```

# week10 demo
A Rust generates a random maze using the Depth-First Search algorithm and prints it to the console.
██████████████████████████████████████████
██                                      ██
██  ██████████████████████████████████████
██                                      ██
██  ██████████████████████████████████████
██                                      ██
██  ██  ██  ██████████████████████████████
██  ██  ██                              ██
██  ██  ██  ██████████████████████████████
██  ██  ██                              ██
██████████████████████████████████████████