use serde::{Serialize, Deserialize};

/// Country is the national political entity and is typically the highest order
/// type returned by the Geocoder.
///
/// The codes match a country name or a two letter [ISO
/// 3166-1](https://en.wikipedia.org/wiki/ISO_3166-1) country code. The API
/// follows the ISO standard for defining countries, and the filtering works
/// best when using the corresponding ISO code of the country.
///
/// **Note: If you receive unexpected results with a country code, verify that
/// you are using a code which includes the countries, dependent territories,
/// and special areas of geographical interest you intend. You can find code
/// information at [Wikipedia: List of ISO 3166 country
/// codes](https://en.wikipedia.org/wiki/List_of_ISO_3166_country_codes) or the
/// [ISO Online Browsing Platform](https://www.iso.org/obp/ui/#search).**

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Country {
    Afghanistan,
    AlandIslands,
    Albania,
    Algeria,
    AmericanSamoa,
    Andorra,
    Angola,
    Anguilla,
    Antarctica,
    AntiguaAndBarbuda,
    Argentina,
    Armenia,
    Aruba,
    Australia,
    Austria,
    Azerbaijan,
    Bahamas,
    Bahrain,
    Bangladesh,
    Barbados,
    Belarus,
    Belgium,
    Belize,
    Benin,
    Bermuda,
    Bhutan,
    Bolivia,
    BonaireSintEustatiusAndSaba,
    BosniaAndHerzegovina,
    Botswana,
    BouvetIsland,
    Brazil,
    BritishIndianOceanTerritory,
    BritishVirginIslands,
    Brunei,
    Bulgaria,
    BurkinaFaso,
    Burundi,
    CaboVerde,
    Cambodia,
    Cameroon,
    Canada,
    CaymanIslands,
    CentralAfricanRepublic,
    Chad,
    Chile,
    China,
    ChristmasIsland,
    CocosKeelingIslands,
    Colombia,
    Comoros,
    CongoBrazzaville,
    CongoKinshasa,
    CookIslands,
    CostaRica,
    CotedIvoire,
    Croatia,
    Cuba,
    Curacao,
    Cyprus,
    Czechia,
    Denmark,
    Djibouti,
    Dominica,
    DominicanRepublic,
    Ecuador,
    Egypt,
    ElSalvador,
    EquatorialGuinea,
    Eritrea,
    Estonia,
    Eswatini,
    Ethiopia,
    FalklandIslands,
    FaroeIslands,
    Fiji,
    Finland,
    France,
    FrenchGuiana,
    FrenchPolynesia,
    FrenchSouthernTerritories,
    Gabon,
    Gambia,
    Georgia,
    Germany,
    Ghana,
    Gibraltar,
    Greece,
    Greenland,
    Grenada,
    Guadeloupe,
    Guam,
    Guatemala,
    Guernsey,
    Guinea,
    GuineaBissau,
    Guyana,
    Haiti,
    HeardAndMcDonaldIslands,
    HolySee,
    Honduras,
    HongKong,
    Hungary,
    Iceland,
    India,
    Indonesia,
    Iran,
    Iraq,
    Ireland,
    IsleOfMan,
    Israel,
    Italy,
    Jamaica,
    Japan,
    Jersey,
    Jordan,
    Kazakhstan,
    Kenya,
    Kiribati,
    Kuwait,
    Kyrgyzstan,
    Laos,
    Latvia,
    Lebanon,
    Lesotho,
    Liberia,
    Libya,
    Liechtenstein,
    Lithuania,
    Luxembourg,
    Macao,
    Madagascar,
    Malawi,
    Malaysia,
    Maldives,
    Mali,
    Malta,
    MarshallIslands,
    Martinique,
    Mauritania,
    Mauritius,
    Mayotte,
    Mexico,
    Micronesia,
    Moldova,
    Monaco,
    Mongolia,
    Montenegro,
    Montserrat,
    Morocco,
    Mozambique,
    Myanmar,
    Namibia,
    Nauru,
    Nepal,
    Netherlands,
    NewCaledonia,
    NewZealand,
    Nicaragua,
    Niger,
    Nigeria,
    Niue,
    NorfolkIsland,
    NorthernMarianaIslands,
    NorthKorea,
    NorthMacedonia,
    Norway,
    Oman,
    Pakistan,
    Palau,
    Palestine,
    Panama,
    PapuaNewGuinea,
    Paraguay,
    Peru,
    Philippines,
    Pitcairn,
    Poland,
    Portugal,
    PuertoRico,
    Qatar,
    Reunion,
    Romania,
    Russia,
    Rwanda,
    Samoa,
    SanMarino,
    SaoTomeAndPrincipe,
    SaudiArabia,
    Senegal,
    Serbia,
    Seychelles,
    SierraLeone,
    Singapore,
    SintMaarten,
    Slovakia,
    Slovenia,
    SolomonIslands,
    Somalia,
    SouthAfrica,
    SouthGeorgiaAndSouthSandwichIslands,
    SouthKorea,
    SouthSudan,
    Spain,
    SriLanka,
    StBarthelemy,
    StHelena,
    StKittsAndNevis,
    StLucia,
    StMartin,
    StPierreAndMiquelon,
    StVincentAndGrenadines,
    Sudan,
    Suriname,
    SvalbardAndJanMayen,
    Sweden,
    Switzerland,
    Syria,
    Taiwan,
    Tajikistan,
    Tanzania,
    Thailand,
    TimorLeste,
    Togo,
    Tokelau,
    Tonga,
    TrinidadAndTobago,
    Tunisia,
    Turkey,
    Turkmenistan,
    TurksAndCaicosIslands,
    Tuvalu,
    Uganda,
    Ukraine,
    UnitedArab,
    UnitedKingdom,
    UnitedStates,
    UnitedStatesMinorOutlyingIslands,
    Uruguay,
    USVirginIslands,
    Uzbekistan,
    Vanuatu,
    Venezuela,
    Vietnam,
    WallisAndFutuna,
    WesternSahara,
    Yemen,
    Zambia,
    Zimbabwe,
} // enum

