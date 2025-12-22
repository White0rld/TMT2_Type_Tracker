# Too Many Types 2 Type Tracker

## What is Too Many Types 2

Too Many Types 2 is a mod for Pokemon Emerald developed by Too Many Productions.

To put it shortly, this mod adds a lot of new types, to the point where tracking them in a spreadsheet or text file is unreasonable, and more importantly unreadable.

This [link](https://www.hackdex.app/hack/too-many-types-2) will explain the mod more in detail and how to play it yourself.

## What does this program do?

This program, as the name states, tracks these new types for you.

You can :
- [x] Add new types
- [x] Remove existing types (in case you make a typo)
- [x] Add a new weakness/resistance
- [ ] See what a type is weak to/resists
- [ ] See what multiple types are weak to/resists

Currently, this program runs as a CLI (Command Line Interface), but I may change this at some point to a GUI (Graphical User Interface).

## How to run the program

As of now, you need cargo to run this program. You can simply enter this command to launch the program with the default settings :
```bash
cargo run
```
The program stores the type chart in a csv file. This is by default the "examples/types.csv" file, but you can change it using :
```bash
cargo run -- -f filename
```
