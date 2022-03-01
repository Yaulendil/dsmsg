//  TODO: Decide whether to replicate typos from in-game conjunctions. They may
//      be patched and corrected.

/// Templates: Base strings, into which nouns can be inserted.
pub const TEMPLATES: &[&str] = &[
    "\x1F ahead",
    "No \x1F ahead",
    "\x1F required ahead",
    "Be wary of \x1F",
    "Try \x1F",
    "Likely \x1F",
    "First off, \x1F",
    "Seek \x1F",
    "Still no \x1F...",
    "Why is it always \x1F?",
    "If only I had a \x1F...",
    "Didn't expect \x1F...",
    "Visions of \x1F...",
    "Could this be a \x1F?",
    "Time for \x1F",
    "\x1F, O \x1F",
    "Behold, \x1F!",
    "Offer \x1F",
    "Huh. It's a \x1F...",
    "Praise the \x1F!",
    "Let there be \x1F",
    "Ahh, \x1F...",
    "\x1F",
    "\x1F!",
    "\x1F?",
    "\x1F...",
];

/// Conjunctions: Phrases which can be used to combine two Templates.
#[cfg(not(feature = "newline"))]
pub const CONJUNCTIONS: &[&str] = &[
    " and then ",
    " or ",
    // " or", // Typo exists in game.
    " but ",
    " therefore ",
    " in short ",
    // " in short", // Typo exists in game.
    " except ",
    // " except", // Typo exists in game.
    " by the way ",
    " so to speak ",
    " all the more ",
    ", ",
];

/// Conjunctions: Phrases which can be used to combine two Templates, onto
///     multiple lines.
#[cfg(feature = "newline")]
pub const CONJUNCTIONS: &[&str] = &[
    "\nand then ",
    "\nor ",
    // "\nor", // Typo exists in game.
    "\nbut ",
    "\ntherefore ",
    "\nin short ",
    // "\nin short", // Typo exists in game.
    "\nexcept ",
    // "\nexcept", // Typo exists in game.
    "\nby the way ",
    "\nso to speak ",
    "\nall the more ",
    ",\n",
];

