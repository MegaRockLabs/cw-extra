mod tests;
mod msg;

pub use cw81_derive::valid_signature_query;
pub use msg::{ValidSignature, ValidSignatureResponse, ValidSignaturesResponse};