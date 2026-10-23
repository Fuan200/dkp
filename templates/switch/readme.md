### Requirements

- devkitPro
- devkitA64
- libnx

```sh
sudo dkp-pacman -S switch-dev
```

See the [devkitPro documentation](https://devkitpro.org/wiki/devkitPro_pacman).

### Project Structure

Inside this project, you will find:

```text
.
├── .gitignore
├── Makefile
├── README.md
└── source/
    └── main.c
```

### Commands

Run all commands from the project root:

| Command | Action |
| :--- | :--- |
| `make` | Builds the Nintendo Switch `.nro` homebrew executable. |
| `make clean` | Removes generated build files. |