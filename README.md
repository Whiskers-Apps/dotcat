# About
This simple program is used to manage dotfiles. 
I find most tools like stow are annoying to deal with so dotcat was born.

# Install
To install the app you can use cargo:
```sh
cargo install dotcat
```

Alternatively you can download from the [releases page](https://github.com/Whiskers-Apps/dotcat/releases)

# Usage
> [!IMPORTANT] 
> It's required to run the `set-dir` command to set the location for your dotifles. Also the dots config should be backed up along with your dotfiles repository.

```sh
dotcat set-dir ~/dotfiles/
```

> You can change ~/dotfiles to whatever folder that fits your needs.

<br>

To add a dotfile:
```sh
# For a file
dotcat add dotcat ~/.config/dotcat.toml

# For a directory
dotcat add nvim ~/.config/nvim
```

> You can also give a relative path and it should work the same.

<br>

To rename a dotfile that you might have messed up the id you can use the rename command:
```sh
dotcat rename konsoleeee konsole
```

<br>

To remove a dotfile:
```sh
dotcat remove neovim
```
> This only requires the dot id

<br>

To link the whole dot config (Useful when setting up your pc or after Arch broke ;-;):
```sh
dotcat link-dots
```

<br>

To unlink the whole dot config:
```sh
dotcat unlink-dots
```

<br>

To migrate the dots from the current dots directory to another:
```sh
dotcat migrate-dir ~/.dots
```
<br>

You can also check the help if you are feeling a bit lost:
```sh
dotcat help
```
