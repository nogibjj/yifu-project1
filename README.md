# Individual Project #1: Information from IP Address
- Build a useful command-line tool in data engineering or machine learning engineering. 
- input one ip address, then you will get the informaton of that address

## Steps to run
- `make format` to format code
- `make lint` to lint
- `make release-arm` to build for arm which is: `cargo lambda build --release --arm64`
- `make deploy` which is this `cargo lambda deploy`

**System Programming in Rust**
![1 1-prompt-engineering](https://user-images.githubusercontent.com/58792/213335664-f459e6ac-018a-4ccf-9563-bbe6d49d72d1.png)

**AWS Microservices Guide**

<img width="738" alt="image" src="https://user-images.githubusercontent.com/77519205/217607808-a50c517a-c348-48e6-9952-85267adb0bb0.png">

<!-- markdown-toc start - Don't edit this section. Run M-x markdown-toc-refresh-toc -->

<!-- markdown-toc end -->

# Installation

`$ cargo install --git`

# Usage

It's really easy to use

`cargo run`

then you woul be asked to enter the ip address

**For example:**

```
cargo run
Enter your ip address:
98.26.57.6
98.26.57.6 - Durham (North Carolina), United States, (36.0229, -78.9464), America/New_York
```

## Compile from Source

Ensure you have a [Rust toolchain installed](https://rustup.rs). Some of the dependencies also require `gcc` to be installed.

```
$ git clone 
$ cd 
$ cargo build --release
$ sudo cp target/release/{{crate_name}} /usr/local/bin/
```

# License

This crate is licensed under

 * [MIT license](http://opensource.org/licenses/MIT)

## References
* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
* [Command Line Applications in Rust](https://rust-cli.github.io/book/index.html)
* [API of Address](https://ipapi.co/api/?ruby#introduction)
* [ipgeolocate](https://github.com/grantshandy/ipgeolocate)

[//]: # (badges)

[rustc-image]: https://img.shields.io/badge/rustc-1.53+-blue.svg
[crate-image]: https://img.shields.io/crates/v/{{project-name}}.svg
[crate-link]: https://crates.io/crates/{{project-name}}
[docs-image]: https://docs.rs/{{project-name}}/badge.svg
[docs-link]: https://docs.rs/{{project-name}}
[deps-image]: https://deps.rs/repo/github/kbknapp/{{project-name}}/status.svg
[deps-link]: https://deps.rs/repo/github/kbknapp/{{project-name}}


# Problems encountered
When trying to run make rundocker after successfully building with make build, I encountered an error that read\
`docker: Error response from daemon: failed to create shim: OCI runtime create failed: runc create failed: unable to start container process: exec: "yifu-project1": executable file not found in $PATH: unknown." and "ERRO[0001] error waiting for container: context canceled`.

To resolve this issue, I updated the package name in the **Cargo.toml** file. However, a new error appeared that read\
`yifu-project1: error while loading shared libraries: libcurl.so.4: cannot open shared object file: No such file or directory`. To fix this, I added the line `RUN apt-get update && apt-get install -y libcurl4` to my Dockerfile.
