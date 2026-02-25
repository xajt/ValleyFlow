use anyhow::Result;
use std::path::PathBuf;

const AUTOSTART_KEY: &str = "ValleyFlow";

pub fn enable_autostart() -> Result<()> {
    let exe_path = std::env::current_exe()?;
    let exe_path_str = exe_path.to_string_lossy().to_string();

    // Windows Registry path for autostart
    let key = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER)
        .open_subkey_with_flags(
            "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
            winreg::enums::KEY_WRITE,
        )?;

    key.set_value(AUTOSTART_KEY, &exe_path_str)?;

    log::info!("Autostart enabled");
    Ok(())
}

pub fn disable_autostart() -> Result<()> {
    let key = winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER)
        .open_subkey_with_flags(
            "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
            winreg::enums::KEY_WRITE,
        )?;

    let _ = key.delete_value(AUTOSTART_KEY);

    log::info!("Autostart disabled");
    Ok(())
}

pub fn is_autostart_enabled() -> bool {
    winreg::RegKey::predef(winreg::enums::HKEY_CURRENT_USER)
        .open_subkey("Software\\Microsoft\\Windows\\CurrentVersion\\Run")
        .and_then(|key| key.get_value::<String, _>(AUTOSTART_KEY))
        .is_ok()
}
