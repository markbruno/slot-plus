//! `Cheats/GBA/<stem>.cht`, in the format RetroArch writes and the libretro database ships:
//!
//! ```text
//! cheats = 2
//! cheat0_desc = "Infinite HP"
//! cheat0_code = "82003B4C+0063"
//! cheat0_enable = true
//! cheat1_desc = "Max Money"
//! cheat1_code = "..."
//! cheat1_enable = false
//! ```
//!
//! Which cheats are on is the file's business: `cheatN_enable` is edited on a computer, the
//! same way `theme.txt` is. What the device adds is one switch per cart, SELECT+X in a game,
//! that turns every enabled cheat on or off together. That switch lives in
//! `System/cheats.ini`, one `stem = on|off` per line, so the `.cht` files stay exactly as
//! RetroArch would read them.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::CART_DIR;

pub const CHEATS_DIR: &str = "Cheats";
pub const CHEATS_SWITCH_FILE: &str = "System/cheats.ini";

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Cheat {
    pub desc: String,
    pub code: String,
    pub enabled: bool,
}

pub fn cheat_path(root: &Path, stem: &str) -> PathBuf {
    root.join(CHEATS_DIR).join(CART_DIR).join(format!("{stem}.cht"))
}

/// Best effort, like every file on the card a person edits by hand. A line that cannot be
/// read is skipped and the rest of the file still counts.
///
/// `cheats = N` is honoured when it is there, and when it is missing every index that has a
/// code is taken, in order: hand-written files forget the count more often than not.
pub fn parse_cht(text: &str) -> Vec<Cheat> {
    let mut kv: HashMap<String, String> = HashMap::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        let value = value.trim();
        let value = value
            .strip_prefix('"')
            .and_then(|v| v.strip_suffix('"'))
            .unwrap_or(value);
        kv.insert(key.trim().to_ascii_lowercase(), value.to_string());
    }
    let count = kv
        .get("cheats")
        .and_then(|n| n.parse::<usize>().ok())
        .unwrap_or_else(|| {
            (0..)
                .take_while(|i| kv.contains_key(&format!("cheat{i}_code")))
                .count()
        });
    (0..count)
        .filter_map(|i| {
            let code = kv.get(&format!("cheat{i}_code"))?.trim().to_string();
            if code.is_empty() {
                return None;
            }
            let enabled = kv
                .get(&format!("cheat{i}_enable"))
                .is_some_and(|v| matches!(v.to_ascii_lowercase().as_str(), "true" | "1"));
            Some(Cheat {
                desc: kv
                    .get(&format!("cheat{i}_desc"))
                    .cloned()
                    .unwrap_or_default(),
                code,
                enabled,
            })
        })
        .collect()
}

/// Every cheat in the cart's file, or nothing when there is no file.
pub fn read_cheats(root: &Path, stem: &str) -> Vec<Cheat> {
    std::fs::read_to_string(cheat_path(root, stem))
        .map(|t| parse_cht(&t))
        .unwrap_or_default()
}

/// The codes to hand the core: the enabled cheats, in file order.
pub fn enabled_codes(cheats: &[Cheat]) -> Vec<String> {
    cheats
        .iter()
        .filter(|c| c.enabled)
        .map(|c| c.code.clone())
        .collect()
}

/// The cart's switch. On unless it was turned off: someone who put a file on the card with
/// cheats enabled in it has already said what they want.
pub fn cheats_on(root: &Path, stem: &str) -> bool {
    crate::ini::value(root, CHEATS_SWITCH_FILE, stem).map_or(true, |v| {
        !matches!(v.to_ascii_lowercase().as_str(), "off" | "0" | "false")
    })
}

pub fn write_cheats_on(root: &Path, stem: &str, on: bool) -> std::io::Result<()> {
    crate::ini::write(root, CHEATS_SWITCH_FILE, stem, if on { "on" } else { "off" })
}

/// A copy of the cart's battery save, taken the first time cheats run on it and never again.
///
/// Cheats write to memory the game believes it owns, and a game that saves while one is
/// running writes whatever it was made to believe. Some of that cannot be undone from inside
/// the game — a walk-through-walls code saved inside a wall, an item the game was never meant
/// to hold — so the save as it was before any cheat touched it is kept beside it, once.
pub fn backup_save_once(root: &Path, stem: &str) {
    let dir = root.join("Saves").join(CART_DIR);
    for ext in ["sav", "srm"] {
        let from = dir.join(format!("{stem}.{ext}"));
        let to = dir.join(format!("{stem}.{ext}.before-cheats"));
        if from.exists() && !to.exists() {
            if let Err(e) = std::fs::copy(&from, &to) {
                eprintln!("slot: cheats: could not back up {}: {e}", from.display());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_a_retroarch_file() {
        let text = "cheats = 2\n\ncheat0_desc = \"Infinite HP\"\ncheat0_code = \"82003B4C+0063\"\ncheat0_enable = true\ncheat1_desc = \"Money\"\ncheat1_code = \"1234\"\ncheat1_enable = false\n";
        let c = parse_cht(text);
        assert_eq!(c.len(), 2);
        assert_eq!(c[0].desc, "Infinite HP");
        assert_eq!(c[0].code, "82003B4C+0063");
        assert!(c[0].enabled);
        assert!(!c[1].enabled);
        assert_eq!(enabled_codes(&c), vec!["82003B4C+0063".to_string()]);
    }

    #[test]
    fn a_missing_count_takes_every_numbered_code() {
        let c = parse_cht("cheat0_code = A\ncheat0_enable = true\ncheat1_code = B\n");
        assert_eq!(c.len(), 2);
        assert!(c[0].enabled && !c[1].enabled);
    }

    #[test]
    fn an_empty_code_is_skipped_rather_than_sent() {
        let c = parse_cht("cheats = 1\ncheat0_code = \"\"\ncheat0_enable = true\n");
        assert!(c.is_empty());
    }
}
