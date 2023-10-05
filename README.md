## Rust Template Manager

Rust Template Manager (rtm) is a simple CLI program used to create/copy/delete files from your system's default template folder.

Rtm works by searching for your system's default template folder, if your platforms doesn't support such directory (like MacOS) or it just doesn't exist for any reason,
rtm will fallback to creating a folder named "rtm" inside your default config directory, in which all the templates files will live in.

**Platforms:**
- Linux: Opens your text editor using `xdg-open` command or use $EDITOR and $VISUAL env vars.
- Windows: Opens your editor the `start` command.
- MacOS: Opens your text editor using the `open` command or use $EDITOR and $VISUAL env vars

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
rtm create <file_name> <flags>
```
Delete one or more files inside your template folder:
```shell
rtm delete <file_name>
```
Edit an existing file inside your template folder:
```shell
rtm edit <file_name> <flags>
```

### Flags
- `ne` - stands for no-edit, it can be used when creating a file to suppress the default prompt to pen the file.
- `pv` - stands for prefer-visual, it can be used to open your system default GUI editor right away, it only works on Linux/Mac and can be used on create and edit command, it doesn't work with `ne` flag on create command.

### Editing files behaviour
By default, rtm will try to open the file on your terminal if you are on Linux/Mac, Windows always use the `start` command.

If rtm can't find the `$EDITOR` env var on your system, it'll try to find the `$VISUAL` env var and if it fails too, it'll
fall back to use `xdg-open` on Linux and `open` on MacOS.

If you use the `-pv` (prefer visual) flag on Linux/Mac (`-pv` can't be used on Windows) then rtm will try to use `xdg-open` and `open`
right away in Linux/Mac respectively and fail right away if the default editor can't be open.

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