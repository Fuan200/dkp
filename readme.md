# dkp

CLI for creating devkitPro homebrew projects

## Install

#### Compile from source

```bash
git clone git@github.com:Fuan200/dkp.git
cd dkp
cargo install --path .
```

#### Debian

Download the package `.deb` from the [latest release](https://github.com/Fuan200/dkp/releases/latest).

```bash
sudo apt install ./dkp_<version>_amd64.deb
```

#### Fedora

Download the package `.rpm` from the [latest release](https://github.com/Fuan200/dkp/releases/latest).

```bash
sudo dnf install ./dkp-<version>-1.x86_64.rpm
```

## Usage

Verify the installation:

```bash
dkp --version
dkp --help
```

Create a project:

```bash
dkp new <your-name-project>
```

## Supported Platforms

- Game Boy Advance
- Nintendo DS
- GameCube
- Wii
- Nintendo 3DS
- Wii U
- Nintendo Switch

## Generated Project

Each generated project includes:

- A platform-specific `Makefile`
- A minimal `source/main.c`
- A `.gitignore`
- A `readme.md` with instructions
- Optional Git repository initialization

## Development

```bash
cargo run -- --help
cargo run new <your-name-project>
```

### Resources

- [Game Boy Advance examples](https://github.com/devkitPro/gba-examples)
- [GameCube examples](https://github.com/devkitPro/gamecube-examples)
- [Nintendo DS examples](https://github.com/devkitPro/nds-examples)
- [Wii examples](https://github.com/devkitPro/wii-examples)
- [Nintendo 3DS examples](https://github.com/devkitPro/3ds-examples)
- [Wii U Toolchain](https://github.com/devkitPro/wut)
- [Nintendo Switch examples](https://github.com/switchbrew/switch-examples)