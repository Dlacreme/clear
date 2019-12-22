# Clear Language

Clear is a language dedicated to build modern Web Application.
See `/specs` for more details about the language
See `/docs` to know how to get started with Clear

## Development

Install `cargo-watch`:
```
$ cargo install cargo-watch
```
Run the application in dev mode
```
$ cargo watch -x 'run -- -l ./demo/clear.toml'
```

## Compiler Flow

This project is a compiler. It will transform Clear code into a binary file:
 1. Clear.toml config file passed as argument will be parsed and used to set the correct build configuration
 2. the 'target' value of Clear.toml is used to start start processing Clear code
