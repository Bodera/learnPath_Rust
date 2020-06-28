# The book

This will be a document with several tons of lines, that's the intention since it records my learning progress through the language official documentation, available [here](https://doc.rust-lang.org/book/foreword.html), and also the original source code [here](https://github.com/rust-lang/book/tree/master/src). Let's check Rust flavor! (*does not sounds so tasty*)

__Note__: The environment in which the code provided was tested is a Debian Linux machine.

## Summary

- [Installation](https://github.com/Bodera/learnPath_Rust/blob/Bodera-rust-dev/Rust-lang-docs/README.md#installation)
- [Updating and uninstalling Rust](https://github.com/Bodera/learnPath_Rust/blob/Bodera-rust-dev/Rust-lang-docs/README.md#updating-and-uninstalling-rust)
- [First example](https://github.com/Bodera/learnPath_Rust/blob/Bodera-rust-dev/Rust-lang-docs/README.md#first-example)
- [Code review](https://github.com/Bodera/learnPath_Rust/blob/Bodera-rust-dev/Rust-lang-docs/README.md#code-review)
- [About build system and package manager](https://github.com/Bodera/learnPath_Rust/blob/Bodera-rust-dev/Rust-lang-docs/README.md#about-build-system-and-package-manager)
- [Creating Rust projects with Cargo](https://github.com/Bodera/learnPath_Rust/blob/Bodera-rust-dev/Rust-lang-docs/README.md#creating-rust-projects-with-Cargo)
- [Coding our first Rust game](https://github.com/Bodera/learnPath_Rust/blob/Bodera-rust-dev/Rust-lang-docs/README.md#coding-our-first-Rust-game)
- [How to generate random numbers](https://github.com/Bodera/learnPath_Rust/blob/Bodera-rust-dev/Rust-lang-docs/README.md#how-to-generate-random-numbers)
- [Using Crates](https://github.com/Bodera/learnPath_Rust/blob/Bodera-rust-dev/Rust-lang-docs/README.md#using-crates)
- [Then generating a random number](https://github.com/Bodera/learnPath_Rust/blob/Bodera-rust-dev/Rust-lang-docs/README.md#then-generating-a-random-number)
- [Comparing the guess to the secret](https://github.com/Bodera/learnPath_Rust/blob/Bodera-rust-dev/Rust-lang-docs/README.md#comparing-the-guess-to-the-secret)
- [Allowing multiple guesses by looping](https://github.com/Bodera/learnPath_Rust/blob/Bodera-rust-dev/Rust-lang-docs/README.md#allowing-multiple-guesses-by-looping)
- [Handling invalid output](https://github.com/Bodera/learnPath_Rust/blob/Bodera-rust-dev/Rust-lang-docs/README.md#handling-invalid-output)
- [The final code](https://github.com/Bodera/learnPath_Rust/blob/Bodera-rust-dev/Rust-lang-docs/README.md#the-final-code)
- [Fundamental topics review](https://github.com/Bodera/learnPath_Rust/blob/Bodera-rust-dev/Rust-lang-docs/README.md#fundamental-topics-review)

### Installation

To install `rustup` tool on Linux or macOS for the latest stable version of Rust

```bash
$ curl https://sh.rustup.rs -sSf | sh
```

The installation script automatically adds Rust to your system *PATH* after your next login. But you can add it into *$PATH* manually by typing:

```bash
$ source $HOME/.cargo/env
```

Or simply add it to your *~/.bash_profile*:

```bash
$ export PATH="$HOME/.cargo/bin:$PATH"
```

To install `rustup` tool on Windows systems for the latest stable version of Rust on Windows, [click here](https://www.rust-lang.org/install.html) and follow the instructions for installing Rust. The __C++ build tools for Visual Studio 2013__ or later is an pre-requisite in order to start to play with Rust.

Now it is time to check our Rust version and confirm that we are ready to go. Open a terminal and type:

```bash
$ rustc --version
```

### Updating and uninstalling Rust

Directly from your terminal, type `rustup update` and that is all. If a new version is available the `rustup` tool will fork it into your machine.

Now if for some reason you need to uninstall Rust, just type in the terminal `rustup self uninstall`.

Rust comes with an offline copy of the documentation. By typing `rustup doc`, a new window will poup-up in your default browser, then you can read it. :thumbsup:

### First example

I really do not like that tradition of printing a message to the console when learning a new programming language, but that is just a tantrum of mine. Let's write our simple, first Rust program.

It is up to you to type the code in a code editor like Visual Studio Code or in an integrated development environment like the Eclipse Rust.

Create a empty folder, this is for organizational purpose only. All Rust programs have the `.rs` extension, remember that.

```bash
$ mkdir rust_fisrt_tutorial
$ cd rust_fisrt_tutorial
$ touch byeWorld.rs
```

Open the *byeWorld* file and write:

```rust
fn main() {
	println!("Goodbye, World!");
}
```

Save it, and compile through the terminal by typing:

```bash
> rustc byeWorld.rs
```

I will let you try and guess the output. If you face some error message do not afraid to ask for Google how to resolve the conflict.

### Code review

The `main` function is not optional, it is always the first code that runs in every executable Rust program.

We can pass parameters to our functions through the parentheses. Also we know that `main` is a function because of that parentheses and the curly brackets.

If you know PEP-8 and other styles for formating code in a good way for visualizaton, well, I'm happy to inform you that Rust has the `rustfmt`. This tool is available in Rust versions >=1.24. Explore the [project repository](https://github.com/rust-lang/rustfmt) in give it a try.

__Identation matters!__ Rust style is to indent with four spaces, not a tab.

We could use `println` instead of `println!`. The difference here is that the first one will call a function, while the second one calls for a macro (every macro can be detected by the exclamation point `!`).

`"Goodbye, World!"` is just a string, passed as an argument to our macro. Most lines of Rust code end with a semicolon.

Finally, the `rustc` is the Rust compiler tool. After compiling successfully, Rust outputs a binary executable.

To run the executable in Linux os macOS:

```bash
$ ./byeWorld
```

To run the executable in Windows:

```bash
$ .\byeWorld.exe
```

Here I will past a paragraph from the Rust 1.30 documentation. Hope you read it.

> Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed. If you give someone a .rb, .py, or .js file, they need to have a Ruby, Python, or JavaScript implementation installed (respectively). But in those languages, you only need one command to compile and run your program. Everything is a trade-off in language design.

### About build system and package manager

Like in [Node.js](https://github.com/Bodera/learnPath_JavaScript/tree/master/%5BCourse%5D%20--%20NodeBR), Rust comes up with his own build system and package manager. It's the Mocholla. :trollface:

No it is not, is the __Cargo__. There is no Mocholla, I was just amusing with you, sorry. Here is the [docs for Cargo](https://doc.rust-lang.org/cargo/commands/index.html).

Here some reasons of the importance of the Cargo for Rust. Cargo is a tool that allows Rust packages to declare their various dependencies and ensure that you’ll always get a repeatable build.

1. It introduces two metadata files with various bits of package information (yes, comparable to the `package.json` and the `package-lock.json`).
2. It fetches and builds your package’s dependencies.
3. It invokes `rustc` or another build tool with the correct parameters to build your package.
4. It introduces conventions to make working with Rust packages easier.

So for smaller programs, `rustc` is fine. But you are not a smaller person (if you are, I'm apologize myself again), and projects have to be capable to grow and scale. There is when Cargo shines. Let's check out our Cargo version, what should come up with your Rust installation, by typing into the terminal the commands below:

```bash
> cargo --version
```

### Creating Rust projects with Cargo

In this section you will learn how manage Rust projects using the Cargo build system.

Let's start by typing the commands below in the terminal.

```bash
> cargo new welcome_cargo
```

You may have an output like this:

> Created binary (application) welcome_cargo package.

The `cargo new` command is used when a new project is going to be created, we must specify a name for it so then we used `welcome_cargo` as the name of our project. Then Cargo has created a new directory with the same name of the project to store the necessary files.

We achieved some big stuff just runing that command:

1. A `Cargo.toml` file.  
2. A `src` directory with a `main.rs` Rust script inside on it.  
3. Initialized a new `Git` repository. :octocat:

Let's understand each one individually.

Cargo uses [TOML](https://github.com/toml-lang/tom)(*Tom’s Obvious, Minimal Language*) for configuration format. It's quite similar to the [YAML](https://yaml.org/spec/1.2/spec.html), it's just better, (just joking).

```toml
[package]
name = "testando"
version = "0.1.0"
authors = ["Bodera <rafaelrafa990@gmail.com>"]
edition = "2018"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

The first line, `[package]`, is a section heading that indicates that the following statements are configuring a package. As we add more information to this file, we’ll add other sections. As far as we use libraries in our project, Cargo adds that packages in the `[dependencies]` section. These dependencies are commonly referred to as *crates*.

Now take a look at the `main.rs` script file inside of the `src` folder.

```rust
fn main() {
    println!("Hello, world!");
}
```

What a masterpiece! See? A classical *Hello World!* program generated automatically. This is for you do not forget that Cargo expects to all of your source files to be stored inside the `src` folder. As a good-practice, the top-level of any project directory in Rust should only contain a README, details about the license adopted, configuration files, and utils __not  related__ to your code.

Cargo is here for help you to organize your projects. Words from the Rust official documentation:

> There’s a place for everything, and everything is in it's place.

And by moving an existing project that does not uses Cargo to a `src` folder, by creating a `Cargo.toml` file you have painless converted it to a project that now uses Cargo as build system and package manager. Cool it isn't?

You are able to change the Version Control System used in your project by specifying in the flag `--vcs` in your `cargo new <project_name>` command.

Now we are going to learn how to build and run our projects with Cargo. These commands below, most be executed once inside the project directory.

- `cargo check` - this command quickly checks your code to make sure it compiles but doesn’t produce an executable.

- `cargo build` - this command creates an executable file in *target/debug/hello_cargo*. It also creates a new file at the top level: the `Cargo.lock`. This file keeps track of the exact versions of dependencies in your project.

- `./target/debug/<binary>` - this commands runs the binary created after using the `cargo build`.

- `cargo run` - this command compile the code and then run the resulting executable all in one command.

- `cargo build --release` - this command cpmpile your code with otimizations, and creates a binary in *target/release/hello_cargo*

Congratulations, the information provided here refer to the Chapter 1 of the official Rust documentation. The next lesson will cover the Rust syntax more in deepth.

### Coding our first Rust game

This will be your first Rust project so get focused and pay attention to the fundamental concepts that gonna be provided.

The ideia is to program a guessing game. The software generates some random number and ask the player to enter a guess.

We will create a new project using __Cargo__ called *guessing_game* and then jump into the new directory created.

```bash
$ cargo new guessing_game
$ cd guessing_game
```

This new directory contains a `Cargo.toml` file and a `main.rs` program in the *src/* folder. Start by checking how the `TOML` file looks like and then compile the `Hello World` using the `run` parameter.

In the same `main.rs` file, delete every line before writting the code below:

```rust
use std::io;

fn main() {
    println!("Welcome to the Olympics Games of guessing!");

    println!("Provide your guess (integer numbers only):");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guess was: {}", guess);
}
```

Now let's cut this little program in pieces.

In order to work with input and output stream we import the `io` module which comes from the standart library `std`.

Then we declare our function `main`.

Inside our function there are a few `println!` macros that prints a String to the screen.

And here is a novelty, the `let` keyword is used when we create variables in Rust. If you read the docs you will learn that by default variables in Rust are immutable.

But if we are not very confident and prefer that our variable could be mutable? Here the keyword `mut` comes in handy.

After these keywords we must inform the name of our variable so we can refer to this memory block along our program. I decided to let the name of the variable as `guess`.

The `=` sign is used for value assignment, and our `guess` variable has the `String::new()` bound to it, a function that returns a new, empty String.

To summarize, the `let mut guess = String::new();` line has created a mutable variable that is currently bound to a new, empty instance of a String. Whew!

The `io` module stores a lot of functions and `stdin()` is one of them. So becomes like this `io::stdin()`, but if we do not have explicit declared `use std::io`, we could simple call the `stdin()` function like that:

```rust
std::io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
```

The `stdin()` function returns an instance of `std::io::Stdin`, which is a type that represents a handle to the standard input for your terminal.

And a layer below of the `stdin()` function we have the `read_line` __method__, useful to get the user input. And we can notice that we've passed an argument to it: `&mut guess`;

The job of `read_line` is to take whatever the user types into standard input and place that into a string, so it takes that string as an argument. The string argument needs to be mutable so the method can change the string’s content by adding the user input.

The `&` indicates that the `&mut guess` argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are a complex feature, and one of Rust’s major advantages is how safe and easy it is to use references.

Never forget that like variables, references are immutable by default.

This line of Rust code was break into two separated lines: that's because it’s often wise to introduce a newline and other whitespace to help break up long lines. We have called two methods, so we organized our code to be more readable.

```rust
io::stdin().read_line(&mut guess).expect("Failed to read line");
//every time you do this a web server downs
```

```rust
io::stdin().read_line(&mut guess)
    .expect("Failed to read line");
//sooooo better
```

Now focus, you have to know that `read_line` method returns a value, a *enum*, the `io::Result`. This value have his variants, they can be or `Ok` or `Err`.

The `Ok` variant indicates the operation was successful, and inside `Ok` is the successfully generated value. The `Err` variant means the operation failed, and `Err` contains information about how or why the operation failed.

The purpose of these `Result` types is to encode error-handling information. Values of the `Result` type, like values of any type, have methods defined on them.

An instance of `io::Result` has an `expect()` method that you can call.

If this instance is an `Err` value, `expect()` will cause the program to crash and display the message that you passed as an argument to `expect()`. If the read_line method returns an `Err`, it would likely be the result of an error coming from the underlying operating system.

If this instance is an `Ok` value, `expect()` will take the return value that `Ok` is holding and return just that value to you so you can use it. In this case, that value is the number of bytes in what the user entered into standard input.

If you don’t call expect, the program will compile, but you’ll get a warning similar to this:

```bash
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `std::result::Result` which must be used
  --> src/main.rs:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: #[warn(unused_must_use)] on by default
```

Rust warns that you haven’t used the `Result` value returned from `read_line`, indicating that the program hasn’t handled a possible error.

The right way to suppress the warning is to actually write error handling, but because you just want to crash this program when a problem occurs, you can use `expect()`. You’ll learn about recovering from errors as far you keep studying.

And our `println!` macros are doing something new too. We've passed the curly brackets `{}` indicating a __placeholder__. For each placeholder you have to list the variable or value that it is holding.

```rust
#![allow(unused_variables)]
fn main() {
    let x = 5;
    let y = 10;

    println!("x = {} and y = {}", x, y);
}
```

Guess what the code above will print. Piece of cake it isn't?

Use `cargo run` to compile and run the first part of our guessing game.

### How to generate random numbers

In order for our game to fulfill it's commitment, we need to secretly generate the number that the user will try to guess. Let's say that this number should be between 1 and 199, and has to be different every time the game is executed. Rust standart library does not has random functionality, but we can use [the *rand* crate](https://crates.io/crates/rand).

### Using Crates

We already studied that crates are a collection of Rust source code files. So far we've been building a *binary crate*, which means that is an executable. The `rand` crate on other hand is a *library crate*, storing code intended to be used in other programs. Okay enough it's enough, let's open the `Cargo.toml` and add a dependency to our Guessing Game project and build it by running `cargo build`.

```toml
[dependencies]
rand = "^0.7.3"
```

Cargo understands [semantic versioning](https://devhints.io/semver). The range declared to the `rand` crate version that our project supports is between is `>=0.7.3` and `<1.0.0`.

```bash
$ cargo build
    Updating crates.io index
  Downloaded rand v0.7.3
  Downloaded getrandom v0.1.14
  Downloaded rand_core v0.5.1
  Downloaded rand_chacha v0.2.1
  Downloaded libc v0.2.66
  Downloaded c2-chacha v0.2.3
  Downloaded cfg-if v0.1.10
  Downloaded ppv-lite86 v0.2.6
   Compiling libc v0.2.66
   Compiling getrandom v0.1.14
   Compiling cfg-if v0.1.10
   Compiling ppv-lite86 v0.2.6
   Compiling c2-chacha v0.2.3
   Compiling rand_core v0.5.1
   Compiling rand_chacha v0.2.1
   Compiling rand v0.7.3
   Compiling guessing_game v0.1.0 (/learnPath_Rust/Rust-lang-docs/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 26.40s
```

Notice that when we have external dependencies Cargo fetches the latest version from a registry, which is a copy of data from [Crates.io](https://crates.io/), a Rust ecosystem place where people from all around the world post their open source Rust crates.

To upate crates dependencies just run `cargo update` for minor patch updates, major patches will have to be explicitly declared in *Cargo.toml*. 

Before moving on, let's talk about the `Cargo.lock` file. When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the Cargo.lock file. When you build your project in the future, Cargo will see that the `Cargo.lock` file exists and use the versions specified there rather than doing all the work of figuring out versions again. This lets you have a reproducible build automatically.

### Then generating a random number

```rust
use std::io;
use rand::Rng;

fn main() {
    println!("Welcome to the Olympics Games of guessing!");

    let secret = rand::thread_rng().gen_range(1, 200);

    println!("The secret generated was: {}", secret);

    println!("Provide your guess (integer numbers only):");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("Your guess was: {}", guess);
}
```

The second line now has `use rand::Rng`, `Rng` trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods. Trits are a language feature that tells the Rust compiler about functionality a type must provide, but we will study this more in the future.

The `rand::thread_rng` function will give us the particular random number generator that we’re going to use: one that is local to the current thread of execution and seeded by the operating system. Then we call the `gen_range method` on the random number generator. This method is defined by the `Rng` trait that we brought into scope with the `use rand::Rng` statement. The `gen_range` method takes two numbers as arguments and generates a random number between them. It’s inclusive on the lower bound but exclusive on the upper bound, so we need to specify 1 and 200 to request a number between 1 and 199.

You won’t just know which traits to use and which methods and functions to call from a crate. Instructions for using a crate are in each crate’s documentation. Run `cargo doc --open` command to build documentation provided by all of your dependencies locally and open it in your browser (also generate a `index.html` file on your project).

And so on we are printing our secret number for debugging purposes, this line will not be present on final version of the game. Try running this program a few times and you shoul get random numbers between 1 and 199.

### Comparing the guess to the secret

```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the Olympics Games of guessing!");

    let secret = rand::thread_rng().gen_range(1, 200);

    println!("The secret generated was: {}", secret);

    println!("Provide your guess (integer numbers only):");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line.");

    let guess: u32 = guess.trim().parse()
        .expect("I've told you to input integers only.");

    println!("Your guess was: {}", guess);

    match guess.cmp(&secret) {
        Ordering::Less => println!("Guessed too small :("),
        Ordering::Greater => println!("Guessed too big xD"),
        Ordering::Equal => println!("Congrats, you win :D"),
    }
}
```

Here is another `use` statement, bringing a type called `std::cmp::Ordering` into scope from the standard library. Like `Result`, `Ordering` is another *enum*, but the variants for `Ordering` are `Less`, `Greater`, and `Equal`. These are the three outcomes that are possible when you compare two values.

The `cmp` method compares two values and can be called on anything that can be compared. It takes a reference to whatever you want to compare with: here it’s comparing the `guess` to the `secret`. Then it returns a variant of the `Ordering` enum we brought into scope with the `use` statement. We use a `match` expression to decide what to do next based on which variant of `Ordering` was returned from the call to `cmp` with the values in `guess` and `secret`.

A `match` expression is made up of *arms*. An *arm* consists of a pattern and the code that should be run if the value given to the beginning of the `match` expression fits that *arm’s* pattern. Rust takes the value given to match and looks through each *arm’s* pattern in turn. The `match` construct and patterns are powerful features in Rust that let you express a variety of situations your code might encounter and make sure that you handle them all. We will study more about this later.

Rust has a strong, static type system. However, it also has type inference. When we wrote `let mut guess = String::new()`, Rust was able to infer that guess should be a `String` and didn’t make us write the type. The `secret`, on the other hand, is a number type. A few number types can have a value between 1 and 100: `i32`, a 32-bit number; `u32`, an unsigned 32-bit number; `i64`, a 64-bit number; as well as others. Rust defaults to an `i32`, which is the type of `secret` unless you add type information elsewhere that would cause Rust to infer a different numerical type. The reason for the error is that Rust cannot compare a string and a number type.

Ultimately, we want to convert the `String` the program reads as input into a real number type so we can compare it numerically to the secret number.

We create a variable named `guess`. But wait, doesn’t the program already have a variable named `guess`? It does, but Rust allows us to *shadow* the previous value of `guess` with a new one. This feature is often used in situations in which you want to convert a value from one type to another type. *Shadowing* lets us reuse the guess variable name rather than forcing us to create two unique variables, such as `guess_str` and `guess` for example.

We bind `guess` to the expression `guess.trim().parse()`. The `guess` in the expression refers to the original `guess` that was a `String` with the input in it. The `trim` method on a `String` instance will eliminate any whitespace at the beginning and end. Although `u32` can contain only numerical characters, the user must press enter to satisfy `read_line`. When the user presses enter, a newline character is added to the string, the `trim` method eliminates `\n`.

The `parse` method on strings parses a string into some kind of number. Because this method can parse a variety of number types, we need to tell Rust the exact number type we want by using `let guess: u32`. The colon (`:`) after `guess` tells Rust we’ll annotate the variable’s type. Rust has a few built-in number types; the `u32` seen here is an unsigned, 32-bit integer. It’s a good default choice for a small positive number. Additionally, the `u32` annotation in this example program and the comparison with `secret` means that Rust will infer that `secret` should be a `u32` as well. So now the comparison will be between two values of the same type!

The call to `parse` could easily cause an error. If, for example, the string contained `A👍%`, there would be no way to convert that to a number. Because it might fail, the `parse` method returns a `Result` type, much as the `read_line` method does (if this is new to you, you must restart reading this from beginning). We’ll treat this `Result` the same way by using the `expect` method again. If `parse` returns an `Err` `Result` variant because it couldn’t create a number from the string, the `expect` call will crash the game and print the message we give it. If `parse` can successfully convert the string to a number, it will return the `Ok` variant of `Result`, and expect will return the number that we want from the `Ok` value.

Run the program a few times to verify the different behavior with different kinds of input: guess the number correctly, guess a number that is too high, and guess a number that is too low.

We have most of the game working now, but the user can make only one guess. Let’s change that by adding a loop!

### Allowing multiple guesses by looping

The `loop` keyword creates an infinite loop.

```rust
loop {
        println!("Please input your guess.");

        // --snip--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("You win!"),
        }
    }
```

Notice that the code avove is doing exactly what we told it to do: ask for another guess forever! It doesn’t seem like the user can quit!

The user could always interrupt the program by using the keyboard shortcut `Ctrl`+`C`. But there’s another way to escape this insatiable monster, if the user enters a non-number answer, the program will crash.

```rust
match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
    }
```

In the coding above, adding the `break` line after `You win!` makes the program exit the loop when the user guesses the secret number correctly. Exiting the loop also means exiting the program, because the loop is the last part of `main`.

### Handling invalid output

To further refine the game’s behavior, rather than crashing the program when the user inputs a non-number, let’s make the game ignore a non-number so the user can continue guessing. The necessary change will looks like this:

```rust
let guess: u32 = match guess.trim().parse() {
    Ok(num) => num,
    Err(_) => continue,
};
```

Switching from an `expect` call to a `match` expression is how you generally move from crashing on an error to handling the error. Remember that `parse` returns a `Result` type and `Result` is an enum that has the variants `Ok` or `Err`. We’re using a `match` expression here, as we did with the `Ordering` result of the `cmp` method.

If `parse` is able to successfully turn the string into a number, it will return an `Ok` value that contains the resulting number. That `Ok` value will match the first *arm’s* pattern, and the match expression will just return the __num__ value that `parse` produced and put inside the `Ok` value. That number will end up right where we want it in the new `guess` variable we’re creating.

If `parse` is not able to turn the string into a number, it will return an `Err` value that contains more information about the error. The `Err` value does not match the `Ok(num)` pattern in the first match *arm*, but it does match the `Err(_)` pattern in the second *arm*. The underscore, `_`, is a __catchall__ value; in this example, we’re saying we want to match all `Err` values, no matter what information they have inside them. So the program will execute the second *arm’s* code, __continue__, which tells the program to go to the next iteration of the `loop` and ask for another guess. So, effectively, the program ignores all errors that `parse` might encounter!

### The final code

```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the Olympics Games of guessing!");

    let secret = rand::thread_rng().gen_range(1, 200);

    println!("I can tell you that the secret generated is a number between 1 and 199");

    loop {
        println!("\nProvide your guess (integer numbers only):");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess was: {}", guess);

        match guess.cmp(&secret) {
            Ordering::Less => println!("Guessed too small :("),
            Ordering::Greater => println!("Guessed too big xD"),
            Ordering::Equal => {
                println!("Congrats, you win :D");
                break;
            }
        }
    }
}
```

Congratulations for having gone so far, you've built a game in Rust from scratch. This project was a hands-on way to introduce you to many new Rust concepts: `let`, `match`, methods, associated functions, the use of external crates, and more.

## Fundamental topics review

Soon!
