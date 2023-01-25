
use phf::phf_map;
use phf::Map;
pub mod iso3166_2;
pub mod iso3166_3;
use std::hash::Hash;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use js_sys::Array;

/// # Sample code
/// ```
/// let country = rust_iso3166::from_alpha2("AU");
/// assert_eq!("AUS", country.unwrap().alpha3); 
/// let country = rust_iso3166::from_alpha3("AUS");
/// assert_eq!("AU", country.unwrap().alpha2);  
/// let country = rust_iso3166::from_numeric(036);
/// assert_eq!("AUS", country.unwrap().alpha3);   
/// let country = rust_iso3166::from_numeric_str("036");
/// assert_eq!("AUS", country.unwrap().alpha3); 
/// 
/// println!("{:?}", country);   
/// println!("{:?}", rust_iso3166::ALL);

/// println!("{:?}", rust_iso3166::ALL_ALPHA2);   
/// println!("{:?}", rust_iso3166::ALL_ALPHA3);   
/// println!("{:?}", rust_iso3166::ALL_NAME);   
/// println!("{:?}", rust_iso3166::ALL_NUMERIC);   
/// println!("{:?}", rust_iso3166::ALL_NUMERIC_STR);   

/// println!("{:?}", rust_iso3166::NUMERIC_MAP);  
/// println!("{:?}", rust_iso3166::ALPHA3_MAP);  
/// println!("{:?}", rust_iso3166::ALPHA2_MAP);  
/// ```

/// Data for each Country Code defined by ISO 3166-1
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CountryCode {
    ///English short name
    name: &'static str,
    ///Alpha-2 code
    alpha2: &'static str,
    ///Alpha-3 code
    alpha3: &'static str,
    ///Numeric code
    numeric: i32,
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct CountryCode {
    ///English short name
    pub name: &'static str,
    ///Alpha-2 code
    pub alpha2: &'static str,
    ///Alpha-3 code
    pub alpha3: &'static str,
    ///Numeric code
    pub numeric: i32,
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
impl CountryCode {

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.into()
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(getter)]
    pub fn alpha2(&self) -> String {
        self.alpha2.into()
    }
    
    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(getter)]
    pub fn alpha3(&self) -> String {
        self.alpha3.into()
    }

    #[cfg(target_arch = "wasm32")]
    #[wasm_bindgen(getter)]
    pub fn numeric(&self) -> i32 {
        self.numeric
    }

    ///Return len 3 String for CountryCode numeric
    pub fn numeric_str (&self) -> String {
        format!("{:03}", self.numeric)
    }

    ///Return Subdivision for ISO 3166-2
    #[cfg(not(target_arch = "wasm32"))]
    pub fn subdivisions (&self) -> Option<&[iso3166_2::Subdivision]> {
        iso3166_2::SUBDIVISION_COUNTRY_MAP.get(self.alpha2).cloned()
    }

    #[cfg(target_arch = "wasm32")]
    pub fn subdivisions (&self) -> Array {
        let ps = iso3166_2::SUBDIVISION_COUNTRY_MAP.get(self.alpha2).cloned();
        let mut vector: Vec<iso3166_2::Subdivision> = Vec::new(); 
        match ps {
            Some(p) => {
                for i in 0..p.len() {
			        vector.push(p[i])
		        }
            },
            None => {

            },
        }

        vector.into_iter().map(JsValue::from).collect()
    }
}
/// Returns the CountryCode with the given Alpha2 code, if exists.
/// #Sample
/// ```
/// let country = rust_iso3166::from_alpha2("AU");
/// assert_eq!("AUS", country.unwrap().alpha3);
/// ```
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn from_alpha2(alpha2: &str) -> Option<CountryCode> {
    ALPHA2_MAP.get(alpha2).cloned()
}

/// Returns the CountryCode with the given Alpha3 code, if exists.
/// #Sample
/// ```
/// let country = rust_iso3166::from_alpha3("AUS");
/// assert_eq!(036, country.unwrap().numeric);
/// ```
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn from_alpha3(alpha3: &str) -> Option<CountryCode> {
    ALPHA3_MAP.get(alpha3).cloned()
}

/// Returns the CountryCode with the given numeric , if exists.
// #Sample
/// ```
/// let country = rust_iso3166::from_numeric(036);
/// assert_eq!("AUS", country.unwrap().alpha3);
/// ```
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn from_numeric(numeric: i32) -> Option<CountryCode> {
    let k = format!("{:03}", numeric);
    NUMERIC_MAP.get(&k).cloned()
}

/// Returns the CountryCode with the given numeric 3 length str, if exists.
// #Sample
/// ```
/// let country = rust_iso3166::from_numeric_str("036");
/// assert_eq!("AUS", country.unwrap().alpha3);
/// ```
#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn from_numeric_str(numeric: &str) -> Option<CountryCode> {
    NUMERIC_MAP.get(numeric).cloned()
}



pub const AF: CountryCode = CountryCode {
    name: "Afghanistan[b]",
    alpha2: "AF",
    alpha3: "AFG",
    numeric: 4,
};


pub const AX: CountryCode = CountryCode {
    name: "Åland Islands",
    alpha2: "AX",
    alpha3: "ALA",
    numeric: 248,
};


pub const AL: CountryCode = CountryCode {
    name: "Albania",
    alpha2: "AL",
    alpha3: "ALB",
    numeric: 8,
};


pub const DZ: CountryCode = CountryCode {
    name: "Algeria",
    alpha2: "DZ",
    alpha3: "DZA",
    numeric: 12,
};


pub const AS: CountryCode = CountryCode {
    name: "American Samoa",
    alpha2: "AS",
    alpha3: "ASM",
    numeric: 16,
};


pub const AD: CountryCode = CountryCode {
    name: "Andorra",
    alpha2: "AD",
    alpha3: "AND",
    numeric: 20,
};


pub const AO: CountryCode = CountryCode {
    name: "Angola",
    alpha2: "AO",
    alpha3: "AGO",
    numeric: 24,
};


pub const AI: CountryCode = CountryCode {
    name: "Anguilla",
    alpha2: "AI",
    alpha3: "AIA",
    numeric: 660,
};


pub const AQ: CountryCode = CountryCode {
    name: "Antarctica",
    alpha2: "AQ",
    alpha3: "ATA",
    numeric: 10,
};


pub const AG: CountryCode = CountryCode {
    name: "Antigua and Barbuda",
    alpha2: "AG",
    alpha3: "ATG",
    numeric: 28,
};


pub const AR: CountryCode = CountryCode {
    name: "Argentina",
    alpha2: "AR",
    alpha3: "ARG",
    numeric: 32,
};


pub const AM: CountryCode = CountryCode {
    name: "Armenia",
    alpha2: "AM",
    alpha3: "ARM",
    numeric: 51,
};


pub const AW: CountryCode = CountryCode {
    name: "Aruba",
    alpha2: "AW",
    alpha3: "ABW",
    numeric: 533,
};


pub const AU: CountryCode = CountryCode {
    name: "Australia",
    alpha2: "AU",
    alpha3: "AUS",
    numeric: 36,
};


pub const AT: CountryCode = CountryCode {
    name: "Austria",
    alpha2: "AT",
    alpha3: "AUT",
    numeric: 40,
};


pub const AZ: CountryCode = CountryCode {
    name: "Azerbaijan",
    alpha2: "AZ",
    alpha3: "AZE",
    numeric: 31,
};


pub const BS: CountryCode = CountryCode {
    name: "Bahamas",
    alpha2: "BS",
    alpha3: "BHS",
    numeric: 44,
};


pub const BH: CountryCode = CountryCode {
    name: "Bahrain",
    alpha2: "BH",
    alpha3: "BHR",
    numeric: 48,
};


pub const BD: CountryCode = CountryCode {
    name: "Bangladesh",
    alpha2: "BD",
    alpha3: "BGD",
    numeric: 50,
};


pub const BB: CountryCode = CountryCode {
    name: "Barbados",
    alpha2: "BB",
    alpha3: "BRB",
    numeric: 52,
};


pub const BY: CountryCode = CountryCode {
    name: "Belarus",
    alpha2: "BY",
    alpha3: "BLR",
    numeric: 112,
};


pub const BE: CountryCode = CountryCode {
    name: "Belgium",
    alpha2: "BE",
    alpha3: "BEL",
    numeric: 56,
};


pub const BZ: CountryCode = CountryCode {
    name: "Belize",
    alpha2: "BZ",
    alpha3: "BLZ",
    numeric: 84,
};


pub const BJ: CountryCode = CountryCode {
    name: "Benin",
    alpha2: "BJ",
    alpha3: "BEN",
    numeric: 204,
};


pub const BM: CountryCode = CountryCode {
    name: "Bermuda",
    alpha2: "BM",
    alpha3: "BMU",
    numeric: 60,
};


pub const BT: CountryCode = CountryCode {
    name: "Bhutan",
    alpha2: "BT",
    alpha3: "BTN",
    numeric: 64,
};


pub const BO: CountryCode = CountryCode {
    name: "Bolivia (Plurinational State of)",
    alpha2: "BO",
    alpha3: "BOL",
    numeric: 68,
};


pub const BQ: CountryCode = CountryCode {
    name: "Bonaire, Sint Eustatius and Saba[c]",
    alpha2: "BQ",
    alpha3: "BES",
    numeric: 535,
};


pub const BA: CountryCode = CountryCode {
    name: "Bosnia and Herzegovina",
    alpha2: "BA",
    alpha3: "BIH",
    numeric: 70,
};


pub const BW: CountryCode = CountryCode {
    name: "Botswana",
    alpha2: "BW",
    alpha3: "BWA",
    numeric: 72,
};


pub const BV: CountryCode = CountryCode {
    name: "Bouvet Island",
    alpha2: "BV",
    alpha3: "BVT",
    numeric: 74,
};


pub const BR: CountryCode = CountryCode {
    name: "Brazil",
    alpha2: "BR",
    alpha3: "BRA",
    numeric: 76,
};


pub const IO: CountryCode = CountryCode {
    name: "British Indian Ocean Territory",
    alpha2: "IO",
    alpha3: "IOT",
    numeric: 86,
};


pub const BN: CountryCode = CountryCode {
    name: "Brunei Darussalam",
    alpha2: "BN",
    alpha3: "BRN",
    numeric: 96,
};


