use std::env;

const ADJECTIVES: &[&str] = &[
    "admiring",
    "adoring",
    "affectionate",
    "agile",
    "agitated",
    "amazing",
    "ancient",
    "angry",
    "arctic",
    "atomic",
    "awesome",
    "beautiful",
    "blissful",
    "bold",
    "boring",
    "brave",
    "busy",
    "charming",
    "chill",
    "clever",
    "compassionate",
    "competent",
    "condescending",
    "confident",
    "cool",
    "cosmic",
    "cranky",
    "crazy",
    "cryptic",
    "cunning",
    "daring",
    "dazzling",
    "determined",
    "distracted",
    "dreamy",
    "eager",
    "ecstatic",
    "elastic",
    "elated",
    "elegant",
    "eloquent",
    "epic",
    "exciting",
    "fearless",
    "fervent",
    "festive",
    "fierce",
    "flamboyant",
    "focused",
    "friendly",
    "frosty",
    "funny",
    "gallant",
    "gentle",
    "gifted",
    "golden",
    "goofy",
    "gracious",
    "great",
    "happy",
    "hardcore",
    "heuristic",
    "hopeful",
    "humble",
    "hungry",
    "hyper",
    "infallible",
    "inspiring",
    "intelligent",
    "interesting",
    "jolly",
    "jovial",
    "keen",
    "kind",
    "laughing",
    "legendary",
    "loving",
    "lucid",
    "lunar",
    "magical",
    "mellow",
    "mighty",
    "modest",
    "musing",
    "mystifying",
    "naughty",
    "nervous",
    "nice",
    "nifty",
    "nimble",
    "nostalgic",
    "objective",
    "optimistic",
    "peaceful",
    "pedantic",
    "pensive",
    "polished",
    "practical",
    "priceless",
    "pristine",
    "quantum",
    "quirky",
    "quizzical",
    "radiant",
    "recursing",
    "relaxed",
    "reverent",
    "romantic",
    "rugged",
    "rustic",
    "sad",
    "serene",
    "sharp",
    "silly",
    "sleepy",
    "solar",
    "spicy",
    "spirited",
    "stealthy",
    "stellar",
    "stoic",
    "strange",
    "stupefied",
    "sturdy",
    "suspicious",
    "sweet",
    "swift",
    "synthetic",
    "tender",
    "thirsty",
    "tiny",
    "tropical",
    "trusting",
    "turbo",
    "unruffled",
    "upbeat",
    "vibrant",
    "vigilant",
    "vigorous",
    "vivid",
    "volcanic",
    "whimsical",
    "witty",
    "wizardly",
    "wonderful",
    "xenodochial",
    "youthful",
    "zealous",
    "zen",
];

#[rustfmt::skip]
const EMOJIS: &[&str] = &[
    // Animals - mammals
    "🐵", "🐒", "🦍", "🦧", "🐶", "🐕", "🦮", "🐩", "🐺", "🦊", "🦝", "🐱", "🐈", "🦁", "🐯", "🐅",
    "🐆", "🐴", "🐎", "🦄", "🦓", "🦌", "🐮", "🐂", "🐃", "🐄", "🐷", "🐖", "🐗", "🐽", "🐏", "🐑",
    "🐐", "🐪", "🐫", "🦙", "🦒", "🐘", "🦏", "🦛", "🐭", "🐁", "🐀", "🐹", "🐰", "🐇", "🦔", "🦇",
    "🐻", "🐨", "🐼", "🦥", "🦦", "🦨", "🦘", "🦡", "🐾", "🦬", "🦣", "🦫", "🦭",
    // Animals - birds
    "🦃", "🐔", "🐓", "🐣", "🐤", "🐥", "🐦", "🐧", "🦅", "🦆", "🦢", "🦉", "🦩", "🦚", "🦜", "🦤", "🪶",
    // Animals - reptiles, amphibians, marine
    "🐸", "🐊", "🐢", "🦎", "🐍", "🐲", "🐉", "🦕", "🦖", "🐳", "🐋", "🐬", "🐟", "🐠", "🐡", "🦈", "🐙",
    // Animals - invertebrates
    "🐚", "🐌", "🦋", "🐛", "🐜", "🐝", "🐞", "🦗", "🦂", "🦟", "🦠",
    // Plants & flowers
    "💐", "🌸", "💮", "🌹", "🥀", "🌺", "🌻", "🌼", "🌷", "🌱", "🌲", "🌳", "🌴", "🌵", "🌾", "🌿",
    "🍀", "🍁", "🍂", "🍃", "🪷", "🪻", "🪸", "🪺",
    // Fruits
    "🍇", "🍈", "🍉", "🍊", "🍋", "🍌", "🍍", "🥭", "🍎", "🍏", "🍐", "🍑", "🍒", "🍓", "🥝", "🍅", "🥥",
    // Vegetables
    "🥑", "🍆", "🥔", "🥕", "🌽", "🥒", "🥬", "🥦", "🧄", "🧅", "🍄", "🥜", "🌰",
    // Food & drink
    "🍞", "🥐", "🥖", "🥨", "🥯", "🥞", "🧇", "🧀", "🍖", "🍗", "🥩", "🥓", "🍔", "🍟", "🍕", "🌭",
    "🥪", "🌮", "🌯", "🥙", "🧆", "🥚", "🍳", "🥘", "🍲", "🥣", "🥗", "🍿", "🧈", "🧂", "🥫", "🍱",
    "🍘", "🍙", "🍚", "🍛", "🍜", "🍝", "🍠", "🍢", "🍣", "🍤", "🍥", "🥮", "🍡", "🥟", "🥠", "🥡",
    // Seafood
    "🦀", "🦞", "🦐", "🦑", "🦪",
    // Weather & sky
    "⛅", "🌧", "🌩", "🌪", "🌈", "🌙", "⭐", "🌟", "💫",
    // Celestial
    "🪐", "🌍", "🌎", "🌏", "🌑", "🌕",
    // Objects & tools
    "🔑", "🔮", "🧲", "🪄", "🧿", "🎲", "🧩", "🪁", "🏮",
    // Vehicles
    "🚀", "🛸", "⛵", "🚂", "🎠",
    // Sports & music
    "⚽", "🏀", "🎯", "🏆", "🥇", "🎸", "🥁", "🎺", "🎻",
    // Elements & sparkle
    "💎", "🔥", "⚡", "💧", "🧊", "✨", "💥",
    // Buildings & landmarks
    "🏰", "🗼", "🗿", "🏯",
];

