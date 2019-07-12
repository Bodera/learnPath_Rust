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

#### How to: update and/or uninstall Rust
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
Like in [Node.js](https://github.com/Bodera/learnPath_JavaScript/tree/master/%5BCourse%5D%20--%20NodeBR), Rust comes up with his own build system and package manager. It's the Mocholla.

No it is not, is the __Cargo__. There is no Mocholla, I was just amusing with you, sorry. Here is the [docs for Cargo](https://doc.rust-lang.org/cargo/commands/index.html).

Here some reasons of the importance of the Cargo for Rust. Cargo is a tool that allows Rust packages to declare their various dependencies and ensure that you’ll always get a repeatable build. 

1. It introduces two metadata files with various bits of package information (yes, comparable to the `package.json` and the `package-lock.json`).
2. It fetches and builds your package’s dependencies.
3. It invokes `rustc` or another build tool with the correct parameters to build your package.
4. It introduces conventions to make working with Rust packages easier.

So for smaller programs, `rustc` is fine. But you are not a smaller person (if you are, I'm apologize myself again), and projects have to be capable to grow and scale. There is when Cargo shines. Let's check out our Cargo version, what should come up with your Rust installation, by typing into the terminal the commands bellow:

```bash
> cargo --version
```

#### Creating Rust projects with Cargo
(work in progress.)

