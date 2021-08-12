// TODO: license

use crate::blob_provider;
use icu_uniset::props::*;
use icu_uniset::enum_props::GeneralCategory;
use icu_uniset::UnicodeSet;

pub fn get_unicode_set(prop_name: &str, prop_value: Option<&str>) -> Option<UnicodeSet> {
    let prop = get_property(prop_name)?;

    let provider = blob_provider::get_static_provider();
    let set = match prop {
        Property::Alphabetic => get_alphabetic_property(&provider),
        Property::AsciiHexDigit => get_ascii_hex_digit_property(&provider),
        Property::BidiControl => get_bidi_control_property(&provider),
        Property::BidiMirrored => get_bidi_mirrored_property(&provider),
        Property::CaseIgnorable => get_case_ignorable_property(&provider),
        Property::Cased => get_cased_property(&provider),
        Property::ChangesWhenCasefolded => get_changes_when_casefolded_property(&provider),
        Property::ChangesWhenLowercased => get_changes_when_lowercased_property(&provider),
        Property::ChangesWhenNfkcCasefolded => get_changes_when_nfkc_casefolded_property(&provider),
        Property::ChangesWhenTitlecased => get_changes_when_titlecased_property(&provider),
        Property::ChangesWhenUppercased => get_changes_when_uppercased_property(&provider),
        Property::Dash => get_dash_property(&provider),
        Property::DefaultIgnorableCodePoint => get_default_ignorable_code_point_property(&provider),
        Property::Deprecated => get_deprecated_property(&provider),
        Property::Diacritic => get_diacritic_property(&provider),
        Property::Emoji => get_emoji_property(&provider),
        Property::EmojiComponent => get_emoji_component_property(&provider),
        Property::EmojiModifierBase => get_emoji_modifier_base_property(&provider),
        Property::EmojiModifier => get_emoji_modifier_property(&provider),
        Property::EmojiPresentation => get_emoji_presentation_property(&provider),
        Property::ExtendedPictographic => get_extended_pictographic_property(&provider),
        Property::Extender => get_extender_property(&provider),
        Property::GraphemeBase => get_grapheme_base_property(&provider),
        Property::GraphemeExtend => get_grapheme_extend_property(&provider),
        Property::HexDigit => get_hex_digit_property(&provider),
        Property::IdContinue => get_id_continue_property(&provider),
        Property::IdStart => get_id_start_property(&provider),
        Property::Ideographic => get_ideographic_property(&provider),
        Property::IdsBinaryOperator => get_ids_binary_operator_property(&provider),
        Property::IdsTrinaryOperator => get_ids_trinary_operator_property(&provider),
        Property::JoinControl => get_join_control_property(&provider),
        Property::LogicalOrderException => get_logical_order_exception_property(&provider),
        Property::Lowercase => get_lowercase_property(&provider),
        Property::Math => get_math_property(&provider),
        Property::NoncharacterCodePoint => get_noncharacter_code_point_property(&provider),
        Property::PatternSyntax => get_pattern_syntax_property(&provider),
        Property::PatternWhiteSpace => get_pattern_white_space_property(&provider),
        Property::QuotationMark => get_quotation_mark_property(&provider),
        Property::Radical => get_radical_property(&provider),
        Property::RegionalIndicator => get_regional_indicator_property(&provider),
        Property::SoftDotted => get_soft_dotted_property(&provider),
        Property::SentenceTerminal => get_sentence_terminal_property(&provider),
        Property::TerminalPunctuation => get_terminal_punctuation_property(&provider),
        Property::UnifiedIdeograph => get_unified_ideograph_property(&provider),
        Property::Uppercase => get_uppercase_property(&provider),
        Property::VariationSelector => get_variation_selector_property(&provider),
        Property::WhiteSpace => get_white_space_property(&provider),
        Property::XidContinue => get_xid_continue_property(&provider),
        Property::XidStart => get_xid_start_property(&provider),

        Property::GeneralCategory => {
            let category = get_general_category(prop_value?)?;
            get_general_category_val_set(&provider, category)
        },

        _ => unimplemented!(),
    }
    .expect("Static data should cover all properties");

    Some(set)
}

// The set of properties supported by the ECMAScript language specification.
enum Property {
    // Binary properties: https://tc39.es/ecma262/#table-binary-unicode-properties
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

    // Enumerated properties: https://tc39.es/ecma262/#table-nonbinary-unicode-properties
    GeneralCategory,
    Script,
    ScriptExtension,

    // Special cases: See https://unicode.org/reports/tr18/#General_Category_Property
    Ascii,
    Any,
    Assigned,
}

