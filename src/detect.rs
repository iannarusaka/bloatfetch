use std::process::Command;

pub struct Found {
    pub name: String,
    pub entry: &'static crate::db::BloatEntry,
    pub size_bytes: Option<u64>,
}

pub fn os_info() -> Option<String> {
    use winreg::enums::*;
    use winreg::RegKey;

    let key = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(r"SOFTWARE\Microsoft\Windows NT\CurrentVersion")
        .ok()?;
    let mut product: String = key.get_value("ProductName").ok()?;
    let build: u32 = key
        .get_value::<String, _>("CurrentBuildNumber")
        .ok()
        .and_then(|b| b.parse().ok())
        .unwrap_or(0);
    if build >= 22000 {
        product = product.replace("Windows 10", "Windows 11");
    }
    let display: String = key.get_value("DisplayVersion").unwrap_or_default();
    if display.is_empty() {
        Some(product)
    } else {
        Some(format!("{product} {display}"))
    }
}

pub fn host_info() -> Option<String> {
    use winreg::enums::*;
    use winreg::RegKey;

    let key = RegKey::predef(HKEY_LOCAL_MACHINE)
        .open_subkey(r"HARDWARE\DESCRIPTION\System\BIOS")
        .ok()?;
    let manufacturer: String = key.get_value("SystemManufacturer").unwrap_or_default();
    let product: String = key.get_value("SystemProductName").unwrap_or_default();
    let combined = format!("{} {}", manufacturer.trim(), product.trim());
    let combined = combined.trim();
    if combined.is_empty() {
        None
    } else {
        Some(combined.to_string())
    }
}

pub struct Scan {
    pub found: Vec<Found>,
    pub total_packages: usize,
}

pub fn scan() -> Scan {
    let mut found = Vec::new();
    let mut total = 0;
    total += scan_registry(&mut found);
    total += scan_appx(&mut found);
    let mut found = dedup(found);
    sort(&mut found);
    Scan { found, total_packages: total }
}

pub fn uptime_ms() -> Option<u64> {
    use windows_sys::Win32::System::SystemInformation::GetTickCount64;
    Some(unsafe { GetTickCount64() })
}

pub fn memory() -> Option<(u64, u64)> {
    use windows_sys::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
    unsafe {
        let mut m: MEMORYSTATUSEX = std::mem::zeroed();
        m.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
        if GlobalMemoryStatusEx(&mut m) == 0 {
            return None;
        }
        Some((m.ullTotalPhys - m.ullAvailPhys, m.ullTotalPhys))
    }
}

pub fn disk_c() -> Option<(u64, u64)> {
    use windows_sys::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;
    let path: Vec<u16> = "C:\\\0".encode_utf16().collect();
    let mut free_avail = 0u64;
    let mut total = 0u64;
    let mut total_free = 0u64;
    unsafe {
        if GetDiskFreeSpaceExW(path.as_ptr(), &mut free_avail, &mut total, &mut total_free) == 0 {
            return None;
        }
    }
    Some((total - total_free, total))
}

pub fn autostart_count() -> usize {
    use winreg::enums::*;
    use winreg::RegKey;
    let roots = [
        (HKEY_LOCAL_MACHINE, r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run"),
        (HKEY_CURRENT_USER, r"SOFTWARE\Microsoft\Windows\CurrentVersion\Run"),
    ];
    let mut n = 0;
    for (hive, path) in roots {
        if let Ok(k) = RegKey::predef(hive).open_subkey(path) {
            n += k.enum_values().flatten().count();
        }
    }
    n
}

fn scan_registry(out: &mut Vec<Found>) -> usize {
    use winreg::enums::*;
    use winreg::RegKey;

    let roots = [
        (HKEY_LOCAL_MACHINE, r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall"),
        (HKEY_LOCAL_MACHINE, r"SOFTWARE\WOW6432Node\Microsoft\Windows\CurrentVersion\Uninstall"),
        (HKEY_CURRENT_USER, r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall"),
    ];

    let mut total = 0;
    for (hive, path) in roots {
        let base = match RegKey::predef(hive).open_subkey(path) {
            Ok(k) => k,
            Err(_) => continue,
        };
        for sub in base.enum_keys().flatten() {
            let key = match base.open_subkey(&sub) {
                Ok(k) => k,
                Err(_) => continue,
            };
            let name: String = match key.get_value("DisplayName") {
                Ok(n) => n,
                Err(_) => continue,
            };
            total += 1;
            let entry = match crate::db::match_bloat(&name) {
                Some(e) => e,
                None => continue,
            };
            let size_bytes = key
                .get_value::<u32, _>("EstimatedSize")
                .ok()
                .map(|kb| kb as u64 * 1024)
                .or_else(|| {
                    let loc: String = key.get_value("InstallLocation").ok()?;
                    let loc = loc.trim();
                    if loc.is_empty() {
                        return None;
                    }
                    dir_size(std::path::Path::new(loc), 6)
                });
            out.push(Found { name, entry, size_bytes });
        }
    }
    total
}

fn scan_appx(out: &mut Vec<Found>) -> usize {
    let output = Command::new("powershell.exe")
        .args([
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            "Get-AppxPackage | ForEach-Object { $_.Name + '|' + $_.InstallLocation }",
        ])
        .output();

    let output = match output {
        Ok(o) => o,
        Err(_) => return 0,
    };
    let stdout = String::from_utf8_lossy(&output.stdout);

    let mut total = 0;
    for line in stdout.lines() {
        let line = line.trim_end_matches('\r');
        let (name, location) = match line.split_once('|') {
            Some(p) => p,
            None => continue,
        };
        let name = name.trim();
        if name.is_empty() {
            continue;
        }
        total += 1;
        let entry = match crate::db::match_bloat(name) {
            Some(e) => e,
            None => continue,
        };
        let location = location.trim();
        let size_bytes = if location.is_empty() {
            None
        } else {
            dir_size(std::path::Path::new(location), 6)
        };
        out.push(Found { name: name.to_string(), entry, size_bytes });
    }
    total
}

fn dir_size(dir: &std::path::Path, depth: u32) -> Option<u64> {
    let rd = std::fs::read_dir(dir).ok()?;
    let mut total = 0u64;
    for e in rd.flatten() {
        let Ok(ft) = e.file_type() else { continue };
        if ft.is_dir() {
            if depth > 0 {
                total += dir_size(&e.path(), depth - 1).unwrap_or(0);
            }
        } else if let Ok(m) = e.metadata() {
            total += m.len();
        }
    }
    Some(total)
}

fn dedup(found: Vec<Found>) -> Vec<Found> {
    use std::collections::HashMap;
    let mut by_name: HashMap<String, Found> = HashMap::new();
    for f in found {
        let key = f.name.to_lowercase();
        match by_name.get(&key) {
            Some(existing) if existing.size_bytes.unwrap_or(0) >= f.size_bytes.unwrap_or(0) => {}
            _ => {
                by_name.insert(key, f);
            }
        }
    }
    by_name.into_values().collect()
}

fn sort(found: &mut [Found]) {
    found.sort_by(|a, b| {
        crate::db::Category::all()
            .iter()
            .position(|c| *c == a.entry.category)
            .cmp(&crate::db::Category::all().iter().position(|c| *c == b.entry.category))
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });
}
