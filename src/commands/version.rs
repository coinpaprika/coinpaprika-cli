use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::output::OutputFormat;

const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const GITHUB_RELEASES_URL: &str =
    "https://api.github.com/repos/coinpaprika/coinpaprika-cli/releases/latest";

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
}

#[derive(Debug, Serialize)]
pub struct VersionCheck {
    pub current: String,
    pub latest: String,
    pub up_to_date: bool,
}

/// Parse a version string into (major, minor, patch).
///
/// Accepts an optional leading 'v' and one to three numeric dot-separated
/// components; missing components default to 0. Returns None for anything
/// else (non-numeric parts, more than three components, empty input).
fn parse_version(version: &str) -> Option<(u64, u64, u64)> {
    let version = version.trim().trim_start_matches('v');
    let mut parts = version.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = match parts.next() {
        Some(p) => p.parse().ok()?,
        None => 0,
    };
    let patch = match parts.next() {
        Some(p) => p.parse().ok()?,
        None => 0,
    };
    if parts.next().is_some() {
        return None;
    }
    Some((major, minor, patch))
}

/// Returns true when `remote` is strictly newer than `current`.
///
/// Compares numerically so that a stale remote release (for example a GitHub
/// release that lags behind the installed binary) is never reported as an
/// update. If either side does not parse as a version, falls back to plain
/// string inequality after stripping any leading 'v', so an unrecognized
/// remote tag still surfaces as an available update instead of panicking.
fn is_newer(remote: &str, current: &str) -> bool {
    match (parse_version(remote), parse_version(current)) {
        (Some(r), Some(c)) => r > c,
        _ => remote.trim().trim_start_matches('v') != current.trim().trim_start_matches('v'),
    }
}

pub async fn execute(output: OutputFormat, raw: bool) -> Result<()> {
    let client = reqwest::Client::builder()
        .user_agent(format!(
            "coinpaprika-cli/{} ({}/{})",
            CURRENT_VERSION,
            std::env::consts::OS,
            std::env::consts::ARCH,
        ))
        .build()?;

    eprintln!("Checking for updates...");

    let result = client.get(GITHUB_RELEASES_URL).send().await;

    let check = match result {
        Ok(resp) if resp.status().is_success() => {
            let release: GitHubRelease = resp.json().await?;
            let latest = release.tag_name.trim_start_matches('v').to_string();
            let up_to_date = !is_newer(&latest, CURRENT_VERSION);
            VersionCheck {
                current: CURRENT_VERSION.to_string(),
                latest,
                up_to_date,
            }
        }
        _ => {
            VersionCheck {
                current: CURRENT_VERSION.to_string(),
                latest: CURRENT_VERSION.to_string(),
                up_to_date: true,
            }
        }
    };

    match output {
        OutputFormat::Table => {
            if check.up_to_date {
                println!("coinpaprika-cli v{} — up to date.", check.current);
            } else {
                println!(
                    "coinpaprika-cli v{} — update available: v{}\n\n  \
                     Update:  cargo install coinpaprika-cli\n  \
                     Release: https://github.com/coinpaprika/coinpaprika-cli/releases/latest",
                    check.current, check.latest
                );
            }
        }
        OutputFormat::Json => {
            crate::output::print_json_wrapped(
                &check,
                crate::output::ResponseMeta::coinpaprika("/version"),
                raw,
            )?;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_versions_are_up_to_date() {
        assert!(!is_newer("0.1.7", "0.1.7"));
        assert!(!is_newer("1.0.0", "1.0.0"));
    }

    #[test]
    fn remote_newer_is_an_update() {
        assert!(is_newer("0.1.8", "0.1.7"));
        assert!(is_newer("0.2.0", "0.1.9"));
        assert!(is_newer("1.0.0", "0.9.9"));
        // numeric compare, not lexicographic
        assert!(is_newer("0.10.0", "0.9.0"));
    }

    #[test]
    fn remote_older_is_not_an_update() {
        // regression: a stale GitHub release must not prompt a downgrade
        assert!(!is_newer("0.1.5", "0.1.7"));
        assert!(!is_newer("0.1.7", "0.2.0"));
        assert!(!is_newer("0.9.0", "0.10.0"));
    }

    #[test]
    fn v_prefix_is_ignored_on_either_side() {
        assert!(!is_newer("v0.1.7", "0.1.7"));
        assert!(!is_newer("0.1.7", "v0.1.7"));
        assert!(is_newer("v0.1.8", "0.1.7"));
        assert!(!is_newer("v0.1.5", "v0.1.7"));
    }

    #[test]
    fn short_versions_default_missing_components_to_zero() {
        assert!(is_newer("0.2", "0.1.7"));
        assert!(!is_newer("0.1", "0.1.0"));
        assert!(is_newer("1", "0.9.9"));
    }

    #[test]
    fn malformed_versions_fall_back_to_inequality_without_panicking() {
        // unparseable remote that differs from current: report an update
        assert!(is_newer("nightly", "0.1.7"));
        assert!(is_newer("0.1.x", "0.1.7"));
        assert!(is_newer("1.2.3.4", "0.1.7"));
        assert!(is_newer("", "0.1.7"));
        // identical unparseable strings: up to date
        assert!(!is_newer("nightly", "nightly"));
    }

    #[test]
    fn parse_version_handles_expected_shapes() {
        assert_eq!(parse_version("0.1.7"), Some((0, 1, 7)));
        assert_eq!(parse_version("v0.1.7"), Some((0, 1, 7)));
        assert_eq!(parse_version("0.1"), Some((0, 1, 0)));
        assert_eq!(parse_version("2"), Some((2, 0, 0)));
        assert_eq!(parse_version(""), None);
        assert_eq!(parse_version("a.b.c"), None);
        assert_eq!(parse_version("1.2.3.4"), None);
    }
}