fn get_property(prop_name: &str) -> Option<Property> {
    Some(match prop_name {
        "Alphabetic" | "Alpha" => Property::Alphabetic,
        "ASCII_Hex_Digit" | "AHex" => Property::AsciiHexDigit,
        "Bidi_Control" | "Bidi_C" => Property::BidiControl,
        "Bidi_Mirrored" | "Bidi_M" => Property::BidiMirrored,
        "Case_Ignorable" | "CI" => Property::CaseIgnorable,
        "Cased" => Property::Cased,
        "Changes_When_Casefolded" | "CWCF" => Property::ChangesWhenCasefolded,
        "Changes_When_Casemapped" | "CWCM" => Property::ChangesWhenCasemapped,
        "Changes_When_Lowercased" | "CWL" => Property::ChangesWhenLowercased,
        "Changes_When_NFKC_Casefolded" | "CWKCF" => Property::ChangesWhenNfkcCasefolded,
        "Changes_When_Titlecased" | "CWT" => Property::ChangesWhenTitlecased,
        "Changes_When_Uppercased" | "CWU" => Property::ChangesWhenUppercased,
        "Dash" => Property::Dash,
        "Default_Ignorable_Code_Point" | "DI" => Property::DefaultIgnorableCodePoint,
        "Deprecated" | "Dep" => Property::Deprecated,
        "Diacritic" | "Dia" => Property::Diacritic,
        "Emoji" => Property::Emoji,
        "Emoji_Component" | "EComp" => Property::EmojiComponent,
        "Emoji_Modifier_Base" | "EBase" => Property::EmojiModifierBase,
        "Emoji_Modifier" | "EMod" => Property::EmojiModifier,
        "Emoji_Presentation" | "EPres" => Property::EmojiPresentation,
        "Extended_Pictographic" | "ExtPict" => Property::ExtendedPictographic,
        "Extender" | "Ext" => Property::Extender,
        "Grapheme_Base" | "Gr_Base" => Property::GraphemeBase,
        "Grapheme_Extend" | "Gr_Ext" => Property::GraphemeExtend,
        "Hex_Digit" | "Hex" => Property::HexDigit,
        "IDS_Binary_Operator" | "IDSB" => Property::IdsBinaryOperator,
        "IDS_Trinary_Operator" | "IDST" => Property::IdsTrinaryOperator,
        "Id_Continue" | "IDC" => Property::IdContinue,
        "Id_Start" | "IDS" => Property::IdStart,
        "Ideographic" | "Ideo" => Property::Ideographic,
        "Join_Control" | "JoinC" => Property::JoinControl,
        "Logical_Order_Exception" | "LOE" => Property::LogicalOrderException,
        "Lowercase" | "Lower" => Property::Lowercase,
        "Math" => Property::Math,
        "Noncharacter_Code_Point" | "NChar" => Property::NoncharacterCodePoint,
        "Pattern_Syntax" | "Pat_Syn" => Property::PatternSyntax,
        "Pattern_White_Space" | "Pat_WS" => Property::PatternWhiteSpace,
        "Quotation_Mark" | "QMark" => Property::QuotationMark,
        "Radical" => Property::Radical,
        "Regional_Indicator" | "RI" => Property::RegionalIndicator,
        "SentenceTerminal" | "STerm" => Property::SentenceTerminal,
        "Soft_Dotted" | "SD" => Property::SoftDotted,
        "Terminal_Punctuation" | "Term" => Property::TerminalPunctuation,
        "Unified_Ideograph" | "UIdeo" => Property::UnifiedIdeograph,
        "Uppercase" | "Upper" => Property::Uppercase,
        "Variation_Selector" | "VS" => Property::VariationSelector,
        "White_Space" | "space" => Property::WhiteSpace,
        "Xid_Continue" | "XIDC" => Property::XidContinue,
        "Xid_Start" | "XIDS" => Property::XidStart,

        "General_Category" | "gc" => Property::GeneralCategory,
        "Script" | "sc" => Property::Script,
        "Script_Extensions" | "scx" => Property::ScriptExtension,

        "ASCII" => Property::Ascii,
        "Any" => Property::Any,
        "Assigned" => Property::Assigned,

        _ => return None,
    })
}

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
        "Modifier_Letter" |"Lm" => GeneralCategory::ModifierLetter,
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
        "Unassigned" | 	"Cn" => GeneralCategory::Unassigned,
        "Uppercase_Letter" | "Lu" => GeneralCategory::UppercaseLetter,
        _ => return None
    })
}

#[test]
fn test_basic() {
    let whitespace1: UnicodeSet = get_unicode_set("space", None).unwrap();
    let whitespace2: UnicodeSet = get_unicode_set("White_Space", None).unwrap();
    assert_eq!(whitespace1.get_inversion_list(), whitespace2.get_inversion_list());
}
