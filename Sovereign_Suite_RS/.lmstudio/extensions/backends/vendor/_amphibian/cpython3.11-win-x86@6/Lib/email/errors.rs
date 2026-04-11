//! errors.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz


pub struct MessageError {
    pub line: String, // TODO: infer type
    pub non_printables: String, // TODO: infer type
}

impl MessageError {
}

pub struct MessageParseError {
    pub line: String, // TODO: infer type
    pub non_printables: String, // TODO: infer type
}

impl MessageParseError {
}

pub struct HeaderParseError {
    pub line: String, // TODO: infer type
    pub non_printables: String, // TODO: infer type
}

impl HeaderParseError {
}

pub struct BoundaryError {
    pub line: String, // TODO: infer type
    pub non_printables: String, // TODO: infer type
}

impl BoundaryError {
}

pub struct MultipartConversionError {
    pub line: String, // TODO: infer type
    pub non_printables: String, // TODO: infer type
}

impl MultipartConversionError {
}

pub struct CharsetError {
    pub line: String, // TODO: infer type
    pub non_printables: String, // TODO: infer type
}

impl CharsetError {
}

pub struct MessageDefect {
    pub line: String, // TODO: infer type
    pub non_printables: String, // TODO: infer type
}

impl MessageDefect {
}

pub struct NoBoundaryInMultipartDefect {
    pub non_printables: String, // TODO: infer type
}

impl NoBoundaryInMultipartDefect {
}

pub struct StartBoundaryNotFoundDefect {
    pub non_printables: String, // TODO: infer type
}

impl StartBoundaryNotFoundDefect {
}

pub struct CloseBoundaryNotFoundDefect {
    pub non_printables: String, // TODO: infer type
}

impl CloseBoundaryNotFoundDefect {
}

pub struct FirstHeaderLineIsContinuationDefect {
    pub non_printables: String, // TODO: infer type
}

impl FirstHeaderLineIsContinuationDefect {
}

pub struct MisplacedEnvelopeHeaderDefect {
    pub non_printables: String, // TODO: infer type
}

impl MisplacedEnvelopeHeaderDefect {
}

pub struct MissingHeaderBodySeparatorDefect {
    pub non_printables: String, // TODO: infer type
}

impl MissingHeaderBodySeparatorDefect {
}

pub const MalformedHeaderDefect: f64 = MissingHeaderBodySeparatorDefect;
pub struct MultipartInvariantViolationDefect {
    pub non_printables: String, // TODO: infer type
}

impl MultipartInvariantViolationDefect {
}

pub struct InvalidMultipartContentTransferEncodingDefect {
    pub non_printables: String, // TODO: infer type
}

impl InvalidMultipartContentTransferEncodingDefect {
}

pub struct UndecodableBytesDefect {
    pub non_printables: String, // TODO: infer type
}

impl UndecodableBytesDefect {
}

pub struct InvalidBase64PaddingDefect {
    pub non_printables: String, // TODO: infer type
}

impl InvalidBase64PaddingDefect {
}

pub struct InvalidBase64CharactersDefect {
    pub non_printables: String, // TODO: infer type
}

impl InvalidBase64CharactersDefect {
}

pub struct InvalidBase64LengthDefect {
    pub non_printables: String, // TODO: infer type
}

impl InvalidBase64LengthDefect {
}

pub struct HeaderDefect {
    pub non_printables: String, // TODO: infer type
}

impl HeaderDefect {
}

pub struct InvalidHeaderDefect {
    pub non_printables: String, // TODO: infer type
}

impl InvalidHeaderDefect {
}

pub struct HeaderMissingRequiredValue {
    pub non_printables: String, // TODO: infer type
}

impl HeaderMissingRequiredValue {
}

pub struct NonPrintableDefect {
    pub non_printables: String, // TODO: infer type
}

impl NonPrintableDefect {
}

pub struct ObsoleteHeaderDefect {
}

impl ObsoleteHeaderDefect {
}

pub struct NonASCIILocalPartDefect {
}

impl NonASCIILocalPartDefect {
}

pub struct InvalidDateDefect {
}

impl InvalidDateDefect {
}