impl From<&Country> for String {
    /// Converts a `Country` enum to a `String` that contains a [ISO 3166-1
    /// Alpha-2](https://en.wikipedia.org/wiki/List_of_ISO_3166_country_codes)
    /// country code.
    fn from(country: &Country) -> String {
        match country {
            Country::Afghanistan => String::from("AF"),
            Country::AlandIslands => String::from("AX"),
            Country::Albania => String::from("AL"),
            Country::Algeria => String::from("DZ"),
            Country::AmericanSamoa => String::from("AS"),
            Country::Andorra => String::from("AD"),
            Country::Angola => String::from("AO"),
            Country::Anguilla => String::from("AI"),
            Country::Antarctica => String::from("AQ"),
            Country::AntiguaAndBarbuda => String::from("AG"),
            Country::Argentina => String::from("AR"),
            Country::Armenia => String::from("AM"),
            Country::Aruba => String::from("AW"),
            Country::Australia => String::from("AU"),
            Country::Austria => String::from("AT"),
            Country::Azerbaijan => String::from("AZ"),
            Country::Bahamas => String::from("BS"),
            Country::Bahrain => String::from("BH"),
            Country::Bangladesh => String::from("BD"),
            Country::Barbados => String::from("BB"),
            Country::Belarus => String::from("BY"),
            Country::Belgium => String::from("BE"),
            Country::Belize => String::from("BZ"),
            Country::Benin => String::from("BJ"),
            Country::Bermuda => String::from("BM"),
            Country::Bhutan => String::from("BT"),
            Country::Bolivia => String::from("BO"),
            Country::BonaireSintEustatiusAndSaba => String::from("BQ"),
            Country::BosniaAndHerzegovina => String::from("BA"),
            Country::Botswana => String::from("BW"),
            Country::BouvetIsland => String::from("BV"),
            Country::Brazil => String::from("BR"),
            Country::BritishIndianOceanTerritory => String::from("IO"),
            Country::BritishVirginIslands => String::from("VG"),
            Country::Brunei => String::from("BN"),
            Country::Bulgaria => String::from("BG"),
            Country::BurkinaFaso => String::from("BF"),
            Country::Burundi => String::from("BI"),
            Country::CaboVerde => String::from("CV"),
            Country::Cambodia => String::from("KH"),
            Country::Cameroon => String::from("CM"),
            Country::Canada => String::from("CA"),
            Country::CaymanIslands => String::from("KY"),
            Country::CentralAfricanRepublic => String::from("CF"),
            Country::Chad => String::from("TD"),
            Country::Chile => String::from("CL"),
            Country::China => String::from("CN"),
            Country::ChristmasIsland => String::from("CX"),
            Country::CocosKeelingIslands => String::from("CC"),
            Country::Colombia => String::from("CO"),
            Country::Comoros => String::from("KM"),
            Country::CongoBrazzaville => String::from("CG"),
            Country::CongoKinshasa => String::from("CD"),
            Country::CookIslands => String::from("CK"),
            Country::CostaRica => String::from("CR"),
            Country::CotedIvoire => String::from("CI"),
            Country::Croatia => String::from("HR"),
            Country::Cuba => String::from("CU"),
            Country::Curacao => String::from("CW"),
            Country::Cyprus => String::from("CY"),
            Country::Czechia => String::from("CZ"),
            Country::Denmark => String::from("DK"),
            Country::Djibouti => String::from("DJ"),
            Country::Dominica => String::from("DM"),
            Country::DominicanRepublic => String::from("DO"),
            Country::Ecuador => String::from("EC"),
            Country::Egypt => String::from("EG"),
            Country::ElSalvador => String::from("SV"),
            Country::EquatorialGuinea => String::from("GQ"),
            Country::Eritrea => String::from("ER"),
            Country::Estonia => String::from("EE"),
            Country::Eswatini => String::from("SZ"),
            Country::Ethiopia => String::from("ET"),
            Country::FalklandIslands => String::from("FK"),
            Country::FaroeIslands => String::from("FO"),
            Country::Fiji => String::from("FJ"),
            Country::Finland => String::from("FI"),
            Country::France => String::from("FR"),
            Country::FrenchGuiana => String::from("GF"),
            Country::FrenchPolynesia => String::from("PF"),
            Country::FrenchSouthernTerritories => String::from("TF"),
            Country::Gabon => String::from("GA"),
            Country::Gambia => String::from("GM"),
            Country::Georgia => String::from("GE"),
            Country::Germany => String::from("DE"),
            Country::Ghana => String::from("GH"),
            Country::Gibraltar => String::from("GI"),
            Country::Greece => String::from("GR"),
            Country::Greenland => String::from("GL"),
            Country::Grenada => String::from("GD"),
            Country::Guadeloupe => String::from("GP"),
            Country::Guam => String::from("GU"),
            Country::Guatemala => String::from("GT"),
            Country::Guernsey => String::from("GG"),
            Country::Guinea => String::from("GN"),
            Country::GuineaBissau => String::from("GW"),
            Country::Guyana => String::from("GY"),
            Country::Haiti => String::from("HT"),
            Country::HeardAndMcDonaldIslands => String::from("HM"),
            Country::HolySee => String::from("VA"),
            Country::Honduras => String::from("HN"),
            Country::HongKong => String::from("HK"),
            Country::Hungary => String::from("HU"),
            Country::Iceland => String::from("IS"),
            Country::India => String::from("IN"),
            Country::Indonesia => String::from("ID"),
            Country::Iran => String::from("IR"),
            Country::Iraq => String::from("IQ"),
            Country::Ireland => String::from("IE"),
            Country::IsleOfMan => String::from("IM"),
            Country::Israel => String::from("IL"),
            Country::Italy => String::from("IT"),
            Country::Jamaica => String::from("JM"),
            Country::Japan => String::from("JP"),
            Country::Jersey => String::from("JE"),
            Country::Jordan => String::from("JO"),
            Country::Kazakhstan => String::from("KZ"),
            Country::Kenya => String::from("KE"),
            Country::Kiribati => String::from("KI"),
            Country::Kuwait => String::from("KW"),
            Country::Kyrgyzstan => String::from("KG"),
            Country::Laos => String::from("LA"),
            Country::Latvia => String::from("LV"),
            Country::Lebanon => String::from("LB"),
            Country::Lesotho => String::from("LS"),
            Country::Liberia => String::from("LR"),
            Country::Libya => String::from("LY"),
            Country::Liechtenstein => String::from("LI"),
            Country::Lithuania => String::from("LT"),
            Country::Luxembourg => String::from("LU"),
            Country::Macao => String::from("MO"),
            Country::Madagascar => String::from("MG"),
            Country::Malawi => String::from("MW"),
            Country::Malaysia => String::from("MY"),
            Country::Maldives => String::from("MV"),
            Country::Mali => String::from("ML"),
            Country::Malta => String::from("MT"),
            Country::MarshallIslands => String::from("MH"),
            Country::Martinique => String::from("MQ"),
            Country::Mauritania => String::from("MR"),
            Country::Mauritius => String::from("MU"),
            Country::Mayotte => String::from("YT"),
            Country::Mexico => String::from("MX"),
            Country::Micronesia => String::from("FM"),
            Country::Moldova => String::from("MD"),
            Country::Monaco => String::from("MC"),
            Country::Mongolia => String::from("MN"),
            Country::Montenegro => String::from("ME"),
            Country::Montserrat => String::from("MS"),
            Country::Morocco => String::from("MA"),
            Country::Mozambique => String::from("MZ"),
            Country::Myanmar => String::from("MM"),
            Country::Namibia => String::from("NA"),
            Country::Nauru => String::from("NR"),
            Country::Nepal => String::from("NP"),
            Country::Netherlands => String::from("NL"),
            Country::NewCaledonia => String::from("NC"),
            Country::NewZealand => String::from("NZ"),
            Country::Nicaragua => String::from("NI"),
            Country::Niger => String::from("NE"),
            Country::Nigeria => String::from("NG"),
            Country::Niue => String::from("NU"),
            Country::NorfolkIsland => String::from("NF"),
            Country::NorthernMarianaIslands => String::from("MP"),
            Country::NorthKorea => String::from("KP"),
            Country::NorthMacedonia => String::from("MK"),
            Country::Norway => String::from("NO"),
            Country::Oman => String::from("OM"),
            Country::Pakistan => String::from("PK"),
            Country::Palau => String::from("PW"),
            Country::Palestine => String::from("PS"),
            Country::Panama => String::from("PA"),
            Country::PapuaNewGuinea => String::from("PG"),
            Country::Paraguay => String::from("PY"),
            Country::Peru => String::from("PE"),
            Country::Philippines => String::from("PH"),
            Country::Pitcairn => String::from("PN"),
            Country::Poland => String::from("PL"),
            Country::Portugal => String::from("PT"),
            Country::PuertoRico => String::from("PR"),
            Country::Qatar => String::from("QA"),
            Country::Reunion => String::from("RE"),
            Country::Romania => String::from("RO"),
            Country::Russia => String::from("RU"),
            Country::Rwanda => String::from("RW"),
            Country::Samoa => String::from("WS"),
            Country::SanMarino => String::from("SM"),
            Country::SaoTomeAndPrincipe => String::from("ST"),
            Country::SaudiArabia => String::from("SA"),
            Country::Senegal => String::from("SN"),
            Country::Serbia => String::from("RS"),
            Country::Seychelles => String::from("SC"),
            Country::SierraLeone => String::from("SL"),
            Country::Singapore => String::from("SG"),
            Country::SintMaarten => String::from("SX"),
            Country::Slovakia => String::from("SK"),
            Country::Slovenia => String::from("SI"),
            Country::SolomonIslands => String::from("SB"),
            Country::Somalia => String::from("SO"),
            Country::SouthAfrica => String::from("ZA"),
            Country::SouthGeorgiaAndSouthSandwichIslands => String::from("GS"),
            Country::SouthKorea => String::from("KR"),
            Country::SouthSudan => String::from("SS"),
            Country::Spain => String::from("ES"),
            Country::SriLanka => String::from("LK"),
            Country::StBarthelemy => String::from("BL"),
            Country::StHelena => String::from("SH"),
            Country::StKittsAndNevis => String::from("KN"),
            Country::StLucia => String::from("LC"),
            Country::StMartin => String::from("MF"),
            Country::StPierreAndMiquelon => String::from("PM"),
            Country::StVincentAndGrenadines => String::from("VC"),
            Country::Sudan => String::from("SD"),
            Country::Suriname => String::from("SR"),
            Country::SvalbardAndJanMayen => String::from("SJ"),
            Country::Sweden => String::from("E"),
            Country::Switzerland => String::from("CH"),
            Country::Syria => String::from("SY"),
            Country::Taiwan => String::from("TW"),
            Country::Tajikistan => String::from("TJ"),
            Country::Tanzania => String::from("TZ"),
            Country::Thailand => String::from("TH"),
            Country::TimorLeste => String::from("TL"),
            Country::Togo => String::from("TG"),
            Country::Tokelau => String::from("TK"),
            Country::Tonga => String::from("TO"),
            Country::TrinidadAndTobago => String::from("TT"),
            Country::Tunisia => String::from("TN"),
            Country::Turkey => String::from("TR"),
            Country::Turkmenistan => String::from("TM"),
            Country::TurksAndCaicosIslands => String::from("TC"),
            Country::Tuvalu => String::from("TV"),
            Country::Uganda => String::from("UG"),
            Country::Ukraine => String::from("UA"),
            Country::UnitedArab => String::from("AE"),
            Country::UnitedKingdom => String::from("GB"),
            Country::UnitedStates => String::from("US"),
            Country::UnitedStatesMinorOutlyingIslands => String::from("UM"),
            Country::Uruguay => String::from("UY"),
            Country::USVirginIslands => String::from("VI"),
            Country::Uzbekistan => String::from("UZ"),
            Country::Vanuatu => String::from("VU"),
            Country::Venezuela => String::from("VE"),
            Country::Vietnam => String::from("VN"),
            Country::WallisAndFutuna => String::from("WF"),
            Country::WesternSahara => String::from("EH"),
            Country::Yemen => String::from("YE"),
            Country::Zambia => String::from("ZM"),
            Country::Zimbabwe => String::from("ZW"),
        } // match
    } // fn
} // enum

