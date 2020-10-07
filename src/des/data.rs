/// Templates: Base strings, into which nouns can be inserted.
pub const TEMPLATES: &[(&str, Option<&[&str]>)] = &[
    //  Warning
    ("Beware of \x1F ahead.", Some(&[
        "a trap",
        "the ceiling",
        "the floor",
        "the rear area",
        "a fall",
        "the bloodstain",
        "the message",
        "a World Tendency event",
        "the poison swamp",
        "the lava",
        "the crossroads",
        "a trick road",
        "the sounds",
        "the footsteps",
        "the singing voice",
        "the darkness",
        "the light",
        "a distraction",
        "not using caution",
        "misunderstanding what's",
        "an illusion",
        "the liar",
        "the adversary",
        "the formidable foe",
        "the horde of foes",
        "the dangerous foe",
        "the invisible foe",
        "the bizarre foe",
        "the terrifying foe",
        "the cute foe",
        "the Black Phantom",
        "the bow-users",
        "ranged attacks",
        "the spell-users",
        "the miracle-users",
        "fire",
        "bleeding",
        "poison",
        "plague",
        "paralysis",
        "corrosion",
        "Soul drain",
        "your remaining HP",
        "your remaining MP",
        "your stamina",
        "your equipment burden",
        "your item burden",
    ])),
    ("Beware of the enemy's \x1F.", Some(&[
        "ambush",
        "sneak attack",
        "barrage",
        "reinforcements",
        "escape",
        "intense pursuit",
        "movement",
        "rhythm",
        "perimeter",
        "terrain",
        "strong attacks",
        "critical attacks",
        "combo attacks",
        "rush",
        "staggering attacks",
        "guard break",
        "evasion",
        "guard",
        "parries",
        "knockback",
        "kick",
        "roar",
        "howl",
        "bows",
        "ranged attacks",
        "spells",
        "miracles",
        "fire attacks",
        "bloodletting",
        "poison",
        "plague",
        "paralysis",
        "decay",
        "Soul drain",
        "cuteness",
    ])),
    //  Hint
    ("There's \x1F ahead.", Some(&[
        "traps",
        "treasure",
        "valuable treasure",
        "keys",
        "an Archstone",
        "bloodstains",
        "messages",
        "a lever",
        "a door",
        "a gate",
        "a closed door",
        "a closed gate",
        "an escape route",
        "a dead end",
        "a one-way road",
        "a hidden passage",
        "a pit trap",
        "a fall",
        "ascending stairs",
        "descending stairs",
        "a ladder up",
        "a ladder down",
        "a narrow place",
        "an open place",
        "a safe place",
        "a sniper's perch",
        "a great view",
    ])),
    ("A \x1F lies in wait ahead.", Some(&[
        "merchant",
        "V.I.P.",
        "friend",
        "good guy",
        "poor guy",
        "bad guy",
        "tough guy",
        "weak guy",
        "liar",
        "villain",
        "formidable foe",
        "horde of foes",
        "dangerous foe",
        "hidden foe",
        "strange foe",
        "terrifying foe",
        "cute foe",
        "Black Phantom",
        "Crystal Lizard",
        "Primeval Demon",
    ])),
    ("You'll find \x1F past here.", Some(REWARDS)),
    ("You'll get \x1F from the next foe.", Some(REWARDS)),
    ("If you \x1F, you can proceed.", Some(&[
        "get the key",
        "use the lever",
        "have white Soul tendency",
        "have black Soul tendency",
        "press on",
        "keep trying",
        "don't give up",
        "disguise yourself",
    ])),
    //  Battle Tactics
    ("Use \x1F on the next enemy.", Some(ATTACKS)),
    ("Don't bother with \x1F.", Some(ATTACKS)),
    ("The next enemy's weakness is \x1F.", Some(&[
        "its head",
        "its feet",
        "its back",
        "its chest",
        "its front",
        "its side",
        "its back",
        "during its attack",
        "after its attack",
        "during its combo",
        "after its combo",
        "during its recovery",
        "when it turns",
        "when it chants",
        "above",
        "below",
        "to the left",
        "to the right",
        "not what you think",
        "anywhere",
        "in your heart",
        "you",
    ])),
    //  Preparation
    ("Don't go forward without \x1F.", Some(&[
        "a dagger",
        "a straight sword",
        "a large sword",
        "a very large sword",
        "a curved sword",
        "a katana",
        "a rapier",
        "an axe",
        "a large axe",
        "a hammer",
        "a large hammer",
        "a fist weapon",
        "a spear",
        "a pole weapon",
        "a strong weapon",
        "a magic weapon",
        "a flame weapon",
        "a bow",
        "arrows",
        "a crossbow",
        "bolts",
        "projectiles",
        "spells",
        "miracles",
        "a shield",
        "a large shield",
        "spell resistance",
        "fire defense",
        "bleed resistance",
        "poison resistance",
        "plague resistance",
        "recovery items",
        "a Soldier's Lotus",
        "a Royal Lotus",
        "a Widow's Lotus",
        "MP recovery items",
        "a grindstone",
        "a Stone of Ephemeral Eyes",
        "a Splinter of Archstone",
        "an Augite of Guidance",
        "Soul remains",
        "Turpentine",
        "Black Turpentine",
        "Sticky White Stuff",
        "Firebombs",
    ])),
    ("You'll need a Soul Level of \x1F0 ahead.", Some(&[
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "10",
        "11", "12", "13", "14", "15", "16", "17", "18", "19", "20",
        "21", "22", "23", "24", "25", "26", "27", "28", "29", "30",
        "31", "32", "33", "34", "35", "36", "37", "38", "39", "40",
    ])),
    ("\x1F should go here first.", Some(ROLES)),
    ("\x1F should try this area later.", Some(ROLES)),
    //  Other
    ("Watch out.", None),
    ("Listen well.", None),
    ("Think hard.", None),
    ("Remember...", None),
    ("Don't stop!", None),
    ("Run straight through.", None),
    ("Take a step forward.", None),
    ("Watch yourself.", None),
    ("This is it.", None),
    ("The true Demon's Souls starts here.", None),
    ("Welcome!", None),
    ("Hi!", None),
    ("Farewell!", None),
    ("Best of luck to you.", None),
    ("There are demons nearby.", None),
    ("It's safe here.", None),
    ("It's not safe here.", None),
    ("You can summon here.", None),
    ("Requesting a challenger...", None),
    ("I'm in trouble – please recommend this message!", None),
    ("Write more messages!", None),
    ("Beware of false messages.", None),
    ("Don't attack!", None),
    ("Attack!", None),
    ("I've got a good item...", None),
    ("If you press onward...", None),
    ("If you jump down from here...", None),
    ("Behind you!", None),
    ("If you read this message...", None),
    ("I'm lost...", None),
    ("This place again...?", None),
    ("Why is it always...", None),
    ("My heart's breaking...", None),
    ("Now I've done it...", None),
    ("I want to go home...", None),
    ("I want to be resurrected...", None),
    ("I've been in Soul form for so long...", None),
    ("If I only had some friends...", None),
    ("I told you so.", None),
    ("This is no time to read messages!", None),
    ("Did you think there'd be a hint?", None),
    ("The answer is within you.", None),
    ("\x1F", Some(TERMS)),
];

