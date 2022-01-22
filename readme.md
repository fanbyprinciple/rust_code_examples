# Learning rust

Install rust from : `https://www.rust-lang.org/tools/install`

1. Building an app with rust
TODO-CLI

https://www.freecodecamp.org/news/how-to-build-a-to-do-app-with-rust/

```
cargo new todo-cli
```

To run a rust command

```
cargo run
```

```cmd
(ash) PS D:\codeplay\rust_code_examples\todo-cli> cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target\debug\todo-cli.exe`
Hello, world!
```

Our goal is to run an app on CLI.

```cmd
(ash) PS D:\codeplay\rust_code_examples\todo-cli> cargo run -- hello world!
    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target\debug\todo-cli.exe hello world!`
"hello", "world!"
```