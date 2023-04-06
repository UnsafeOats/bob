# Bob (the Builder)
Small scripting language for Rust build scripts.

# Motivation
I normally use Nushell for my shell, but most people use Bash/Zsh.  This means that for simple scripts I want to run to set up the environment for a program, I either need to:
  1) Write two scripts (.nu and .sh)
  2) Omit .sh so others can use but is a hassle for me
  3) Omit .nu so I can use but is a hassle for others
  4) Write the entire process in Rust itself

Now, Rust is absolutely awesome, but for doing a lot of really simple environment setup it just seems annoying and overkill.  Now, I can write a small script and call it directly from the Rust code in my `build.rs` file.

# Syntax
```
READ <filepath> -> <variable>
WRITE <filepath> <text>
PRINT <text>
APPEND <string1> <string2> -> <variable>
```

Putting them all together into a simple script would look like:
```
WRITE test.txt "why u so ugly bruh?"
READ test.txt statement
PRINT statement
APPEND statement " because i am dawg." -> reply
PRINT reply
```

If you save the script above to a file named `example.btb`, you can then run it inside your `build.rs` file like so:
```Rust
use bob_the::Script;

fn main() {
    let script = Script::new("example.btb");
    script.run().unwrap();
}
```
# Future
I want to keep this dead-simple, uber-bare-bones, but I'd also like to add the following features:
  - FOR: for-loops
  - HOME: returns user's home directory
  - CONCAT: allow user to chain strings/variables together to build system paths

# License
MIT
