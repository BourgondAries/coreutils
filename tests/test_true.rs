use common::util::*;

static UTIL_NAME: &'static str = "true";

#[test]
fn test_exit_code() {
    let (_, mut ucmd) = testing(UTIL_NAME);
    let exit_status = ucmd.run().success;
    assert_eq!(exit_status, true);
}