pub const BG: CountryCode = CountryCode {
    name: "Bulgaria",
    alpha2: "BG",
    alpha3: "BGR",
    numeric: 100,
};


pub const BF: CountryCode = CountryCode {
    name: "Burkina Faso",
    alpha2: "BF",
    alpha3: "BFA",
    numeric: 854,
};


pub const BI: CountryCode = CountryCode {
    name: "Burundi",
    alpha2: "BI",
    alpha3: "BDI",
    numeric: 108,
};


pub const CV: CountryCode = CountryCode {
    name: "Cabo Verde",
    alpha2: "CV",
    alpha3: "CPV",
    numeric: 132,
};


pub const KH: CountryCode = CountryCode {
    name: "Cambodia",
    alpha2: "KH",
    alpha3: "KHM",
    numeric: 116,
};


pub const CM: CountryCode = CountryCode {
    name: "Cameroon",
    alpha2: "CM",
    alpha3: "CMR",
    numeric: 120,
};


pub const CA: CountryCode = CountryCode {
    name: "Canada",
    alpha2: "CA",
    alpha3: "CAN",
    numeric: 124,
};


pub const KY: CountryCode = CountryCode {
    name: "Cayman Islands",
    alpha2: "KY",
    alpha3: "CYM",
    numeric: 136,
};


pub const CF: CountryCode = CountryCode {
    name: "Central African Republic",
    alpha2: "CF",
    alpha3: "CAF",
    numeric: 140,
};


pub const TD: CountryCode = CountryCode {
    name: "Chad",
    alpha2: "TD",
    alpha3: "TCD",
    numeric: 148,
};


pub const CL: CountryCode = CountryCode {
    name: "Chile",
    alpha2: "CL",
    alpha3: "CHL",
    numeric: 152,
};


pub const CN: CountryCode = CountryCode {
    name: "China[b]",
    alpha2: "CN",
    alpha3: "CHN",
    numeric: 156,
};


pub const CX: CountryCode = CountryCode {
    name: "Christmas Island",
    alpha2: "CX",
    alpha3: "CXR",
    numeric: 162,
};


pub const CC: CountryCode = CountryCode {
    name: "Cocos (Keeling) Islands",
    alpha2: "CC",
    alpha3: "CCK",
    numeric: 166,
};


pub const CO: CountryCode = CountryCode {
    name: "Colombia",
    alpha2: "CO",
    alpha3: "COL",
    numeric: 170,
};


pub const KM: CountryCode = CountryCode {
    name: "Comoros",
    alpha2: "KM",
    alpha3: "COM",
    numeric: 174,
};


pub const CG: CountryCode = CountryCode {
    name: "Congo",
    alpha2: "CG",
    alpha3: "COG",
    numeric: 178,
};


pub const CD: CountryCode = CountryCode {
    name: "Congo, Democratic Republic of the",
    alpha2: "CD",
    alpha3: "COD",
    numeric: 180,
};


pub const CK: CountryCode = CountryCode {
    name: "Cook Islands",
    alpha2: "CK",
    alpha3: "COK",
    numeric: 184,
};


pub const CR: CountryCode = CountryCode {
    name: "Costa Rica",
    alpha2: "CR",
    alpha3: "CRI",
    numeric: 188,
};


pub const CI: CountryCode = CountryCode {
    name: "Côte d'Ivoire",
    alpha2: "CI",
    alpha3: "CIV",
    numeric: 384,
};


pub const HR: CountryCode = CountryCode {
    name: "Croatia",
    alpha2: "HR",
    alpha3: "HRV",
    numeric: 191,
};


pub const CU: CountryCode = CountryCode {
    name: "Cuba",
    alpha2: "CU",
    alpha3: "CUB",
    numeric: 192,
};


pub const CW: CountryCode = CountryCode {
    name: "Curaçao",
    alpha2: "CW",
    alpha3: "CUW",
    numeric: 531,
};


pub const CY: CountryCode = CountryCode {
    name: "Cyprus[b]",
    alpha2: "CY",
    alpha3: "CYP",
    numeric: 196,
};


pub const CZ: CountryCode = CountryCode {
    name: "Czechia",
    alpha2: "CZ",
    alpha3: "CZE",
    numeric: 203,
};


pub const DK: CountryCode = CountryCode {
    name: "Denmark",
    alpha2: "DK",
    alpha3: "DNK",
    numeric: 208,
};


pub const DJ: CountryCode = CountryCode {
    name: "Djibouti",
    alpha2: "DJ",
    alpha3: "DJI",
    numeric: 262,
};


pub const DM: CountryCode = CountryCode {
    name: "Dominica",
    alpha2: "DM",
    alpha3: "DMA",
    numeric: 212,
};


pub const DO: CountryCode = CountryCode {
    name: "Dominican Republic",
    alpha2: "DO",
    alpha3: "DOM",
    numeric: 214,
};


pub const EC: CountryCode = CountryCode {
    name: "Ecuador",
    alpha2: "EC",
    alpha3: "ECU",
    numeric: 218,
};


pub const EG: CountryCode = CountryCode {
    name: "Egypt",
    alpha2: "EG",
    alpha3: "EGY",
    numeric: 818,
};


pub const SV: CountryCode = CountryCode {
    name: "El Salvador",
    alpha2: "SV",
    alpha3: "SLV",
    numeric: 222,
};


pub const GQ: CountryCode = CountryCode {
    name: "Equatorial Guinea",
    alpha2: "GQ",
    alpha3: "GNQ",
    numeric: 226,
};


pub const ER: CountryCode = CountryCode {
    name: "Eritrea",
    alpha2: "ER",
    alpha3: "ERI",
    numeric: 232,
};


pub const EE: CountryCode = CountryCode {
    name: "Estonia",
    alpha2: "EE",
    alpha3: "EST",
    numeric: 233,
};


pub const SZ: CountryCode = CountryCode {
    name: "Eswatini",
    alpha2: "SZ",
    alpha3: "SWZ",
    numeric: 748,
};


pub const ET: CountryCode = CountryCode {
    name: "Ethiopia",
    alpha2: "ET",
    alpha3: "ETH",
    numeric: 231,
};


pub const FK: CountryCode = CountryCode {
    name: "Falkland Islands (Malvinas)[b]",
    alpha2: "FK",
    alpha3: "FLK",
    numeric: 238,
};


pub const FO: CountryCode = CountryCode {
    name: "Faroe Islands",
    alpha2: "FO",
    alpha3: "FRO",
    numeric: 234,
};


pub const FJ: CountryCode = CountryCode {
    name: "Fiji",
    alpha2: "FJ",
    alpha3: "FJI",
    numeric: 242,
};


pub const FI: CountryCode = CountryCode {
    name: "Finland",
    alpha2: "FI",
    alpha3: "FIN",
    numeric: 246,
};


pub const FR: CountryCode = CountryCode {
    name: "France",
    alpha2: "FR",
    alpha3: "FRA",
    numeric: 250,
};


pub const GF: CountryCode = CountryCode {
    name: "French Guiana",
    alpha2: "GF",
    alpha3: "GUF",
    numeric: 254,
};


pub const PF: CountryCode = CountryCode {
    name: "French Polynesia",
    alpha2: "PF",
    alpha3: "PYF",
    numeric: 258,
};


pub const TF: CountryCode = CountryCode {
    name: "French Southern Territories",
    alpha2: "TF",
    alpha3: "ATF",
    numeric: 260,
};


pub const GA: CountryCode = CountryCode {
    name: "Gabon",
    alpha2: "GA",
    alpha3: "GAB",
    numeric: 266,
};


pub const GM: CountryCode = CountryCode {
    name: "Gambia",
    alpha2: "GM",
    alpha3: "GMB",
    numeric: 270,
};


pub const GE: CountryCode = CountryCode {
    name: "Georgia",
    alpha2: "GE",
    alpha3: "GEO",
    numeric: 268,
};


pub const DE: CountryCode = CountryCode {
    name: "Germany",
    alpha2: "DE",
    alpha3: "DEU",
    numeric: 276,
};


pub const GH: CountryCode = CountryCode {
    name: "Ghana",
    alpha2: "GH",
    alpha3: "GHA",
    numeric: 288,
};


pub const GI: CountryCode = CountryCode {
    name: "Gibraltar",
    alpha2: "GI",
    alpha3: "GIB",
    numeric: 292,
};


pub const GR: CountryCode = CountryCode {
    name: "Greece",
    alpha2: "GR",
    alpha3: "GRC",
    numeric: 300,
};


pub const GL: CountryCode = CountryCode {
    name: "Greenland",
    alpha2: "GL",
    alpha3: "GRL",
    numeric: 304,
};


pub const GD: CountryCode = CountryCode {
    name: "Grenada",
    alpha2: "GD",
    alpha3: "GRD",
    numeric: 308,
};


pub const GP: CountryCode = CountryCode {
    name: "Guadeloupe",
    alpha2: "GP",
    alpha3: "GLP",
    numeric: 312,
};


pub const GU: CountryCode = CountryCode {
    name: "Guam",
    alpha2: "GU",
    alpha3: "GUM",
    numeric: 316,
};


pub const GT: CountryCode = CountryCode {
    name: "Guatemala",
    alpha2: "GT",
    alpha3: "GTM",
    numeric: 320,
};


pub const GG: CountryCode = CountryCode {
    name: "Guernsey",
    alpha2: "GG",
    alpha3: "GGY",
    numeric: 831,
};


pub const GN: CountryCode = CountryCode {
    name: "Guinea",
    alpha2: "GN",
    alpha3: "GIN",
    numeric: 324,
};


pub const GW: CountryCode = CountryCode {
    name: "Guinea-Bissau",
    alpha2: "GW",
    alpha3: "GNB",
    numeric: 624,
};


pub const GY: CountryCode = CountryCode {
    name: "Guyana",
    alpha2: "GY",
    alpha3: "GUY",
    numeric: 328,
};


pub const HT: CountryCode = CountryCode {
    name: "Haiti",
    alpha2: "HT",
    alpha3: "HTI",
    numeric: 332,
};


pub const HM: CountryCode = CountryCode {
    name: "Heard Island and McDonald Islands",
    alpha2: "HM",
    alpha3: "HMD",
    numeric: 334,
};


