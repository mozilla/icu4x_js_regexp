// This file is licensed under the same terms as ICU4X.
// For details, please see the LICENSE file.

use crate::blob_provider;
use icu_uniset::enum_props::{GeneralCategory, Script};
use icu_uniset::props::*;
use icu_uniset::{UnicodeSet, UnicodeSetBuilder};

/// This implements the evaluation of the
/// [UnicodePropertyValueExpression production](
/// https://tc39.es/ecma262/multipage/text-processing.html#prod-UnicodePropertyValueExpression)
/// of the ECMAScript specification. Given a property name and an
/// optional property value, this returns the UnicodeSet of characters
/// matching that property, if it exists.
///
/// Note that the algorithm defined by ECMA262 differs from the
/// matching rules for symbolic values in
/// [UAX44](https://unicode.org/reports/tr44/#Matching_Symbolic):
/// case, whitespace, dashes, and underscores are not ignored, and the
/// Is prefix is not supported.
pub fn get_unicode_set(prop_name: &str, prop_value: Option<&str>) -> Option<UnicodeSet> {
    match prop_value {
        Some(value) => get_unicode_set_by_name_and_value(prop_name, value),
        None => get_unicode_set_by_name(prop_name),
    }
}

//  UnicodePropertyValueExpression :: UnicodePropertyName = UnicodePropertyValue
fn get_unicode_set_by_name_and_value(prop_name: &str, prop_value: &str) -> Option<UnicodeSet> {
    let provider = blob_provider::get_static_provider();

    // Steps 1-3
    let prop = get_enumerated_property(prop_name)?;

    // Steps 4-5
    let set = match prop {
        EnumeratedProperty::GeneralCategory => {
            let category = get_general_category(prop_value)?;
            get_general_category_val_set(&provider, category)
        }

        EnumeratedProperty::Script => {
            let script = get_script(prop_value)?;
            get_script_val_set(&provider, script)
        }

        EnumeratedProperty::ScriptExtension => {
            todo!("Script_Extensions")
        }
    }
    .expect("Static data should be available for all properties");

    // Step 6
    Some(set)
}

