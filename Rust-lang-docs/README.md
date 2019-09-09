## The book
This will be another README with thousands of lines. I won't break it in series. For that you can check by youserlf [the docs](https://doc.rust-lang.org/book/second-edition/index.html) and also has the source code accessible in [they GitHub](https://github.com/rust-lang/book/tree/master/second-edition/src). Let's explore a new programming language!

__Note__: All code here was tested in a Linux environment running Debian distro in his stable version. 

#### Installation
* For installing `rustup` tool on Linux or macOS for the latest stable version of Rust
```bash
> curl https://sh.rustup.rs -sSf | sh
```

The installation script automatically adds Rust to your system PATH after your next login. But you can add it into $PATH manually by typing:
```bash
> source $HOME/.cargo/env
```

Or simply add it to your *~/.bash_profile*:
```bash
> export PATH="$HOME/.cargo/bin:$PATH"
```

* For installing `rustup` tool on Windows systems for the latest stable version of Rust
On Windows, [click here](https://www.rust-lang.org/install.html) and follow the instructions for installing Rust. Note that the C++ build tools for Visual Studio 2013 or later is an pre-requisite in order to start to play with Rust.

* Checking if you've done it rigth
Now it is time to check our Rust version and confirm that we are ready to go. Open a terminal and type:
```bash
> rustc --version
```

#### How to: update or uninstall Rust
Directly from your terminal, type `rustup update`. And that is all, if a new version is available the rustup tool will fork it to your machine.

Now if for some reason you need to uninstall Rust, just type in the terminal `rustup self uninstall`.

Also Rust comes with an offline copy of the documentation. By typing `rustup doc`, a new window will poup-up in your default browser, then you can read it. :thumbsup:

#### Goodbye, World!
I really do not like that tradition of printing a message to the console when learning a new programming language, but that is just a tantrum of mine. Let's write our simple, first rust program. 

Now, you can prefer to write syntax with a code editor like Visual Studio Code or with the support of an integrated development environment like the Eclipse Rust.

Create a empty folder, this is for organizational purpose only. All Rust programs have the *.rs* extension, please remember that.

```bash
fn main() {
	println!("Goodbye, World!");
}
```

Save it, and compile it by typing in a terminal window:
```bash
> rustc <filename>.rs
```

I will let you try and guess the output. If you face some error message do not afraid to ask for Google how to resolve the conflict.

#### Cool. But what in tarnation did you just have done here?
Okay, the `main` function is not optional, it is always the first code that runs in every executable Rust program.

We can pass parameters to our functions inside the parentheses. Also we now that `main` is a function because of that parentheses and the curly brackets.

If you know PEP-8 and other styles for formating code in a good way for visualizaton, well, I'm happy to inform you that Rust has the `rustfmt`. This tool is available in Rust versions >=1.24. Explore the [project repository](https://github.com/rust-lang/rustfmt) in give it a try.

__Identation matters!__ Rust style is to indent with four spaces, not a tab.

We could use `println` instead of `println!`. The difference here is that the first one will call a function, while the second one calls for a macro (every macro can be detected by the exclamation point `!`).

`"Goodbye, World!"` is just a string, passed as an argument to our macro. Most lines of Rust code end with a semicolon.

Finally, the `rustc` is the Rust compiler tool. After compiling successfully, Rust outputs a binary executable.

To run the executable in Linux os macOS:
```bash
> ./<filename>
```

To run the executable in Windows:
```bash
> .\<filename>.exe
```

Here I will past a paragraph from the Rust 1.30 documentation. Hope you read it.

> Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed. If you give someone a .rb, .py, or .js file, they need to have a Ruby, Python, or JavaScript implementation installed (respectively). But in those languages, you only need one command to compile and run your program. Everything is a trade-off in language design.

#### The build system and package manager (yes, comparable to npm)
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

#### Creating Rust projects with Cargo

Hi, in this section you will learn how manage Rust projects using the Cargo build system.

Let's start by typing the commands below in the terminal.

```bash
> cargo new welcome_cargo
```

You may have an output like this: 

> <span style="color:green;">Created</span> binary (application) `welcome_cargo` package

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

* `cargo check` - this command quickly checks your code to make sure it compiles but doesn’t produce an executable.

* `cargo build` - this command creates an executable file in *target/debug/hello_cargo*. It also creates a new file at the top level: the `Cargo.lock`. This file keeps track of the exact versions of dependencies in your project.

* `./target/debug/<binary>` - this commands runs the binary created after using the `cargo build`.

* `cargo run` - this command compile the code and then run the resulting executable all in one command.

* `cargo build --release` - this command cpmpile your code with otimizations, and creates a binary in *target/release/hello_cargo*

Congratulations, the information provided here refer to the Chapter 1 of the official Rust documentation. The next lesson will cover the Rust syntax more in deepth.