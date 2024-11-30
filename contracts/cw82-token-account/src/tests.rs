#[cfg(test)]
mod tests {
    use cosmwasm_std::{testing::mock_dependencies , to_json_binary, Binary
    };
    use crate::query::verify_arbitrary;

    const MSG: &str = "dGVzdA==";
    const PUBKEY: &str = "A2LjUH7Q0gi7+Wi0/MnXMZqN8slsz7iHMfTWp8xUXspH";
    const SIGNATURE: &str = "6UDr+Cu5+6SAgbMRj3hQfXZecdpxsmznLfTMcWkXPDl1DBJRNg+XrFal3BqF8TWJ+o9KM8+z5sfZZ1hfUPkSbg==";
    const ACCOUNT : &str = "stars1v85m4sxnndwmswtd8jrz3cd2m8u8eegqdxyluz";


    #[test]
    fn amino_check() {

        let deps = mock_dependencies();

        let ok = verify_arbitrary(
            deps.as_ref(),
            ACCOUNT,
            to_json_binary(MSG).unwrap(),
            Binary::from_base64(SIGNATURE).unwrap(),
            Binary::from_base64(PUBKEY).unwrap().as_slice(),
        ).unwrap();
        assert!(ok);
    }


}