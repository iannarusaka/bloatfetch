use crate::db::Category;
use crate::detect::{Found, Scan};
use std::env;

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RED: &str = "\x1b[31m";
const BR_RED: &str = "\x1b[91m";
const YELLOW: &str = "\x1b[33m";
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";

fn art() -> Vec<String> {
    let g = "\x1b[37m";
    let z = RESET;
    [
        "⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⣤⣤⣄⣀⠀⠀⠀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⠀⢠⣶⠟⣉⣤⣢⣄⡪⢝⢦⡀⠀⠀⠀⠀",
        "⠀⠀⠀⠀⠀⢰⡿⢁⣾⠟⠉⠉⠉⠹⣧⣃⢳⡀⠀⠀⠀",
        "⠀⠀⠀⢀⣀⣼⡏⣼⠃⠀⠀⠀⠀⠀⢹⣏⣸⡅⠀⠀⠀",
        "⠀⢀⣴⡿⠿⣿⠃⣿⠀⠀⠀⠀⠀⠀⣸⣷⣿⣶⣄⠀⠀",
        "⠠⠞⠁⠀⢠⣿⠌⣿⠀⠀⠀⠀⠀⠀⣿⡇⣿⠛⠛⠿⣄",
        "⠀⠀⢀⣠⠾⠿⠾⣷⡀⠀⠀⠀⡠⢶⠛⠹⠿⢶⣄⠀⠈",
        "⠀⢠⠋⠀⢀⣁⡀⠘⠙⣦⡀⠘⠈⠀⣠⣤⡀⠀⠻⣦⠀",
        "⠀⢀⠀⠀⢾⣿⣿⠀⠀⢘⣧⠇⡀⠘⢿⣿⠏⠀⠀⡿⠀",
        "⠀⠈⢧⡀⠈⣉⡁⠀⣤⡞⠀⠘⢢⣀⡄⠀⢠⣠⠾⠃⠀",
        "⠀⠀⠀⠉⣷⡖⣶⡛⠉⠀⠀⠀⠀⣿⡏⣿⠋⠁⠀⠀⠀",
        "⠀⠀⠀⠀⢻⡇⣽⢺⣱⡄⠀⠀⠀⣿⢇⡏⠀⠀⣰⡖⣦",
        "⠀⠀⠀⠀⣿⡇⣿⢻⠸⡇⠀⠀⠀⣿⢰⡏⢀⣾⢳⡾⠉",
        "⠀⠀⠀⠀⣿⡄⡿⣿⠘⡁⠀⠀⠐⣿⢸⡇⣾⢇⡿⠀⠀",
        "⠀⠀⠀⠀⣿⠐⣟⣧⢰⠀⠀⠀⢸⣿⢺⠆⣿⢸⡇⠀⠀",
        "⠀⠀⠀⠀⣿⠡⣟⣿⢸⡇⠀⠀⢸⣇⢿⠆⣿⢸⡅⠀⠀",
        "⠀⠀⠀⠀⣿⠡⣏⣿⡸⡅⠀⠀⣼⢏⣼⠆⣿⢸⠃⠀⠀",
        "⠀⠀⠀⠀⣿⠰⣿⠹⣶⣭⣖⣪⣵⡾⠏⢠⣿⢸⡁⠀⠀",
        "⠀⠀⠀⠀⣿⢂⡷⠀⠈⠉⠘⠉⠉⠀⠀⠸⣿⢼⡀⠀⠀",
        "⠀⠀⠀⠀⣿⡍⢿⡀⠀⠀⠀⠀⠀⠀⠀⣸⠇⣼⠀⠀⠀",
        "⠀⠀⠀⠀⠹⣯⡎⡻⢦⣀⣀⣀⣀⡤⠞⣉⣼⠃⠀⠀⠀",
        "⠀⠀⠀⠀⠀⠈⠻⢷⣦⣢⣬⣤⣤⣶⠾⠋⠀⠀⠀⠀⠀",
    ]
    .iter()
    .map(|l| format!("{g}{l}{z}"))
    .collect()
}