// UnicodePropertyValueExpression :: LoneUnicodePropertyNameOrValue
fn get_unicode_set_by_name(prop_name: &str) -> Option<UnicodeSet> {
    let provider = blob_provider::get_static_provider();

    // Steps 1-2.
    if let Some(general_category) = get_general_category(prop_name) {
        let set = get_general_category_val_set(&provider, general_category)
            .expect("Static data should be available for all properties");
        return Some(set);
    }

    // Step 3.
    let prop = get_binary_property(prop_name)?;

    // Steps 4-5.
    use BinaryProperty as BP;
    let set = match prop {
        BP::Alphabetic => get_alphabetic_property(&provider),
        BP::AsciiHexDigit => get_ascii_hex_digit_property(&provider),
        BP::BidiControl => get_bidi_control_property(&provider),
        BP::BidiMirrored => get_bidi_mirrored_property(&provider),
        BP::CaseIgnorable => get_case_ignorable_property(&provider),
        BP::Cased => get_cased_property(&provider),
        BP::ChangesWhenCasefolded => get_changes_when_casefolded_property(&provider),
        BP::ChangesWhenCasemapped => get_changes_when_casemapped_property(&provider),
        BP::ChangesWhenLowercased => get_changes_when_lowercased_property(&provider),
        BP::ChangesWhenNfkcCasefolded => get_changes_when_nfkc_casefolded_property(&provider),
        BP::ChangesWhenTitlecased => get_changes_when_titlecased_property(&provider),
        BP::ChangesWhenUppercased => get_changes_when_uppercased_property(&provider),
        BP::Dash => get_dash_property(&provider),
        BP::DefaultIgnorableCodePoint => get_default_ignorable_code_point_property(&provider),
        BP::Deprecated => get_deprecated_property(&provider),
        BP::Diacritic => get_diacritic_property(&provider),
        BP::Emoji => get_emoji_property(&provider),
        BP::EmojiComponent => get_emoji_component_property(&provider),
        BP::EmojiModifierBase => get_emoji_modifier_base_property(&provider),
        BP::EmojiModifier => get_emoji_modifier_property(&provider),
        BP::EmojiPresentation => get_emoji_presentation_property(&provider),
        BP::ExtendedPictographic => get_extended_pictographic_property(&provider),
        BP::Extender => get_extender_property(&provider),
        BP::GraphemeBase => get_grapheme_base_property(&provider),
        BP::GraphemeExtend => get_grapheme_extend_property(&provider),
        BP::HexDigit => get_hex_digit_property(&provider),
        BP::IdContinue => get_id_continue_property(&provider),
        BP::IdStart => get_id_start_property(&provider),
        BP::Ideographic => get_ideographic_property(&provider),
        BP::IdsBinaryOperator => get_ids_binary_operator_property(&provider),
        BP::IdsTrinaryOperator => get_ids_trinary_operator_property(&provider),
        BP::JoinControl => get_join_control_property(&provider),
        BP::LogicalOrderException => get_logical_order_exception_property(&provider),
        BP::Lowercase => get_lowercase_property(&provider),
        BP::Math => get_math_property(&provider),
        BP::NoncharacterCodePoint => get_noncharacter_code_point_property(&provider),
        BP::PatternSyntax => get_pattern_syntax_property(&provider),
        BP::PatternWhiteSpace => get_pattern_white_space_property(&provider),
        BP::QuotationMark => get_quotation_mark_property(&provider),
        BP::Radical => get_radical_property(&provider),
        BP::RegionalIndicator => get_regional_indicator_property(&provider),
        BP::SoftDotted => get_soft_dotted_property(&provider),
        BP::SentenceTerminal => get_sentence_terminal_property(&provider),
        BP::TerminalPunctuation => get_terminal_punctuation_property(&provider),
        BP::UnifiedIdeograph => get_unified_ideograph_property(&provider),
        BP::Uppercase => get_uppercase_property(&provider),
        BP::VariationSelector => get_variation_selector_property(&provider),
        BP::WhiteSpace => get_white_space_property(&provider),
        BP::XidContinue => get_xid_continue_property(&provider),
        BP::XidStart => get_xid_start_property(&provider),

        BP::Ascii => {
            let mut builder = UnicodeSetBuilder::new();
            builder.add_range(&('\u{0}'..='\u{7f}'));
            Ok(builder.build())
        }
        BP::Any => Ok(UnicodeSet::all()),
        BP::Assigned => {
            let mut builder = UnicodeSetBuilder::new();
            let unassigned = get_general_category_val_set(&provider, GeneralCategory::Unassigned)
                .expect("Static data should include Gc=Unassigned");
            builder.add_set(&unassigned);
            builder.complement();
            Ok(builder.build())
        }
    }
    .expect("Static data should be available for all properties");

    Some(set)
}

// Table 69: Non-binary Unicode property aliases and their canonical property names
// https://tc39.es/ecma262/multipage/text-processing.html#table-nonbinary-unicode-properties
enum EnumeratedProperty {
    GeneralCategory,
    Script,
    ScriptExtension,
}

fn get_enumerated_property(prop_name: &str) -> Option<EnumeratedProperty> {
    Some(match prop_name {
        "General_Category" | "gc" => EnumeratedProperty::GeneralCategory,
        "Script" | "sc" => EnumeratedProperty::Script,
        "Script_Extensions" | "scx" => EnumeratedProperty::ScriptExtension,

        _ => return None,
    })
}

// Table 70: Binary Unicode property aliases and their canonical property names
// https://tc39.es/ecma262/multipage/text-processing.html#table-binary-unicode-properties
enum BinaryProperty {
    Alphabetic,
    AsciiHexDigit,
    BidiControl,
    BidiMirrored,
    CaseIgnorable,
    Cased,
    ChangesWhenCasefolded,
    ChangesWhenCasemapped,
    ChangesWhenLowercased,
    ChangesWhenNfkcCasefolded,
    ChangesWhenTitlecased,
    ChangesWhenUppercased,
    Dash,
    DefaultIgnorableCodePoint,
    Deprecated,
    Diacritic,
    Emoji,
    EmojiComponent,
    EmojiModifierBase,
    EmojiModifier,
    EmojiPresentation,
    ExtendedPictographic,
    Extender,
    GraphemeBase,
    GraphemeExtend,
    HexDigit,
    IdsBinaryOperator,
    IdsTrinaryOperator,
    IdContinue,
    IdStart,
    Ideographic,
    JoinControl,
    LogicalOrderException,
    Lowercase,
    Math,
    NoncharacterCodePoint,
    PatternSyntax,
    PatternWhiteSpace,
    QuotationMark,
    Radical,
    RegionalIndicator,
    SentenceTerminal,
    SoftDotted,
    TerminalPunctuation,
    UnifiedIdeograph,
    Uppercase,
    VariationSelector,
    WhiteSpace,
    XidContinue,
    XidStart,

