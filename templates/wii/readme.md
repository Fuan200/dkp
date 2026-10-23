### Requirements

- devkitPro
- devkitPPC
- libngc

```bash
sudo dkp-pacman -S wii-dev
```

See the [devkitPro documentation](https://devkitpro.org/wiki/devkitPro_pacman).

### Project Structure

Inside this project, you will find:

```text
.
├── .gitignore
├── Makefile
├── readme.md
└── source/
    └── main.c
```

### Commands

Run all commands from the project root:

| Command | Action |
| :--- | :--- |
| `make` | Builds the Wii `.dol` executable. |
| `make clean` | Removes generated build files. |
