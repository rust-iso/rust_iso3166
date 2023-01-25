#!/usr/bin/env python
# -*- coding: utf-8 -*-
import csv
import re

pre_code = """
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
"""

f = csv.reader(open('iso3166_3.data', 'r'), delimiter=',', quotechar='"')

print(pre_code)

codes = []
line = 0
for x in f:
    line = line + 1
    if line == 1:
        continue

    code = x[3].strip()
    code = re.sub(r"\[note.*?\]", "", code).strip()
    name = re.sub(r"\[note.*?\]", "", x[0].strip())
    former = x[1].strip().split(",")
    # former = re.sub(r"\[note.*?\]","",former)

    former_alpha2 = former[0].strip()
    former_alpha3 = former[1].strip()
    former_numeric = former[2].strip()
    former_numeric = re.sub(r"\[note.*?\]", "", former_numeric).strip()
    if former_numeric == "-":
        former_numeric = "0"

    former_numeric = str(int(former_numeric))

    validity = x[2].split("\xe2\x80\x93")
    validity = x[2].split("–")
    validity_from = re.sub(r"\[note.*?\]", "", validity[0])
    validity_to = re.sub(r"\[note.*?\]", "", validity[1])

    desc = re.sub(r'\(.*?\)', '', x[4])
    desc = re.sub(r"\[note.*?\]", "", desc).replace("\n", "")
    # print x
    # print validity
    codes.append(code)
    print("""
pub const %s: CountryCode3 = CountryCode3 {
    code: "%s",
    name: "%s",
    former: CountryCode {
        name: "%s",
        alpha2: "%s",
        alpha3: "%s",
        numeric: %s,
    },
    validity: &[%s,%s],
    desc: "%s",
    new_countries: &[
""" % (code, code, name, name, former_alpha2, former_alpha3, former_numeric, validity_from, validity_to, desc))
    new_countries = re.findall(r'(.*?)\((\w+), (\w+), (\d+)\)', x[4])
    for c in new_countries:
        c_name = c[0]
        c_alpha2 = c[1]
        c_alpha3 = c[2]
        c_numeric = c[3]
        c_numeric = str(int(c_numeric))
        c_name = c_name.replace("Name changed to", "").replace(
            "Merged into", "").replace("Divided into:", "").strip()
        print(""" CountryCode {
        name: "%s",
        alpha2: "%s",
        alpha3: "%s",
        numeric: %s,
},""" % (c_name, c_alpha2, c_alpha3, c_numeric))
    print("""
    ],
};
""")

print("""
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
""")


print("""
///CountryCode map with  alpha4 Code key 
pub const ALPHA4_MAP: Map<&str, CountryCode3> = phf_map! {
""")
for x in codes:
    print("\"%s\" => %s," % (x, x))
print("""
};
""")

print("""
///ALL the Countrys struct
pub const ALL: & [CountryCode3] = &[
""")
for x in codes:
    print("%s," % (x))
print("""
];
""")