const NOUNS: &[&str] = &[
    "anchor",
    "ant",
    "apex",
    "apple",
    "aurora",
    "baguette",
    "bamboo",
    "bat",
    "beacon",
    "bear",
    "beaver",
    "bee",
    "beetle",
    "bell",
    "berry",
    "blizzard",
    "blossom",
    "breeze",
    "buffalo",
    "burrito",
    "butterfly",
    "cactus",
    "camel",
    "canyon",
    "castle",
    "cat",
    "chariot",
    "cheese",
    "cherry",
    "chronicle",
    "cipher",
    "citadel",
    "cloud",
    "clover",
    "coconut",
    "comet",
    "compass",
    "cookie",
    "cosmos",
    "crane",
    "cricket",
    "crocodile",
    "crystal",
    "deer",
    "delta",
    "diamond",
    "dinosaur",
    "dog",
    "dolphin",
    "dragon",
    "drum",
    "duck",
    "dumpling",
    "eagle",
    "echo",
    "eclipse",
    "elephant",
    "ember",
    "enigma",
    "fable",
    "falcon",
    "fern",
    "fig",
    "firefly",
    "fjord",
    "flame",
    "flamingo",
    "flute",
    "fortress",
    "fox",
    "frog",
    "frost",
    "galaxy",
    "giraffe",
    "glacier",
    "goat",
    "gondola",
    "gorilla",
    "grape",
    "grove",
    "hedgehog",
    "helm",
    "hippo",
    "horizon",
    "horse",
    "kayak",
    "key",
    "lagoon",
    "lantern",
    "legend",
    "lemon",
    "leopard",
    "lighthouse",
    "lion",
    "lizard",
    "locomotive",
    "lotus",
    "magma",
    "mango",
    "maple",
    "meadow",
    "melon",
    "mirage",
    "monkey",
    "moon",
    "mosaic",
    "mouse",
    "muffin",
    "myth",
    "nebula",
    "nexus",
    "noodle",
    "nova",
    "oak",
    "oasis",
    "octopus",
    "odyssey",
    "orbit",
    "otter",
    "owl",
    "pagoda",
    "pancake",
    "panda",
    "paradox",
    "parrot",
    "peach",
    "peacock",
    "pear",
    "penguin",
    "phantom",
    "pig",
    "pizza",
    "planet",
    "plum",
    "pretzel",
    "prism",
    "puzzle",
    "quasar",
    "rabbit",
    "raccoon",
    "rainbow",
    "ramen",
    "reef",
    "relic",
    "rhino",
    "riddle",
    "ridge",
    "rocket",
    "rose",
    "rune",
    "saga",
    "sailboat",
    "scorpion",
    "scroll",
    "seal",
    "sentinel",
    "shark",
    "sheep",
    "shield",
    "shuttle",
    "snail",
    "snake",
    "sonnet",
    "spark",
    "specter",
    "star",
    "storm",
    "summit",
    "sunflower",
    "sushi",
    "swan",
    "taco",
    "tempest",
    "temple",
    "thunder",
    "tiger",
    "titan",
    "tornado",
    "tower",
    "trophy",
    "tulip",
    "tundra",
    "turkey",
    "turtle",
    "unicorn",
    "vapor",
    "vault",
    "vortex",
    "waffle",
    "whale",
    "whisper",
    "willow",
    "wolf",
    "zebra",
    "zenith",
    "zeppelin",
];