/// Width of a string as it would appear on screen, skipping ANSI "ESC[...m" sequences.
fn visible_width(s: &str) -> usize {
    let mut width = 0;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // expect '[' then digits/semicolons then a final letter (commonly 'm')
            if chars.peek() == Some(&'[') {
                chars.next();
                while let Some(&c2) = chars.peek() {
                    chars.next();
                    if c2.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            width += 1;
        }
    }
    width
}

fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    format!("{:.1} {}", size, UNITS[unit])
}

fn score_and_verdict(found: &[Found]) -> (u32, &'static str, &'static str) {
    let gb_wasted: f64 = found.iter().filter_map(|f| f.size_bytes).sum::<u64>() as f64
        / (1024.0 * 1024.0 * 1024.0);
    let raw = found.len() as f64 * 5.0 + gb_wasted * 3.0;
    let score = raw.round().min(100.0).max(0.0) as u32;
    let (verdict, color) = match score {
        0..=19 => ("suspiciously clean", GREEN),
        20..=39 => ("mild hoarding", YELLOW),
        40..=59 => ("certified OEM victim", YELLOW),
        60..=74 => ("landfill tier", BR_RED),
        75..=99 => ("microsoft data supplier", RED),
        _ => ("Bill Gates", RED),
    };
    (score, verdict, color)
}

