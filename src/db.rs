#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Category {
    Manufacturer,
    Microsoft,
    Games,
    Trialware,
    Adware,
}

impl Category {
    pub fn label(&self) -> &'static str {
        match self {
            Category::Manufacturer => "Manufacturer Crapware",
            Category::Microsoft => "Microsoft Preinstalled",
            Category::Games => "Game Stubs",
            Category::Trialware => "Trialware",
            Category::Adware => "Adware / Sponsored",
        }
    }

    pub fn all() -> &'static [Category] {
        &[
            Category::Manufacturer,
            Category::Microsoft,
            Category::Games,
            Category::Trialware,
            Category::Adware,
        ]
    }
}

pub struct BloatEntry {
    pub pattern: &'static str,
    pub category: Category,
    pub snark: &'static str,
}

macro_rules! entry {
    ($pattern:expr, $cat:expr, $snark:expr) => {
        BloatEntry {
            pattern: $pattern,
            category: $cat,
            snark: $snark,
        }
    };
}

pub static BLOAT_DB: &[BloatEntry] = &[
    // ---- Manufacturer crapware ----
    entry!("dell supportassist", Category::Manufacturer, "It's reassuring to know Dell is always looking after my laptop."),
    entry!("dell update", Category::Manufacturer, "I love that my drivers stay current without me lifting a finger."),
    entry!("dell digital delivery", Category::Manufacturer, "My purchased software just appears on its own, which feels like the future."),
    entry!("dell peripheral manager", Category::Manufacturer, "Finally, one place to manage every accessory I own."),
    entry!("dell optimizer", Category::Manufacturer, "You can really feel the machine learning your habits over time."),
    entry!("hp wolf security", Category::Manufacturer, "An extra layer of security on top of Windows gives me real peace of mind."),
    entry!("hp jumpstart", Category::Manufacturer, "A lovely guided tour that made setting up my new PC a breeze."),
    entry!("hp support assistant", Category::Manufacturer, "It's like having an HP technician living right in my taskbar."),
    entry!("hp smart", Category::Manufacturer, "Printing from my phone has honestly changed my life."),
    entry!("hp system event utility", Category::Manufacturer, "My function keys have never felt more responsive."),
    entry!("hp connection optimizer", Category::Manufacturer, "My Wi-Fi feels noticeably smoother since I noticed this was installed."),
    entry!("lenovo vantage", Category::Manufacturer, "Everything about my ThinkPad in one beautiful dashboard."),
    entry!("lenovo app explorer", Category::Manufacturer, "A great way to discover quality apps I would never have found myself."),
    entry!("lenovo companion", Category::Manufacturer, "It really does feel like a companion on my computing journey."),
    entry!("lenovo utility", Category::Manufacturer, "Simple, quiet, and always there when the hardware needs it."),
    entry!("lenovo now", Category::Manufacturer, "I appreciate being kept in the loop about new Lenovo offers."),
    entry!("asus armoury crate", Category::Manufacturer, "The RGB control alone is worth every megabyte."),
    entry!("myasus", Category::Manufacturer, "It makes my laptop feel truly mine."),
    entry!("asus giftbox", Category::Manufacturer, "Who doesn't love opening a box of free software?"),
    entry!("asus splendid", Category::Manufacturer, "The colors on my screen have genuinely never looked better."),
    entry!("acer collection", Category::Manufacturer, "A thoughtful starter pack for anyone new to computers."),
    entry!("acer care center", Category::Manufacturer, "It's comforting that someone is watching over my PC's health."),
    entry!("acer quick access", Category::Manufacturer, "All my most important toggles, one click away."),
    entry!("acer configuration manager", Category::Manufacturer, "Set it once and it quietly keeps everything just right."),
    entry!("msi center", Category::Manufacturer, "Real-time stats on every component make me feel like a pro."),
    entry!("msi app player", Category::Manufacturer, "Playing my phone games on a big screen is a dream come true."),
    entry!("samsung update", Category::Manufacturer, "My Samsung software is always the latest version, automatically."),
    entry!("samsung settings", Category::Manufacturer, "Settings designed specifically for my hardware, which Windows can't offer."),
    entry!("realtek audio console", Category::Manufacturer, "The equalizer presets make my laptop speakers sound like a studio."),
    entry!("intel driver & support assistant", Category::Manufacturer, "Intel personally makes sure my drivers are perfect."),
    entry!("killer control center", Category::Manufacturer, "My games get the bandwidth they deserve, guaranteed."),
    entry!("toshiba service station", Category::Manufacturer, "Faithful service updates from a brand I've trusted for decades."),

    // ---- Microsoft preinstalled ----
    entry!("cortana", Category::Microsoft, "I ask her something every morning and she always answers."),
    entry!("clipchamp", Category::Microsoft, "A full video editor included for free is incredible value."),
    entry!("microsoft solitaire collection", Category::Microsoft, "My daily challenge streak is the highlight of my lunch break."),
    entry!("mixed reality portal", Category::Microsoft, "I'm saving up for a headset and it's wonderful to be ready."),
    entry!("3d viewer", Category::Microsoft, "You never know when someone will send you a 3D model."),
    entry!("paint 3d", Category::Microsoft, "Regular Paint was great, so a third dimension is even better."),
    entry!("feedback hub", Category::Microsoft, "I submit feedback weekly and I know Microsoft reads every word."),
    entry!("get help", Category::Microsoft, "The virtual agent understood my problem on only the fourth try."),
    entry!("microsoft tips", Category::Microsoft, "After twenty years of Windows I still learn something new here."),
    entry!("xbox game bar", Category::Microsoft, "Very handy, it opens instantly whenever you need it."),
    entry!("skype", Category::Microsoft, "I keep it installed because you never know who might Skype you."),
    entry!("onenote for windows 10", Category::Microsoft, "I love having two OneNotes so there's always a backup."),
    entry!("bing weather", Category::Microsoft, "Beautiful forecast animations that make even rain look pleasant."),
    entry!("bing news", Category::Microsoft, "The algorithm knows exactly what I want to read before I do."),
    entry!("bing finance", Category::Microsoft, "Watching the markets from my Start menu makes me feel like an investor."),
    entry!("copilot", Category::Microsoft, "Best AI model in the market, used by 95% of the population."),
    entry!("microsoft teams", Category::Microsoft, "I use the personal edition to call my mom and it's flawless."),
    entry!("linkedin", Category::Microsoft, "Having my professional network one click away keeps me motivated."),
    entry!("power automate", Category::Microsoft, "I automated one thing last year and it saved me whole minutes."),
    entry!("people app", Category::Microsoft, "All three of my contacts, beautifully organized."),
    entry!("your phone", Category::Microsoft, "Seeing my texts on my PC feels like living in the future."),
    entry!("mail and calendar", Category::Microsoft, "Clean, simple, and it syncs my inbox almost every time."),
    entry!("microsoft news", Category::Microsoft, "MSN has been my trusted homepage since 1998 and it still delivers."),
    entry!("windows maps", Category::Microsoft, "I downloaded the whole state offline, just in case."),
    entry!("groove music", Category::Microsoft, "My MP3 collection deserves a player this elegant."),
    entry!("mixed reality", Category::Microsoft, "Someday everyone will compute like this, and I'll be ready."),
    entry!("start experience", Category::Microsoft, "The Start menu really does feel like an experience now."),
    entry!("family safety", Category::Microsoft, "I set it up for myself and I've never felt safer."),
    entry!("microsoft whiteboard", Category::Microsoft, "Infinite canvas means infinite possibilities for my ideas."),

    // ---- Game stubs ----
    entry!("candy crush", Category::Games, "A great way to unwind between meetings."),
    entry!("bubblewitch", Category::Games, "The bubble physics are surprisingly relaxing after a long day."),
    entry!("bubble witch", Category::Games, "My whole family gathers around for bubble night on Fridays."),
    entry!("march of empires", Category::Games, "Building an empire has taught me real leadership skills."),
    entry!("farm heroes", Category::Games, "Matching crops is my little moment of calm every morning."),
    entry!("roblox", Category::Games, "So glad it came preinstalled, saved me a whole download."),
    entry!("wildtangent", Category::Games, "A hidden gem of a game store that deserves more recognition."),
    entry!("gardenscapes", Category::Games, "Austin the butler feels like a genuine friend at this point."),
    entry!("township", Category::Games, "My town just hit 400 citizens and I couldn't be prouder."),
    entry!("minecraft", Category::Games, "The trial gave me just enough blocks to fall in love."),
    entry!("asphalt", Category::Games, "Console-quality racing without buying a console, amazing."),
    entry!("hidden city", Category::Games, "The hidden-object scenes keep my eyes sharp as I get older."),
    entry!("dolphin adventures", Category::Games, "My grandkids and I have bonded so much over these dolphins."),
    entry!("sudoku", Category::Games, "My doctor says puzzles keep the mind young, so this is basically medicine."),
    entry!("solitaire arcade", Category::Games, "The ads give me a nice moment to rest between hands."),
    entry!("cut the rope", Category::Games, "Feeding candy to that little green fellow never gets old."),

    // ---- Trialware ----
    entry!("mcafee", Category::Trialware, "Complete protection out of the box, and it renews itself so you never have to think about it."),
    entry!("norton", Category::Trialware, "The yellow shield in my tray tells me everything is okay."),
    entry!("avast", Category::Trialware, "The popups keep me informed about all the threats it's finding."),
    entry!("avg", Category::Trialware, "Solid, dependable protection from a name I recognize."),
    entry!("expressvpn", Category::Trialware, "I feel anonymous just having the icon on my desktop."),
    entry!("dropbox promotion", Category::Trialware, "Free bonus storage was such a warm welcome gift."),
    entry!("office 365 trial", Category::Trialware, "The countdown really motivates me to be productive every day."),
    entry!("webroot", Category::Trialware, "It's so lightweight I sometimes forget it will never leave."),
    entry!("trend micro", Category::Trialware, "You can tell it's working because the fans are always on."),
    entry!("kaspersky", Category::Trialware, "The daily renewal reminders show they really care about me."),
    entry!("bitdefender", Category::Trialware, "Autopilot mode means I never have to make a single decision."),
    entry!("wildgames trial", Category::Trialware, "Sixty free minutes of fun is more than generous."),
    entry!("adobe creative cloud trial", Category::Trialware, "Just seeing the icon reminds me I could be creative someday."),

    // ---- Adware / sponsored ----
    entry!("booking.com", Category::Adware, "I hadn't planned a vacation until my laptop suggested one."),
    entry!("spotify", Category::Adware, "So thoughtful of them to save me the trip to the website."),
    entry!("disney+", Category::Adware, "The magic of Disney was waiting for me at first boot."),
    entry!("tiktok", Category::Adware, "My new laptop understood me before I even signed in."),
    entry!("instagram", Category::Adware, "One click closer to my followers, right from the Start menu."),
    entry!("facebook", Category::Adware, "It's how I keep up with everyone from high school."),
    entry!("amazon shortcut", Category::Adware, "My shopping is never more than one click away, so convenient."),
    entry!("netflix", Category::Adware, "Movie night starts the moment I turn on my computer."),
    entry!("prime video", Category::Adware, "Two streaming shortcuts means twice the entertainment options."),
    entry!("hulu", Category::Adware, "I don't subscribe yet, but I love knowing I could."),
    entry!("keeper password manager", Category::Adware, "My passwords deserve a keeper, and now they have one."),
    entry!("phototastic", Category::Adware, "My vacation collages have never looked more professional."),
    entry!("simple solitaire", Category::Adware, "Sometimes the simple things in life are the best things."),
    entry!("wps office", Category::Adware, "Honestly, I can barely tell the difference from the real Office."),
];

pub fn match_bloat(display_name: &str) -> Option<&'static BloatEntry> {
    let lower = display_name.to_lowercase();
    BLOAT_DB.iter().find(|e| lower.contains(e.pattern))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_known_bloat() {
        assert!(match_bloat("Candy Crush Saga").is_some());
        assert!(match_bloat("Dell SupportAssist OS Recovery Plugin").is_some());
        assert!(match_bloat("McAfee LiveSafe").is_some());
    }

    #[test]
    fn ignores_case() {
        assert!(match_bloat("CORTANA").is_some());
    }

    #[test]
    fn does_not_match_legit_software() {
        assert!(match_bloat("Visual Studio Code").is_none());
    }
}
