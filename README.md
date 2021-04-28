### Rust Template Manager

Rust Template Manager (rtm) is a simple CLI program used to create/copy/delete files from your system default template folder.

It is meant to be minimal and straightforward so it only works on systems with greatly defined default template folders.

### Syntax
Print your default template folder path:
```shell
rtm folder
```
Listing files inside your template folder:
```shell
rtm list
```
Copy a file from your template folder, the name must be compatible.
```shell
rtm copy <file_name>
```
Create a file inside your default template folder:
```shell
rtm create <file_name>
```
Delete a file inside your default template folder:
```shell
rtm delete <file_name>
```

### Installing
#### Build from source
You'll need the rust programming language and it's tools, the easiest way is to install [rustup](https://www.rust-lang.org/tools/install).

When all tools are installed you can follow:
```shell
# Clone the repository.
git clone https://gitlab.com/bluesden/rust-template-manager

# Go to the program folder.
cd rust-template-manager

# Build with cargo.
cargo build --release

# Now you can move the 'rtm' binary that lies inside 'target/release' to your prefered location
cd target/release
mv rtm $HOME/.local/bin

# or
sudo mv rtm /usr/bin
```