    // Special cases: See https://unicode.org/reports/tr18/#General_Category_Property
    Ascii,
    Any,
    Assigned,
}

fn get_binary_property(prop_name: &str) -> Option<BinaryProperty> {
    Some(match prop_name {
        "Alphabetic" | "Alpha" => BinaryProperty::Alphabetic,
        "ASCII_Hex_Digit" | "AHex" => BinaryProperty::AsciiHexDigit,
        "Bidi_Control" | "Bidi_C" => BinaryProperty::BidiControl,
        "Bidi_Mirrored" | "Bidi_M" => BinaryProperty::BidiMirrored,
        "Case_Ignorable" | "CI" => BinaryProperty::CaseIgnorable,
        "Cased" => BinaryProperty::Cased,
        "Changes_When_Casefolded" | "CWCF" => BinaryProperty::ChangesWhenCasefolded,
        "Changes_When_Casemapped" | "CWCM" => BinaryProperty::ChangesWhenCasemapped,
        "Changes_When_Lowercased" | "CWL" => BinaryProperty::ChangesWhenLowercased,
        "Changes_When_NFKC_Casefolded" | "CWKCF" => BinaryProperty::ChangesWhenNfkcCasefolded,
        "Changes_When_Titlecased" | "CWT" => BinaryProperty::ChangesWhenTitlecased,
        "Changes_When_Uppercased" | "CWU" => BinaryProperty::ChangesWhenUppercased,
        "Dash" => BinaryProperty::Dash,
        "Default_Ignorable_Code_Point" | "DI" => BinaryProperty::DefaultIgnorableCodePoint,
        "Deprecated" | "Dep" => BinaryProperty::Deprecated,
        "Diacritic" | "Dia" => BinaryProperty::Diacritic,
        "Emoji" => BinaryProperty::Emoji,
        "Emoji_Component" | "EComp" => BinaryProperty::EmojiComponent,
        "Emoji_Modifier_Base" | "EBase" => BinaryProperty::EmojiModifierBase,
        "Emoji_Modifier" | "EMod" => BinaryProperty::EmojiModifier,
        "Emoji_Presentation" | "EPres" => BinaryProperty::EmojiPresentation,
        "Extended_Pictographic" | "ExtPict" => BinaryProperty::ExtendedPictographic,
        "Extender" | "Ext" => BinaryProperty::Extender,
        "Grapheme_Base" | "Gr_Base" => BinaryProperty::GraphemeBase,
        "Grapheme_Extend" | "Gr_Ext" => BinaryProperty::GraphemeExtend,
        "Hex_Digit" | "Hex" => BinaryProperty::HexDigit,
        "IDS_Binary_Operator" | "IDSB" => BinaryProperty::IdsBinaryOperator,
        "IDS_Trinary_Operator" | "IDST" => BinaryProperty::IdsTrinaryOperator,
        "Id_Continue" | "IDC" => BinaryProperty::IdContinue,
        "Id_Start" | "IDS" => BinaryProperty::IdStart,
        "Ideographic" | "Ideo" => BinaryProperty::Ideographic,
        "Join_Control" | "JoinC" => BinaryProperty::JoinControl,
        "Logical_Order_Exception" | "LOE" => BinaryProperty::LogicalOrderException,
        "Lowercase" | "Lower" => BinaryProperty::Lowercase,
        "Math" => BinaryProperty::Math,
        "Noncharacter_Code_Point" | "NChar" => BinaryProperty::NoncharacterCodePoint,
        "Pattern_Syntax" | "Pat_Syn" => BinaryProperty::PatternSyntax,
        "Pattern_White_Space" | "Pat_WS" => BinaryProperty::PatternWhiteSpace,
        "Quotation_Mark" | "QMark" => BinaryProperty::QuotationMark,
        "Radical" => BinaryProperty::Radical,
        "Regional_Indicator" | "RI" => BinaryProperty::RegionalIndicator,
        "SentenceTerminal" | "STerm" => BinaryProperty::SentenceTerminal,
        "Soft_Dotted" | "SD" => BinaryProperty::SoftDotted,
        "Terminal_Punctuation" | "Term" => BinaryProperty::TerminalPunctuation,
        "Unified_Ideograph" | "UIdeo" => BinaryProperty::UnifiedIdeograph,
        "Uppercase" | "Upper" => BinaryProperty::Uppercase,
        "Variation_Selector" | "VS" => BinaryProperty::VariationSelector,
        "White_Space" | "space" => BinaryProperty::WhiteSpace,
        "Xid_Continue" | "XIDC" => BinaryProperty::XidContinue,
        "Xid_Start" | "XIDS" => BinaryProperty::XidStart,

        "ASCII" => BinaryProperty::Ascii,
        "Any" => BinaryProperty::Any,
        "Assigned" => BinaryProperty::Assigned,

        _ => return None,
    })
}

