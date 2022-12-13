# Simple Todo
A minimal cli-todo app built in rust

## Installation
Clone this repository:

`git@github.com:ceganwyer/simple-todo.git`

Enter the directory:

`cd simple-todo`

Build the app:

`cargo build --release`

Copy the binary to .cargo/bin:

`cp target/release/simple-todo ~/.cargo/bin`

## Usage
Run the app with the command `simple-todo`

The main menu of the app has the following options:
```
  l) List todo item - shows you all of the existing todo items
  a) Add todo item - Prompts you for a title and adds a new todo item
  d) Mark item as done - Prompts you for an id and marks that todo item as done
  r) Remove todo item - Prompts you for an id and removes that todo item
   ──────────────
  x) Exit - Exists the program
  h) Help, list all options - Shows all the commands
```
