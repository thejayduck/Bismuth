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
bismuth --silent --mode bg-max
```
Or you can also simply do;
```
bismuth
```
Which is going to set your wallpaper using `bg-fill` option, and send a notification.

The wallpaper gets saved at `$HOME`/.config with the name `.wallpaper.jpg`.

Example `/home/$USER/.local/share/.wallpaper.jpg`

### Commands
| Command                  | Description                                |
|--------------------------|--------------------------------------------|
| `--silent`, `-s`         | Disable notifications.                     |
| `--mode`, `-m` `<MODE>`  | Set feh scaling options.                   |
| `--help`, `-h`           | Display help information.                  |
| `--version`, `-V`        | Show the version of the script.            |

### `--help`
```
Usage: bismuth [OPTIONS]

Options:
  -s, --silent
          Disables notifications

  -m, --mode <MODE>
          Specifies the scaling options for Feh.
          
          Available modes: - `BgCenter`: Centers the image on the screen without scaling. - `BgFill`: Scales the image to fit the screen and preserves aspect ratio. - `BgMax`: Scales the image to the maximum size with black borders on one side. - `BgScale`: Fills the screen but doesn't preserve the aspect raio. - `BgTile`: Tiles the image on the screen.
          
          [default: bg-fill]
          [possible values: bg-center, bg-fill, bg-max, bg-scale, bg-tile]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

### `--silent`
Disables notifications when the wallpaper is successfully set.

### `--mode <MODE>`
- `bg-center`: Centers the image on the screen without scaling.
- `bg-fill`: Scales the image to fit the screen and preserves aspect ratio.
- `bg-max`: Scales the image to the maximum size with black borders on one side.
- `bg-scale`: Fills the screen, but doesn't preserve the aspect ratio.
- `bg-tile`: Tiles the image on the screen.

### To-Do
- [x] Save image as `.wallpaper.jpg` for `.fehbg`.