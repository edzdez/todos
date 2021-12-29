# Todos

A simple CLI app to keep track of your todos.

## Usage

todos <list, add, remove>

## Command-line Options

### Options:

* **-s <num-to-show>**, **--show <num-to-show>**: specify the number of items to show
* **-f <file-path>**, **--file <file-path>**: specify the file path for the todo-list file

## To Build:

**todos** is written in [Rust](https://www.rust-lang.org/). The Rust compiler can be installed by following the
instructions on the [official download page](https://www.rust-lang.org/tools/install).

```shell
$ git clone "https://github.com/edzdez/todos.git"
$ cd todos
$ cargo build --release
```