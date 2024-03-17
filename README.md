<a href="https://www.linkedin.com/in/henri-marques/">
 <img style="border-radius: 50%;" src="https://avatars.githubusercontent.com/u/37425086?v=4" width="100px;" alt=""/>
 <br />
 <sub><b>Henrique Marques</b></sub></a> <a href="https://www.linkedin.com/in/henri-marques/" title="Linkedin">🧑🏻‍💻
 </a>

Made with ❤️ by Henrique Marques

[![Twitter Badge](https://img.shields.io/badge/-@Henrimarques18-1ca0f1?style=flat-square&labelColor=1ca0f1&logo=twitter&logoColor=white&link=https://twitter.com/Henrimarques18)](https://twitter.com/Henrimarques18) [![Linkedin Badge](https://img.shields.io/badge/-Henrique_Marques-blue?style=flat-square&logo=Linkedin&logoColor=white&link=https://www.linkedin.com/in/henri-marques/)](https://www.linkedin.com/in/henri-marques/)
[![Gmail Badge](https://img.shields.io/badge/-henmarques-c14438?style=flat-square&logo=Gmail&logoColor=white&link=mailto:henmarques2009@gmail.com)](mailto:henmarques2009@gmail.com)

---

<h1 align="center">
    <a href="https://doc.rust-lang.org/book/title-page.html">🔗 Rust Exercises</a>
</h1>
<p align="center">🚀 Repo with exercises and projects developed with Rust language</p>

### 🏗 Running/Creating the project

```bash
# Create a binary project
$ cargo new -- bin {name_project}
```

```bash
# Install cargo watch
$ cargo install cargo-watch
```

```bash
# Install sqlx-cli
$ cargo install sqlx-cli
```

```bash
# Build project
$ cargo build
```

```bash
# Build and generate release
$ cargo build && cargo build --release
```

```bash
# Run default with cargo
$ cargo run
```

```bash
# Run with backtrace in case of panic!
$ RUST_BACKTRACE=1 cargo run
```

```bash
# Cargo will format the code
$ cargo fmt
```

```bash
# Run with watch (reload automatically when code is changed)
$ cargo watch -q -c -w src/ -x run
```

```bash
# Run with watch (reload automatically when code is changed) - Opt 2
$ cargo watch -x run
```

```bash
# Run docker compose (-d for not show logs)
$ docker-compose up -d
```

```bash
# Stop docker compose
$ docker-compose down
```

```bash
# Create migration with sqlx
$ sqlx migrate add -r init
```

```bash
# Apply migrate
$ sqlx migrate run
```

```bash
# Revert migrate
$ sqlx migrate revert
```

### Documentation

- [Cargo Watch](https://crates.io/crates/cargo-watch)