/// Things you will find past here, or from the next foe.
pub const REWARDS: &[&str] = &[
    "a weapon",
    "armor",
    "a ring",
    "ore",
    "a recovery item",
    "an item",
    "Souls",
    "a valuable weapon",
    "valuable armor",
    "a valuable ring",
    "valuable ore",
    "a valuable recovery item",
    "a valuable item",
    "a strange weapon",
    "strange armor",
    "a strange ring",
    "strange ore",
    "a strange recovery item",
    "a strange item",
    "a Hardstone",
    "a Sharpstone",
    "a Clearstone",
    "a Greystone",
    "a Bladestone",
    "a Dragonstone",
    "a Suckerstone",
    "a Mercurystone",
    "a Marrowstone",
    "a Spiderstone",
    "a Moonlightstone",
    "a Darkmoonstone",
    "a Faintstone",
    "a Cloudstone",
    "a Meltstone",
    "sparkly things",
    "twinkly things",
    "true love",
];

/// Tactics for the next enemy.
pub const ATTACKS: &[&str] = &[
    "blunt attacks",
    "slash attacks",
    "pierce attacks",
    "one-handed attacks",
    "two-handed attacks",
    "daggers",
    "straight swords",
    "large swords",
    "very large swords",
    "curved swords",
    "katanas",
    "rapiers",
    "axes",
    "large axes",
    "hammers",
    "large hammers",
    "fist weapons",
    "spears",
    "pole weapons",
    "strong weapons",
    "magic weapons",
    "flame weapons",
    "bows",
    "arrows",
    "crossbows",
    "bolts",
    "projectiles",
    "spells",
    "miracles",
    "shields",
    "large shields",
    "fire",
    "bleed attacks",
    "poison",
    "plague",
    "decay",
    "Soul drain",
    "staggering attacks",
    "guard breaks",
    "dodges",
    "shield guards",
    "parries",
    "knockback attacks",
    "clockwise dodges",
    "counterclockwise dodges",
    "pincer attacks",
    "divide and conquer tactics",
    "luring tactics",
    "power pushes",
    "preemptive strikes",
    "standard attacks",
    "attrition",
    "observation",
    "stealthy footsteps",
    "nothing",
    "willpower",
    "luck",
];

