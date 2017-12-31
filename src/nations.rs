use phf::Map;

pub fn get_nation_desc(n: usize) -> &'static str {
    NATIONS_BY_ID.get(&(n as u32)).unwrap_or_else(
        || {
            println!("unknown nation {}", n);
            &"unknown nation"
        }
    )
}

pub static NATIONS_BY_ID: Map<u32, &'static str> = phf_map! { 
    5u32 => "Arcoscephale 	Golden Era",
    6u32 => "Ermor 	New Faith",
    7u32 => "Ulm 	Enigma of Steel",
    8u32 => "Marverni 	Time of Druids",
    9u32 => "Sauromatia 	Amazon Queens",
    10u32 => "T’ien Ch’i 	Spring and Autumn",
    11u32 => "Machaka 	Lion Kings",
    12u32 => "Mictlan 	Reign of Blood",
    13u32 => "Abysia 	Children of Flame",
    14u32 => "Caelum 	Eagle Kings",
    15u32 => "C’tis 	Lizard Kings",
    16u32 => "Pangaea 	Age of Revelry",
    17u32 => "Agartha 	Pale Ones",
    18u32 => "Tir na n'Og 	Land of the Ever Young",
    19u32 => "Fomoria 	The Cursed Ones",
    20u32 => "Vanheim 	Age of Vanir",
    21u32 => "Helheim 	Dusk and Death",
    22u32 => "Niefelheim 	Sons of Winter",
    24u32 => "Rus 	Sons of Heaven",
    25u32 => "Kailasa 	Rise of the Ape Kings",
    26u32 => "Lanka 	Land of Demons",
    27u32 => "Yomi 	Oni Kings",
    28u32 => "Hinnom 	Sons of the Fallen",
    29u32 => "Ur 	The First City",
    30u32 => "Berytos 	Phoenix Empire",
    31u32 => "Xibalba 	Vigil of the Sun",
    36u32 => "Atlantis 	Emergence of the Deep Ones",
    37u32 => "R’lyeh 	Time of Aboleths",
    38u32 => "Pelagia 	Pearl Kings",
    39u32 => "Oceania 	Coming of the Capricorns",
    40u32 => "Therodos 	Telkhine Spectre",
    43u32 => "Arcoscephale 	The Old Kingdom",
    44u32 => "Ermor 	Ashen Empire",
    45u32 => "Sceleria 	Reformed Empire",
    46u32 => "Pythium 	Emerald Empire",
    47u32 => "Man 	Tower of Avalon",
    48u32 => "Eriu 	Last of the Tuatha",
    49u32 => "Ulm 	Forges of Ulm",
    50u32 => "Marignon 	Fiery Justice",
    51u32 => "Mictlan 	Reign of the Lawgiver",
    52u32 => "T’ien Ch’i 	Imperial Bureaucracy",
    53u32 => "Machaka 	Reign of Sorcerors",
    54u32 => "Agartha 	Golem Cult",
    55u32 => "Abysia 	Blood and Fire",
    56u32 => "Caelum 	Reign of the Seraphim",
    57u32 => "C’tis 	Miasma",
    58u32 => "Pangaea 	Age of Bronze",
    59u32 => "Asphodel 	Carrion Woods",
    60u32 => "Vanheim 	Arrival of Man",
    61u32 => "Jotunheim 	Iron Woods",
    62u32 => "Vanarus 	Land of the Chuds",
    63u32 => "Bandar Log 	Land of the Apes",
    64u32 => "Shinuyama 	Land of the Bakemono",
    65u32 => "Ashdod 	Reign of the Anakim",
    66u32 => "Uruk 	City States",
    67u32 => "Nazca 	Kingdom of the Sun",
    68u32 => "Xibalba 	Flooded Caves",
    73u32 => "Atlantis 	Kings of the Deep",
    74u32 => "R’lyeh 	Fallen Star",
    75u32 => "Pelagia 	Triton Kings",
    76u32 => "Oceania 	Mermidons",
    77u32 => "Ys 	Morgen Queens",
    80u32 => "Arcoscephale 	Sibylline Guidance",
    81u32 => "Pythium 	Serpent Cult",
    82u32 => "Lemur 	Soul Gate",
    83u32 => "Man 	Towers of Chelms",
    84u32 => "Ulm 	Black Forest",
    85u32 => "Marignon 	Conquerors of the Sea",
    86u32 => "Mictlan 	Blood and Rain",
    87u32 => "T’ien Ch’i 	Barbarian Kings",
    89u32 => "Jomon 	Human Daimyos",
    90u32 => "Agartha 	Ktonian Dead",
    91u32 => "Abysia 	Blood of Humans",
    92u32 => "Caelum 	Return of the Raptors",
    93u32 => "C’tis 	Desert Tombs",
    94u32 => "Pangaea 	New Era",
    95u32 => "Midgård 	Age of Men",
    96u32 => "Utgård 	Well of Urd",
    97u32 => "Bogarus 	Age of Heroes",
    98u32 => "Patala 	Reign of the Nagas",
    99u32 => "Gath 	Last of the Giants",
    100u32 => "Ragha 	Dual Kingdom",
    101u32 => "Xibalba 	Return of the Zotz",
    106u32 => "Atlantis 	Frozen Sea",
    107u32 => "R’lyeh 	Dreamlands",
    108u32 => "Erytheia 	Kingdom of Two Worlds",
};