pub const VA: CountryCode = CountryCode {
    name: "Holy See",
    alpha2: "VA",
    alpha3: "VAT",
    numeric: 336,
};


pub const HN: CountryCode = CountryCode {
    name: "Honduras",
    alpha2: "HN",
    alpha3: "HND",
    numeric: 340,
};


pub const HK: CountryCode = CountryCode {
    name: "Hong Kong",
    alpha2: "HK",
    alpha3: "HKG",
    numeric: 344,
};


pub const HU: CountryCode = CountryCode {
    name: "Hungary",
    alpha2: "HU",
    alpha3: "HUN",
    numeric: 348,
};


pub const IS: CountryCode = CountryCode {
    name: "Iceland",
    alpha2: "IS",
    alpha3: "ISL",
    numeric: 352,
};


pub const IN: CountryCode = CountryCode {
    name: "India",
    alpha2: "IN",
    alpha3: "IND",
    numeric: 356,
};


pub const ID: CountryCode = CountryCode {
    name: "Indonesia",
    alpha2: "ID",
    alpha3: "IDN",
    numeric: 360,
};


pub const IR: CountryCode = CountryCode {
    name: "Iran (Islamic Republic of)",
    alpha2: "IR",
    alpha3: "IRN",
    numeric: 364,
};


pub const IQ: CountryCode = CountryCode {
    name: "Iraq",
    alpha2: "IQ",
    alpha3: "IRQ",
    numeric: 368,
};


pub const IE: CountryCode = CountryCode {
    name: "Ireland",
    alpha2: "IE",
    alpha3: "IRL",
    numeric: 372,
};


pub const IM: CountryCode = CountryCode {
    name: "Isle of Man",
    alpha2: "IM",
    alpha3: "IMN",
    numeric: 833,
};


pub const IL: CountryCode = CountryCode {
    name: "Israel",
    alpha2: "IL",
    alpha3: "ISR",
    numeric: 376,
};


pub const IT: CountryCode = CountryCode {
    name: "Italy",
    alpha2: "IT",
    alpha3: "ITA",
    numeric: 380,
};


pub const JM: CountryCode = CountryCode {
    name: "Jamaica",
    alpha2: "JM",
    alpha3: "JAM",
    numeric: 388,
};


pub const JP: CountryCode = CountryCode {
    name: "Japan",
    alpha2: "JP",
    alpha3: "JPN",
    numeric: 392,
};


pub const JE: CountryCode = CountryCode {
    name: "Jersey",
    alpha2: "JE",
    alpha3: "JEY",
    numeric: 832,
};


pub const JO: CountryCode = CountryCode {
    name: "Jordan",
    alpha2: "JO",
    alpha3: "JOR",
    numeric: 400,
};


pub const KZ: CountryCode = CountryCode {
    name: "Kazakhstan",
    alpha2: "KZ",
    alpha3: "KAZ",
    numeric: 398,
};


pub const KE: CountryCode = CountryCode {
    name: "Kenya",
    alpha2: "KE",
    alpha3: "KEN",
    numeric: 404,
};


pub const KI: CountryCode = CountryCode {
    name: "Kiribati",
    alpha2: "KI",
    alpha3: "KIR",
    numeric: 296,
};


pub const KP: CountryCode = CountryCode {
    name: "Korea (Democratic People's Republic of)",
    alpha2: "KP",
    alpha3: "PRK",
    numeric: 408,
};


pub const KR: CountryCode = CountryCode {
    name: "Korea, Republic of",
    alpha2: "KR",
    alpha3: "KOR",
    numeric: 410,
};


pub const KW: CountryCode = CountryCode {
    name: "Kuwait",
    alpha2: "KW",
    alpha3: "KWT",
    numeric: 414,
};


pub const KG: CountryCode = CountryCode {
    name: "Kyrgyzstan",
    alpha2: "KG",
    alpha3: "KGZ",
    numeric: 417,
};


pub const LA: CountryCode = CountryCode {
    name: "Lao People's Democratic Republic",
    alpha2: "LA",
    alpha3: "LAO",
    numeric: 418,
};


pub const LV: CountryCode = CountryCode {
    name: "Latvia",
    alpha2: "LV",
    alpha3: "LVA",
    numeric: 428,
};


pub const LB: CountryCode = CountryCode {
    name: "Lebanon",
    alpha2: "LB",
    alpha3: "LBN",
    numeric: 422,
};


pub const LS: CountryCode = CountryCode {
    name: "Lesotho",
    alpha2: "LS",
    alpha3: "LSO",
    numeric: 426,
};


pub const LR: CountryCode = CountryCode {
    name: "Liberia",
    alpha2: "LR",
    alpha3: "LBR",
    numeric: 430,
};


pub const LY: CountryCode = CountryCode {
    name: "Libya",
    alpha2: "LY",
    alpha3: "LBY",
    numeric: 434,
};


pub const LI: CountryCode = CountryCode {
    name: "Liechtenstein",
    alpha2: "LI",
    alpha3: "LIE",
    numeric: 438,
};


pub const LT: CountryCode = CountryCode {
    name: "Lithuania",
    alpha2: "LT",
    alpha3: "LTU",
    numeric: 440,
};


pub const LU: CountryCode = CountryCode {
    name: "Luxembourg",
    alpha2: "LU",
    alpha3: "LUX",
    numeric: 442,
};


pub const MO: CountryCode = CountryCode {
    name: "Macao",
    alpha2: "MO",
    alpha3: "MAC",
    numeric: 446,
};


pub const MG: CountryCode = CountryCode {
    name: "Madagascar",
    alpha2: "MG",
    alpha3: "MDG",
    numeric: 450,
};


pub const MW: CountryCode = CountryCode {
    name: "Malawi",
    alpha2: "MW",
    alpha3: "MWI",
    numeric: 454,
};


pub const MY: CountryCode = CountryCode {
    name: "Malaysia",
    alpha2: "MY",
    alpha3: "MYS",
    numeric: 458,
};


pub const MV: CountryCode = CountryCode {
    name: "Maldives",
    alpha2: "MV",
    alpha3: "MDV",
    numeric: 462,
};


pub const ML: CountryCode = CountryCode {
    name: "Mali",
    alpha2: "ML",
    alpha3: "MLI",
    numeric: 466,
};


pub const MT: CountryCode = CountryCode {
    name: "Malta",
    alpha2: "MT",
    alpha3: "MLT",
    numeric: 470,
};


pub const MH: CountryCode = CountryCode {
    name: "Marshall Islands",
    alpha2: "MH",
    alpha3: "MHL",
    numeric: 584,
};


pub const MQ: CountryCode = CountryCode {
    name: "Martinique",
    alpha2: "MQ",
    alpha3: "MTQ",
    numeric: 474,
};


pub const MR: CountryCode = CountryCode {
    name: "Mauritania",
    alpha2: "MR",
    alpha3: "MRT",
    numeric: 478,
};


pub const MU: CountryCode = CountryCode {
    name: "Mauritius",
    alpha2: "MU",
    alpha3: "MUS",
    numeric: 480,
};


pub const YT: CountryCode = CountryCode {
    name: "Mayotte",
    alpha2: "YT",
    alpha3: "MYT",
    numeric: 175,
};


pub const MX: CountryCode = CountryCode {
    name: "Mexico",
    alpha2: "MX",
    alpha3: "MEX",
    numeric: 484,
};


pub const FM: CountryCode = CountryCode {
    name: "Micronesia (Federated States of)",
    alpha2: "FM",
    alpha3: "FSM",
    numeric: 583,
};


pub const MD: CountryCode = CountryCode {
    name: "Moldova, Republic of",
    alpha2: "MD",
    alpha3: "MDA",
    numeric: 498,
};


pub const MC: CountryCode = CountryCode {
    name: "Monaco",
    alpha2: "MC",
    alpha3: "MCO",
    numeric: 492,
};


pub const MN: CountryCode = CountryCode {
    name: "Mongolia",
    alpha2: "MN",
    alpha3: "MNG",
    numeric: 496,
};


pub const ME: CountryCode = CountryCode {
    name: "Montenegro",
    alpha2: "ME",
    alpha3: "MNE",
    numeric: 499,
};


pub const MS: CountryCode = CountryCode {
    name: "Montserrat",
    alpha2: "MS",
    alpha3: "MSR",
    numeric: 500,
};


pub const MA: CountryCode = CountryCode {
    name: "Morocco",
    alpha2: "MA",
    alpha3: "MAR",
    numeric: 504,
};


pub const MZ: CountryCode = CountryCode {
    name: "Mozambique",
    alpha2: "MZ",
    alpha3: "MOZ",
    numeric: 508,
};


pub const MM: CountryCode = CountryCode {
    name: "Myanmar",
    alpha2: "MM",
    alpha3: "MMR",
    numeric: 104,
};


pub const NA: CountryCode = CountryCode {
    name: "Namibia",
    alpha2: "NA",
    alpha3: "NAM",
    numeric: 516,
};


pub const NR: CountryCode = CountryCode {
    name: "Nauru",
    alpha2: "NR",
    alpha3: "NRU",
    numeric: 520,
};


pub const NP: CountryCode = CountryCode {
    name: "Nepal",
    alpha2: "NP",
    alpha3: "NPL",
    numeric: 524,
};


pub const NL: CountryCode = CountryCode {
    name: "Netherlands",
    alpha2: "NL",
    alpha3: "NLD",
    numeric: 528,
};


pub const NC: CountryCode = CountryCode {
    name: "New Caledonia",
    alpha2: "NC",
    alpha3: "NCL",
    numeric: 540,
};


pub const NZ: CountryCode = CountryCode {
    name: "New Zealand",
    alpha2: "NZ",
    alpha3: "NZL",
    numeric: 554,
};


pub const NI: CountryCode = CountryCode {
    name: "Nicaragua",
    alpha2: "NI",
    alpha3: "NIC",
    numeric: 558,
};


pub const NE: CountryCode = CountryCode {
    name: "Niger",
    alpha2: "NE",
    alpha3: "NER",
    numeric: 562,
};


pub const NG: CountryCode = CountryCode {
    name: "Nigeria",
    alpha2: "NG",
    alpha3: "NGA",
    numeric: 566,
};