impl From<String> for Country {
    /// Gets a `Country` enum from a `String` that contains a valid [ISO 3166-1
    /// Alpha-2](https://en.wikipedia.org/wiki/List_of_ISO_3166_country_codes)
    /// country code.
    fn from(country: String) -> Country {
        match country.as_ref() {
            "AF" => Country::Afghanistan,
            "AX" => Country::AlandIslands,
            "AL" => Country::Albania,
            "DZ" => Country::Algeria,
            "AS" => Country::AmericanSamoa,
            "AD" => Country::Andorra,
            "AO" => Country::Angola,
            "AI" => Country::Anguilla,
            "AQ" => Country::Antarctica,
            "AG" => Country::AntiguaAndBarbuda,
            "AR" => Country::Argentina,
            "AM" => Country::Armenia,
            "AW" => Country::Aruba,
            "AU" => Country::Australia,
            "AT" => Country::Austria,
            "AZ" => Country::Azerbaijan,
            "BS" => Country::Bahamas,
            "BH" => Country::Bahrain,
            "BD" => Country::Bangladesh,
            "BB" => Country::Barbados,
            "BY" => Country::Belarus,
            "BE" => Country::Belgium,
            "BZ" => Country::Belize,
            "BJ" => Country::Benin,
            "BM" => Country::Bermuda,
            "BT" => Country::Bhutan,
            "BO" => Country::Bolivia,
            "BQ" => Country::BonaireSintEustatiusAndSaba,
            "BA" => Country::BosniaAndHerzegovina,
            "BW" => Country::Botswana,
            "BV" => Country::BouvetIsland,
            "BR" => Country::Brazil,
            "IO" => Country::BritishIndianOceanTerritory,
            "VG" => Country::BritishVirginIslands,
            "BN" => Country::Brunei,
            "BG" => Country::Bulgaria,
            "BF" => Country::BurkinaFaso,
            "BI" => Country::Burundi,
            "CV" => Country::CaboVerde,
            "KH" => Country::Cambodia,
            "CM" => Country::Cameroon,
            "CA" => Country::Canada,
            "KY" => Country::CaymanIslands,
            "CF" => Country::CentralAfricanRepublic,
            "TD" => Country::Chad,
            "CL" => Country::Chile,
            "CN" => Country::China,
            "CX" => Country::ChristmasIsland,
            "CC" => Country::CocosKeelingIslands,
            "CO" => Country::Colombia,
            "KM" => Country::Comoros,
            "CG" => Country::CongoBrazzaville,
            "CD" => Country::CongoKinshasa,
            "CK" => Country::CookIslands,
            "CR" => Country::CostaRica,
            "CI" => Country::CotedIvoire,
            "HR" => Country::Croatia,
            "CU" => Country::Cuba,
            "CW" => Country::Curacao,
            "CY" => Country::Cyprus,
            "CZ" => Country::Czechia,
            "DK" => Country::Denmark,
            "DJ" => Country::Djibouti,
            "DM" => Country::Dominica,
            "DO" => Country::DominicanRepublic,
            "EC" => Country::Ecuador,
            "EG" => Country::Egypt,
            "SV" => Country::ElSalvador,
            "GQ" => Country::EquatorialGuinea,
            "ER" => Country::Eritrea,
            "EE" => Country::Estonia,
            "SZ" => Country::Eswatini,
            "ET" => Country::Ethiopia,
            "FK" => Country::FalklandIslands,
            "FO" => Country::FaroeIslands,
            "FJ" => Country::Fiji,
            "FI" => Country::Finland,
            "FR" => Country::France,
            "GF" => Country::FrenchGuiana,
            "PF" => Country::FrenchPolynesia,
            "TF" => Country::FrenchSouthernTerritories,
            "GA" => Country::Gabon,
            "GM" => Country::Gambia,
            "GE" => Country::Georgia,
            "DE" => Country::Germany,
            "GH" => Country::Ghana,
            "GI" => Country::Gibraltar,
            "GR" => Country::Greece,
            "GL" => Country::Greenland,
            "GD" => Country::Grenada,
            "GP" => Country::Guadeloupe,
            "GU" => Country::Guam,
            "GT" => Country::Guatemala,
            "GG" => Country::Guernsey,
            "GN" => Country::Guinea,
            "GW" => Country::GuineaBissau,
            "GY" => Country::Guyana,
            "HT" => Country::Haiti,
            "HM" => Country::HeardAndMcDonaldIslands,
            "VA" => Country::HolySee,
            "HN" => Country::Honduras,
            "HK" => Country::HongKong,
            "HU" => Country::Hungary,
            "IS" => Country::Iceland,
            "IN" => Country::India,
            "ID" => Country::Indonesia,
            "IR" => Country::Iran,
            "IQ" => Country::Iraq,
            "IE" => Country::Ireland,
            "IM" => Country::IsleOfMan,
            "IL" => Country::Israel,
            "IT" => Country::Italy,
            "JM" => Country::Jamaica,
            "JP" => Country::Japan,
            "JE" => Country::Jersey,
            "JO" => Country::Jordan,
            "KZ" => Country::Kazakhstan,
            "KE" => Country::Kenya,
            "KI" => Country::Kiribati,
            "KW" => Country::Kuwait,
            "KG" => Country::Kyrgyzstan,
            "LA" => Country::Laos,
            "LV" => Country::Latvia,
            "LB" => Country::Lebanon,
            "LS" => Country::Lesotho,
            "LR" => Country::Liberia,
            "LY" => Country::Libya,
            "LI" => Country::Liechtenstein,
            "LT" => Country::Lithuania,
            "LU" => Country::Luxembourg,
            "MO" => Country::Macao,
            "MG" => Country::Madagascar,
            "MW" => Country::Malawi,
            "MY" => Country::Malaysia,
            "MV" => Country::Maldives,
            "ML" => Country::Mali,
            "MT" => Country::Malta,
            "MH" => Country::MarshallIslands,
            "MQ" => Country::Martinique,
            "MR" => Country::Mauritania,
            "MU" => Country::Mauritius,
            "YT" => Country::Mayotte,
            "MX" => Country::Mexico,
            "FM" => Country::Micronesia,
            "MD" => Country::Moldova,
            "MC" => Country::Monaco,
            "MN" => Country::Mongolia,
            "ME" => Country::Montenegro,
            "MS" => Country::Montserrat,
            "MA" => Country::Morocco,
            "MZ" => Country::Mozambique,
            "MM" => Country::Myanmar,
            "NA" => Country::Namibia,
            "NR" => Country::Nauru,
            "NP" => Country::Nepal,
            "NL" => Country::Netherlands,
            "NC" => Country::NewCaledonia,
            "NZ" => Country::NewZealand,
            "NI" => Country::Nicaragua,
            "NE" => Country::Niger,
            "NG" => Country::Nigeria,
            "NU" => Country::Niue,
            "NF" => Country::NorfolkIsland,
            "MP" => Country::NorthernMarianaIslands,
            "KP" => Country::NorthKorea,
            "MK" => Country::NorthMacedonia,
            "NO" => Country::Norway,
            "OM" => Country::Oman,
            "PK" => Country::Pakistan,
            "PW" => Country::Palau,
            "PS" => Country::Palestine,
            "PA" => Country::Panama,
            "PG" => Country::PapuaNewGuinea,
            "PY" => Country::Paraguay,
            "PE" => Country::Peru,
            "PH" => Country::Philippines,
            "PN" => Country::Pitcairn,
            "PL" => Country::Poland,
            "PT" => Country::Portugal,
            "PR" => Country::PuertoRico,
            "QA" => Country::Qatar,
            "RE" => Country::Reunion,
            "RO" => Country::Romania,
            "RU" => Country::Russia,
            "RW" => Country::Rwanda,
            "WS" => Country::Samoa,
            "SM" => Country::SanMarino,
            "ST" => Country::SaoTomeAndPrincipe,
            "SA" => Country::SaudiArabia,
            "SN" => Country::Senegal,
            "RS" => Country::Serbia,
            "SC" => Country::Seychelles,
            "SL" => Country::SierraLeone,
            "SG" => Country::Singapore,
            "SX" => Country::SintMaarten,
            "SK" => Country::Slovakia,
            "SI" => Country::Slovenia,
            "SB" => Country::SolomonIslands,
            "SO" => Country::Somalia,
            "ZA" => Country::SouthAfrica,
            "GS" => Country::SouthGeorgiaAndSouthSandwichIslands,
            "KR" => Country::SouthKorea,
            "SS" => Country::SouthSudan,
            "ES" => Country::Spain,
            "LK" => Country::SriLanka,
            "BL" => Country::StBarthelemy,
            "SH" => Country::StHelena,
            "KN" => Country::StKittsAndNevis,
            "LC" => Country::StLucia,
            "MF" => Country::StMartin,
            "PM" => Country::StPierreAndMiquelon,
            "VC" => Country::StVincentAndGrenadines,
            "SD" => Country::Sudan,
            "SR" => Country::Suriname,
            "SJ" => Country::SvalbardAndJanMayen,
            "SE" => Country::Sweden,
            "CH" => Country::Switzerland,
            "SY" => Country::Syria,
            "TW" => Country::Taiwan,
            "TJ" => Country::Tajikistan,
            "TZ" => Country::Tanzania,
            "TH" => Country::Thailand,
            "TL" => Country::TimorLeste,
            "TG" => Country::Togo,
            "TK" => Country::Tokelau,
            "TO" => Country::Tonga,
            "TT" => Country::TrinidadAndTobago,
            "TN" => Country::Tunisia,
            "TR" => Country::Turkey,
            "TM" => Country::Turkmenistan,
            "TC" => Country::TurksAndCaicosIslands,
            "TV" => Country::Tuvalu,
            "UG" => Country::Uganda,
            "UA" => Country::Ukraine,
            "AE" => Country::UnitedArab,
            "GB" => Country::UnitedKingdom,
            "US" => Country::UnitedStates,
            "UM" => Country::UnitedStatesMinorOutlyingIslands,
            "UY" => Country::Uruguay,
            "VI" => Country::USVirginIslands,
            "UZ" => Country::Uzbekistan,
            "VU" => Country::Vanuatu,
            "VE" => Country::Venezuela,
            "VN" => Country::Vietnam,
            "WF" => Country::WallisAndFutuna,
            "EH" => Country::WesternSahara,
            "YE" => Country::Yemen,
            "ZM" => Country::Zambia,
            "ZW" => Country::Zimbabwe,
            _ => panic!("'{}' is not a valid ISO 3166-1 Alpha-2 country code. \
                Note that the country code must be in uppercase. \
                For a list of country codes see \
                https://en.wikipedia.org/wiki/List_of_ISO_3166_country_codes",
                country),
        } // match
    } // fn
} // impl