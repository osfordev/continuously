# Continuously Backup

## Quick Start

```shell
cargo build
cargo test
cargo run --package continuously_backup_bin -- --help

cargo build --target x86_64-unknown-linux-gnu
```

## Components

```text
                          ┌──────────────────────┐                           
                          │  Continuously Backup │                           
                          │     Rust Library     │                           
                          └────▲──────▲─────▲────┘                           
                               │      │     │                                
              ┌────────────────┴────┐ │ ┌───┴─────────────────┐              
              │ Continuously Backup │ │ │ Continuously Backup │              
              │  Rust Library .so   │ │ │  Rust Library Cli   │              
              └─────▲───────────────┘ │ └─────────────▲───────┘              
                    │                 │               │                      
 ┌──────────────────┴───────┐  ┌──────┴───────┐  ┌────┴-───────────────────┐ 
 │                          │  │              │  │                         │ 
 │  Other Language Clients  │  │ Rust Clients │  │ Shell/Scripting Clients │ 
 │                          │  │              │  │                         │ 
 └──────────────────────────┘  └──────────────┘  └─────────────────────────┘ 
                                                                             
Created in https://asciiflow.com/
```

## Developer Notes

### Workstation Requirements

- [ ] [rustup](https://rust-lang.github.io/rustup/index.html)

### IDE

#### VSCode

- [Rust in Visual Studio Code](https://code.visualstudio.com/docs/languages/rust)

### Use Cargo(Rust) via Docker

```shell
alias cargo='docker run --rm --env USER="${USER}" --user "$(id -u)":"$(id -g)" --volume /etc/passwd:/etc/passwd:ro --volume /etc/group:/etc/group:ro --volume "${PWD}":/work -w /work rust:1.82.0 cargo'
```