// Table 71: Value aliases and canonical values for the Unicode property General_Category
// https://tc39.es/ecma262/multipage/text-processing.html#table-unicode-general-category-values
fn get_general_category(gc_name: &str) -> Option<GeneralCategory> {
    Some(match gc_name {
        "Cased_Letter" | "LC" => GeneralCategory::CasedLetter,
        "Close_Punctuation" | "Pe" => GeneralCategory::ClosePunctuation,
        "Connector_Punctuation" | "Pc" => GeneralCategory::ConnectorPunctuation,
        "Control" | "Cc" | "cntrl" => GeneralCategory::Control,
        "Currency_Symbol" | "Sc" => GeneralCategory::CurrencySymbol,
        "Dash_Punctuation" | "Pd" => GeneralCategory::DashPunctuation,
        "Decimal_Number" | "Nd" | "digit" => GeneralCategory::Digit,
        "Enclosing_Mark" | "Me" => GeneralCategory::EnclosingMark,
        "Final_Punctuation" | "Pf" => GeneralCategory::FinalPunctuation,
        "Format" | "Cf" => GeneralCategory::Format,
        "Initial_Punctuation" | "Pi" => GeneralCategory::InitialPunctuation,
        "Letter" | "L" => GeneralCategory::Letter,
        "Letter_Number" | "Nl" => GeneralCategory::LetterNumber,
        "Line_Separator" | "Zl" => GeneralCategory::LineSeparator,
        "Lowercase_Letter" | "Ll" => GeneralCategory::LowercaseLetter,
        "Mark" | "M" | "Combining_Mark" => GeneralCategory::Mark,
        "Math_Symbol" | "Sm" => GeneralCategory::MathSymbol,
        "Modifier_Letter" | "Lm" => GeneralCategory::ModifierLetter,
        "Modifier_Symbol" | "Sk" => GeneralCategory::ModifierSymbol,
        "Nonspacing_Mark" | "Mn" => GeneralCategory::NonspacingMark,
        "Number" | "N" => GeneralCategory::Number,
        "Open_Punctuation" | "Ps" => GeneralCategory::OpenPunctuation,
        "Other" | "C" => GeneralCategory::Other,
        "Other_Letter" | "Lo" => GeneralCategory::OtherLetter,
        "Other_Number" | "No" => GeneralCategory::OtherNumber,
        "Other_Punctuation" | "Po" => GeneralCategory::OtherPunctuation,
        "Other_Symbol" | "So" => GeneralCategory::OtherSymbol,
        "Paragraph_Separator" | "Zp" => GeneralCategory::ParagraphSeparator,
        "Private_Use" | "Co" => GeneralCategory::PrivateUse,
        "Punctuation" | "P" | "punct" => GeneralCategory::Punctuation,
        "Separator" | "Z" => GeneralCategory::Separator,
        "Space_Separator" | "Zs" => GeneralCategory::SpaceSeparator,
        "Spacing_Mark" | "Mc" => GeneralCategory::SpacingMark,
        "Surrogate" | "Cs" => GeneralCategory::Surrogate,
        "Symbol" | "S" => GeneralCategory::Symbol,
        "Titlecase_Letter" | "Lt" => GeneralCategory::TitlecaseLetter,
        "Unassigned" | "Cn" => GeneralCategory::Unassigned,
        "Uppercase_Letter" | "Lu" => GeneralCategory::UppercaseLetter,
        _ => return None,
    })
}

