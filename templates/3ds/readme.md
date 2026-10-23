## Requirements

- devkitPro
- devkitARM
- libctru

Install the required packages with:

```sh
sudo dkp-pacman -S 3ds-dev
```

See the [devkitPro documentation](https://devkitpro.org/wiki/devkitPro_pacman).

## Project Structure

Inside this project, you will find:

```text
.
├── .gitignore
├── Makefile
├── readme.md
└── source/
    └── main.c
```

## Commands

Run all commands from the project root:

| Command | Action |
| :--- | :--- |
| `make` | Builds the Nintendo 3DS `.3dsx` homebrew executable. |
| `make clean` | Removes generated build files. |