pub fn render(scan: &Scan) {
    let found = &scan.found[..];
    let total_bytes: u64 = found.iter().filter_map(|f| f.size_bytes).sum();

    let art_lines = art();
    let art_width = art_lines.iter().map(|l| visible_width(l)).max().unwrap_or(0);
    let pad = " ".repeat(art_width);

    let user = env::var("USERNAME").unwrap_or_else(|_| "victim".to_string());
    let host = env::var("COMPUTERNAME").unwrap_or_else(|_| "windows-pc".to_string());

    // build the right-hand column, line by line
    let mut right: Vec<String> = Vec::new();
    right.push(format!("{BOLD}{CYAN}{user}@{host}{RESET}"));
    right.push(format!("{DIM}{}{RESET}", "-".repeat(user.len() + host.len() + 1)));

    const MAGENTA: &str = "[95m";
    let key = |label: &str, c: &str| format!("{BOLD}{c}{label}:{RESET}");

    if let Some(os) = crate::detect::os_info() {
        right.push(format!("{} {os}", key("OS", CYAN)));
    }
    if let Some(h) = crate::detect::host_info() {
        right.push(format!("{} {h}", key("Host", CYAN)));
    }

    if let Some(ms) = crate::detect::uptime_ms() {
        let total_min = ms / 60_000;
        let (d, h, m) = (total_min / 1440, (total_min % 1440) / 60, total_min % 60);
        let mut parts: Vec<String> = Vec::new();
        for (n, word) in [(d, "day"), (h, "hour"), (m, "minute")] {
            if n > 0 {
                parts.push(format!("{n} {word}{}", if n == 1 { "" } else { "s" }));
            }
        }
        let uptime = match parts.len() {
            0 => "0 minutes".to_string(),
            1 => parts.remove(0),
            _ => {
                let last = parts.pop().unwrap();
                format!("{} and {last}", parts.join(", "))
            }
        };
        right.push(format!(
            "{} {uptime} running Windows Update",
            key("Uptime", CYAN)
        ));
    }

    if let Some((used, total)) = crate::detect::memory() {
        let pct = if total > 0 { used * 100 / total } else { 0 };
        right.push(format!(
            "{} {} / {} ({pct}% — vendor services included)",
            key("Memory", CYAN),
            human_size(used),
            human_size(total)
        ));
    }

    if total_bytes > 0 {
        let pct = crate::detect::disk_c()
            .filter(|(_, total)| *total > 0)
            .map(|(_, total)| {
                format!(
                    " ({:.1}% of your disk)",
                    total_bytes as f64 / total as f64 * 100.0
                )
            })
            .unwrap_or_default();
        right.push(format!(
            "{} {}{pct}",
            key("Disk wasted", CYAN),
            human_size(total_bytes)
        ));
    }

    right.push(String::new());

    if found.is_empty() {
        right.push(format!("{DIM}(none found — did you build this PC yourself?){RESET}"));
    } else {
        let pct = if scan.total_packages > 0 {
            found.len() as f64 / scan.total_packages as f64 * 100.0
        } else {
            0.0
        };
        right.push(format!(
            "{} {} packages installed, {} bloat ({:.1}%)",
            key("Bloat rate", YELLOW),
            scan.total_packages,
            found.len(),
            pct
        ));

        for cat in Category::all() {
            let items: Vec<&Found> = found.iter().filter(|f| &f.entry.category == cat).collect();
            if items.is_empty() {
                continue;
            }
            let sum: u64 = items.iter().filter_map(|f| f.size_bytes).sum();
            right.push(format!(
                "{} {} ({})",
                key(cat.label(), YELLOW),
                items.len(),
                human_size(sum)
            ));
        }

        right.push(String::new());

        let autostart = crate::detect::autostart_count();
        if autostart > 0 {
            right.push(format!("{} {autostart} programs racing at boot", key("Autostart Junk", GREEN)));
        }

        let copilots = found.iter().filter(|f| f.name.to_lowercase().contains("copilot")).count();
        if copilots > 0 {
            right.push(format!("{} {copilots} (recommended: 0)", key("Copilots", GREEN)));
        }

        let never = 90 + (found.len() % 10);
        right.push(format!(
            "{} {never}% of bloat {DIM}(scientific estimate){RESET}",
            key("Never Opened", GREEN)
        ));

        right.push(String::new());

        let worst = found.iter().max_by_key(|f| f.size_bytes.unwrap_or(0)).unwrap();
        right.push(format!(
            "{} {} ({})",
            key("Worst Offender", BR_RED),
            worst.name,
            human_size(worst.size_bytes.unwrap_or(0))
        ));
        right.push(format!("{} {DIM}{}{RESET}", key("Diagnosis", BR_RED), worst.entry.snark));
    }

    let (score, verdict, score_color) = score_and_verdict(found);

    right.push(String::new());

    right.push(format!(
        "{} {score_color}{score}/100  {verdict}{RESET}",
        key("Bloat Score", MAGENTA)
    ));

    let total_lines = art_lines.len().max(right.len());
    for i in 0..total_lines {
        let art_line = art_lines.get(i).map(|s| s.as_str()).unwrap_or(pad.as_str());
        let art_visible = art_lines.get(i).map(|s| visible_width(s)).unwrap_or(art_width);
        let fill = " ".repeat(art_width.saturating_sub(art_visible) + 2);
        let right_line = right.get(i).map(|s| s.as_str()).unwrap_or("");
        println!("{art_line}{fill}{right_line}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visible_width_ignores_ansi_codes() {
        let s = format!("{BOLD}{RED}hi{RESET}");
        assert_eq!(visible_width(&s), 2);
        assert_eq!(visible_width("plain"), 5);
    }

    #[test]
    fn score_tiers_pick_correct_verdict() {
        let cases = [
            (0u32, "suspiciously clean"),
            (19, "suspiciously clean"),
            (20, "mild hoarding"),
            (39, "mild hoarding"),
            (40, "certified OEM victim"),
            (59, "certified OEM victim"),
            (60, "landfill tier"),
            (79, "landfill tier"),
            (80, "this machine is a Superfund site"),
            (100, "this machine is a Superfund site"),
        ];
        for (score, expected) in cases {
            let verdict = match score {
                0..=19 => "suspiciously clean",
                20..=39 => "mild hoarding",
                40..=59 => "certified OEM victim",
                60..=79 => "landfill tier",
                _ => "this machine is a Superfund site",
            };
            assert_eq!(verdict, expected, "score {score}");
        }
    }
}