pub const NU: CountryCode = CountryCode {
    name: "Niue",
    alpha2: "NU",
    alpha3: "NIU",
    numeric: 570,
};


pub const NF: CountryCode = CountryCode {
    name: "Norfolk Island",
    alpha2: "NF",
    alpha3: "NFK",
    numeric: 574,
};


pub const MK: CountryCode = CountryCode {
    name: "North Macedonia",
    alpha2: "MK",
    alpha3: "MKD",
    numeric: 807,
};


pub const MP: CountryCode = CountryCode {
    name: "Northern Mariana Islands",
    alpha2: "MP",
    alpha3: "MNP",
    numeric: 580,
};


pub const NO: CountryCode = CountryCode {
    name: "Norway",
    alpha2: "NO",
    alpha3: "NOR",
    numeric: 578,
};


pub const OM: CountryCode = CountryCode {
    name: "Oman",
    alpha2: "OM",
    alpha3: "OMN",
    numeric: 512,
};


pub const PK: CountryCode = CountryCode {
    name: "Pakistan",
    alpha2: "PK",
    alpha3: "PAK",
    numeric: 586,
};


pub const PW: CountryCode = CountryCode {
    name: "Palau",
    alpha2: "PW",
    alpha3: "PLW",
    numeric: 585,
};


pub const PS: CountryCode = CountryCode {
    name: "Palestine, State of[b]",
    alpha2: "PS",
    alpha3: "PSE",
    numeric: 275,
};


pub const PA: CountryCode = CountryCode {
    name: "Panama",
    alpha2: "PA",
    alpha3: "PAN",
    numeric: 591,
};


pub const PG: CountryCode = CountryCode {
    name: "Papua New Guinea",
    alpha2: "PG",
    alpha3: "PNG",
    numeric: 598,
};


pub const PY: CountryCode = CountryCode {
    name: "Paraguay",
    alpha2: "PY",
    alpha3: "PRY",
    numeric: 600,
};


pub const PE: CountryCode = CountryCode {
    name: "Peru",
    alpha2: "PE",
    alpha3: "PER",
    numeric: 604,
};


pub const PH: CountryCode = CountryCode {
    name: "Philippines",
    alpha2: "PH",
    alpha3: "PHL",
    numeric: 608,
};


pub const PN: CountryCode = CountryCode {
    name: "Pitcairn",
    alpha2: "PN",
    alpha3: "PCN",
    numeric: 612,
};


pub const PL: CountryCode = CountryCode {
    name: "Poland",
    alpha2: "PL",
    alpha3: "POL",
    numeric: 616,
};


pub const PT: CountryCode = CountryCode {
    name: "Portugal",
    alpha2: "PT",
    alpha3: "PRT",
    numeric: 620,
};


pub const PR: CountryCode = CountryCode {
    name: "Puerto Rico",
    alpha2: "PR",
    alpha3: "PRI",
    numeric: 630,
};


pub const QA: CountryCode = CountryCode {
    name: "Qatar",
    alpha2: "QA",
    alpha3: "QAT",
    numeric: 634,
};


pub const RE: CountryCode = CountryCode {
    name: "Réunion",
    alpha2: "RE",
    alpha3: "REU",
    numeric: 638,
};


pub const RO: CountryCode = CountryCode {
    name: "Romania",
    alpha2: "RO",
    alpha3: "ROU",
    numeric: 642,
};


pub const RU: CountryCode = CountryCode {
    name: "Russian Federation",
    alpha2: "RU",
    alpha3: "RUS",
    numeric: 643,
};


pub const RW: CountryCode = CountryCode {
    name: "Rwanda",
    alpha2: "RW",
    alpha3: "RWA",
    numeric: 646,
};


pub const BL: CountryCode = CountryCode {
    name: "Saint Barthélemy",
    alpha2: "BL",
    alpha3: "BLM",
    numeric: 652,
};


pub const SH: CountryCode = CountryCode {
    name: "Saint Helena, Ascension and Tristan da Cunha[d]",
    alpha2: "SH",
    alpha3: "SHN",
    numeric: 654,
};


pub const KN: CountryCode = CountryCode {
    name: "Saint Kitts and Nevis",
    alpha2: "KN",
    alpha3: "KNA",
    numeric: 659,
};


pub const LC: CountryCode = CountryCode {
    name: "Saint Lucia",
    alpha2: "LC",
    alpha3: "LCA",
    numeric: 662,
};


pub const MF: CountryCode = CountryCode {
    name: "Saint Martin (French part)",
    alpha2: "MF",
    alpha3: "MAF",
    numeric: 663,
};


pub const PM: CountryCode = CountryCode {
    name: "Saint Pierre and Miquelon",
    alpha2: "PM",
    alpha3: "SPM",
    numeric: 666,
};


pub const VC: CountryCode = CountryCode {
    name: "Saint Vincent and the Grenadines",
    alpha2: "VC",
    alpha3: "VCT",
    numeric: 670,
};


pub const WS: CountryCode = CountryCode {
    name: "Samoa",
    alpha2: "WS",
    alpha3: "WSM",
    numeric: 882,
};


pub const SM: CountryCode = CountryCode {
    name: "San Marino",
    alpha2: "SM",
    alpha3: "SMR",
    numeric: 674,
};


pub const ST: CountryCode = CountryCode {
    name: "Sao Tome and Principe",
    alpha2: "ST",
    alpha3: "STP",
    numeric: 678,
};


pub const SA: CountryCode = CountryCode {
    name: "Saudi Arabia",
    alpha2: "SA",
    alpha3: "SAU",
    numeric: 682,
};


pub const SN: CountryCode = CountryCode {
    name: "Senegal",
    alpha2: "SN",
    alpha3: "SEN",
    numeric: 686,
};


pub const RS: CountryCode = CountryCode {
    name: "Serbia",
    alpha2: "RS",
    alpha3: "SRB",
    numeric: 688,
};


pub const SC: CountryCode = CountryCode {
    name: "Seychelles",
    alpha2: "SC",
    alpha3: "SYC",
    numeric: 690,
};


pub const SL: CountryCode = CountryCode {
    name: "Sierra Leone",
    alpha2: "SL",
    alpha3: "SLE",
    numeric: 694,
};


pub const SG: CountryCode = CountryCode {
    name: "Singapore",
    alpha2: "SG",
    alpha3: "SGP",
    numeric: 702,
};


pub const SX: CountryCode = CountryCode {
    name: "Sint Maarten (Dutch part)",
    alpha2: "SX",
    alpha3: "SXM",
    numeric: 534,
};


pub const SK: CountryCode = CountryCode {
    name: "Slovakia",
    alpha2: "SK",
    alpha3: "SVK",
    numeric: 703,
};


pub const SI: CountryCode = CountryCode {
    name: "Slovenia",
    alpha2: "SI",
    alpha3: "SVN",
    numeric: 705,
};


pub const SB: CountryCode = CountryCode {
    name: "Solomon Islands",
    alpha2: "SB",
    alpha3: "SLB",
    numeric: 90,
};


pub const SO: CountryCode = CountryCode {
    name: "Somalia",
    alpha2: "SO",
    alpha3: "SOM",
    numeric: 706,
};


pub const ZA: CountryCode = CountryCode {
    name: "South Africa",
    alpha2: "ZA",
    alpha3: "ZAF",
    numeric: 710,
};


pub const GS: CountryCode = CountryCode {
    name: "South Georgia and the South Sandwich Islands",
    alpha2: "GS",
    alpha3: "SGS",
    numeric: 239,
};


pub const SS: CountryCode = CountryCode {
    name: "South Sudan",
    alpha2: "SS",
    alpha3: "SSD",
    numeric: 728,
};


pub const ES: CountryCode = CountryCode {
    name: "Spain",
    alpha2: "ES",
    alpha3: "ESP",
    numeric: 724,
};


pub const LK: CountryCode = CountryCode {
    name: "Sri Lanka",
    alpha2: "LK",
    alpha3: "LKA",
    numeric: 144,
};


pub const SD: CountryCode = CountryCode {
    name: "Sudan",
    alpha2: "SD",
    alpha3: "SDN",
    numeric: 729,
};


pub const SR: CountryCode = CountryCode {
    name: "Suriname",
    alpha2: "SR",
    alpha3: "SUR",
    numeric: 740,
};


pub const SJ: CountryCode = CountryCode {
    name: "Svalbard and Jan Mayen[e]",
    alpha2: "SJ",
    alpha3: "SJM",
    numeric: 744,
};


pub const SE: CountryCode = CountryCode {
    name: "Sweden",
    alpha2: "SE",
    alpha3: "SWE",
    numeric: 752,
};


pub const CH: CountryCode = CountryCode {
    name: "Switzerland",
    alpha2: "CH",
    alpha3: "CHE",
    numeric: 756,
};


pub const SY: CountryCode = CountryCode {
    name: "Syrian Arab Republic",
    alpha2: "SY",
    alpha3: "SYR",
    numeric: 760,
};


pub const TW: CountryCode = CountryCode {
    name: "Taiwan, Province of China[b]",
    alpha2: "TW",
    alpha3: "TWN",
    numeric: 158,
};


pub const TJ: CountryCode = CountryCode {
    name: "Tajikistan",
    alpha2: "TJ",
    alpha3: "TJK",
    numeric: 762,
};


pub const TZ: CountryCode = CountryCode {
    name: "Tanzania, United Republic of",
    alpha2: "TZ",
    alpha3: "TZA",
    numeric: 834,
};


pub const TH: CountryCode = CountryCode {
    name: "Thailand",
    alpha2: "TH",
    alpha3: "THA",
    numeric: 764,
};


pub const TL: CountryCode = CountryCode {
    name: "Timor-Leste",
    alpha2: "TL",
    alpha3: "TLS",
    numeric: 626,
};


pub const TG: CountryCode = CountryCode {
    name: "Togo",
    alpha2: "TG",
    alpha3: "TGO",
    numeric: 768,
};


pub const TK: CountryCode = CountryCode {
    name: "Tokelau",
    alpha2: "TK",
    alpha3: "TKL",
    numeric: 772,
};


pub const TO: CountryCode = CountryCode {
    name: "Tonga",
    alpha2: "TO",
    alpha3: "TON",
    numeric: 776,
};


