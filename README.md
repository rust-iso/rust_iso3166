# league/iso3166

A rust crate providing ISO 3166-1 support.

## What is ISO 3166-1

> ISO 3166-1 is part of the ISO 3166 standard published by the International Organization for Standardization (ISO), and defines codes for the names of countries, dependent territories, and special areas of geographical interest. The official name of the standard is Codes for the representation of names of countries and their subdivisions – Part 1: Country codes. It defines three sets of country codes:
> * ISO 3166-1 alpha-2 – two-letter country codes which are the most widely used of the three, and used most prominently for the Internet's country code top-level domains (with a few exceptions).
> * ISO 3166-1 alpha-3 – three-letter country codes which allow a better visual association between the codes and the country names than the alpha-2 codes.
> * ISO 3166-1 numeric – three-digit country codes which are identical to those developed and maintained by the United Nations Statistics Division, with the advantage of script (writing system) independence, and hence useful for people or systems using non-Latin scripts.
>
> *-- [Wikipedia](http://en.wikipedia.org/wiki/ISO_3166-1)*

## What is ISO 3166-2

> ISO 3166-2 is part of the ISO 3166 standard published by the International Organization for Standardization (ISO), and defines codes for identifying the principal subdivisions (e.g., provinces or states) of all countries coded in ISO 3166-1. The official name of the standard is Codes for the representation of names of countries and their subdivisions – Part 2: Country subdivision code. It was first published in 1998.

> * The purpose of ISO 3166-2 is to establish an international standard of short and unique alphanumeric codes to represent the relevant administrative divisions and dependent territories of all countries in a more convenient and less ambiguous form than their full names. Each complete ISO 3166-2 code consists of two parts, separated by a hyphen:[1]
> * The first part is the ISO 3166-1 alpha-2 code of the country;
> * The second part is a string of up to three alphanumeric characters, which is usually obtained from national sources and stems from coding systems already in use in the country concerned, but may also be developed by the ISO itself.
>
> *-- [Wikipedia](http://en.wikipedia.org/wiki/ISO_3166-2)*

## What is ISO 3166-3

> ISO 3166-3 is part of the ISO 3166 standard published by the International Organization for Standardization (ISO), and defines codes for country names which have been deleted from ISO 3166-1 since its first publication in 1974. The official name of the standard is Codes for the representation of names of countries and their subdivisions – Part 3: Code for formerly used names of countries.[1] It was first published in 1999.

> * Each former country name in ISO 3166-3 is assigned a four-letter alphabetic code. The first two letters are the ISO 3166-1 alpha-2 code of the former country, while the last two letters are allocated according to the following rules:[2]

> * If the country changed its name, the new ISO 3166-1 alpha-2 code is used (e.g., Burma changed its name to Myanmar, whose new alpha-2 code is MM), or the special code AA is used if its alpha-2 code was not changed (e.g., Byelorussian SSR changed its name to Belarus, which has kept the same alpha-2 code).
> * If the country merged into an existing country, the ISO 3166-1 alpha-2 code of this country is used (e.g., the German Democratic Republic merged into Germany, whose alpha-2 code is DE).
> * If the country was divided into several parts, the special code HH is used to indicate that there is no single successor country (e.g., Czechoslovakia was divided into the Czech Republic and Slovakia), with the exception of Serbia and Montenegro, for which XX is used to avoid duplicate use of the same ISO 3166-3 code, as the alpha-2 code CS had twice been deleted from ISO 3166-1, the first time due to the split of Czechoslovakia and the second time due to the split of Serbia and Montenegro.

> *-- [Wikipedia](http://en.wikipedia.org/wiki/ISO_3166-3)*

## Installing

``` sh
[dependencies]
rust_iso3166 = "0.1.2"
```

## Using

See [using](https://iso3166.thephpleague.com/using) section of the documentation.

Quick guide:

``` rust
let country = rust_iso3166::from_alpha2("AU");
let country = rust_iso3166::from_alpha3("AUS");
let country = rust_iso3166::from_numeric(036);
let country = rust_iso3166::from_numeric_str("036");

println!("{:?}", rust_iso3166::ALL);

println!("{:?}", rust_iso3166::ALL_ALPHA2);   
println!("{:?}", rust_iso3166::ALL_ALPHA3);   
println!("{:?}", rust_iso3166::ALL_NAME);   
println!("{:?}", rust_iso3166::ALL_NUMERIC);   
println!("{:?}", rust_iso3166::ALL_NUMERIC_STR);   

println!("{:?}", rust_iso3166::NUMERIC_MAP);  
println!("{:?}", rust_iso3166::ALPHA3_MAP);  
println!("{:?}", rust_iso3166::ALPHA2_MAP);  

// for ISO 3166-2
let country = rust_iso3166::from_alpha2("GB").unwrap();
let subdivisions = country.subdivisions();
assert!(subdivisions.unwrap().len() > 0);
let country = rust_iso3166::iso3166_2::from_code("GB-EDH");
assert_eq!("Edinburgh, City of", country.unwrap().name); 

// for ISO 3166-3
let sub = rust_iso3166::iso3166_3::from_code("PZPA");
assert_eq!("Panama Canal Zone", sub.unwrap().name);
```

Data sample:

``` rust
CountryCode { 
    name: "Australia",
    alpha2: "AU", 
    alpha3: "AUS", 
    numeric: 36 
}

 iso3166_2::Subdivision {
    name: "Bādghīs",
    code: "AF-BDG",
    subdivision_type: "Province",
    country_name: "Afghanistan",
    country_code: "AF",
    region_code: "AF-BDG",
}

iso3166_3::CountryCode3 {
    code: "BQAQ",
    name: "British Antarctic Territory",
    former: CountryCode { 
        name: "British Antarctic Territory",
        alpha2: "BQ", 
        alpha3: "ATB", 
        numeric: 0 
    },
    new_countries: [
        CountryCode { 
            name: "Antarctica",
            alpha2: "AQ", 
            alpha3: "ATA", 
            numeric: 010
        },    
    ],
    validity: [1974,1979],
    desc: "Merged into Antarctica",
}
```

## Contributing

Feel free to submit a pull request or create an issue.
or request to [rust-iso](https://github.com/rust-iso) 

## License

rust-iso/rust_iso3166 is licensed under the Apache-2.0 license.

## Source(s)

* [ISO 3166](https://en.wikipedia.org/wiki/ISO_3166) by [Wikipedia](http://www.wikipedia.org)
* [ISO 3166-1](https://en.wikipedia.org/wiki/ISO_3166-1) by [Wikipedia](http://www.wikipedia.org)
* [ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) by [Wikipedia](http://www.wikipedia.org)
* [ISO 3166-3](https://en.wikipedia.org/wiki/ISO_3166-3) by [Wikipedia](http://www.wikipedia.org)
* [www.iso.org](http://www.iso.org)