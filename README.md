# Bismuth

## Table of Contents

- [About](#about)
- [Dependencies](#dependencies)
  - [Arch](#for-arch)
  - [Debian, Ubuntu and Mint](#for-debian-ubuntu-and-mint)
- [Installation](#installation)
- [Usage](#usage)
  - [Commands](#commands)
  - [silent](#silent)
  - [mode](#mode-mode)
  - [custom command](#custom-command-custom_command)
- [To-Do](#to-do)

## About
Bismuth is a lightweight Rust script that sets your desktop wallpaper to the latest daily Bing image. 

## Dependencies
|Dependency|Link                                              |
|----------|--------------------------------------------------|
|feh       |[Github](https://github.com/derf/feh)             |
|libnotify |[Gitlab](https://gitlab.gnome.org/GNOME/libnotify)|

### For Arch
```
paru -S feh libnotify
```
```
yay -S feh libnotify
```
```
sudo pacman -S feh libnotify
```

### For Debian, Ubuntu and Mint
```
sudo apt install feh libnotify-dev
```

## Installation
1\. Clone the repository and cd into it.
```
git clone "https://github.com/thejayduck/bismuth"
cd Bismuth
```
2\. Build Bismuth
```
cargo build --release
```

## Usage
Here's an example usage of the script.
```
bismuth --silent --mode max
```

---

Or you can also simply do;

Which calls `feh` using `--bg-fill` option as default.
```
bismuth
```

---

Here's an example of a custom command, in this case using `swaybg`.

`%` represents the file destination.
```
bismuth -c 'swaybg --image %'
```

The wallpaper gets saved in `$HOME/.local/share/bismuth/.wallpaper.jpg`.

### Commands
| Command                                      | Description                                |
|----------------------------------------------|--------------------------------------------|
| `--silent`, `-s`                             | Disable notifications.                     |
| `--mode`, `-m` `<MODE>`                      | Set feh scaling options.                   |
| `--custom-command`, `-c` `<CUSTOM_COMMAND>`  | Set background using custom command.       |
| `--help`, `-h`                               | Display help information.                  |
| `--version`, `-V`                            | Show the version of the script.            |

### `--help`
```
Usage: bismuth [OPTIONS]

Options:
  -s, --silent
          Disables notifications
  -m, --mode <MODE>
          Specifies the scaling options for Feh [default: fill] [possible values: center, fill, max, scale, tile]
  -c, --custom-command <CUSTOM_COMMAND>
          Call custom wallpaper command
  -h, --help
          Print help (see more with '--help')
  -V, --version
          Print version
```

### `--silent`
Disables notifications when the wallpaper is successfully set.

### `--mode <MODE>`
- `center`: Centers the image on the screen without scaling.
- `fill`: Scales the image to fit the screen and preserves aspect ratio.
- `max`: Scales the image to the maximum size with black borders on one side.
- `scale`: Fills the screen, but doesn't preserve the aspect ratio.
- `tile`: Tiles the image on the screen.

### `--custom-command <CUSTOM_COMMAND>`
Sets wallpaper using a custom command.

Example `bismuth -c "swaybg --image %"` 

The `%` symbol is important as it signifies the file destination.

The custom command gets saved in `$HOME/.local/share/bismuth/config.txt`.

### To-Do
- [x] Save image as `.wallpaper.jpg` for `.fehbg`.
- [x] Custom command support.
- [x] Save custom command. 
