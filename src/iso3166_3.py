#!/usr/bin/env python
# -*- coding: utf-8 -*- 
import csv,re

pre_code = """
use phf::phf_map;
use phf::Map;
use crate::CountryCode;

/// Data for each Country Code defined by ISO 3166-1
#[derive(Debug,Copy, Clone)]
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

f =  csv.reader(open('iso3166_3.data', 'rb'), delimiter=',', quotechar='"')

print pre_code

codes = []
line = 0
for x in f:
    line = line + 1
    if line == 1: continue
    
    code = x[3].strip()
    code = re.sub(r"\[note.*?\]","",code).strip()
    name = re.sub(r"\[note.*?\]","",x[0].strip())
    former = x[1].strip().split(",")
    # former = re.sub(r"\[note.*?\]","",former)

    former_alpha2 = former[0].strip()
    former_alpha3 = former[1].strip()
    former_numeric = former[2].strip()
    former_numeric = re.sub(r"\[note.*?\]","",former_numeric).strip()
    if former_numeric == "-":
        former_numeric = "0"
    validity = x[2].split("\xe2\x80\x93")
    validity_from = re.sub(r"\[note.*?\]","",validity[0])
    validity_to = re.sub(r"\[note.*?\]","",validity[1])

    desc = re.sub(r'\(.*?\)', '', x[4])
    desc = re.sub(r"\[note.*?\]","",desc).replace("\n","")
    # print x
    # print validity
    codes.append(code)
    print """
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
""" % (code,code,name,name,former_alpha2,former_alpha3,former_numeric,validity_from,validity_to,desc)
    new_countries = re.findall(r'(.*?)\((\w+), (\w+), (\d+)\)', x[4])
    for c in new_countries:
        c_name = c[0]
        c_alpha2 = c[1]
        c_alpha3 = c[2]
        c_numeric = c[3]
        c_name = c_name.replace("Name changed to","").replace("Merged into","").replace("Divided into:","").strip()
        print """ CountryCode {
        name: "%s",
        alpha2: "%s",
        alpha3: "%s",
        numeric: %s,
},""" % (c_name,c_alpha2,c_alpha3,c_numeric)
    print """
    ],
};
""" 

print """
/// Returns the CountryCode3 with the given Alpha4 code, if exists.
/// #Sample
/// ```
/// let sub = rust_iso3166::iso3166_3::from_code("PZPA");
/// assert_eq!("Panama Canal Zone", sub.unwrap().name);
/// ```
pub fn from_code(alpha4: &str) -> Option<CountryCode3> {
    ALPHA4_MAP.get(alpha4).cloned()
}
"""


print """
///CountryCode map with  alpha4 Code key 
pub const ALPHA4_MAP: Map<&str, CountryCode3> = phf_map! {
"""
for x in codes:
    print "\"%s\" => %s," % (x,x)
print """
};
"""

print """
///ALL the Countrys struct
pub const ALL: &'static [CountryCode3] = &[
"""
for x in codes:
    print "%s," % (x)
print """
];
"""


