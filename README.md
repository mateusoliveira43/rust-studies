# Rust studies :pencil:

[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)

Initial Rust studies.

## About

TODO

## Running

To follow this section, we will use [Docky](https://github.com/mateusoliveira43/docky) and [Rust Official Docker Image](https://hub.docker.com/_/rust).

### Requirements

To run the commands presented in the following sections, the following tools are necessary:

- [Python](https://wiki.python.org/moin/BeginnersGuide/Download) 3.7 or higher
- [Docker](https://docs.docker.com/get-docker/)
- [Docker Compose](https://docs.docker.com/compose/install/)

### setup

To connect to the Container's shell, run
```
./scripts/docky.py run
```
To exit the Container's shell, run `CTRL+D` or `exit`.

To clean up the workspace, run
```
./scripts/docky.py down
```

**Run the commands of the following sections in the Container shell.**

### 1

To change to the folder of the section, run
```
cd learn_rust/1/
```

To see the Rust compiler version, run
```
rustc --version
```

To compile a Rust file, run
```
rustc hello.rs
```

To run the compiled file, run
```
./hello
```

To see the Rust package manager version, run
```
cargo --version
```

To create a new Rust package, run
```
cargo new project_name
```

To change to the folder of the package, run
```
cd project_name
```

To run the package, run
```
cargo run
```

To install Rust formatter, run
```
rustup component add rustfmt
```

To check the Rust code format, run
```
cargo fmt -v --check
```

To format Rust code, run
```
cargo fmt
```

## References :books:

- [@rochacbruno's Learn Rust (PT-BR)](https://www.youtube.com/playlist?list=PLjSf4DcGBdiGCNOrCoFgtj0KrUq1MRUME)
- https://doc.rust-lang.org/book/title-page.html