pub const TT: CountryCode = CountryCode {
    name: "Trinidad and Tobago",
    alpha2: "TT",
    alpha3: "TTO",
    numeric: 780,
};


pub const TN: CountryCode = CountryCode {
    name: "Tunisia",
    alpha2: "TN",
    alpha3: "TUN",
    numeric: 788,
};


pub const TR: CountryCode = CountryCode {
    name: "Turkey",
    alpha2: "TR",
    alpha3: "TUR",
    numeric: 792,
};


pub const TM: CountryCode = CountryCode {
    name: "Turkmenistan",
    alpha2: "TM",
    alpha3: "TKM",
    numeric: 795,
};


pub const TC: CountryCode = CountryCode {
    name: "Turks and Caicos Islands",
    alpha2: "TC",
    alpha3: "TCA",
    numeric: 796,
};


pub const TV: CountryCode = CountryCode {
    name: "Tuvalu",
    alpha2: "TV",
    alpha3: "TUV",
    numeric: 798,
};


pub const UG: CountryCode = CountryCode {
    name: "Uganda",
    alpha2: "UG",
    alpha3: "UGA",
    numeric: 800,
};


pub const UA: CountryCode = CountryCode {
    name: "Ukraine",
    alpha2: "UA",
    alpha3: "UKR",
    numeric: 804,
};


pub const AE: CountryCode = CountryCode {
    name: "United Arab Emirates",
    alpha2: "AE",
    alpha3: "ARE",
    numeric: 784,
};


pub const GB: CountryCode = CountryCode {
    name: "United Kingdom of Great Britain and Northern Ireland",
    alpha2: "GB",
    alpha3: "GBR",
    numeric: 826,
};


pub const US: CountryCode = CountryCode {
    name: "United States of America",
    alpha2: "US",
    alpha3: "USA",
    numeric: 840,
};


pub const UM: CountryCode = CountryCode {
    name: "United States Minor Outlying Islands[f]",
    alpha2: "UM",
    alpha3: "UMI",
    numeric: 581,
};


pub const UY: CountryCode = CountryCode {
    name: "Uruguay",
    alpha2: "UY",
    alpha3: "URY",
    numeric: 858,
};


pub const UZ: CountryCode = CountryCode {
    name: "Uzbekistan",
    alpha2: "UZ",
    alpha3: "UZB",
    numeric: 860,
};


pub const VU: CountryCode = CountryCode {
    name: "Vanuatu",
    alpha2: "VU",
    alpha3: "VUT",
    numeric: 548,
};


pub const VE: CountryCode = CountryCode {
    name: "Venezuela (Bolivarian Republic of)",
    alpha2: "VE",
    alpha3: "VEN",
    numeric: 862,
};


pub const VN: CountryCode = CountryCode {
    name: "Viet Nam",
    alpha2: "VN",
    alpha3: "VNM",
    numeric: 704,
};


pub const VG: CountryCode = CountryCode {
    name: "Virgin Islands (British)",
    alpha2: "VG",
    alpha3: "VGB",
    numeric: 92,
};


pub const VI: CountryCode = CountryCode {
    name: "Virgin Islands (U.S.)",
    alpha2: "VI",
    alpha3: "VIR",
    numeric: 850,
};


pub const WF: CountryCode = CountryCode {
    name: "Wallis and Futuna",
    alpha2: "WF",
    alpha3: "WLF",
    numeric: 876,
};


pub const EH: CountryCode = CountryCode {
    name: "Western Sahara[b]",
    alpha2: "EH",
    alpha3: "ESH",
    numeric: 732,
};


pub const YE: CountryCode = CountryCode {
    name: "Yemen",
    alpha2: "YE",
    alpha3: "YEM",
    numeric: 887,
};


pub const ZM: CountryCode = CountryCode {
    name: "Zambia",
    alpha2: "ZM",
    alpha3: "ZMB",
    numeric: 894,
};


pub const ZW: CountryCode = CountryCode {
    name: "Zimbabwe",
    alpha2: "ZW",
    alpha3: "ZWE",
    numeric: 716,
};



///CountryCode map with  alpha2 Code key 
pub const ALPHA2_MAP: Map<&str, CountryCode> = phf_map! {


"AF" => AF,
"AX" => AX,
"AL" => AL,
"DZ" => DZ,
"AS" => AS,
"AD" => AD,
"AO" => AO,
"AI" => AI,
"AQ" => AQ,
"AG" => AG,
"AR" => AR,
"AM" => AM,
"AW" => AW,
"AU" => AU,
"AT" => AT,
"AZ" => AZ,
"BS" => BS,
"BH" => BH,
"BD" => BD,
"BB" => BB,
"BY" => BY,
"BE" => BE,
"BZ" => BZ,
"BJ" => BJ,
"BM" => BM,
"BT" => BT,
"BO" => BO,
"BQ" => BQ,
"BA" => BA,
"BW" => BW,
"BV" => BV,
"BR" => BR,
"IO" => IO,
"BN" => BN,
"BG" => BG,
"BF" => BF,
"BI" => BI,
"CV" => CV,
"KH" => KH,
"CM" => CM,
"CA" => CA,
"KY" => KY,
"CF" => CF,
"TD" => TD,
"CL" => CL,
"CN" => CN,
"CX" => CX,
"CC" => CC,
"CO" => CO,
"KM" => KM,
"CG" => CG,
"CD" => CD,
"CK" => CK,
"CR" => CR,
"CI" => CI,
"HR" => HR,
"CU" => CU,
"CW" => CW,
"CY" => CY,
"CZ" => CZ,
"DK" => DK,
"DJ" => DJ,
"DM" => DM,
"DO" => DO,
"EC" => EC,
"EG" => EG,
"SV" => SV,
"GQ" => GQ,
"ER" => ER,
"EE" => EE,
"SZ" => SZ,
"ET" => ET,
"FK" => FK,
"FO" => FO,
"FJ" => FJ,
"FI" => FI,
"FR" => FR,
"GF" => GF,
"PF" => PF,
"TF" => TF,
"GA" => GA,
"GM" => GM,
"GE" => GE,
"DE" => DE,
"GH" => GH,
"GI" => GI,
"GR" => GR,
"GL" => GL,
"GD" => GD,
"GP" => GP,
"GU" => GU,
"GT" => GT,
"GG" => GG,
"GN" => GN,
"GW" => GW,
"GY" => GY,
"HT" => HT,
"HM" => HM,
"VA" => VA,
"HN" => HN,
"HK" => HK,
"HU" => HU,
"IS" => IS,
"IN" => IN,
"ID" => ID,
"IR" => IR,
"IQ" => IQ,
"IE" => IE,
"IM" => IM,
"IL" => IL,
"IT" => IT,
"JM" => JM,
"JP" => JP,
"JE" => JE,
"JO" => JO,
"KZ" => KZ,
"KE" => KE,
"KI" => KI,
"KP" => KP,
"KR" => KR,
"KW" => KW,
"KG" => KG,
"LA" => LA,
"LV" => LV,
"LB" => LB,
"LS" => LS,
"LR" => LR,
"LY" => LY,
"LI" => LI,
"LT" => LT,
"LU" => LU,
"MO" => MO,
"MG" => MG,
"MW" => MW,
"MY" => MY,
"MV" => MV,
"ML" => ML,
"MT" => MT,
"MH" => MH,
"MQ" => MQ,
"MR" => MR,
"MU" => MU,
"YT" => YT,
"MX" => MX,
"FM" => FM,
"MD" => MD,
"MC" => MC,
"MN" => MN,
"ME" => ME,
"MS" => MS,
"MA" => MA,
"MZ" => MZ,
"MM" => MM,
"NA" => NA,
"NR" => NR,
"NP" => NP,
"NL" => NL,
"NC" => NC,
"NZ" => NZ,
"NI" => NI,
"NE" => NE,
"NG" => NG,
"NU" => NU,
"NF" => NF,
"MK" => MK,
"MP" => MP,
"NO" => NO,
"OM" => OM,
"PK" => PK,
"PW" => PW,
"PS" => PS,
"PA" => PA,
"PG" => PG,
"PY" => PY,
"PE" => PE,
"PH" => PH,
"PN" => PN,
"PL" => PL,
"PT" => PT,
"PR" => PR,
"QA" => QA,
"RE" => RE,
"RO" => RO,
"RU" => RU,
"RW" => RW,
"BL" => BL,
"SH" => SH,
"KN" => KN,
"LC" => LC,
"MF" => MF,
"PM" => PM,
"VC" => VC,
"WS" => WS,
"SM" => SM,
"ST" => ST,
"SA" => SA,
"SN" => SN,
"RS" => RS,
"SC" => SC,
"SL" => SL,
"SG" => SG,
"SX" => SX,
"SK" => SK,
"SI" => SI,
"SB" => SB,
"SO" => SO,
"ZA" => ZA,
"GS" => GS,
"SS" => SS,
"ES" => ES,
"LK" => LK,
"SD" => SD,
"SR" => SR,
"SJ" => SJ,
"SE" => SE,
"CH" => CH,
"SY" => SY,
"TW" => TW,
"TJ" => TJ,
"TZ" => TZ,
"TH" => TH,
"TL" => TL,
"TG" => TG,
"TK" => TK,
"TO" => TO,
"TT" => TT,
"TN" => TN,
"TR" => TR,
"TM" => TM,
"TC" => TC,
"TV" => TV,
"UG" => UG,
"UA" => UA,
"AE" => AE,
"GB" => GB,
"US" => US,
"UM" => UM,
"UY" => UY,
"UZ" => UZ,
"VU" => VU,
"VE" => VE,
"VN" => VN,
"VG" => VG,
"VI" => VI,
"WF" => WF,
"EH" => EH,
"YE" => YE,
"ZM" => ZM,
"ZW" => ZW,


};


