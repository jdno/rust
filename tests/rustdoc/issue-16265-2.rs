#![crate_name="issue_16265_2"]

// @hasraw issue_16265_2/index.html 'source'

trait Y {}
impl Y for Option<u32> {}
