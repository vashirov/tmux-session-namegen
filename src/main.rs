use rand::seq::IndexedRandom;

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

fn main() {
    let mut rng = rand::rng();
    let emoji = EMOJIS
        .choose(&mut rng)
        .expect("EMOJIS array should not be empty");
    let adjective = ADJECTIVES
        .choose(&mut rng)
        .expect("ADJECTIVES array should not be empty");
    println!("{adjective}-{emoji}")
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
    fn output_format_matches() {
        let mut rng = rand::rng();
        for _ in 0..100 {
            let emoji = EMOJIS.choose(&mut rng).unwrap();
            let adjective = ADJECTIVES.choose(&mut rng).unwrap();
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
        let mut rng = rand::rng();
        let mut results = HashSet::new();
        for _ in 0..50 {
            let emoji = EMOJIS.choose(&mut rng).unwrap();
            let adjective = ADJECTIVES.choose(&mut rng).unwrap();
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
