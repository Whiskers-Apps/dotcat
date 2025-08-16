# About
This simple program is used to manage dotfiles. 
I find most tools like stow are annoying to deal with so I created a cli app to manage them.

# Install
To install the app you can use cargo:
```sh
cargo install dotcat
```

Alternatively you can download from the [releases page](https://github.com/Whiskers-Apps/dotcat/releases)

# Usage
> [!IMPORTANT] 
> It's required to run the setup command to set the location for your dotifles.

```sh
dotcat setup ~/dotfiles/
```
> You can change ~/dotfiles to whatever folder that fits your needs.

<br>

To add a dotfile:
```sh
# For a file
dotcat link ~/.config/dotcat.toml

# For a directory
dotcat link ~/.config/nvim

# For a file and skip the name prompt
dotcat link ~/.config/nvim neovim
```
> You can also give a relative path and it should work the same. Also if no name is provided it will prompt for one.

<br>

To remove a dotfile:
```sh
dotcat unlink neovim
```
> This only requires the dot name

<br>

To link the whole dot database (Useful when setting up your pc or after Arch broke ;-;):
```sh
dotcat link-db
```

<br>

To unlink the whole dot database:
```sh
dotcat unlink-db
```

<br>

To migrate the dots from the current dots directory to another:
```sh
dotcat migrate ~/.dots
```
<br>

You can also check the help if you are feeling a bit lost:
```sh
dotcat help
```
