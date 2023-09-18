## Rust Template Manager

Rust Template Manager (rtm) is a simple CLI program used to create/copy/delete files from your system's default template folder.

Rtm works by searching for your system's default template folder, if your platforms doesn't support such directory (like MacOS) or it just doesn't exist for any reason,
rtm will fallback to creating a folder named "rtm" inside your default config directory, in which all the templates files will live in.

**Platforms:**
- Linux (WSL and Native) using $VISUAL and $EDITOR env vars.
- Windows using the `start` command
- MacOS using the `open` command

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
Create a file inside your template folder:
```shell
rtm create <file_name>
```
Delete one or more files inside your template folder:
```shell
rtm delete <file_name>
```
Edit an existing file inside your template folder:
```shell
rtm edit <file_name>
```

### Editing files behaviour
On Linux rtm searches for the environmental variables `$VISUAL` and `$EDITOR`, it tries both variables.

On Windows it simply invokes the `start` command which takes care of opening a program based on the file extension,
or automatically asks you to provide the software you want to open the file with.

On MacOS it uses the `open` command that handles automatically opening files with your prefered text editor (same as Windows).

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
### Tests
Environment tests are provided inside the `tests/` folder, it contains a tiny python script
that's going to test `rtm` commands on a real system (Only available for Linux).