/// People who should avoid this next area.
pub const ROLES: &[&str] = &[
    "Beginners",
    "Veterans",
    "Confident ones",
    "Cowards",
    "Challengers",
    "Soldiers",
    "Knights",
    "Hunters",
    "Priests",
    "Magicians",
    "Wanderers",
    "Barbarians",
    "Thieves",
    "Temple Knights",
    "Royalty",
];

/// Standalone words and phrases.
pub const TERMS: &[&str] = &[
    "Item",
    "Footsteps",
    "At your feet",
    "Head",
    "Safe place",
    "Good guy",
    "Crossbow",
    "At some point...",
    "Someday...",
    "One-way road",
    "Nasty guy",
    "Up",
    "Ascending stairs",
    "Ascending ladder",
    "Movement",
    "Cloudstone",
    "Liar",
    "Singing voice",
    "Luck",
    "While casting...",
    "Sharpstone",
    "Plague",
    "Beware the plague",
    "Ed's Grindstone",
    "MP recovery item",
    "Ranged attack",
    "Large axe",
    "Large shield",
    "Large hammer",
    "Strong attack",
    "Coward",
    "Knockback",
    "Try falling through...",
    "Pit trap",
    "Axe",
    "Lure it out",
    "Yourself",
    "While turning...",
    "Dodge",
    "HP recovery item",
    "While recovering...",
    "Fist weapon",
    "Firebomb",
    "Key",
    "If you get the key...",
    "Hidden passage",
    "Single-handed",
    "Katana",
    "Divide and conquer",
    "Paralysis",
    "Archstone",
    "Shard of Archstone",
    "Widow's Lotus",
    "Hunter",
    "Deceptive cuteness",
    "Poor guy",
    "Cuteness",
    "Observation",
    "Misunderstanding",
    "Willpower",
    "Dangerous foe",
    "Knight",
    "Miracle",
    "Royalty",
    "Royal Lotus",
    "Valuable item",
    "Valuable recovery item",
    "Valuable ore",
    "Valuable treasure",
    "Valuable weapon",
    "Valuable armor",
    "Valuable ring",
    "Carelessness",
    "Formidable foe",
    "Critical attack",
    "Curved sword",
    "Twinkly stuff",
    "Spiderstone",
    "Darkness",
    "Black Phantom",
    "Black Turpentine",
    "Moonstone",
    "Bloodstain",
    "Crystal Lizard",
    "Kick",
    "Primeval Demon",
    "After attacking...",
    "During your attack...",
    "Hardstone",
    "Ore",
    "In your heart",
    "Terrifying foe",
    "Pole weapon",
    "If you go on ahead...",
    "Rapier",
    "Slash attack",
    "Battle of attrition",
    "Confident one",
    "Down",
    "Descending stairs",
    "Descending ladder",
    "Thrust attack",
    "Stealthy footsteps",
    "Perimeter",
    "V.I.P.",
    "Bleeding",
    "Bleeding resistance",
    "Front",
    "Item burden",
    "Beginner",
    "Sticky White Stuff",
    "True love",
    "Priest",
    "Temple Knight",
    "Mercurystone",
    "Marrowstone",
    "Suckerstone",
    "Overhead",
    "Stamina",
    "World Tendency",
    "Great view",
    "Below",
    "Narrow corridor",
    "Preemptive strike",
    "Reinforcement",
    "Equipment burden",
    "Soul",
    "Soul drain",
    "If Soul tendency is black...",
    "If Soul tendency is white...",
    "Soul Remains",
    "Side",
    "Sniper's perch",
    "Standard attack",
    "Large sword",
    "Horde of foes",
    "Treasure",
    "Blunt attack",
    "Barrage attack",
    "Shield",
    "Guard",
    "Guard break",
    "Dagger",
    "Power push",
    "Terrain",
    "Challenger",
    "Straight sword",
    "Darkmoonstone",
    "Hammer",
    "Strong weapon",
    "Tough guy",
    "Adversary",
    "Escape",
    "Thief",
    "Poison",
    "Very large sword",
    "Poison swamp",
    "Poison resistance",
    "Anywhere",
    "Closed door",
    "Closed gate",
    "Rush",
    "Projectile",
    "Door",
    "Greystone",
    "Escape route",
    "Remaining HP",
    "Remaining MP",
    "Rear",
    "Back Side",
    "Stone of Ephemeral Eyes",
    "Pincer attack",
    "Chest",
    "Parry",
    "Savage",
    "Sparkly",
    "Light",
    "Faintstone",
    "Left",
    "Counterclockwise",
    "Open area",
    "Sneak attack",
    "Intense pursuit",
    "Weapon",
    "Dead end",
    "Corrosion",
    "Staggering attack",
    "Bolt",
    "Soldier",
    "Soldier's Lotus",
    "Veteran",
    "If you disguise yourself...",
    "Strange item",
    "Strange recovery item",
    "Strange ore",
    "Strange foe",
    "Strange weapon",
    "Strange armor",
    "Strange ring",
    "Armor",
    "Roar",
    "Wanderer",
    "Howl",
    "Other",
    "Fire",
    "Fire defense",
    "Flame weapon",
    "Magician",
    "Ambush",
    "Turpentine",
    "Spell",
    "Magic defense",
    "Magic weapon",
    "Trick road",
    "Invisible foe",
    "Right",
    "Clockwise",
    "Augite of Guidance",
    "Clearstone",
    "Illusion",
    "Ignore",
    "Message",
    "Merchant",
    "Sounds",
    "Gate",
    "Arrow",
    "Bladestone",
    "Spear",
    "Friend",
    "Ring",
    "Bow",
    "Lava",
    "Meltstone",
    "Pay attention",
    "Weak guy",
    "Fall",
    "Rhythm",
    "Dragonstone",
    "Two-handed attack",
    "Lever",
    "If you use the lever...",
    "Combo attack",
    "After combo",
    "During combo",
    "Crossroads",
    "Trap",
];