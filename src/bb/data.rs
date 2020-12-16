/// Templates: Base strings, into which nouns can be inserted.
pub const TEMPLATES: &[&str] = &[
    "fear \x1F",
    "remember \x1F",
    "time for \x1F",
    "it's the scourge of \x1F",
    "reeks of \x1F",
    "\x1F is effective",
    "beware of \x1F",
    "treat \x1F with care",
    "it is all thanks to \x1F",
    "despicable \x1F",
    "woeful \x1F",
    "wondrous \x1F",
    "nothing but \x1F here",
    "\x1F waits ahead",
    "you must accept \x1F",
    "have mercy, \x1F",
    "no mercy for \x1F",
    "have audience with \x1F",
    "reminiscent of \x1F",
    "oh, \x1F!",
    "you've come to the right place",
    "bless us with blood",
    "may the good blood guide your way",
    "fear your blindness",
    "the sky and the cosmos are one",
    "let us cleanse these foul streets",
    "you're in the know, right?",
    "oh, I can't wait... hee hee...",
    "take a step forward",
    "turn back",
    "those with faith will be spared",
    "don't be fooled",
    "pitiful, really",
    "behind you",
    "don't you dare look at me!",
    "sincerest thanks",
    "a hunter is never alone",
    "please, carry on in my stead",
    "run!",
    "don't give up!",
];

/// Conjunctions: Phrases which can be used to combine two Templates.
#[cfg(not(feature = "newline"))]
pub const CONJUNCTIONS: &[&str] = &[
    " and ",
    " but ",
    " or ",
    " therefore ",
    " eventually ",
    ", ",
];

/// Conjunctions: Phrases which can be used to combine two Templates, onto
///     multiple lines.
#[cfg(feature = "newline")]
pub const CONJUNCTIONS: &[&str] = &[
    "\nand ",
    "\nbut ",
    "\nor ",
    "\ntherefore ",
    "\neventually ",
    ",\n",
];

/// Words: Nouns, tactics, concepts, and anything else which can be inserted
///     into a Template to form a Message.
pub const WORDS: &[&str] = &[
    //  Creatures
    "Beast",
    "Man-beast",
    "Giant beast",
    "Abhorrent beast",
    "Infected one",
    "Foe",
    "Strong foe",
    "Giant foe",
    "Terrible foe",
    "Hound",
    "Bird",
    "Snake",
    "Animal",
    "Insect",
    "Watcher",
    "Shaman",
    "Dead",
    "Foul spirit",
    "The lost",
    "Malformed thing",
    "Monster",
    "Unknown thing",
    "Slimy thing",
    "Blobby thing",
    "Kin of the cosmos",
    "Evil eye",
    "False god",
    "Superior being",
    "Messenger",
    "Doll",
    //  Humans
    "Man",
    "Woman",
    "Elderly",
    "Ailing one",
    "Madman",
    "Keeper",
    "Mob",
    "Wheelchair",
    "Small gent",
    "Small lady",
    "Titan",
    "Amazon",
    "Fatty",
    "Dullard",
    "Liar",
    "Scoundrel",
    "Child",
    "Friend",
    "Darling",
    "Master",
    "Infant",
    "Queen",
    "Yourself",
    "Hunter",
    "Cooperator",
    "Adversary",
    "Executioner",
    "Vileblood",
    "Hunter of Hunters",
    "Blood-addled Hunter",
    //  Tactics A
    "Physical attack",
    "Blunt attack",
    "Thrust attack",
    "Blood attack",
    "Arcane",
    "Fire",
    "Bolt",
    "Quick weapon",
    "Long weapon",
    "Poison",
    "Frenzy",
    "Exploiting species",
    "Beast transformation",
    "Firearm",
    "Blunderbuss",
    "Torch",
    "Shield",
    "Rally",
    "Charge attack",
    "Visceral attack",
    "Rolling",
    "Quickstep",
    "Blood vial",
    "Quicksilver Bullet",
    "Medicine",
    "Special medicine",
    "Projectile",
    "Oil",
    "Coarse paper",
    "Special item",
    //  Tactics B
    "Ambush",
    "Pincer attack",
    "Sneak attack",
    "Patrol",
    "Reinforcements caller",
    "\"Focus on attacks\"",
    "\"Focus on evasion\"",
    "\"Focus on healing\"",
    "\"Close-range fight\"",
    "\"Long-range fight\"",
    "\"Hit-and-run\"",
    "Sniping",
    "Counter",
    "\"Attack from behind\"",
    "\"Open when attacking\"",
    "\"Strike and be struck\"",
    "\"Kill in order\"",
    "\"Kill first\"",
    "Charging forth",
    "Lure",
    "Stealth",
    "Ignoring",
    "Retreat",
    "Use of terrain",
    "Tight spot",
    "High spot",
    "Fall",
    "Alertness",
    "Unbreakable will",
    "Leaden constitution",
    //  Places/Things
    "Blood Echoes",
    "Insight",
    "Bloodstone",
    "Blood Gem",
    "Rune",
    "Ritual material",
    "Key",
    "Item",
    "Special item",
    "Paleblood",
    "Message",
    "Rating",
    "Dead body",
    "Treasure",
    "Lever",
    "Statue",
    "Light",
    "Bonfire",
    "Footing",
    "Trap",
    "Yharnam",
    "Clinic",
    "Grand Cathedral",
    "Church",
    "Safe place",
    "Old labyrinth",
    "Workshop",
    "Healing Church",
    "Hidden path",
    "Unseen Village",
    //  Concepts
    "Hunting",
    "Night",
    "Dawn",
    "Blood",
    "Warm blood",
    "Scourge",
    "Life",
    "Nightmare",
    "Moon",
    "Cosmos",
    "Eye",
    "Oedon",
    "Communion",
    "Donation",
    "Ritual",
    "Contact",
    "Encounter",
    "Evolution",
    "Oath",
    "Corruption",
    "Execution",
    "Cleansing",
    "Prayer",
    "Curse",
    "Defilement",
    "Sinister",
    "Courage",
    "Respect",
    "Inquisitiveness",
    "Pity",
    "Grief",
    "Joy",
    "Wrath",
    "Sanity",
    "Madness",
    "Fervor",
    "Seduction",
    "Feasting",
    "Tastiness",
    "Tonsil",
    "Metamorphosis",
    "Common sense",
    "Darkness",
    "Secret",
    "Singing",
    "Sobbing",
    "Howling",
    "\"All's well\"",
    "The unseen",
    "All",
];
