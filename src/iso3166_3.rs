
use phf::phf_map;
use phf::Map;
use crate::CountryCode;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
use js_sys::Array;

/// Data for each Country Code defined by ISO 3166-1
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CountryCode3 {
    ///ISO 3166-3 code
    code: &'static str,
    ///Former country name
    name: &'static str,

    former: CountryCode,
    new_countries: &'static [CountryCode],

    ///Period of validity
    validity: &'static [i32],
    ///Decription
    desc: &'static str,
   
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl CountryCode3 {
    #[wasm_bindgen(getter)]
    pub fn code(&self) -> String {
        self.code.into()
    }

    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.into()
    }

    #[wasm_bindgen(getter)]
    pub fn former(&self) -> CountryCode {
        self.former
    }

    #[wasm_bindgen(getter)]
    pub fn new_countries(&self) -> Array {
    	let mut vector: Vec<CountryCode> = Vec::new(); 
        // self.individual_languages.into_serde().unwrap();
		for i in 0..self.new_countries.len() {
			vector.push(self.new_countries[i])
		}
		vector.into_iter().map(JsValue::from).collect()
    }
    
    #[wasm_bindgen(getter)]
    pub fn desc(&self) -> String {
        self.desc.into()
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CountryCode3 {
    ///ISO 3166-3 code
    pub code: &'static str,
    ///Former country name
    pub name: &'static str,

    pub former: CountryCode,
    pub new_countries: &'static [CountryCode],

    ///Period of validity
    pub validity: &'static [i32],
    ///Decription
    pub desc: &'static str,
   
}


pub const BQAQ: CountryCode3 = CountryCode3 {
    code: "BQAQ",
    name: "British Antarctic Territory",
    former: CountryCode {
        name: "British Antarctic Territory",
        alpha2: "BQ",
        alpha3: "ATB",
        numeric: 0,
    },
    validity: &[1974,1979],
    desc: "Merged into Antarctica ",
    new_countries: &[

 CountryCode {
        name: "Antarctica",
        alpha2: "AQ",
        alpha3: "ATA",
        numeric: 10,
},

    ],
};


pub const BUMM: CountryCode3 = CountryCode3 {
    code: "BUMM",
    name: "Burma",
    former: CountryCode {
        name: "Burma",
        alpha2: "BU",
        alpha3: "BUR",
        numeric: 104,
    },
    validity: &[1974,1989],
    desc: "Name changed to Myanmar ",
    new_countries: &[

 CountryCode {
        name: "Myanmar",
        alpha2: "MM",
        alpha3: "MMR",
        numeric: 104,
},

    ],
};


pub const BYAA: CountryCode3 = CountryCode3 {
    code: "BYAA",
    name: "Byelorussian SSR",
    former: CountryCode {
        name: "Byelorussian SSR",
        alpha2: "BY",
        alpha3: "BYS",
        numeric: 112,
    },
    validity: &[1974,1992],
    desc: "Name changed to Belarus ",
    new_countries: &[

 CountryCode {
        name: "Belarus",
        alpha2: "BY",
        alpha3: "BLR",
        numeric: 112,
},

    ],
};


pub const CTKI: CountryCode3 = CountryCode3 {
    code: "CTKI",
    name: "Canton and Enderbury Islands",
    former: CountryCode {
        name: "Canton and Enderbury Islands",
        alpha2: "CT",
        alpha3: "CTE",
        numeric: 128,
    },
    validity: &[1974,1984],
    desc: "Merged into Kiribati ",
    new_countries: &[

 CountryCode {
        name: "Kiribati",
        alpha2: "KI",
        alpha3: "KIR",
        numeric: 296,
},

    ],
};


pub const CSHH: CountryCode3 = CountryCode3 {
    code: "CSHH",
    name: "Czechoslovakia",
    former: CountryCode {
        name: "Czechoslovakia",
        alpha2: "CS",
        alpha3: "CSK",
        numeric: 200,
    },
    validity: &[1974,1993],
    desc: "Divided into: Czechia  Slovakia ",
    new_countries: &[

 CountryCode {
        name: "Czechia",
        alpha2: "CZ",
        alpha3: "CZE",
        numeric: 203,
},
 CountryCode {
        name: "Slovakia",
        alpha2: "SK",
        alpha3: "SVK",
        numeric: 703,
},

    ],
};


pub const DYBJ: CountryCode3 = CountryCode3 {
    code: "DYBJ",
    name: "Dahomey",
    former: CountryCode {
        name: "Dahomey",
        alpha2: "DY",
        alpha3: "DHY",
        numeric: 204,
    },
    validity: &[1974,1977],
    desc: "Name changed to Benin ",
    new_countries: &[

 CountryCode {
        name: "Benin",
        alpha2: "BJ",
        alpha3: "BEN",
        numeric: 204,
},

    ],
};


pub const NQAQ: CountryCode3 = CountryCode3 {
    code: "NQAQ",
    name: "Dronning Maud Land",
    former: CountryCode {
        name: "Dronning Maud Land",
        alpha2: "NQ",
        alpha3: "ATN",
        numeric: 216,
    },
    validity: &[1974,1983],
    desc: "Merged into Antarctica ",
    new_countries: &[

 CountryCode {
        name: "Antarctica",
        alpha2: "AQ",
        alpha3: "ATA",
        numeric: 10,
},

    ],
};


pub const TPTL: CountryCode3 = CountryCode3 {
    code: "TPTL",
    name: "East Timor",
    former: CountryCode {
        name: "East Timor",
        alpha2: "TP",
        alpha3: "TMP",
        numeric: 626,
    },
    validity: &[1974,2002],
    desc: "Name changed to Timor-Leste ",
    new_countries: &[

 CountryCode {
        name: "Timor-Leste",
        alpha2: "TL",
        alpha3: "TLS",
        numeric: 626,
},

    ],
};


pub const FXFR: CountryCode3 = CountryCode3 {
    code: "FXFR",
    name: "France, Metropolitan",
    former: CountryCode {
        name: "France, Metropolitan",
        alpha2: "FX",
        alpha3: "FXX",
        numeric: 249,
    },
    validity: &[1993,1997],
    desc: "Merged into France ",
    new_countries: &[

 CountryCode {
        name: "France",
        alpha2: "FR",
        alpha3: "FRA",
        numeric: 250,
},

    ],
};


pub const AIDJ: CountryCode3 = CountryCode3 {
    code: "AIDJ",
    name: "French Afars and Issas",
    former: CountryCode {
        name: "French Afars and Issas",
        alpha2: "AI",
        alpha3: "AFI",
        numeric: 262,
    },
    validity: &[1974,1977],
    desc: "Name changed to Djibouti ",
    new_countries: &[

 CountryCode {
        name: "Djibouti",
        alpha2: "DJ",
        alpha3: "DJI",
        numeric: 262,
},

    ],
};


pub const FQHH: CountryCode3 = CountryCode3 {
    code: "FQHH",
    name: "French Southern and Antarctic Territories",
    former: CountryCode {
        name: "French Southern and Antarctic Territories",
        alpha2: "FQ",
        alpha3: "ATF",
        numeric: 0,
    },
    validity: &[1974,1979],
    desc: "Divided into: Part of Antarctica   French Southern Territories ",
    new_countries: &[

 CountryCode {
        name: "Part of Antarctica",
        alpha2: "AQ",
        alpha3: "ATA",
        numeric: 10,
},
 CountryCode {
        name: "French Southern Territories",
        alpha2: "TF",
        alpha3: "ATF",
        numeric: 260,
},

    ],
};


pub const DDDE: CountryCode3 = CountryCode3 {
    code: "DDDE",
    name: "German Democratic Republic",
    former: CountryCode {
        name: "German Democratic Republic",
        alpha2: "DD",
        alpha3: "DDR",
        numeric: 278,
    },
    validity: &[1974,1990],
    desc: "Merged into Germany ",
    new_countries: &[

 CountryCode {
        name: "Germany",
        alpha2: "DE",
        alpha3: "DEU",
        numeric: 276,
},

    ],
};


pub const GEHH: CountryCode3 = CountryCode3 {
    code: "GEHH",
    name: "Gilbert Islands",
    former: CountryCode {
        name: "Gilbert Islands",
        alpha2: "GE",
        alpha3: "GEL",
        numeric: 296,
    },
    validity: &[1974,1979],
    desc: "Name changed to Kiribati ",
    new_countries: &[

 CountryCode {
        name: "Kiribati",
        alpha2: "KI",
        alpha3: "KIR",
        numeric: 296,
},

    ],
};


pub const JTUM: CountryCode3 = CountryCode3 {
    code: "JTUM",
    name: "Johnston Island",
    former: CountryCode {
        name: "Johnston Island",
        alpha2: "JT",
        alpha3: "JTN",
        numeric: 396,
    },
    validity: &[1974,1986],
    desc: "Merged into United States Minor Outlying Islands ",
    new_countries: &[

 CountryCode {
        name: "United States Minor Outlying Islands",
        alpha2: "UM",
        alpha3: "UMI",
        numeric: 581,
},

    ],
};


pub const MIUM: CountryCode3 = CountryCode3 {
    code: "MIUM",
    name: "Midway Islands",
    former: CountryCode {
        name: "Midway Islands",
        alpha2: "MI",
        alpha3: "MID",
        numeric: 488,
    },
    validity: &[1974,1986],
    desc: "Merged into United States Minor Outlying Islands ",
    new_countries: &[

 CountryCode {
        name: "United States Minor Outlying Islands",
        alpha2: "UM",
        alpha3: "UMI",
        numeric: 581,
},

    ],
};


pub const ANHH: CountryCode3 = CountryCode3 {
    code: "ANHH",
    name: "Netherlands Antilles",
    former: CountryCode {
        name: "Netherlands Antilles",
        alpha2: "AN",
        alpha3: "ANT",
        numeric: 530,
    },
    validity: &[1974,2010 ],
    desc: "Divided into: Bonaire, Sint Eustatius and Saba   Curaçao  Sint Maarten  ",
    new_countries: &[

 CountryCode {
        name: "Bonaire, Sint Eustatius and Saba",
        alpha2: "BQ",
        alpha3: "BES",
        numeric: 535,
},
 CountryCode {
        name: "Curaçao",
        alpha2: "CW",
        alpha3: "CUW",
        numeric: 531,
},
 CountryCode {
        name: "Sint Maarten (Dutch part)",
        alpha2: "SX",
        alpha3: "SXM",
        numeric: 534,
},

    ],
};


pub const NTHH: CountryCode3 = CountryCode3 {
    code: "NTHH",
    name: "Neutral Zone",
    former: CountryCode {
        name: "Neutral Zone",
        alpha2: "NT",
        alpha3: "NTZ",
        numeric: 536,
    },
    validity: &[1974,1993],
    desc: "Divided into: Part of Iraq  Part of Saudi Arabia ",
    new_countries: &[

 CountryCode {
        name: "Part of Iraq",
        alpha2: "IQ",
        alpha3: "IRQ",
        numeric: 368,
},
 CountryCode {
        name: "Part of Saudi Arabia",
        alpha2: "SA",
        alpha3: "SAU",
        numeric: 682,
},

    ],
};


pub const NHVU: CountryCode3 = CountryCode3 {
    code: "NHVU",
    name: "New Hebrides",
    former: CountryCode {
        name: "New Hebrides",
        alpha2: "NH",
        alpha3: "NHB",
        numeric: 548,
    },
    validity: &[1974,1980],
    desc: "Name changed to Vanuatu ",
    new_countries: &[

 CountryCode {
        name: "Vanuatu",
        alpha2: "VU",
        alpha3: "VUT",
        numeric: 548,
},

    ],
};


pub const PCHH: CountryCode3 = CountryCode3 {
    code: "PCHH",
    name: "Pacific Islands (Trust Territory)",
    former: CountryCode {
        name: "Pacific Islands (Trust Territory)",
        alpha2: "PC",
        alpha3: "PCI",
        numeric: 582,
    },
    validity: &[1974,1986],
    desc: "Divided into: Marshall Islands  Micronesia   Northern Mariana Islands  Palau ",
    new_countries: &[

 CountryCode {
        name: "Marshall Islands",
        alpha2: "MH",
        alpha3: "MHL",
        numeric: 584,
},
 CountryCode {
        name: "Micronesia (Federated States of)",
        alpha2: "FM",
        alpha3: "FSM",
        numeric: 583,
},
 CountryCode {
        name: "Northern Mariana Islands",
        alpha2: "MP",
        alpha3: "MNP",
        numeric: 580,
},
 CountryCode {
        name: "Palau",
        alpha2: "PW",
        alpha3: "PLW",
        numeric: 585,
},

    ],
};


pub const PZPA: CountryCode3 = CountryCode3 {
    code: "PZPA",
    name: "Panama Canal Zone",
    former: CountryCode {
        name: "Panama Canal Zone",
        alpha2: "PZ",
        alpha3: "PCZ",
        numeric: 0,
    },
    validity: &[1974,1980],
    desc: "Merged into Panama ",
    new_countries: &[

 CountryCode {
        name: "Panama",
        alpha2: "PA",
        alpha3: "PAN",
        numeric: 591,
},

    ],
};


pub const CSXX: CountryCode3 = CountryCode3 {
    code: "CSXX",
    name: "Serbia and Montenegro",
    former: CountryCode {
        name: "Serbia and Montenegro",
        alpha2: "CS",
        alpha3: "SCG",
        numeric: 891,
    },
    validity: &[2003,2006],
    desc: "Divided into: Montenegro  Serbia ",
    new_countries: &[

 CountryCode {
        name: "Montenegro",
        alpha2: "ME",
        alpha3: "MNE",
        numeric: 499,
},
 CountryCode {
        name: "Serbia",
        alpha2: "RS",
        alpha3: "SRB",
        numeric: 688,
},

    ],
};


pub const SKIN: CountryCode3 = CountryCode3 {
    code: "SKIN",
    name: "Sikkim",
    former: CountryCode {
        name: "Sikkim",
        alpha2: "SK",
        alpha3: "SKM",
        numeric: 0,
    },
    validity: &[1974,1975],
    desc: "Merged into India ",
    new_countries: &[

 CountryCode {
        name: "India",
        alpha2: "IN",
        alpha3: "IND",
        numeric: 356,
},

    ],
};


pub const RHZW: CountryCode3 = CountryCode3 {
    code: "RHZW",
    name: "Southern Rhodesia",
    former: CountryCode {
        name: "Southern Rhodesia",
        alpha2: "RH",
        alpha3: "RHO",
        numeric: 716,
    },
    validity: &[1974,1980],
    desc: "Name changed to Zimbabwe ",
    new_countries: &[

 CountryCode {
        name: "Zimbabwe",
        alpha2: "ZW",
        alpha3: "ZWE",
        numeric: 716,
},

    ],
};


pub const PUUM: CountryCode3 = CountryCode3 {
    code: "PUUM",
    name: "United States Miscellaneous Pacific Islands",
    former: CountryCode {
        name: "United States Miscellaneous Pacific Islands",
        alpha2: "PU",
        alpha3: "PUS",
        numeric: 849,
    },
    validity: &[1974,1986],
    desc: "Merged into United States Minor Outlying Islands ",
    new_countries: &[

 CountryCode {
        name: "United States Minor Outlying Islands",
        alpha2: "UM",
        alpha3: "UMI",
        numeric: 581,
},

    ],
};


pub const HVBF: CountryCode3 = CountryCode3 {
    code: "HVBF",
    name: "Upper Volta",
    former: CountryCode {
        name: "Upper Volta",
        alpha2: "HV",
        alpha3: "HVO",
        numeric: 854,
    },
    validity: &[1974,1984],
    desc: "Name changed to Burkina Faso ",
    new_countries: &[

 CountryCode {
        name: "Burkina Faso",
        alpha2: "BF",
        alpha3: "BFA",
        numeric: 854,
},

    ],
};


pub const SUHH: CountryCode3 = CountryCode3 {
    code: "SUHH",
    name: "USSR",
    former: CountryCode {
        name: "USSR",
        alpha2: "SU",
        alpha3: "SUN",
        numeric: 810,
    },
    validity: &[1974,1992],
    desc: "Divided into:  Armenia  Azerbaijan  Estonia  Georgia  Kazakhstan  Kyrgyzstan  Latvia  Lithuania  Moldova, Republic of  Russian Federation  Tajikistan  Turkmenistan  Uzbekistan ",
    new_countries: &[

 CountryCode {
        name: "Armenia",
        alpha2: "AM",
        alpha3: "ARM",
        numeric: 51,
},
 CountryCode {
        name: "Azerbaijan",
        alpha2: "AZ",
        alpha3: "AZE",
        numeric: 31,
},
 CountryCode {
        name: "Estonia",
        alpha2: "EE",
        alpha3: "EST",
        numeric: 233,
},
 CountryCode {
        name: "Georgia",
        alpha2: "GE",
        alpha3: "GEO",
        numeric: 268,
},
 CountryCode {
        name: "Kazakhstan",
        alpha2: "KZ",
        alpha3: "KAZ",
        numeric: 398,
},
 CountryCode {
        name: "Kyrgyzstan",
        alpha2: "KG",
        alpha3: "KGZ",
        numeric: 417,
},
 CountryCode {
        name: "Latvia",
        alpha2: "LV",
        alpha3: "LVA",
        numeric: 428,
},
 CountryCode {
        name: "Lithuania",
        alpha2: "LT",
        alpha3: "LTU",
        numeric: 440,
},
 CountryCode {
        name: "Moldova, Republic of",
        alpha2: "MD",
        alpha3: "MDA",
        numeric: 498,
},
 CountryCode {
        name: "Russian Federation",
        alpha2: "RU",
        alpha3: "RUS",
        numeric: 643,
},
 CountryCode {
        name: "Tajikistan",
        alpha2: "TJ",
        alpha3: "TJK",
        numeric: 762,
},
 CountryCode {
        name: "Turkmenistan",
        alpha2: "TM",
        alpha3: "TKM",
        numeric: 795,
},
 CountryCode {
        name: "Uzbekistan",
        alpha2: "UZ",
        alpha3: "UZB",
        numeric: 860,
},

    ],
};


pub const VDVN: CountryCode3 = CountryCode3 {
    code: "VDVN",
    name: "Viet-Nam, Democratic Republic of",
    former: CountryCode {
        name: "Viet-Nam, Democratic Republic of",
        alpha2: "VD",
        alpha3: "VDR",
        numeric: 0,
    },
    validity: &[1974,1977],
    desc: "Merged into Viet Nam ",
    new_countries: &[

 CountryCode {
        name: "Viet Nam",
        alpha2: "VN",
        alpha3: "VNM",
        numeric: 704,
},

    ],
};


pub const WKUM: CountryCode3 = CountryCode3 {
    code: "WKUM",
    name: "Wake Island",
    former: CountryCode {
        name: "Wake Island",
        alpha2: "WK",
        alpha3: "WAK",
        numeric: 872,
    },
    validity: &[1974,1986],
    desc: "Merged into United States Minor Outlying Islands ",
    new_countries: &[

 CountryCode {
        name: "United States Minor Outlying Islands",
        alpha2: "UM",
        alpha3: "UMI",
        numeric: 581,
},

    ],
};


pub const YDYE: CountryCode3 = CountryCode3 {
    code: "YDYE",
    name: "Yemen, Democratic",
    former: CountryCode {
        name: "Yemen, Democratic",
        alpha2: "YD",
        alpha3: "YMD",
        numeric: 720,
    },
    validity: &[1974,1990],
    desc: "Merged into Yemen ",
    new_countries: &[

 CountryCode {
        name: "Yemen",
        alpha2: "YE",
        alpha3: "YEM",
        numeric: 887,
},

    ],
};


pub const YUCS: CountryCode3 = CountryCode3 {
    code: "YUCS",
    name: "Yugoslavia",
    former: CountryCode {
        name: "Yugoslavia",
        alpha2: "YU",
        alpha3: "YUG",
        numeric: 891,
    },
    validity: &[1974,2003],
    desc: "Name changed to Serbia and Montenegro ",
    new_countries: &[

 CountryCode {
        name: "Serbia and Montenegro",
        alpha2: "CS",
        alpha3: "SCG",
        numeric: 891,
},

    ],
};


pub const ZRCD: CountryCode3 = CountryCode3 {
    code: "ZRCD",
    name: "Zaire",
    former: CountryCode {
        name: "Zaire",
        alpha2: "ZR",
        alpha3: "ZAR",
        numeric: 180,
    },
    validity: &[1974,1997],
    desc: "Name changed to Congo, Democratic Republic of the ",
    new_countries: &[

 CountryCode {
        name: "Congo, Democratic Republic of the",
        alpha2: "CD",
        alpha3: "COD",
        numeric: 180,
},

    ],
};


/// Returns the CountryCode3 with the given Alpha4 code, if exists.
/// #Sample
/// ```
/// let sub = rust_iso3166::iso3166_3::from_code("PZPA");
/// assert_eq!("Panama Canal Zone", sub.unwrap().name);
/// ```
#[cfg_attr(target_arch = "wasm32", wasm_bindgen(js_name = from_code_iso_3166_3))]
pub fn from_code(alpha4: &str) -> Option<CountryCode3> {
    ALPHA4_MAP.get(alpha4).cloned()
}


///CountryCode map with  alpha4 Code key 
pub const ALPHA4_MAP: Map<&str, CountryCode3> = phf_map! {

"BQAQ" => BQAQ,
"BUMM" => BUMM,
"BYAA" => BYAA,
"CTKI" => CTKI,
"CSHH" => CSHH,
"DYBJ" => DYBJ,
"NQAQ" => NQAQ,
"TPTL" => TPTL,
"FXFR" => FXFR,
"AIDJ" => AIDJ,
"FQHH" => FQHH,
"DDDE" => DDDE,
"GEHH" => GEHH,
"JTUM" => JTUM,
"MIUM" => MIUM,
"ANHH" => ANHH,
"NTHH" => NTHH,
"NHVU" => NHVU,
"PCHH" => PCHH,
"PZPA" => PZPA,
"CSXX" => CSXX,
"SKIN" => SKIN,
"RHZW" => RHZW,
"PUUM" => PUUM,
"HVBF" => HVBF,
"SUHH" => SUHH,
"VDVN" => VDVN,
"WKUM" => WKUM,
"YDYE" => YDYE,
"YUCS" => YUCS,
"ZRCD" => ZRCD,

};


///ALL the Countrys struct
pub const ALL: & [CountryCode3] = &[

BQAQ,
BUMM,
BYAA,
CTKI,
CSHH,
DYBJ,
NQAQ,
TPTL,
FXFR,
AIDJ,
FQHH,
DDDE,
GEHH,
JTUM,
MIUM,
ANHH,
NTHH,
NHVU,
PCHH,
PZPA,
CSXX,
SKIN,
RHZW,
PUUM,
HVBF,
SUHH,
VDVN,
WKUM,
YDYE,
YUCS,
ZRCD,

];

