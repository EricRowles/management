
// use rand::Rng;
use rand::seq::SliceRandom;

pub fn select_from_list(list: [&'static str; 26]) -> String {
    list.choose(&mut rand::thread_rng()).unwrap().clone().to_string()
}

pub const FIRST_NAMES: [&'static str; 26] = [
    "Alfred",
    "Bart",
    "Conelly",
    "Darrius",
    "Elfriede",
    "Frija",
    "Ghorza",
    "Haizea",
    "Illia",
    "Joan",
    "Krita",
    "Lombardo",
    "Malborn",
    "Noster",
    "Orla",
    "Preid",
    "Quaranir",
    "Raster",
    "Straag",
    "Tvolf",
    "Umana",
    "Vrihi",
    "Waughin",
    "Xylvia",
    "Yoora",
    "Zifri",
];

pub const VERB: [&'static str; 26] = [
    "Argue",
    "Beg",
    "Cry",
    "Dance",
    "Eat",
    "Fight",
    "Gamble",
    "Hunt",
    "Infiltrate",
    "Joust",
    "Kill",
    "Loot",
    "Maim",
    "Nap",
    "Observe",
    "Pillage",
    "Quarrel",
    "Raid",
    "Sail",
    "Terrify",
    "Undermine",
    "Vandalize",
    "Wrestle",
    "X-ray",
    "Yell",
    "Zap",
];

pub const ANIMALS: [&'static str; 26] = [
    "Apes",
    "Bears",
    "Cats",
    "Dogs",
    "Elephants",
    "Frogs",
    "Giraffes",
    "Horses",
    "Iguanas",
    "Jaguars",
    "Kangaroos",
    "Lions",
    "Monkeys",
    "Narwhals",
    "Owls",
    "Pigs",
    "Quails",
    "Rabbits",
    "Snakes",
    "Tigers",
    "Unicorns",
    "Vultures",
    "Wolves",
    "Xenomorphs",
    "Yaks",
    "Zebras",
];