///CountryCode map with  alpha3 Code key 
pub const ALPHA3_MAP: Map<&str, CountryCode> = phf_map! {


"AFG" => AF,
"ALA" => AX,
"ALB" => AL,
"DZA" => DZ,
"ASM" => AS,
"AND" => AD,
"AGO" => AO,
"AIA" => AI,
"ATA" => AQ,
"ATG" => AG,
"ARG" => AR,
"ARM" => AM,
"ABW" => AW,
"AUS" => AU,
"AUT" => AT,
"AZE" => AZ,
"BHS" => BS,
"BHR" => BH,
"BGD" => BD,
"BRB" => BB,
"BLR" => BY,
"BEL" => BE,
"BLZ" => BZ,
"BEN" => BJ,
"BMU" => BM,
"BTN" => BT,
"BOL" => BO,
"BES" => BQ,
"BIH" => BA,
"BWA" => BW,
"BVT" => BV,
"BRA" => BR,
"IOT" => IO,
"BRN" => BN,
"BGR" => BG,
"BFA" => BF,
"BDI" => BI,
"CPV" => CV,
"KHM" => KH,
"CMR" => CM,
"CAN" => CA,
"CYM" => KY,
"CAF" => CF,
"TCD" => TD,
"CHL" => CL,
"CHN" => CN,
"CXR" => CX,
"CCK" => CC,
"COL" => CO,
"COM" => KM,
"COG" => CG,
"COD" => CD,
"COK" => CK,
"CRI" => CR,
"CIV" => CI,
"HRV" => HR,
"CUB" => CU,
"CUW" => CW,
"CYP" => CY,
"CZE" => CZ,
"DNK" => DK,
"DJI" => DJ,
"DMA" => DM,
"DOM" => DO,
"ECU" => EC,
"EGY" => EG,
"SLV" => SV,
"GNQ" => GQ,
"ERI" => ER,
"EST" => EE,
"SWZ" => SZ,
"ETH" => ET,
"FLK" => FK,
"FRO" => FO,
"FJI" => FJ,
"FIN" => FI,
"FRA" => FR,
"GUF" => GF,
"PYF" => PF,
"ATF" => TF,
"GAB" => GA,
"GMB" => GM,
"GEO" => GE,
"DEU" => DE,
"GHA" => GH,
"GIB" => GI,
"GRC" => GR,
"GRL" => GL,
"GRD" => GD,
"GLP" => GP,
"GUM" => GU,
"GTM" => GT,
"GGY" => GG,
"GIN" => GN,
"GNB" => GW,
"GUY" => GY,
"HTI" => HT,
"HMD" => HM,
"VAT" => VA,
"HND" => HN,
"HKG" => HK,
"HUN" => HU,
"ISL" => IS,
"IND" => IN,
"IDN" => ID,
"IRN" => IR,
"IRQ" => IQ,
"IRL" => IE,
"IMN" => IM,
"ISR" => IL,
"ITA" => IT,
"JAM" => JM,
"JPN" => JP,
"JEY" => JE,
"JOR" => JO,
"KAZ" => KZ,
"KEN" => KE,
"KIR" => KI,
"PRK" => KP,
"KOR" => KR,
"KWT" => KW,
"KGZ" => KG,
"LAO" => LA,
"LVA" => LV,
"LBN" => LB,
"LSO" => LS,
"LBR" => LR,
"LBY" => LY,
"LIE" => LI,
"LTU" => LT,
"LUX" => LU,
"MAC" => MO,
"MDG" => MG,
"MWI" => MW,
"MYS" => MY,
"MDV" => MV,
"MLI" => ML,
"MLT" => MT,
"MHL" => MH,
"MTQ" => MQ,
"MRT" => MR,
"MUS" => MU,
"MYT" => YT,
"MEX" => MX,
"FSM" => FM,
"MDA" => MD,
"MCO" => MC,
"MNG" => MN,
"MNE" => ME,
"MSR" => MS,
"MAR" => MA,
"MOZ" => MZ,
"MMR" => MM,
"NAM" => NA,
"NRU" => NR,
"NPL" => NP,
"NLD" => NL,
"NCL" => NC,
"NZL" => NZ,
"NIC" => NI,
"NER" => NE,
"NGA" => NG,
"NIU" => NU,
"NFK" => NF,
"MKD" => MK,
"MNP" => MP,
"NOR" => NO,
"OMN" => OM,
"PAK" => PK,
"PLW" => PW,
"PSE" => PS,
"PAN" => PA,
"PNG" => PG,
"PRY" => PY,
"PER" => PE,
"PHL" => PH,
"PCN" => PN,
"POL" => PL,
"PRT" => PT,
"PRI" => PR,
"QAT" => QA,
"REU" => RE,
"ROU" => RO,
"RUS" => RU,
"RWA" => RW,
"BLM" => BL,
"SHN" => SH,
"KNA" => KN,
"LCA" => LC,
"MAF" => MF,
"SPM" => PM,
"VCT" => VC,
"WSM" => WS,
"SMR" => SM,
"STP" => ST,
"SAU" => SA,
"SEN" => SN,
"SRB" => RS,
"SYC" => SC,
"SLE" => SL,
"SGP" => SG,
"SXM" => SX,
"SVK" => SK,
"SVN" => SI,
"SLB" => SB,
"SOM" => SO,
"ZAF" => ZA,
"SGS" => GS,
"SSD" => SS,
"ESP" => ES,
"LKA" => LK,
"SDN" => SD,
"SUR" => SR,
"SJM" => SJ,
"SWE" => SE,
"CHE" => CH,
"SYR" => SY,
"TWN" => TW,
"TJK" => TJ,
"TZA" => TZ,
"THA" => TH,
"TLS" => TL,
"TGO" => TG,
"TKL" => TK,
"TON" => TO,
"TTO" => TT,
"TUN" => TN,
"TUR" => TR,
"TKM" => TM,
"TCA" => TC,
"TUV" => TV,
"UGA" => UG,
"UKR" => UA,
"ARE" => AE,
"GBR" => GB,
"USA" => US,
"UMI" => UM,
"URY" => UY,
"UZB" => UZ,
"VUT" => VU,
"VEN" => VE,
"VNM" => VN,
"VGB" => VG,
"VIR" => VI,
"WLF" => WF,
"ESH" => EH,
"YEM" => YE,
"ZMB" => ZM,
"ZWE" => ZW,


};


///CountryCode map with  3 len numeric str Code key 
pub const NUMERIC_MAP: Map<&str, CountryCode> = phf_map! {


"004" => AF,
"248" => AX,
"008" => AL,
"012" => DZ,
"016" => AS,
"020" => AD,
"024" => AO,
"660" => AI,
"010" => AQ,
"028" => AG,
"032" => AR,
"051" => AM,
"533" => AW,
"036" => AU,
"040" => AT,
"031" => AZ,
"044" => BS,
"048" => BH,
"050" => BD,
"052" => BB,
"112" => BY,
"056" => BE,
"084" => BZ,
"204" => BJ,
"060" => BM,
"064" => BT,
"068" => BO,
"535" => BQ,
"070" => BA,
"072" => BW,
"074" => BV,
"076" => BR,
"086" => IO,
"096" => BN,
"100" => BG,
"854" => BF,
"108" => BI,
"132" => CV,
"116" => KH,
"120" => CM,
"124" => CA,
"136" => KY,
"140" => CF,
"148" => TD,
"152" => CL,
"156" => CN,
"162" => CX,
"166" => CC,
"170" => CO,
"174" => KM,
"178" => CG,
"180" => CD,
"184" => CK,
"188" => CR,
"384" => CI,
"191" => HR,
"192" => CU,
"531" => CW,
"196" => CY,
"203" => CZ,
"208" => DK,
"262" => DJ,
"212" => DM,
"214" => DO,
"218" => EC,
"818" => EG,
"222" => SV,
"226" => GQ,
"232" => ER,
"233" => EE,
"748" => SZ,
"231" => ET,
"238" => FK,
"234" => FO,
"242" => FJ,
"246" => FI,
"250" => FR,
"254" => GF,
"258" => PF,
"260" => TF,
"266" => GA,
"270" => GM,
"268" => GE,
"276" => DE,
"288" => GH,
"292" => GI,
"300" => GR,
"304" => GL,
"308" => GD,
"312" => GP,
"316" => GU,
"320" => GT,
"831" => GG,
"324" => GN,
"624" => GW,
"328" => GY,
"332" => HT,
"334" => HM,
"336" => VA,
"340" => HN,
"344" => HK,
"348" => HU,
"352" => IS,
"356" => IN,
"360" => ID,
"364" => IR,
"368" => IQ,
"372" => IE,
"833" => IM,
"376" => IL,
"380" => IT,
"388" => JM,
"392" => JP,
"832" => JE,
"400" => JO,
"398" => KZ,
"404" => KE,
"296" => KI,
"408" => KP,
"410" => KR,
"414" => KW,
"417" => KG,
"418" => LA,
"428" => LV,
"422" => LB,
"426" => LS,
"430" => LR,
"434" => LY,
"438" => LI,
"440" => LT,
"442" => LU,
"446" => MO,
"450" => MG,
"454" => MW,
"458" => MY,
"462" => MV,
"466" => ML,
"470" => MT,
"584" => MH,
"474" => MQ,
"478" => MR,
"480" => MU,
"175" => YT,
"484" => MX,
"583" => FM,
"498" => MD,
"492" => MC,
"496" => MN,
"499" => ME,
"500" => MS,
"504" => MA,
"508" => MZ,
"104" => MM,
"516" => NA,
"520" => NR,
"524" => NP,
"528" => NL,
"540" => NC,
"554" => NZ,
"558" => NI,
"562" => NE,
"566" => NG,
"570" => NU,
"574" => NF,
"807" => MK,
"580" => MP,
"578" => NO,
"512" => OM,
"586" => PK,
"585" => PW,
"275" => PS,
"591" => PA,
"598" => PG,
"600" => PY,
"604" => PE,
"608" => PH,
"612" => PN,
"616" => PL,
"620" => PT,
"630" => PR,
"634" => QA,
"638" => RE,
"642" => RO,
"643" => RU,
"646" => RW,
"652" => BL,
"654" => SH,
"659" => KN,
"662" => LC,
"663" => MF,
"666" => PM,
"670" => VC,
"882" => WS,
"674" => SM,
"678" => ST,
"682" => SA,
"686" => SN,
"688" => RS,
"690" => SC,
"694" => SL,
"702" => SG,
"534" => SX,
"703" => SK,
"705" => SI,
"090" => SB,
"706" => SO,
"710" => ZA,
"239" => GS,
"728" => SS,
"724" => ES,
"144" => LK,
"729" => SD,
"740" => SR,
"744" => SJ,
"752" => SE,
"756" => CH,
"760" => SY,
"158" => TW,
"762" => TJ,
"834" => TZ,
"764" => TH,
"626" => TL,
"768" => TG,
"772" => TK,
"776" => TO,
"780" => TT,
"788" => TN,
"792" => TR,
"795" => TM,
"796" => TC,
"798" => TV,
"800" => UG,
"804" => UA,
"784" => AE,
"826" => GB,
"840" => US,
"581" => UM,
"858" => UY,
"860" => UZ,
"548" => VU,
"862" => VE,
"704" => VN,
"092" => VG,
"850" => VI,
"876" => WF,
"732" => EH,
"887" => YE,
"894" => ZM,
"716" => ZW,


};


