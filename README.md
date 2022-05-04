# todo-cli

A simple command-line todo app written in Rust. It supports *add a new task*, *remove a task*, *mark a task as complete* and *lists all the tasks.*

![Rust][Rust]&nbsp;
![code size][code size]&nbsp;
[![License][License]][License file]&nbsp;

## Usage

```powershell
> .\todo-cli.exe -h
```

```powershell
todo-cli 0.1.0
A simple todo app written in Rust

USAGE:
    todo-cli.exe [OPTIONS] [SUBCOMMAND]

OPTIONS:
    -f, --file <FILE_PATH>    Sets a custom config file
    -h, --help                Print help information
    -V, --version             Print version information

SUBCOMMANDS:
    add       Add a new task
    done      Mark a task as done
    help      Print this message or the help of the given subcommand(s)
    list      List all tasks
    remove    Remove a task
```

## Installation

1. Install [Rust](https://www.rust-lang.org/tools/install) and clone the repo

    ```sh
    git clone https://github.com/ashishlamsal/todo-cli.git
    ```

2. In project directory, to add a new task, perform the following:

    ```sh
    cargo run -- add "My new task"
    ```

    > This stores todos list in default `.todos.json` file in *home* directory.

3. In order to store todos list in `mytodo.json` file, perform the following:

    ```sh
    cargo run -- -f mytodo.json add "My new task"
    ```

4. For help and to view other options, perform the following:

    ```sh
    cargo run -- -h
    ```

## Sample

![Sample Output][Sample Output] &nbsp;

## License

This project is licensed under the MIT License - see the [LICENSE][License file] file for details.

[Rust]: https://img.shields.io/badge/-Rust-000000?style=flat-square&logo=rust&logoColor=ffffff
[code size]: https://img.shields.io/github/languages/code-size/ashishlamsal/todo-cli?style=flat-square
[License]: https://img.shields.io/github/license/ashishlamsal/todo-cli?style=flat-square
[License file]: https://github.com/ashishlamsal/todo-cli/blob/main/LICENSE
[Sample Output]: ./sample/output.png "Sample Output"