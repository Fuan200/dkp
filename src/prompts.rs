use dialoguer::{Select, theme::ColorfulTheme};

pub enum Platform {
    GameBoyAdvance,
    NintendoDS,
    Wii,
    Nintendo3DS,
    WiiU,
    Switch,
}

impl Platform {
    pub fn label(&self) -> &str {
        match self {
            Self::GameBoyAdvance => "Game Boy Advance",
            Self::NintendoDS => "Nintendo DS",
            Self::Wii => "Wii",
            Self::Nintendo3DS => "Nintendo 3DS",
            Self::WiiU => "Wii U",
            Self::Switch => "Nintendo Switch",
        }
    }
}

pub fn select_platform() -> dialoguer::Result<Platform> {
    let choices = [
        "Game Boy Advance",
        "Nintendo DS",
        "Wii",
        "Nintendo 3DS",
        "Wii U",
        "Nintendo Switch",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a platform")
        .items(&choices)
        .default(0)
        .interact()?;

    Ok(match selection {
        0 => Platform::GameBoyAdvance,
        1 => Platform::NintendoDS,
        2 => Platform::Wii,
        3 => Platform::Nintendo3DS,
        4 => Platform::WiiU,
        _ => Platform::Switch,
    })
}