///ALL the names of Countrys
pub const ALL_NAME: & [&str] = &[


"Afghanistan[b]",
"Åland Islands",
"Albania",
"Algeria",
"American Samoa",
"Andorra",
"Angola",
"Anguilla",
"Antarctica",
"Antigua and Barbuda",
"Argentina",
"Armenia",
"Aruba",
"Australia",
"Austria",
"Azerbaijan",
"Bahamas",
"Bahrain",
"Bangladesh",
"Barbados",
"Belarus",
"Belgium",
"Belize",
"Benin",
"Bermuda",
"Bhutan",
"Bolivia (Plurinational State of)",
"Bonaire, Sint Eustatius and Saba[c]",
"Bosnia and Herzegovina",
"Botswana",
"Bouvet Island",
"Brazil",
"British Indian Ocean Territory",
"Brunei Darussalam",
"Bulgaria",
"Burkina Faso",
"Burundi",
"Cabo Verde",
"Cambodia",
"Cameroon",
"Canada",
"Cayman Islands",
"Central African Republic",
"Chad",
"Chile",
"China[b]",
"Christmas Island",
"Cocos (Keeling) Islands",
"Colombia",
"Comoros",
"Congo",
"Congo, Democratic Republic of the",
"Cook Islands",
"Costa Rica",
"Côte d'Ivoire",
"Croatia",
"Cuba",
"Curaçao",
"Cyprus[b]",
"Czechia",
"Denmark",
"Djibouti",
"Dominica",
"Dominican Republic",
"Ecuador",
"Egypt",
"El Salvador",
"Equatorial Guinea",
"Eritrea",
"Estonia",
"Eswatini",
"Ethiopia",
"Falkland Islands (Malvinas)[b]",
"Faroe Islands",
"Fiji",
"Finland",
"France",
"French Guiana",
"French Polynesia",
"French Southern Territories",
"Gabon",
"Gambia",
"Georgia",
"Germany",
"Ghana",
"Gibraltar",
"Greece",
"Greenland",
"Grenada",
"Guadeloupe",
"Guam",
"Guatemala",
"Guernsey",
"Guinea",
"Guinea-Bissau",
"Guyana",
"Haiti",
"Heard Island and McDonald Islands",
"Holy See",
"Honduras",
"Hong Kong",
"Hungary",
"Iceland",
"India",
"Indonesia",
"Iran (Islamic Republic of)",
"Iraq",
"Ireland",
"Isle of Man",
"Israel",
"Italy",
"Jamaica",
"Japan",
"Jersey",
"Jordan",
"Kazakhstan",
"Kenya",
"Kiribati",
"Korea (Democratic People's Republic of)",
"Korea, Republic of",
"Kuwait",
"Kyrgyzstan",
"Lao People's Democratic Republic",
"Latvia",
"Lebanon",
"Lesotho",
"Liberia",
"Libya",
"Liechtenstein",
"Lithuania",
"Luxembourg",
"Macao",
"Madagascar",
"Malawi",
"Malaysia",
"Maldives",
"Mali",
"Malta",
"Marshall Islands",
"Martinique",
"Mauritania",
"Mauritius",
"Mayotte",
"Mexico",
"Micronesia (Federated States of)",
"Moldova, Republic of",
"Monaco",
"Mongolia",
"Montenegro",
"Montserrat",
"Morocco",
"Mozambique",
"Myanmar",
"Namibia",
"Nauru",
"Nepal",
"Netherlands",
"New Caledonia",
"New Zealand",
"Nicaragua",
"Niger",
"Nigeria",
"Niue",
"Norfolk Island",
"North Macedonia",
"Northern Mariana Islands",
"Norway",
"Oman",
"Pakistan",
"Palau",
"Palestine, State of[b]",
"Panama",
"Papua New Guinea",
"Paraguay",
"Peru",
"Philippines",
"Pitcairn",
"Poland",
"Portugal",
"Puerto Rico",
"Qatar",
"Réunion",
"Romania",
"Russian Federation",
"Rwanda",
"Saint Barthélemy",
"Saint Helena, Ascension and Tristan da Cunha[d]",
"Saint Kitts and Nevis",
"Saint Lucia",
"Saint Martin (French part)",
"Saint Pierre and Miquelon",
"Saint Vincent and the Grenadines",
"Samoa",
"San Marino",
"Sao Tome and Principe",
"Saudi Arabia",
"Senegal",
"Serbia",
"Seychelles",
"Sierra Leone",
"Singapore",
"Sint Maarten (Dutch part)",
"Slovakia",
"Slovenia",
"Solomon Islands",
"Somalia",
"South Africa",
"South Georgia and the South Sandwich Islands",
"South Sudan",
"Spain",
"Sri Lanka",
"Sudan",
"Suriname",
"Svalbard and Jan Mayen[e]",
"Sweden",
"Switzerland",
"Syrian Arab Republic",
"Taiwan, Province of China[b]",
"Tajikistan",
"Tanzania, United Republic of",
"Thailand",
"Timor-Leste",
"Togo",
"Tokelau",
"Tonga",
"Trinidad and Tobago",
"Tunisia",
"Turkey",
"Turkmenistan",
"Turks and Caicos Islands",
"Tuvalu",
"Uganda",
"Ukraine",
"United Arab Emirates",
"United Kingdom of Great Britain and Northern Ireland",
"United States of America",
"United States Minor Outlying Islands[f]",
"Uruguay",
"Uzbekistan",
"Vanuatu",
"Venezuela (Bolivarian Republic of)",
"Viet Nam",
"Virgin Islands (British)",
"Virgin Islands (U.S.)",
"Wallis and Futuna",
"Western Sahara[b]",
"Yemen",
"Zambia",
"Zimbabwe",


];


///ALL the alpha2 codes of Countrys
pub const ALL_ALPHA2: & [&str] = &[


"AF",
"AX",
"AL",
"DZ",
"AS",
"AD",
"AO",
"AI",
"AQ",
"AG",
"AR",
"AM",
"AW",
"AU",
"AT",
"AZ",
"BS",
"BH",
"BD",
"BB",
"BY",
"BE",
"BZ",
"BJ",
"BM",
"BT",
"BO",
"BQ",
"BA",
"BW",
"BV",
"BR",
"IO",
"BN",
"BG",
"BF",
"BI",
"CV",
"KH",
"CM",
"CA",
"KY",
"CF",
"TD",
"CL",
"CN",
"CX",
"CC",
"CO",
"KM",
"CG",
"CD",
"CK",
"CR",
"CI",
"HR",
"CU",
"CW",
"CY",
"CZ",
"DK",
"DJ",
"DM",
"DO",
"EC",
"EG",
"SV",
"GQ",
"ER",
"EE",
"SZ",
"ET",
"FK",
"FO",
"FJ",
"FI",
"FR",
"GF",
"PF",
"TF",
"GA",
"GM",
"GE",
"DE",
"GH",
"GI",
"GR",
"GL",
"GD",
"GP",
"GU",
"GT",
"GG",
"GN",
"GW",
"GY",
"HT",
"HM",
"VA",
"HN",
"HK",
"HU",
"IS",
"IN",
"ID",
"IR",
"IQ",
"IE",
"IM",
"IL",
"IT",
"JM",
"JP",
"JE",
"JO",
"KZ",
"KE",
"KI",
"KP",
"KR",
"KW",
"KG",
"LA",
"LV",
"LB",
"LS",
"LR",
"LY",
"LI",
"LT",
"LU",
"MO",
"MG",
"MW",
"MY",
"MV",
"ML",
"MT",
"MH",
"MQ",
"MR",
"MU",
"YT",
"MX",
"FM",
"MD",
"MC",
"MN",
"ME",
"MS",
"MA",
"MZ",
"MM",
"NA",
"NR",
"NP",
"NL",
"NC",
"NZ",
"NI",
"NE",
"NG",
"NU",
"NF",
"MK",
"MP",
"NO",
"OM",
"PK",
"PW",
"PS",
"PA",
"PG",
"PY",
"PE",
"PH",
"PN",
"PL",
"PT",
"PR",
"QA",
"RE",
"RO",
"RU",
"RW",
"BL",
"SH",
"KN",
"LC",
"MF",
"PM",
"VC",
"WS",
"SM",
"ST",
"SA",
"SN",
"RS",
"SC",
"SL",
"SG",
"SX",
"SK",
"SI",
"SB",
"SO",
"ZA",
"GS",
"SS",
"ES",
"LK",
"SD",
"SR",
"SJ",
"SE",
"CH",
"SY",
"TW",
"TJ",
"TZ",
"TH",
"TL",
"TG",
"TK",
"TO",
"TT",
"TN",
"TR",
"TM",
"TC",
"TV",
"UG",
"UA",
"AE",
"GB",
"US",
"UM",
"UY",
"UZ",
"VU",
"VE",
"VN",
"VG",
"VI",
"WF",
"EH",
"YE",
"ZM",
"ZW",


];


