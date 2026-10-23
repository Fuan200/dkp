use dialoguer::{Confirm, Select, theme::ColorfulTheme};

#[derive(Clone, Copy)]
pub enum Platform {
    GameBoyAdvance,
    GameCube,
    NintendoDS,
    Wii,
    Nintendo3DS,
    WiiU,
    Switch,
}

impl Platform {
    pub const ALL: [Self; 7] = [
        Self::GameBoyAdvance,
        Self::GameCube,
        Self::NintendoDS,
        Self::Wii,
        Self::Nintendo3DS,
        Self::WiiU,
        Self::Switch,
    ];

    pub fn label(&self) -> &str {
        match self {
            Self::GameBoyAdvance => "Game Boy Advance",
            Self::GameCube => "Game Cube",
            Self::NintendoDS => "Nintendo DS",
            Self::Wii => "Wii",
            Self::Nintendo3DS => "Nintendo 3DS",
            Self::WiiU => "Wii U",
            Self::Switch => "Nintendo Switch",
        }
    }

    pub fn template_directory(&self) -> &str {
        match self {
            Self::GameBoyAdvance => "gba",
            Self::GameCube => "gamecube",
            Self::NintendoDS => "nds",
            Self::Wii => "wii",
            Self::Nintendo3DS => "3ds",
            Self::WiiU => "wiiu",
            Self::Switch => "switch",
        }
    }
}

pub fn select_platform() -> dialoguer::Result<Platform> {
    let choices: Vec<&str> = Platform::ALL
        .iter()
        .map(|platform| platform.label())
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a platform")
        .items(&choices)
        .default(0)
        .interact()?;

    Ok(Platform::ALL[selection])
}

pub fn confirm_git_init() -> dialoguer::Result<bool> {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Initialize a Git Repository?")
        .default(true)
        .interact()
}