// Table 72: Value aliases and canonical values for the Unicode properties Script and Script_Extensions.
// https://tc39.es/ecma262/multipage/text-processing.html#table-unicode-script-values
fn get_script(script_name: &str) -> Option<Script> {
    Some(match script_name {
        "Adlam" | "Adlm" => Script::Adlam,
        "Ahom" => Script::Ahom,
        "Anatolian_Hieroglyphs" | "Hluw" => Script::AnatolianHieroglyphs,
        "Arabic" | "Arab" => Script::Arabic,
        "Armenian" | "Armn" => Script::Armenian,
        "Avestan" | "Avst" => Script::Avestan,
        "Balinese" | "Bali" => Script::Balinese,
        "Bamum" | "Bamu" => Script::Bamum,
        "Bassa_Vah" | "Bass" => Script::BassaVah,
        "Batak" | "Batk" => Script::Batak,
        "Bengali" | "Beng" => Script::Bengali,
        "Bhaiksuki" | "Bhks" => Script::Bhaiksuki,
        "Bopomofo" | "Bopo" => Script::Bopomofo,
        "Brahmi" | "Brah" => Script::Brahmi,
        "Braille" | "Brai" => Script::Braille,
        "Buginese" | "Bugi" => Script::Buginese,
        "Buhid" | "Buhd" => Script::Buhid,
        "Canadian_Aboriginal" | "Cans" => Script::CanadianAboriginal,
        "Carian" | "Cari" => Script::Carian,
        "Caucasian_Albanian" | "Aghb" => Script::CaucasianAlbanian,
        "Chakma" | "Cakm" => Script::Chakma,
        "Cham" => Script::Cham,
        "Cherokee" | "Cher" => Script::Cherokee,
        "Chorasmian" | "Chrs" => Script::Chorasmian,
        "Common" | "Zyyy" => Script::Common,
        "Coptic" | "Copt" => Script::Coptic,
        "Cuneiform" | "Xsux" => Script::Cuneiform,
        "Cypriot" | "Cprt" => Script::Cypriot,
        "Cypro_Minoan" | "Cpmn" => Script::CyproMinoan,
        "Cyrillic" | "Cyrl" => Script::Cyrillic,
        "Deseret" | "Dsrt" => Script::Deseret,
        "Devanagari" | "Deva" => Script::Devanagari,
        "Dives_Akuru" | "Diak" => Script::DivesAkuru,
        "Dogra" | "Dogr" => Script::Dogra,
        "Duployan" | "Dupl" => Script::Duployan,
        "Egyptian_Hieroglyphs" | "Egyp" => Script::EgyptianHieroglyphs,
        "Elbasan" | "Elba" => Script::Elbasan,
        "Elymaic" | "Elym" => Script::Elymaic,
        "Ethiopic" | "Ethi" => Script::Ethiopic,
        "Georgian" | "Geor" => Script::Georgian,
        "Glagolitic" | "Glag" => Script::Glagolitic,
        "Gothic" | "Goth" => Script::Gothic,
        "Grantha" | "Gran" => Script::Grantha,
        "Greek" | "Grek" => Script::Greek,
        "Gujarati" | "Gujr" => Script::Gujarati,
        "Gunjala_Gondi" | "Gong" => Script::GunjalaGondi,
        "Gurmukhi" | "Guru" => Script::Gurmukhi,
        "Han" | "Hani" => Script::Han,
        "Hangul" | "Hang" => Script::Hangul,
        "Hanifi_Rohingya" | "Rohg" => Script::HanifiRohingya,
        "Hanunoo" | "Hano" => Script::Hanunoo,
        "Hatran" | "Hatr" => Script::Hatran,
        "Hebrew" | "Hebr" => Script::Hebrew,
        "Hiragana" | "Hira" => Script::Hiragana,
        "Imperial_Aramaic" | "Armi" => Script::ImperialAramaic,
        "Inherited" | "Zinh" => Script::Inherited,
        "Inscriptional_Pahlavi" | "Phli" => Script::InscriptionalPahlavi,
        "Inscriptional_Parthian" | "Prti" => Script::InscriptionalParthian,
        "Javanese" | "Java" => Script::Javanese,
        "Kaithi" | "Kthi" => Script::Kaithi,
        "Kannada" | "Knda" => Script::Kannada,
        "Katakana" | "Kana" => Script::Katakana,
        "Kayah_Li" | "Kali" => Script::KayahLi,
        "Kharoshthi" | "Khar" => Script::Kharoshthi,
        "Khitan_Small_Script" | "Kits" => Script::KhitanSmallScript,
        "Khmer" | "Khmr" => Script::Khmer,
        "Khojki" | "Khoj" => Script::Khojki,
        "Khudawadi" | "Sind" => Script::Khudawadi,
        "Lao" | "Laoo" => Script::Lao,
        "Latin" | "Latn" => Script::Latin,
        "Lepcha" | "Lepc" => Script::Lepcha,
        "Limbu" | "Limb" => Script::Limbu,
        "Linear_A" | "Lina" => Script::LinearA,
        "Linear_B" | "Linb" => Script::LinearB,
        "Lisu" => Script::Lisu,
        "Lycian" | "Lyci" => Script::Lycian,
        "Lydian" | "Lydi" => Script::Lydian,
        "Mahajani" | "Mahj" => Script::Mahajani,
        "Makasar" | "Maka" => Script::Makasar,
        "Malayalam" | "Mlym" => Script::Malayalam,
        "Mandaic" | "Mand" => Script::Mandaic,
        "Manichaean" | "Mani" => Script::Manichaean,
        "Marchen" | "Marc" => Script::Marchen,
        "Masaram_Gondi" | "Gonm" => Script::MasaramGondi,
        "Medefaidrin" | "Medf" => Script::Medefaidrin,
        "Meetei_Mayek" | "Mtei" => Script::MeeteiMayek,
        "Mende_Kikakui" | "Mend" => Script::MendeKikakui,
        "Meroitic_Cursive" | "Merc" => Script::MeroiticCursive,
        "Meroitic_Hieroglyphs" | "Mero" => Script::MeroiticHieroglyphs,
        "Miao" | "Plrd" => Script::Miao,
        "Modi" => Script::Modi,
        "Mongolian" | "Mong" => Script::Mongolian,
        "Mro" | "Mroo" => Script::Mro,
        "Multani" | "Mult" => Script::Multani,
        "Myanmar" | "Mymr" => Script::Myanmar,
        "Nabataean" | "Nbat" => Script::Nabataean,
        "Nandinagari" | "Nand" => Script::Nandinagari,
        "New_Tai_Lue" | "Talu" => Script::NewTaiLue,
        "Newa" => Script::Newa,
        "Nko" | "Nkoo" => Script::Nko,
        "Nushu" | "Nshu" => Script::Nushu,
        "Nyiakeng_Puachue_Hmong" | "Hmnp" => Script::NyiakengPuachueHmong,
        "Ogham" | "Ogam" => Script::Ogham,
        "Ol_Chiki" | "Olck" => Script::OlChiki,
        "Old_Hungarian" | "Hung" => Script::OldHungarian,
        "Old_Italic" | "Ital" => Script::OldItalic,
        "Old_North_Arabian" | "Narb" => Script::OldNorthArabian,
        "Old_Permic" | "Perm" => Script::OldPermic,
        "Old_Persian" | "Xpeo" => Script::OldPersian,
        "Old_Sogdian" | "Sogo" => Script::OldSogdian,
        "Old_South_Arabian" | "Sarb" => Script::OldSouthArabian,
        "Old_Turkic" | "Orkh" => Script::OldTurkic,
        "Old_Uyghur" | "Ougr" => Script::OldUyghur,
        "Oriya" | "Orya" => Script::Oriya,
        "Osage" | "Osge" => Script::Osage,
        "Osmanya" | "Osma" => Script::Osmanya,
        "Pahawh_Hmong" | "Hmng" => Script::PahawhHmong,
        "Palmyrene" | "Palm" => Script::Palmyrene,
        "Pau_Cin_Hau" | "Pauc" => Script::PauCinHau,
        "Phags_Pa" | "Phag" => Script::PhagsPa,
        "Phoenician" | "Phnx" => Script::Phoenician,
        "Psalter_Pahlavi" | "Phlp" => Script::PsalterPahlavi,
        "Rejang" | "Rjng" => Script::Rejang,
        "Runic" | "Runr" => Script::Runic,
        "Samaritan" | "Samr" => Script::Samaritan,
        "Saurashtra" | "Saur" => Script::Saurashtra,
        "Sharada" | "Shrd" => Script::Sharada,
        "Shavian" | "Shaw" => Script::Shavian,
        "Siddham" | "Sidd" => Script::Siddham,
        "SignWriting" | "Sgnw" => Script::SignWriting,
        "Sinhala" | "Sinh" => Script::Sinhala,
        "Sogdian" | "Sogd" => Script::Sogdian,
        "Sora_Sompeng" | "Sora" => Script::SoraSompeng,
        "Soyombo" | "Soyo" => Script::Soyombo,
        "Sundanese" | "Sund" => Script::Sundanese,
        "Syloti_Nagri" | "Sylo" => Script::SylotiNagri,
        "Syriac" | "Syrc" => Script::Syriac,
        "Tagalog" | "Tglg" => Script::Tagalog,
        "Tagbanwa" | "Tagb" => Script::Tagbanwa,
        "Tai_Le" | "Tale" => Script::TaiLe,
        "Tai_Tham" | "Lana" => Script::TaiTham,
        "Tai_Viet" | "Tavt" => Script::TaiViet,
        "Takri" | "Takr" => Script::Takri,
        "Tamil" | "Taml" => Script::Tamil,
        "Tangsa" | "Tnsa" => Script::Tangsa,
        "Tangut" | "Tang" => Script::Tangut,
        "Telugu" | "Telu" => Script::Telugu,
        "Thaana" | "Thaa" => Script::Thaana,
        "Thai" => Script::Thai,
        "Tibetan" | "Tibt" => Script::Tibetan,
        "Tifinagh" | "Tfng" => Script::Tifinagh,
        "Tirhuta" | "Tirh" => Script::Tirhuta,
        "Toto" => Script::Toto,
        "Ugaritic" | "Ugar" => Script::Ugaritic,
        "Unknown" | "Zzzz" => Script::Unknown,
        "Vai" | "Vaii" => Script::Vai,
        "Vithkuqi" | "Vith" => Script::Vithkuqi,
        "Wancho" | "Wcho" => Script::Wancho,
        "Warang_Citi" | "Wara" => Script::WarangCiti,
        "Yezidi" | "Yezi" => Script::Yezidi,
        "Yi" | "Yiii" => Script::Yi,
        "Zanabazar_Square" | "Zanb" => Script::ZanabazarSquare,
        _ => return None,
    })
}

