// TODO: License

use icu_uniset::props::*;
use icu_uniset::*;

mod blob_provider;
mod property;

use property::Property;

pub fn get_unicode_set(prop_name: &str, _prop_value: Option<&str>) -> Option<UnicodeSet> {
    let prop = property::get_property(prop_name)?;

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
        _ => unimplemented!(),
    }
    .expect("Static data should cover all properties");

    Some(set)
}

#[test]
fn test_basic() {
    use icu_uniset::UnicodeSet;

    let _whitespace1: UnicodeSet = get_unicode_set("space", None).unwrap();
    let _whitespace2: UnicodeSet = get_unicode_set("White_Space", None).unwrap();
}
