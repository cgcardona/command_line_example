# Command Line Example

This is a basic command line example in [Rust](https://www.rust-lang.org) using [Clap](https://clap.rs).

## Installation

Clone the repo

```
git clone https://github.com/cgcardona/command_line_example.git
```

Then `cd` in to the directory

```
cd command_line_example
```

Next build the app

```
cargo build
```

## Things to notice

- [Define flags and options in yml](./src/config.yml)
- [Parse command line flags and options](https://github.com/cgcardona/command_line_example/blob/master/src/configuration.rs#L9)

## Usage

```
./target/debug/command_line_example
Ok(
    Configuration {
        amazing: false,
        epic: 11,
    },
)
```

```
./target/debug/command_line_example --amazing
Ok(
    Configuration {
        amazing: true,
        epic: 11,
    },
)
```

```
./target/debug/command_line_example -a
Ok(
    Configuration {
        amazing: true,
        epic: 11,
    },
)
```

```
./target/debug/command_line_example --epic 12
Ok(
    Configuration {
        amazing: false,
        epic: 12,
    },
)
```

```
./target/debug/command_line_example -e 12
Ok(
    Configuration {
        amazing: false,
        epic: 12,
    },
)
```
