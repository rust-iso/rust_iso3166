#[cfg(feature = "serde")]
mod tests {
    use rust_iso3166::iso3166_2::from_code as from_code_2;
    use rust_iso3166::iso3166_3::from_code as from_code_3;
    use rust_iso3166::{from_alpha2, from_alpha3};

    #[test]
    fn test_country_code_serde() {
        let au = from_alpha2("AU").unwrap();
        let au_json = serde_json::to_string(&au).unwrap();
        assert_eq!(au_json, "\"AU\"");

        let us = from_alpha3("USA").unwrap();
        let us_json = serde_json::to_string(&us).unwrap();
        assert_eq!(us_json, "\"US\"");

        let au_deserialized: rust_iso3166::CountryCode = serde_json::from_str("\"AU\"").unwrap();
        assert_eq!(au_deserialized.alpha2, "AU");
        assert_eq!(au_deserialized.alpha3, "AUS");

        let us_deserialized: rust_iso3166::CountryCode = serde_json::from_str("\"USA\"").unwrap();
        assert_eq!(us_deserialized.alpha2, "US");
        assert_eq!(us_deserialized.alpha3, "USA");
    }

    #[test]
    fn test_subdivision_serde() {
        let gb_edh = from_code_2("GB-EDH").unwrap();
        let gb_edh_json = serde_json::to_string(&gb_edh).unwrap();
        assert_eq!(gb_edh_json, "\"GB-EDH\"");

        let gb_edh_deserialized: rust_iso3166::iso3166_2::Subdivision =
            serde_json::from_str("\"GB-EDH\"").unwrap();
        assert_eq!(gb_edh_deserialized.code, "GB-EDH");
        assert_eq!(gb_edh_deserialized.name, "Edinburgh, City of");
    }

    #[test]
    fn test_country_code3_serde() {
        let pzpa = from_code_3("PZPA").unwrap();
        let pzpa_json = serde_json::to_string(&pzpa).unwrap();
        assert_eq!(pzpa_json, "\"PZPA\"");

        let pzpa_deserialized: rust_iso3166::iso3166_3::CountryCode3 =
            serde_json::from_str("\"PZPA\"").unwrap();
        assert_eq!(pzpa_deserialized.code, "PZPA");
        assert_eq!(pzpa_deserialized.name, "Panama Canal Zone");
    }

    #[test]
    fn test_country_code_case_insensitive() {
        let au_lower: rust_iso3166::CountryCode = serde_json::from_str("\"au\"").unwrap();
        assert_eq!(au_lower.alpha2, "AU");
        assert_eq!(au_lower.alpha3, "AUS");

        let us_mixed: rust_iso3166::CountryCode = serde_json::from_str("\"uSa\"").unwrap();
        assert_eq!(us_mixed.alpha2, "US");
        assert_eq!(us_mixed.alpha3, "USA");

        let gb_upper: rust_iso3166::CountryCode = serde_json::from_str("\"GB\"").unwrap();
        assert_eq!(gb_upper.alpha2, "GB");
        assert_eq!(gb_upper.alpha3, "GBR");
    }

    #[test]
    fn test_subdivision_case_insensitive() {
        let gb_edh_lower: rust_iso3166::iso3166_2::Subdivision =
            serde_json::from_str("\"gb-edh\"").unwrap();
        assert_eq!(gb_edh_lower.code, "GB-EDH");
        assert_eq!(gb_edh_lower.name, "Edinburgh, City of");

        let gb_edh_mixed: rust_iso3166::iso3166_2::Subdivision =
            serde_json::from_str("\"Gb-Edh\"").unwrap();
        assert_eq!(gb_edh_mixed.code, "GB-EDH");
        assert_eq!(gb_edh_mixed.name, "Edinburgh, City of");
    }

    #[test]
    fn test_country_code3_case_insensitive() {
        let pzpa_lower: rust_iso3166::iso3166_3::CountryCode3 =
            serde_json::from_str("\"pzpa\"").unwrap();
        assert_eq!(pzpa_lower.code, "PZPA");
        assert_eq!(pzpa_lower.name, "Panama Canal Zone");

        let pzpa_mixed: rust_iso3166::iso3166_3::CountryCode3 =
            serde_json::from_str("\"PzPa\"").unwrap();
        assert_eq!(pzpa_mixed.code, "PZPA");
        assert_eq!(pzpa_mixed.name, "Panama Canal Zone");
    }

    #[test]
    fn test_invalid_country_code() {
        let result: Result<rust_iso3166::CountryCode, _> = serde_json::from_str("\"XX\"");
        assert!(result.is_err());

        let result: Result<rust_iso3166::CountryCode, _> = serde_json::from_str("\"XXX\"");
        assert!(result.is_err());

        let result: Result<rust_iso3166::CountryCode, _> = serde_json::from_str("\"\"");
        assert!(result.is_err());

        let result: Result<rust_iso3166::CountryCode, _> = serde_json::from_str("\"1234\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_subdivision() {
        let result: Result<rust_iso3166::iso3166_2::Subdivision, _> =
            serde_json::from_str("\"XX-XX\"");
        assert!(result.is_err());

        let result: Result<rust_iso3166::iso3166_2::Subdivision, _> =
            serde_json::from_str("\"GB-XXX\"");
        assert!(result.is_err());

        let result: Result<rust_iso3166::iso3166_2::Subdivision, _> = serde_json::from_str("\"\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_country_code3() {
        let result: Result<rust_iso3166::iso3166_3::CountryCode3, _> =
            serde_json::from_str("\"XXXX\"");
        assert!(result.is_err());

        let result: Result<rust_iso3166::iso3166_3::CountryCode3, _> =
            serde_json::from_str("\"XXX\"");
        assert!(result.is_err());

        let result: Result<rust_iso3166::iso3166_3::CountryCode3, _> = serde_json::from_str("\"\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_boundary_cases() {
        let result: Result<rust_iso3166::CountryCode, _> = serde_json::from_str("\"036\"");
        assert!(result.is_err()); // Direct deserialization of numeric codes is not currently supported

        let result: Result<rust_iso3166::CountryCode, _> = serde_json::from_str("\"A@\"");
        assert!(result.is_err());

        let result: Result<rust_iso3166::CountryCode, _> = serde_json::from_str("\"🫡❤️\"");
        assert!(result.is_err());

        let result: Result<rust_iso3166::CountryCode, _> =
            serde_json::from_str("\"ABCDEFGHIJKLMNOPQRSTUVWXYZ\"");
        assert!(result.is_err());
    }
}