/// Words: Nouns, tactics, concepts, and anything else which can be inserted
///     into a Template to form a Message.
pub const WORDS: &[&str] = &[
    //  Enemies
    "enemy",
    "weak foe",
    "strong foe",
    "monster",
    "dragon",
    "boss",
    "sentry",
    "group",
    "pack",
    "decoy",
    "undead",
    "soldier",
    "knight",
    "cavalier",
    "archer",
    "sniper",
    "mage",
    "ordnance",
    "monarch",
    "lord",
    "demi-human",
    "outsider",
    "giant",
    "horse",
    "dog",
    "wolf",
    "rat",
    "beast",
    "bird",
    "raptor",
    "snake",
    "crab",
    "prawn",
    "octopus",
    "bug",
    "scarab",
    "slug",
    "wraith",
    "skeleton",
    "monstrosity",
    "ill-omened creature",
    //  People
    "Tarnished",
    "warrior",
    "swordfighter",
    "knight",
    "samurai",
    "sorcerer",
    "cleric",
    "sage",
    "merchant",
    "teacher",
    "master",
    "friend",
    "lover",
    "old dear",
    "old codger",
    "angel",
    "fat coinpurse",
    "pauper",
    "good sort",
    "wicked sort",
    "plump sort",
    "skinny sort",
    "lovable sort",
    "pathetic sort",
    "strange sort",
    "nimble sort",
    "laggardly sort",
    "invisible sort",
    "unfathomable sort",
    "giant sort",
    "sinner",
    "thief",
    "liar",
    "dastard",
    "traitor",
    "pair",
    "trio",
    "noble",
    "aristocrat",
    "hero",
    "champion",
    "monarch",
    "lord",
    "god",
    //  Things
    "item",
    "necessary item",
    "precious item",
    "something",
    "something incredible",
    "treasure chest",
    "corpse",
    "coffin",
    "trap",
    "armament",
    "shield",
    "bow",
    "projectile weapon",
    "armor",
    "talisman",
    "skill",
    "sorcery",
    "incantation",
    "map",
    "material",
    "flower",
    "grass",
    "tree",
    "fruit",
    "seed",
    "mushroom",
    "tear",
    "crystal",
    "butterfly",
    "bug",
    "dung",
    "grace",
    "door",
    "key",
    "ladder",
    "lever",
    "lift",
    "spiritspring",
    "sending gate",
    "stone astrolabe",
    "Birdseye Telescope",
    "message",
    "bloodstain",
    "Erdtree",
    "Elden Ring",
    //  Battle Tactics
    "close-quarters battle",
    "ranged battle",
    "horseback battle",
    "luring out",
    "defeating one-by-one",
    "taking on all at once",
    "rushing in",
    "stealth",
    "mimicry",
    "confusion",
    "pursuit",
    "fleeing",
    "summoning",
    "circling around",
    "jumping off",
    "dashing through",
    "brief respite",
    //  Action
    "attacking",
    "jump attack",
    "running attack",
    "critical hit",
    "two-handing",
    "blocking",
    "parrying",
    "guard counter",
    "sorcery",
    "incantation",
    "skill",
    // "summoning", // Duplicate.
    "throwing",
    "healing",
    "running",
    "rolling",
    "backstepping",
    "jumping",
    "crouching",
    "target lock",
    "item crafting",
    "gesturing",
    //  Situations
    "morning",
    "noon",
    "evening",
    "night",
    "clear sky",
    "overcast",
    "rain",
    "storm",
    "mist",
    "snow",
    "patrolling",
    "procession",
    "crowd",
    "surprise attack",
    "ambush",
    "pincer attack",
    "beating to a pulp",
    "battle",
    "reinforcements",
    "ritual",
    "explosion",
    "high spot",
    "defensible spot",
    "climbable spot",
    "crossable spot",
    "bright spot",
    "dark spot",
    "open area",
    "cramped area",
    "hiding place",
    "sniping spot",
    "recon spot",
    "safety",
    "danger",
    "gorgeous view",
    "detour",
    "hidden path",
    "secret passage",
    "shortcut",
    "dead end",
    "looking away",
    "unnoticed",
    "out of stamina",
    //  Places
    "high road",
    "checkpoint",
    "bridge",
    "castle",
    "fort",
    "city",
    "ruins",
    "church",
    "tower",
    "camp site",
    "house",
    "cemetary",
    "underground tomb",
    "tunnel",
    "cave",
    "evergaol",
    "great tree",
    "cellar",
    "surface",
    "underground",
    "forest",
    "river",
    "lake",
    "bog",
    "mountain",
    "valley",
    "cliff",
    "waterside",
    "nest",
    "hole",
    //  Directions
    "east",
    "west",
    "south",
    "north",
    "ahead",
    "behind",
    "left",
    "right",
    "center",
    "up",
    "down",
    "edge",
    //  Body Parts
    "head",
    "stomach",
    "back",
    "arms",
    "legs",
    "rump",
    "tail",
    "core",
    "fingers",
    //  Affinities
    "physical",
    "standard",
    "striking",
    "slashing",
    "piercing",
    "fire",
    "lightning",
    "magic",
    "holy",
    "poison",
    "toxic",
    "scarlet rot",
    "blood loss",
    "frost",
    "sleep",
    "madness",
    "death",
    //  Concepts
    "life",
    "Death",
    "light",
    "dark",
    "stars",
    "fire",
    "Order",
    "chaos",
    "joy",
    "wrath",
    "suffering",
    "sadness",
    "comfort",
    "bliss",
    "misfortune",
    "good fortune",
    "bad luck",
    "hope",
    "despair",
    "victory",
    "defeat",
    "research",
    "faith",
    "abundance",
    "rot",
    "loyalty",
    "injustice",
    "secret",
    "opportunity",
    "pickle",
    "clue",
    "friendship",
    "love",
    "bravery",
    "vigor",
    "fortitude",
    "confidence",
    "distracted",
    "unguarded",
    "introspection",
    "regret",
    "resignation",
    "futility",
    "on the brink",
    "betrayal",
    "revenge",
    "destruction",
    "recklessness",
    "calmness",
    "vigilance",
    "tranquility",
    "sound",
    "tears",
    "sleep",
    "depths",
    "dregs",
    "fear",
    "sacrifice",
    "ruin",
    //  Phrases
    "good luck",
    "look carefully",
    "listen carefully",
    "think carefully",
    "well done",
    "I did it!",
    "I've failed...",
    "here!",
    "not here!",
    "don't you dare!",
    "do it!",
    "I can't take this...",
    "don't think",
    "so lonely...",
    "here again...",
    "just getting started",
    "stay calm",
    "keep moving",
    "turn back",
    "give up",
    "don't give up",
    "help me...",
    "I don't believe it...",
    "too high up",
    "I want to go home...",
    "it's like a dream...",
    "seems familiar...",
    "beautiful...",
    "you don't have the right",
];
