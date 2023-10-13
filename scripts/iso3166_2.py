#!/usr/bin/env python
# -*- coding: utf-8 -*-
import csv

pre_code = """
use phf::phf_map;
use phf::Map;
#[cfg(all(direct_wasm,target_arch = "wasm32"))]
use wasm_bindgen::prelude::*;

/// # Sample code
/// ```
/// let country = rust_iso3166::from_alpha2("GB").unwrap();
/// let subdivisions = country.subdivisions();
/// assert!(subdivisions.unwrap().len() > 0);
/// let country = rust_iso3166::iso3166_2::from_code("GB-EDH");
/// assert_eq!("Edinburgh, City of", country.unwrap().name);
/// println!("{:?}", rust_iso3166::iso3166_2::SUBDIVISION_COUNTRY_MAP); 
/// println!("{:?}", rust_iso3166::iso3166_2::SUBDIVISION_MAP); 
/// ```

/// Data for each Country Code defined by ISO 3166-2
#[cfg(all(direct_wasm,target_arch = "wasm32"))]
#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Subdivision {
    ///Name
    name: &'static str,
    ///Subdivision Type
    subdivision_type: &'static str,
    ///Code
    code: &'static str,
    ///Country Name
    country_name: &'static str,
    ///Country Code
    country_code: &'static str,
    ///Region Code
    region_code: &'static str,
}

#[cfg(all(direct_wasm,target_arch = "wasm32"))]
#[wasm_bindgen]
impl Subdivision {
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.into()
    }

    #[wasm_bindgen(getter)]
    pub fn code(&self) -> String {
        self.code.into()
    }

    #[wasm_bindgen(getter)]
    pub fn subdivision_type(&self) -> String {
        self.subdivision_type.into()
    }

    #[wasm_bindgen(getter)]
    pub fn country_name(&self) -> String {
        self.country_name.into()
    }

    #[wasm_bindgen(getter)]
    pub fn country_code(&self) -> String {
        self.country_code.into()
    }

    #[wasm_bindgen(getter)]
    pub fn region_code(&self) -> String {
        self.region_code.into()
    }
}

#[cfg(any(not(direct_wasm),not(target_arch = "wasm32")))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct Subdivision {
    ///Name
    pub name: &'static str,
    ///Subdivision Type
    pub subdivision_type: &'static str,
    ///Code
    pub code: &'static str,
    ///Country Name
    pub country_name: &'static str,
    ///Country Code
    pub country_code: &'static str,
    ///Region Code
    pub region_code: &'static str,
}

/// Returns the Subdivision with the given code, if exists.
/// #Sample
/// ```
/// let sub = rust_iso3166::iso3166_2::from_code("SE-O");
/// assert_eq!("Västra Götalands län", sub.unwrap().name);
/// ```
#[cfg_attr(all(direct_wasm,target_arch = "wasm32"), wasm_bindgen(js_name = from_code_iso_3166_2))]
pub fn from_code(code: &str) -> Option<Subdivision> {
    SUBDIVISION_MAP.get(code).cloned()
}
"""
print(pre_code)
f = csv.reader(open("iso3166_2.data", "r"), delimiter=",", quotechar='"')
subdivisions = {}
for x in f:
    ts1 = x[1].split("-")

    region_code = x[1]
    country_code = x[4]
    country_name = x[0]
    sub_name = x[2]
    sub_code = x[1]
    sub_type = x[3]
    var_name = sub_code.replace("-", "_")

    if not country_code in subdivisions:
        subdivisions[country_code] = []
    subdivisions[country_code].append(
        {
            "name": sub_name,
            "var_name": var_name,
            "code": sub_code,
            "type": sub_type,
            "country_name": country_name,
            "country_code": country_code,
            "region_code": region_code,
        }
    )
    print(
        """pub const %s: Subdivision = Subdivision {
    name: "%s",
    code: "%s",
    subdivision_type: "%s",
    country_name: "%s",
    country_code: "%s",
    region_code: "%s",
};
"""
        % (
            var_name,
            sub_name,
            sub_code,
            sub_type,
            country_name,
            country_code,
            region_code,
        )
    )


print(
    """
///Subdivision map with  Code key 
pub const SUBDIVISION_MAP: Map<&str, Subdivision> = phf_map! {
"""
)
for key in subdivisions:
    for sub in subdivisions[key]:
        print('"%s" => %s,' % (sub["code"], sub["var_name"]))
print(
    """
};
"""
)

print(
    """
///Subdivision map with  Code key 
pub const SUBDIVISION_COUNTRY_MAP: Map<&str, &[Subdivision]> = phf_map! {
"""
)
for key in subdivisions:
    print('"%s" => &[' % (key))
    for sub in subdivisions[key]:
        print("%s," % (sub["var_name"]))
    print("],")
print(
    """
};
"""
)
