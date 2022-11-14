use super::util::helpers::find_file_or_readme_ref;
use crate::linter::{
    check::{CheckId, CheckInput, CheckOutput},
    CheckSet,
};
use anyhow::Result;
use lazy_static::lazy_static;
use regex::RegexSet;

/// Check identifier.
pub(crate) const ID: CheckId = "roadmap";

/// Check score weight.
pub(crate) const WEIGHT: usize = 1;

/// Check sets this check belongs to.
pub(crate) const CHECK_SETS: [CheckSet; 1] = [CheckSet::Community];

/// Patterns used to locate a file in the repository.
const FILE_PATTERNS: [&str; 1] = ["roadmap*"];

lazy_static! {
    #[rustfmt::skip]
    static ref README_REF: RegexSet = RegexSet::new(vec![
        r"(?im)^#+.*roadmap.*$",
        r"(?im)^roadmap$",
        r"(?i)\[.*roadmap.*\]\(.*\)",
    ]).expect("exprs in README_REF to be valid");
}

/// Check main function.
pub(crate) fn check(input: &CheckInput) -> Result<CheckOutput> {
    // File in repo or reference in README file
    find_file_or_readme_ref(input, &FILE_PATTERNS, &README_REF)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_ref_match() {
        assert!(README_REF.is_match("# Roadmap"));
        assert!(README_REF.is_match(
            r"
...
## Project roadmap and others
...
            "
        ));
        assert!(README_REF.is_match(
            r"
...
Roadmap
-------
...
            "
        ));
        assert!(README_REF.is_match("[Project roadmap](...)"));
    }
}