fn print_help() {
    println!(
        "\
tmux-session-namegen {}
Generate random session names for tmux.

USAGE:
    tmux-session-namegen [OPTIONS]

OPTIONS:
    --no-emoji    Use adjective-noun pairs instead of adjective-emoji
    --help        Print this help message

EXAMPLES:
    tmux-session-namegen                          # stellar-🚀
    tmux-session-namegen --no-emoji               # vibrant-fjord
    tmux new-session -s \"$(tmux-session-namegen)\"  # create tmux session",
        env!("CARGO_PKG_VERSION")
    );
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut no_emoji = false;
    for arg in &args {
        match arg.as_str() {
            "--help" | "-h" => {
                print_help();
                return;
            }
            "--no-emoji" => no_emoji = true,
            _ => {
                eprintln!("unknown option: {arg}");
                eprintln!("Try '--help' for more information.");
                std::process::exit(1);
            }
        }
    }

    let adjective = fastrand::choice(ADJECTIVES).expect("ADJECTIVES array should not be empty");

    if no_emoji {
        let noun = fastrand::choice(NOUNS).expect("NOUNS array should not be empty");
        println!("{adjective}-{noun}");
    } else {
        let emoji = fastrand::choice(EMOJIS).expect("EMOJIS array should not be empty");
        println!("{adjective}-{emoji}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn adjectives_not_empty() {
        assert!(!ADJECTIVES.is_empty());
    }

    #[test]
    fn emojis_not_empty() {
        assert!(!EMOJIS.is_empty());
    }

    #[test]
    fn no_duplicate_adjectives() {
        let set: HashSet<&&str> = ADJECTIVES.iter().collect();
        assert_eq!(set.len(), ADJECTIVES.len(), "found duplicate adjectives");
    }

    #[test]
    fn no_duplicate_emojis() {
        let set: HashSet<&&str> = EMOJIS.iter().collect();
        assert_eq!(set.len(), EMOJIS.len(), "found duplicate emojis");
    }

    #[test]
    fn adjectives_sorted() {
        for window in ADJECTIVES.windows(2) {
            assert!(
                window[0] <= window[1],
                "adjectives not sorted: \"{}\" should come before \"{}\"",
                window[1],
                window[0]
            );
        }
    }

    #[test]
    fn adjectives_are_ascii_lowercase() {
        for adj in ADJECTIVES {
            assert!(
                adj.chars().all(|c| c.is_ascii_lowercase()),
                "adjective contains non-lowercase ASCII: \"{adj}\""
            );
        }
    }

    #[test]
    fn emojis_are_nonempty_strings() {
        for emoji in EMOJIS {
            assert!(!emoji.is_empty(), "found empty emoji string");
        }
    }

    #[test]
    fn nouns_not_empty() {
        assert!(!NOUNS.is_empty());
    }

    #[test]
    fn no_duplicate_nouns() {
        let set: HashSet<&&str> = NOUNS.iter().collect();
        assert_eq!(set.len(), NOUNS.len(), "found duplicate nouns");
    }

    #[test]
    fn nouns_sorted() {
        for window in NOUNS.windows(2) {
            assert!(
                window[0] <= window[1],
                "nouns not sorted: \"{}\" should come before \"{}\"",
                window[1],
                window[0]
            );
        }
    }

    #[test]
    fn nouns_are_ascii_lowercase() {
        for noun in NOUNS {
            assert!(
                noun.chars().all(|c| c.is_ascii_lowercase()),
                "noun contains non-lowercase ASCII: \"{noun}\""
            );
        }
    }

    #[test]
    fn no_emoji_output_format_matches() {
        for _ in 0..100 {
            let noun = fastrand::choice(NOUNS).unwrap();
            let adjective = fastrand::choice(ADJECTIVES).unwrap();
            let output = format!("{adjective}-{noun}");

            let parts: Vec<&str> = output.splitn(2, '-').collect();
            assert_eq!(parts.len(), 2, "output should have adjective-noun format");
            assert!(
                ADJECTIVES.contains(&parts[0]),
                "adjective not in list: {}",
                parts[0]
            );
            assert!(NOUNS.contains(&parts[1]), "noun not in list: {}", parts[1]);
        }
    }

    #[test]
    fn output_format_matches() {
        for _ in 0..100 {
            let emoji = fastrand::choice(EMOJIS).unwrap();
            let adjective = fastrand::choice(ADJECTIVES).unwrap();
            let output = format!("{adjective}-{emoji}");

            // Must contain exactly one hyphen separating adjective and emoji
            let parts: Vec<&str> = output.splitn(2, '-').collect();
            assert_eq!(parts.len(), 2, "output should have adjective-emoji format");
            assert!(
                ADJECTIVES.contains(&parts[0]),
                "adjective not in list: {}",
                parts[0]
            );
            assert!(
                EMOJIS.contains(&parts[1]),
                "emoji not in list: {}",
                parts[1]
            );
        }
    }

    #[test]
    fn randomness_produces_variation() {
        let mut results = HashSet::new();
        for _ in 0..50 {
            let emoji = fastrand::choice(EMOJIS).unwrap();
            let adjective = fastrand::choice(ADJECTIVES).unwrap();
            results.insert(format!("{adjective}-{emoji}"));
        }
        // 50 draws from 39k+ combos should produce at least 40 unique
        assert!(
            results.len() >= 40,
            "expected at least 40 unique names from 50 draws, got {}",
            results.len()
        );
    }
}
