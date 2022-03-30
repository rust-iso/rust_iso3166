# league/iso3166

A rust crate providing ISO 3166-1 support.

## What is ISO 3166-1

> ISO 3166-1 is part of the ISO 3166 standard published by the International Organization for Standardization (ISO), and defines codes for the names of countries, dependent territories, and special areas of geographical interest. The official name of the standard is Codes for the representation of names of countries and their subdivisions – Part 1: Country codes. It defines three sets of country codes:
> * ISO 3166-1 alpha-2 – two-letter country codes which are the most widely used of the three, and used most prominently for the Internet's country code top-level domains (with a few exceptions).
> * ISO 3166-1 alpha-3 – three-letter country codes which allow a better visual association between the codes and the country names than the alpha-2 codes.
> * ISO 3166-1 numeric – three-digit country codes which are identical to those developed and maintained by the United Nations Statistics Division, with the advantage of script (writing system) independence, and hence useful for people or systems using non-Latin scripts.
>
> *-- [Wikipedia](http://en.wikipedia.org/wiki/ISO_3166-1)*

## Installing

``` sh
[dependencies]
rust_iso3166 = "0.1.1"
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

```

Data sample:

``` rust

CountryCode { 
    name: "Australia",
    alpha2: "AU", 
    alpha3: "AUS", 
    numeric: 36 
}
```

## Contributing

Feel free to submit a pull request or create an issue.

## License

league/iso3166 is licensed under the MIT license.

## Source(s)

* [ISO 3166-1](http://en.wikipedia.org/wiki/ISO_3166-1) by [Wikipedia](http://www.wikipedia.org) licensed under [CC BY-SA 3.0 Unported License](http://en.wikipedia.org/wiki/Wikipedia:Text_of_Creative_Commons_Attribution-ShareAlike_3.0_Unported_License)
* [www.iso.org](http://www.iso.org)