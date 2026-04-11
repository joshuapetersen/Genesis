//! configparser.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::collections::{MutableMapping};
// use crate::functools;
// use crate::itertools;
// use regex::Regex;
// use crate::warnings;

pub const __all__: &str = ["NoSectionError" ,"DuplicateOptionError" ,"DuplicateSectionError" ,;
pub const _default_dict: /* inferred */ = dict;
pub const DEFAULTSECT: &str = "DEFAULT";
pub const MAX_INTERPOLATION_DEPTH: u64 = 10;
pub struct Error {
    pub message: String, // TODO: infer type
    pub section: String, // TODO: infer type
    pub args: String, // TODO: infer type
    pub source: String, // TODO: infer type
    pub lineno: String, // TODO: infer type
    pub option: String, // TODO: infer type
    pub reference: String, // TODO: infer type
    pub errors: String, // TODO: infer type
    pub line: String, // TODO: infer type
    pub _dict: String, // TODO: infer type
    pub _sections: String, // TODO: infer type
    pub _defaults: String, // TODO: infer type
    pub _converters: String, // TODO: infer type
    pub _proxies: String, // TODO: infer type
    pub _delimiters: String, // TODO: infer type
    pub _optcre: String, // TODO: infer type
    pub _comment_prefixes: String, // TODO: infer type
    pub _inline_comment_prefixes: String, // TODO: infer type
    pub _strict: String, // TODO: infer type
    pub _allow_no_value: String, // TODO: infer type
    pub _empty_lines_in_values: String, // TODO: infer type
    pub default_section: String, // TODO: infer type
    pub _interpolation: String, // TODO: infer type
    pub _parser: String, // TODO: infer type
    pub _name: String, // TODO: infer type
    pub _data: String, // TODO: infer type
}

impl Error {
}

pub struct NoSectionError {
    pub section: String, // TODO: infer type
    pub args: String, // TODO: infer type
    pub source: String, // TODO: infer type
    pub lineno: String, // TODO: infer type
    pub option: String, // TODO: infer type
    pub reference: String, // TODO: infer type
    pub errors: String, // TODO: infer type
    pub line: String, // TODO: infer type
    pub _dict: String, // TODO: infer type
    pub _sections: String, // TODO: infer type
    pub _defaults: String, // TODO: infer type
    pub _converters: String, // TODO: infer type
    pub _proxies: String, // TODO: infer type
    pub _delimiters: String, // TODO: infer type
    pub _optcre: String, // TODO: infer type
    pub _comment_prefixes: String, // TODO: infer type
    pub _inline_comment_prefixes: String, // TODO: infer type
    pub _strict: String, // TODO: infer type
    pub _allow_no_value: String, // TODO: infer type
    pub _empty_lines_in_values: String, // TODO: infer type
    pub default_section: String, // TODO: infer type
    pub _interpolation: String, // TODO: infer type
    pub _parser: String, // TODO: infer type
    pub _name: String, // TODO: infer type
    pub _data: String, // TODO: infer type
}

impl NoSectionError {
}

pub struct DuplicateSectionError {
    pub section: String, // TODO: infer type
    pub source: String, // TODO: infer type
    pub lineno: String, // TODO: infer type
    pub args: String, // TODO: infer type
    pub option: String, // TODO: infer type
    pub reference: String, // TODO: infer type
    pub errors: String, // TODO: infer type
    pub line: String, // TODO: infer type
    pub _dict: String, // TODO: infer type
    pub _sections: String, // TODO: infer type
    pub _defaults: String, // TODO: infer type
    pub _converters: String, // TODO: infer type
    pub _proxies: String, // TODO: infer type
    pub _delimiters: String, // TODO: infer type
    pub _optcre: String, // TODO: infer type
    pub _comment_prefixes: String, // TODO: infer type
    pub _inline_comment_prefixes: String, // TODO: infer type
    pub _strict: String, // TODO: infer type
    pub _allow_no_value: String, // TODO: infer type
    pub _empty_lines_in_values: String, // TODO: infer type
    pub default_section: String, // TODO: infer type
    pub _interpolation: String, // TODO: infer type
    pub _parser: String, // TODO: infer type
    pub _name: String, // TODO: infer type
    pub _data: String, // TODO: infer type
}

impl DuplicateSectionError {
}

pub struct DuplicateOptionError {
    pub section: String, // TODO: infer type
    pub option: String, // TODO: infer type
    pub source: String, // TODO: infer type
    pub lineno: String, // TODO: infer type
    pub args: String, // TODO: infer type
    pub reference: String, // TODO: infer type
    pub errors: String, // TODO: infer type
    pub line: String, // TODO: infer type
    pub _dict: String, // TODO: infer type
    pub _sections: String, // TODO: infer type
    pub _defaults: String, // TODO: infer type
    pub _converters: String, // TODO: infer type
    pub _proxies: String, // TODO: infer type
    pub _delimiters: String, // TODO: infer type
    pub _optcre: String, // TODO: infer type
    pub _comment_prefixes: String, // TODO: infer type
    pub _inline_comment_prefixes: String, // TODO: infer type
    pub _strict: String, // TODO: infer type
    pub _allow_no_value: String, // TODO: infer type
    pub _empty_lines_in_values: String, // TODO: infer type
    pub default_section: String, // TODO: infer type
    pub _interpolation: String, // TODO: infer type
    pub _parser: String, // TODO: infer type
    pub _name: String, // TODO: infer type
    pub _data: String, // TODO: infer type
}

impl DuplicateOptionError {
}

pub struct NoOptionError {
    pub option: String, // TODO: infer type
    pub section: String, // TODO: infer type
    pub args: String, // TODO: infer type
    pub reference: String, // TODO: infer type
    pub source: String, // TODO: infer type
    pub errors: String, // TODO: infer type
    pub lineno: String, // TODO: infer type
    pub line: String, // TODO: infer type
    pub _dict: String, // TODO: infer type
    pub _sections: String, // TODO: infer type
    pub _defaults: String, // TODO: infer type
    pub _converters: String, // TODO: infer type
    pub _proxies: String, // TODO: infer type
    pub _delimiters: String, // TODO: infer type
    pub _optcre: String, // TODO: infer type
    pub _comment_prefixes: String, // TODO: infer type
    pub _inline_comment_prefixes: String, // TODO: infer type
    pub _strict: String, // TODO: infer type
    pub _allow_no_value: String, // TODO: infer type
    pub _empty_lines_in_values: String, // TODO: infer type
    pub default_section: String, // TODO: infer type
    pub _interpolation: String, // TODO: infer type
    pub _parser: String, // TODO: infer type
    pub _name: String, // TODO: infer type
    pub _data: String, // TODO: infer type
}

impl NoOptionError {
}

pub struct InterpolationError {
    pub option: String, // TODO: infer type
    pub section: String, // TODO: infer type
    pub args: String, // TODO: infer type
    pub reference: String, // TODO: infer type
    pub source: String, // TODO: infer type
    pub errors: String, // TODO: infer type
    pub lineno: String, // TODO: infer type
    pub line: String, // TODO: infer type
    pub _dict: String, // TODO: infer type
    pub _sections: String, // TODO: infer type
    pub _defaults: String, // TODO: infer type
    pub _converters: String, // TODO: infer type
    pub _proxies: String, // TODO: infer type
    pub _delimiters: String, // TODO: infer type
    pub _optcre: String, // TODO: infer type
    pub _comment_prefixes: String, // TODO: infer type
    pub _inline_comment_prefixes: String, // TODO: infer type
    pub _strict: String, // TODO: infer type
    pub _allow_no_value: String, // TODO: infer type
    pub _empty_lines_in_values: String, // TODO: infer type
    pub default_section: String, // TODO: infer type
    pub _interpolation: String, // TODO: infer type
    pub _parser: String, // TODO: infer type
    pub _name: String, // TODO: infer type
    pub _data: String, // TODO: infer type
}

impl InterpolationError {
}

pub struct InterpolationMissingOptionError {
    pub reference: String, // TODO: infer type
    pub args: String, // TODO: infer type
    pub source: String, // TODO: infer type
    pub errors: String, // TODO: infer type
    pub lineno: String, // TODO: infer type
    pub line: String, // TODO: infer type
    pub _dict: String, // TODO: infer type
    pub _sections: String, // TODO: infer type
    pub _defaults: String, // TODO: infer type
    pub _converters: String, // TODO: infer type
    pub _proxies: String, // TODO: infer type
    pub _delimiters: String, // TODO: infer type
    pub _optcre: String, // TODO: infer type
    pub _comment_prefixes: String, // TODO: infer type
    pub _inline_comment_prefixes: String, // TODO: infer type
    pub _strict: String, // TODO: infer type
    pub _allow_no_value: String, // TODO: infer type
    pub _empty_lines_in_values: String, // TODO: infer type
    pub default_section: String, // TODO: infer type
    pub _interpolation: String, // TODO: infer type
    pub _parser: String, // TODO: infer type
    pub _name: String, // TODO: infer type
    pub _data: String, // TODO: infer type
}

impl InterpolationMissingOptionError {
}

pub struct InterpolationSyntaxError {
    pub args: String, // TODO: infer type
    pub source: String, // TODO: infer type
    pub errors: String, // TODO: infer type
    pub lineno: String, // TODO: infer type
    pub line: String, // TODO: infer type
    pub _dict: String, // TODO: infer type
    pub _sections: String, // TODO: infer type
    pub _defaults: String, // TODO: infer type
    pub _converters: String, // TODO: infer type
    pub _proxies: String, // TODO: infer type
    pub _delimiters: String, // TODO: infer type
    pub _optcre: String, // TODO: infer type
    pub _comment_prefixes: String, // TODO: infer type
    pub _inline_comment_prefixes: String, // TODO: infer type
    pub _strict: String, // TODO: infer type
    pub _allow_no_value: String, // TODO: infer type
    pub _empty_lines_in_values: String, // TODO: infer type
    pub default_section: String, // TODO: infer type
    pub _interpolation: String, // TODO: infer type
    pub _parser: String, // TODO: infer type
    pub _name: String, // TODO: infer type
    pub _data: String, // TODO: infer type
}

impl InterpolationSyntaxError {
}

pub struct InterpolationDepthError {
    pub args: String, // TODO: infer type
    pub source: String, // TODO: infer type
    pub errors: String, // TODO: infer type
    pub lineno: String, // TODO: infer type
    pub line: String, // TODO: infer type
    pub _dict: String, // TODO: infer type
    pub _sections: String, // TODO: infer type
    pub _defaults: String, // TODO: infer type
    pub _converters: String, // TODO: infer type
    pub _proxies: String, // TODO: infer type
    pub _delimiters: String, // TODO: infer type
    pub _optcre: String, // TODO: infer type
    pub _comment_prefixes: String, // TODO: infer type
    pub _inline_comment_prefixes: String, // TODO: infer type
    pub _strict: String, // TODO: infer type
    pub _allow_no_value: String, // TODO: infer type
    pub _empty_lines_in_values: String, // TODO: infer type
    pub default_section: String, // TODO: infer type
    pub _interpolation: String, // TODO: infer type
    pub _parser: String, // TODO: infer type
    pub _name: String, // TODO: infer type
    pub _data: String, // TODO: infer type
}

impl InterpolationDepthError {
}

pub struct ParsingError {
    pub source: String, // TODO: infer type
    pub errors: String, // TODO: infer type
    pub args: String, // TODO: infer type
    pub lineno: String, // TODO: infer type
    pub line: String, // TODO: infer type
    pub _dict: String, // TODO: infer type
    pub _sections: String, // TODO: infer type
    pub _defaults: String, // TODO: infer type
    pub _converters: String, // TODO: infer type
    pub _proxies: String, // TODO: infer type
    pub _delimiters: String, // TODO: infer type
    pub _optcre: String, // TODO: infer type
    pub _comment_prefixes: String, // TODO: infer type
    pub _inline_comment_prefixes: String, // TODO: infer type
    pub _strict: String, // TODO: infer type
    pub _allow_no_value: String, // TODO: infer type
    pub _empty_lines_in_values: String, // TODO: infer type
    pub default_section: String, // TODO: infer type
    pub _interpolation: String, // TODO: infer type
    pub _parser: String, // TODO: infer type
    pub _name: String, // TODO: infer type
    pub _data: String, // TODO: infer type
}

impl ParsingError {
}

pub struct MissingSectionHeaderError {
    pub source: String, // TODO: infer type
    pub lineno: String, // TODO: infer type
    pub line: String, // TODO: infer type
    pub args: String, // TODO: infer type
    pub _dict: String, // TODO: infer type
    pub _sections: String, // TODO: infer type
    pub _defaults: String, // TODO: infer type
    pub _converters: String, // TODO: infer type
    pub _proxies: String, // TODO: infer type
    pub _delimiters: String, // TODO: infer type
    pub _optcre: String, // TODO: infer type
    pub _comment_prefixes: String, // TODO: infer type
    pub _inline_comment_prefixes: String, // TODO: infer type
    pub _strict: String, // TODO: infer type
    pub _allow_no_value: String, // TODO: infer type
    pub _empty_lines_in_values: String, // TODO: infer type
    pub default_section: String, // TODO: infer type
    pub _interpolation: String, // TODO: infer type
    pub _parser: String, // TODO: infer type
    pub _name: String, // TODO: infer type
    pub _data: String, // TODO: infer type
}

impl MissingSectionHeaderError {
}

pub const _UNSET: f64 = object ( );
pub struct Interpolation {
    pub _dict: String, // TODO: infer type
    pub _sections: String, // TODO: infer type
    pub _defaults: String, // TODO: infer type
    pub _converters: String, // TODO: infer type
    pub _proxies: String, // TODO: infer type
    pub _delimiters: String, // TODO: infer type
    pub _optcre: String, // TODO: infer type
    pub _comment_prefixes: String, // TODO: infer type
    pub _inline_comment_prefixes: String, // TODO: infer type
    pub _strict: String, // TODO: infer type
    pub _allow_no_value: String, // TODO: infer type
    pub _empty_lines_in_values: String, // TODO: infer type
    pub default_section: String, // TODO: infer type
    pub _interpolation: String, // TODO: infer type
    pub _parser: String, // TODO: infer type
    pub _name: String, // TODO: infer type
    pub _data: String, // TODO: infer type
}

impl Interpolation {
}

pub struct BasicInterpolation {
    pub _dict: String, // TODO: infer type
    pub _sections: String, // TODO: infer type
    pub _defaults: String, // TODO: infer type
    pub _converters: String, // TODO: infer type
    pub _proxies: String, // TODO: infer type
    pub _delimiters: String, // TODO: infer type
    pub _optcre: String, // TODO: infer type
    pub _comment_prefixes: String, // TODO: infer type
    pub _inline_comment_prefixes: String, // TODO: infer type
    pub _strict: String, // TODO: infer type
    pub _allow_no_value: String, // TODO: infer type
    pub _empty_lines_in_values: String, // TODO: infer type
    pub default_section: String, // TODO: infer type
    pub _interpolation: String, // TODO: infer type
    pub _parser: String, // TODO: infer type
    pub _name: String, // TODO: infer type
    pub _data: String, // TODO: infer type
}

impl BasicInterpolation {
}

pub struct ExtendedInterpolation {
    pub _dict: String, // TODO: infer type
    pub _sections: String, // TODO: infer type
    pub _defaults: String, // TODO: infer type
    pub _converters: String, // TODO: infer type
    pub _proxies: String, // TODO: infer type
    pub _delimiters: String, // TODO: infer type
    pub _optcre: String, // TODO: infer type
    pub _comment_prefixes: String, // TODO: infer type
    pub _inline_comment_prefixes: String, // TODO: infer type
    pub _strict: String, // TODO: infer type
    pub _allow_no_value: String, // TODO: infer type
    pub _empty_lines_in_values: String, // TODO: infer type
    pub default_section: String, // TODO: infer type
    pub _interpolation: String, // TODO: infer type
    pub _parser: String, // TODO: infer type
    pub _name: String, // TODO: infer type
    pub _data: String, // TODO: infer type
}

impl ExtendedInterpolation {
}

pub struct LegacyInterpolation {
    pub _dict: String, // TODO: infer type
    pub _sections: String, // TODO: infer type
    pub _defaults: String, // TODO: infer type
    pub _converters: String, // TODO: infer type
    pub _proxies: String, // TODO: infer type
    pub _delimiters: String, // TODO: infer type
    pub _optcre: String, // TODO: infer type
    pub _comment_prefixes: String, // TODO: infer type
    pub _inline_comment_prefixes: String, // TODO: infer type
    pub _strict: String, // TODO: infer type
    pub _allow_no_value: String, // TODO: infer type
    pub _empty_lines_in_values: String, // TODO: infer type
    pub default_section: String, // TODO: infer type
    pub _interpolation: String, // TODO: infer type
    pub _parser: String, // TODO: infer type
    pub _name: String, // TODO: infer type
    pub _data: String, // TODO: infer type
}

impl LegacyInterpolation {
}

pub struct RawConfigParser {
    pub _dict: String, // TODO: infer type
    pub _sections: String, // TODO: infer type
    pub _defaults: String, // TODO: infer type
    pub _converters: String, // TODO: infer type
    pub _proxies: String, // TODO: infer type
    pub _delimiters: String, // TODO: infer type
    pub _optcre: String, // TODO: infer type
    pub _comment_prefixes: String, // TODO: infer type
    pub _inline_comment_prefixes: String, // TODO: infer type
    pub _strict: String, // TODO: infer type
    pub _allow_no_value: String, // TODO: infer type
    pub _empty_lines_in_values: String, // TODO: infer type
    pub default_section: String, // TODO: infer type
    pub _interpolation: String, // TODO: infer type
    pub _parser: String, // TODO: infer type
    pub _name: String, // TODO: infer type
    pub _data: String, // TODO: infer type
}

impl RawConfigParser {
}

pub struct ConfigParser {
    pub _interpolation: String, // TODO: infer type
    pub _parser: String, // TODO: infer type
    pub _name: String, // TODO: infer type
    pub _data: String, // TODO: infer type
}

impl ConfigParser {
}

pub struct SafeConfigParser {
    pub _parser: String, // TODO: infer type
    pub _name: String, // TODO: infer type
    pub _data: String, // TODO: infer type
}

impl SafeConfigParser {
}

pub struct SectionProxy {
    pub _parser: String, // TODO: infer type
    pub _name: String, // TODO: infer type
    pub _data: String, // TODO: infer type
}

impl SectionProxy {
}

pub struct ConverterMapping {
    pub _parser: String, // TODO: infer type
    pub _data: String, // TODO: infer type
}

impl ConverterMapping {
}