///ALL the alpha3 codes of Countrys
pub const ALL_ALPHA3: & [&str] = &[


"AFG",
"ALA",
"ALB",
"DZA",
"ASM",
"AND",
"AGO",
"AIA",
"ATA",
"ATG",
"ARG",
"ARM",
"ABW",
"AUS",
"AUT",
"AZE",
"BHS",
"BHR",
"BGD",
"BRB",
"BLR",
"BEL",
"BLZ",
"BEN",
"BMU",
"BTN",
"BOL",
"BES",
"BIH",
"BWA",
"BVT",
"BRA",
"IOT",
"BRN",
"BGR",
"BFA",
"BDI",
"CPV",
"KHM",
"CMR",
"CAN",
"CYM",
"CAF",
"TCD",
"CHL",
"CHN",
"CXR",
"CCK",
"COL",
"COM",
"COG",
"COD",
"COK",
"CRI",
"CIV",
"HRV",
"CUB",
"CUW",
"CYP",
"CZE",
"DNK",
"DJI",
"DMA",
"DOM",
"ECU",
"EGY",
"SLV",
"GNQ",
"ERI",
"EST",
"SWZ",
"ETH",
"FLK",
"FRO",
"FJI",
"FIN",
"FRA",
"GUF",
"PYF",
"ATF",
"GAB",
"GMB",
"GEO",
"DEU",
"GHA",
"GIB",
"GRC",
"GRL",
"GRD",
"GLP",
"GUM",
"GTM",
"GGY",
"GIN",
"GNB",
"GUY",
"HTI",
"HMD",
"VAT",
"HND",
"HKG",
"HUN",
"ISL",
"IND",
"IDN",
"IRN",
"IRQ",
"IRL",
"IMN",
"ISR",
"ITA",
"JAM",
"JPN",
"JEY",
"JOR",
"KAZ",
"KEN",
"KIR",
"PRK",
"KOR",
"KWT",
"KGZ",
"LAO",
"LVA",
"LBN",
"LSO",
"LBR",
"LBY",
"LIE",
"LTU",
"LUX",
"MAC",
"MDG",
"MWI",
"MYS",
"MDV",
"MLI",
"MLT",
"MHL",
"MTQ",
"MRT",
"MUS",
"MYT",
"MEX",
"FSM",
"MDA",
"MCO",
"MNG",
"MNE",
"MSR",
"MAR",
"MOZ",
"MMR",
"NAM",
"NRU",
"NPL",
"NLD",
"NCL",
"NZL",
"NIC",
"NER",
"NGA",
"NIU",
"NFK",
"MKD",
"MNP",
"NOR",
"OMN",
"PAK",
"PLW",
"PSE",
"PAN",
"PNG",
"PRY",
"PER",
"PHL",
"PCN",
"POL",
"PRT",
"PRI",
"QAT",
"REU",
"ROU",
"RUS",
"RWA",
"BLM",
"SHN",
"KNA",
"LCA",
"MAF",
"SPM",
"VCT",
"WSM",
"SMR",
"STP",
"SAU",
"SEN",
"SRB",
"SYC",
"SLE",
"SGP",
"SXM",
"SVK",
"SVN",
"SLB",
"SOM",
"ZAF",
"SGS",
"SSD",
"ESP",
"LKA",
"SDN",
"SUR",
"SJM",
"SWE",
"CHE",
"SYR",
"TWN",
"TJK",
"TZA",
"THA",
"TLS",
"TGO",
"TKL",
"TON",
"TTO",
"TUN",
"TUR",
"TKM",
"TCA",
"TUV",
"UGA",
"UKR",
"ARE",
"GBR",
"USA",
"UMI",
"URY",
"UZB",
"VUT",
"VEN",
"VNM",
"VGB",
"VIR",
"WLF",
"ESH",
"YEM",
"ZMB",
"ZWE",


];


///ALL the 3 length numeric str codes of Countrys
pub const ALL_NUMERIC_STR: & [&str] = &[


"004",
"248",
"008",
"012",
"016",
"020",
"024",
"660",
"010",
"028",
"032",
"051",
"533",
"036",
"040",
"031",
"044",
"048",
"050",
"052",
"112",
"056",
"084",
"204",
"060",
"064",
"068",
"535",
"070",
"072",
"074",
"076",
"086",
"096",
"100",
"854",
"108",
"132",
"116",
"120",
"124",
"136",
"140",
"148",
"152",
"156",
"162",
"166",
"170",
"174",
"178",
"180",
"184",
"188",
"384",
"191",
"192",
"531",
"196",
"203",
"208",
"262",
"212",
"214",
"218",
"818",
"222",
"226",
"232",
"233",
"748",
"231",
"238",
"234",
"242",
"246",
"250",
"254",
"258",
"260",
"266",
"270",
"268",
"276",
"288",
"292",
"300",
"304",
"308",
"312",
"316",
"320",
"831",
"324",
"624",
"328",
"332",
"334",
"336",
"340",
"344",
"348",
"352",
"356",
"360",
"364",
"368",
"372",
"833",
"376",
"380",
"388",
"392",
"832",
"400",
"398",
"404",
"296",
"408",
"410",
"414",
"417",
"418",
"428",
"422",
"426",
"430",
"434",
"438",
"440",
"442",
"446",
"450",
"454",
"458",
"462",
"466",
"470",
"584",
"474",
"478",
"480",
"175",
"484",
"583",
"498",
"492",
"496",
"499",
"500",
"504",
"508",
"104",
"516",
"520",
"524",
"528",
"540",
"554",
"558",
"562",
"566",
"570",
"574",
"807",
"580",
"578",
"512",
"586",
"585",
"275",
"591",
"598",
"600",
"604",
"608",
"612",
"616",
"620",
"630",
"634",
"638",
"642",
"643",
"646",
"652",
"654",
"659",
"662",
"663",
"666",
"670",
"882",
"674",
"678",
"682",
"686",
"688",
"690",
"694",
"702",
"534",
"703",
"705",
"090",
"706",
"710",
"239",
"728",
"724",
"144",
"729",
"740",
"744",
"752",
"756",
"760",
"158",
"762",
"834",
"764",
"626",
"768",
"772",
"776",
"780",
"788",
"792",
"795",
"796",
"798",
"800",
"804",
"784",
"826",
"840",
"581",
"858",
"860",
"548",
"862",
"704",
"092",
"850",
"876",
"732",
"887",
"894",
"716",


];


///ALL the  numeric  codes of Countrys
pub const ALL_NUMERIC: & [i32] = &[


4,
248,
8,
12,
16,
20,
24,
660,
10,
28,
32,
51,
533,
36,
40,
31,
44,
48,
50,
52,
112,
56,
84,
204,
60,
64,
68,
535,
70,
72,
74,
76,
86,
96,
100,
854,
108,
132,
116,
120,
124,
136,
140,
148,
152,
156,
162,
166,
170,
174,
178,
180,
184,
188,
384,
191,
192,
531,
196,
203,
208,
262,
212,
214,
218,
818,
222,
226,
232,
233,
748,
231,
238,
234,
242,
246,
250,
254,
258,
260,
266,
270,
268,
276,
288,
292,
300,
304,
308,
312,
316,
320,
831,
324,
624,
328,
332,
334,
336,
340,
344,
348,
352,
356,
360,
364,
368,
372,
833,
376,
380,
388,
392,
832,
400,
398,
404,
296,
408,
410,
414,
417,
418,
428,
422,
426,
430,
434,
438,
440,
442,
446,
450,
454,
458,
462,
466,
470,
584,
474,
478,
480,
175,
484,
583,
498,
492,
496,
499,
500,
504,
508,
104,
516,
520,
524,
528,
540,
554,
558,
562,
566,
570,
574,
807,
580,
578,
512,
586,
585,
275,
591,
598,
600,
604,
608,
612,
616,
620,
630,
634,
638,
642,
643,
646,
652,
654,
659,
662,
663,
666,
670,
882,
674,
678,
682,
686,
688,
690,
694,
702,
534,
703,
705,
90,
706,
710,
239,
728,
724,
144,
729,
740,
744,
752,
756,
760,
158,
762,
834,
764,
626,
768,
772,
776,
780,
788,
792,
795,
796,
798,
800,
804,
784,
826,
840,
581,
858,
860,
548,
862,
704,
92,
850,
876,
732,
887,
894,
716,


];


///ALL the Countrys struct
pub const ALL: & [CountryCode] = &[


AF,
AX,
AL,
DZ,
AS,
AD,
AO,
AI,
AQ,
AG,
AR,
AM,
AW,
AU,
AT,
AZ,
BS,
BH,
BD,
BB,
BY,
BE,
BZ,
BJ,
BM,
BT,
BO,
BQ,
BA,
BW,
BV,
BR,
IO,
BN,
BG,
BF,
BI,
CV,
KH,
CM,
CA,
KY,
CF,
TD,
CL,
CN,
CX,
CC,
CO,
KM,
CG,
CD,
CK,
CR,
CI,
HR,
CU,
CW,
CY,
CZ,
DK,
DJ,
DM,
DO,
EC,
EG,
SV,
GQ,
ER,
EE,
SZ,
ET,
FK,
FO,
FJ,
FI,
FR,
GF,
PF,
TF,
GA,
GM,
GE,
DE,
GH,
GI,
GR,
GL,
GD,
GP,
GU,
GT,
GG,
GN,
GW,
GY,
HT,
HM,
VA,
HN,
HK,
HU,
IS,
IN,
ID,
IR,
IQ,
IE,
IM,
IL,
IT,
JM,
JP,
JE,
JO,
KZ,
KE,
KI,
KP,
KR,
KW,
KG,
LA,
LV,
LB,
LS,
LR,
LY,
LI,
LT,
LU,
MO,
MG,
MW,
MY,
MV,
ML,
MT,
MH,
MQ,
MR,
MU,
YT,
MX,
FM,
MD,
MC,
MN,
ME,
MS,
MA,
MZ,
MM,
NA,
NR,
NP,
NL,
NC,
NZ,
NI,
NE,
NG,
NU,
NF,
MK,
MP,
NO,
OM,
PK,
PW,
PS,
PA,
PG,
PY,
PE,
PH,
PN,
PL,
PT,
PR,
QA,
RE,
RO,
RU,
RW,
BL,
SH,
KN,
LC,
MF,
PM,
VC,
WS,
SM,
ST,
SA,
SN,
RS,
SC,
SL,
SG,
SX,
SK,
SI,
SB,
SO,
ZA,
GS,
SS,
ES,
LK,
SD,
SR,
SJ,
SE,
CH,
SY,
TW,
TJ,
TZ,
TH,
TL,
TG,
TK,
TO,
TT,
TN,
TR,
TM,
TC,
TV,
UG,
UA,
AE,
GB,
US,
UM,
UY,
UZ,
VU,
VE,
VN,
VG,
VI,
WF,
EH,
YE,
ZM,
ZW,


];

