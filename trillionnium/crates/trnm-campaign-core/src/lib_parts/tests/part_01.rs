#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib_parts/tests/campaign_tests_01.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib_parts/tests/campaign_tests_02.rs"));
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib_parts/tests/campaign_tests_03.rs"));
}