#[test]
fn test_basic() {
    let whitespace1: UnicodeSet = get_unicode_set("space", None).unwrap();
    let whitespace2: UnicodeSet = get_unicode_set("White_Space", None).unwrap();
    assert_eq!(
        whitespace1.get_inversion_list(),
        whitespace2.get_inversion_list()
    );
    assert!(whitespace1.contains(' '));
}

#[test]
fn test_script() {
    let cyrillic1: UnicodeSet = get_unicode_set("Script", Some("Cyrillic")).unwrap();
    let cyrillic2: UnicodeSet = get_unicode_set("sc", Some("Cyrl")).unwrap();
    assert_eq!(
        cyrillic1.get_inversion_list(),
        cyrillic2.get_inversion_list()
    );
    assert!(cyrillic1.contains('\u{0410}')); // U+0410 CYRILLIC CAPITAL LETTER A
}

#[test]
fn test_special() {
    let any = get_unicode_set("Any", None).unwrap();
    assert_eq!(any.get_inversion_list(), vec![0, char::MAX as u32 + 1]);

    let ascii = get_unicode_set("ASCII", None).unwrap();
    assert_eq!(ascii.get_inversion_list(), vec![0, 0x80]);

    let assigned = get_unicode_set("Assigned", None).unwrap();
    let unassigned = get_unicode_set("General_Category", Some("Unassigned")).unwrap();
    let mut builder = UnicodeSetBuilder::new();
    builder.add_set(&assigned);
    builder.add_set(&unassigned);
    assert_eq!(
        builder.build().get_inversion_list(),
        any.get_inversion_list()
    );
}
