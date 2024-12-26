<h1 align="center">
  <code>pinocchio-log-macro</code>
</h1>
<p align="center">
 <img width="350" alt="pinocchio-log-macro" src="https://github.com/user-attachments/assets/9b100f7c-216d-4849-b27d-3436f88af1bf"/>
</p>
<p align="center">
 Companion <code>log!</code> macro for <a href="https://crates.io/crates/pinocchio-log"><code>pinocchio-log</code></a>.
</p>
<p align="center">
  <a href="https://crates.io/crates/pinocchio-log-macro"><img src="https://img.shields.io/crates/v/pinocchio-log-macro?logo=rust" /></a>
</p>

## Overview

The macro automates the creation of a `Logger` object to log a message. It support a subset of the [`format!`](https://doc.rust-lang.org/std/fmt/) syntax. The macro parses the format string at compile time and generates the calls to a `Logger` object to generate the corresponding formatted message.

## Usage

The macro works very similar to `solana-program` [`msg!`](https://docs.rs/solana-program/latest/solana_program/macro.msg.html) macro.

To output a simple message (static `&str`):
```rust
use pinocchio_log::log

log!("a simple log");
```

To ouput a formatted message:
```rust
use pinocchio_log::log

let amount = 1_000_000_000;
log!("transfer amount: {}", amount);
```

Since a `Logger` size is statically determined, messages are limited to `200` length by default. When logging larger messages, it is possible to increase the logger buffer size:
```rust
use pinocchio_log::log

let very_long_message = "...";
log!(500, "message: {}", very_long_message);
```

It is possible to include a precision formatting for numeric values:
```rust
use pinocchio_log::log

let lamports = 1_000_000_000;
log!("transfer amount (SOL: {:.9}", lamports);
```

For `&str` types, it is possible to specify a maximim length and a truncation strategy:
```rust
use pinocchio_log::log

let program_name = "pinocchio-program";
// log message: "...program"
log!("{:<.10}", program_name); 
// log message: "pinocchio-..."
log!("{:>.10}", program_name); 
```

## License

The code is licensed under the [Apache License Version 2.0](LICENSE)
