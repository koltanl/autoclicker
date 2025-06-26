use clap::Parser;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub debug: bool,
    pub beep: bool,
    pub command: ConfigCommand,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ConfigCommand {
    Run {
        device_query: String,
        left_bind: u16,
        right_bind: u16,
        lock_unlock_bind: Option<u16>,
        hold: bool,
        grab: bool,
        cooldown: u64,
        cooldown_press_release: u64,
    },
    RunLegacy {
        device_query: String,
        cooldown: u64,
        cooldown_press_release: u64,
    },
}

#[derive(clap::Subcommand, Debug)]
pub enum Command {
    Run {
        /// Device name or path when the first character is `/`
        /// (first looks for exact match, then takes the first device that contains the name)
        #[arg(short = 'd')]
        device_query: String,

        /// Bind left autoclicker to keycode
        /// Mouse: 275 ButtonSide
        /// Keyboard: 26 LeftBrace
        #[arg(short)]
        left_bind: u16,

        /// Bind right autoclicker to keycode
        /// Mouse: 276 ButtonExtra
        /// Keyboard: 27 RightBrace
        #[arg(short)]
        right_bind: u16,

        /// Bind lock/unlock to keycode
        /// Mouse: 274 ButtonMiddle
        /// With this you can bind to the lefr and right button, and the bindings will be used when is unlocked.
        /// Useful for mouses without side buttons.
        #[arg(short = 'T')]
        lock_unlock_bind: Option<u16>,

        /// Hold mode, when a keybind is pressed the autoclicker will be active until the keybind release
        #[arg(short = 'H', default_value_t = false)]
        hold: bool,

        /// This will grab the device,
        #[arg(long, default_value_t = false)]
        grab: bool,

        /// Set the cooldown in milliseconds
        #[arg(short, default_value_t = 25)]
        cooldown: u64,

        /// Set cooldown in milliseconds, between press and release
        #[arg(short = 'C', default_value_t = 0)]
        cooldown_press_release: u64,
    },
    RunLegacy {
        /// Device name or path when the first character is `/`
        /// (first looks for exact match, then takes the first device that contains the name)
        #[arg(short = 'd')]
        device_query: String,

        /// Set the cooldown in milliseconds
        #[arg(short, default_value_t = 25)]
        cooldown: u64,

        /// Set cooldown in milliseconds, between press and release
        #[arg(short = 'C', default_value_t = 0)]
        cooldown_press_release: u64,
    },
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value_t = false)]
    pub debug: bool,

    /// For not beeping when the autoclicker state is changed
    #[arg(long, default_value_t = false)]
    pub beep: bool,

    /// Load configuration from JSON file
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    /// Load default config.json from current directory
    #[arg(short, long, default_value_t = false)]
    pub default: bool,

    #[command(subcommand)]
    pub command: Option<Command>,
}

impl Args {
    pub fn load_from_config_or_default(mut self) -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = if self.default {
            Some(PathBuf::from("config.json"))
        } else {
            self.config.clone()
        };

        if let Some(config_path) = config_path {
            let config_content = std::fs::read_to_string(&config_path)?;
            let config: Config = serde_json::from_str(&config_content)?;
            
            // Override with config values if not set via CLI
            if !self.debug {
                self.debug = config.debug;
            }
            if !self.beep {
                self.beep = config.beep;
            }
            if self.command.is_none() {
                self.command = Some(config.command.into());
            }
        }
        Ok(self)
    }
}

impl From<ConfigCommand> for Command {
    fn from(config_cmd: ConfigCommand) -> Self {
        match config_cmd {
            ConfigCommand::Run {
                device_query,
                left_bind,
                right_bind,
                lock_unlock_bind,
                hold,
                grab,
                cooldown,
                cooldown_press_release,
            } => Command::Run {
                device_query,
                left_bind,
                right_bind,
                lock_unlock_bind,
                hold,
                grab,
                cooldown,
                cooldown_press_release,
            },
            ConfigCommand::RunLegacy {
                device_query,
                cooldown,
                cooldown_press_release,
            } => Command::RunLegacy {
                device_query,
                cooldown,
                cooldown_press_release,
            },
        }
    }
}

impl Config {
    pub fn save_to_file(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}
