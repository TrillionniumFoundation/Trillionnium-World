//! Strict endpoint and bounded-file policy for the configuration preflight.
//!
//! This is a deliberately restricted HTTPS base-URL grammar, not a general
//! URL parser. No DNS is performed. Runtime connectors still MUST apply the
//! approved egress policy to every resolved/connected address and deny redirects.
//! Filesystem checks are point-in-time diagnostics, not a secure-open capability
//! for the eventual service or evidence writer.

use std::fs::{self, File};
use std::io::Read;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::path::{Component, Path};

pub const ENDPOINT_POLICY: &str = "trillionnium_world_endpoint_intake_v1";

fn valid_port(suffix: &str) -> Result<(), &'static str> {
    if suffix.is_empty() {
        return Ok(());
    }
    let text = suffix.strip_prefix(':').ok_or("invalid port delimiter")?;
    if text.is_empty()
        || text.len() > 5
        || (text.len() > 1 && text.starts_with('0'))
        || !text.bytes().all(|b| b.is_ascii_digit())
        || text.parse::<u16>().ok().filter(|port| *port != 0).is_none()
    {
        return Err("port must be canonical decimal in 1..=65535");
    }
    Ok(())
}

fn blocked_ipv4(ip: Ipv4Addr) -> bool {
    ip.is_loopback() || ip.is_unspecified() || ip.is_multicast() || ip.is_broadcast()
}

pub fn endpoint_host(value: &str) -> Result<String, &'static str> {
    if value.is_empty()
        || value.len() > 512
        || !value.is_ascii()
        || value.bytes().any(|b| b.is_ascii_whitespace() || b.is_ascii_control())
        || value.chars().any(|c| matches!(c, '\\' | '?' | '#' | '%' | '@'))
    {
        return Err("endpoint must be bounded ASCII without credentials, escapes, query or fragment");
    }
    let rest = value.strip_prefix("https://").ok_or("endpoint must use https://")?;
    let (authority, path) = rest.split_once('/').map_or((rest, ""), |(a, p)| (a, p));
    // A normalized API prefix is permitted; encoded separators and dot segments
    // are not. No downstream URL implementation gets a chance to reinterpret it.
    if !path.bytes().all(|b| b.is_ascii_alphanumeric() || b"-._~/".contains(&b))
        || path.split('/').any(|segment| matches!(segment, "." | ".."))
    {
        return Err("invalid API path prefix");
    }
    if authority.is_empty() {
        return Err("empty endpoint authority");
    }
    if let Some(rest) = authority.strip_prefix('[') {
        let (literal, suffix) = rest.split_once(']').ok_or("unclosed IPv6 literal")?;
        valid_port(suffix)?;
        let ip: Ipv6Addr = literal.parse().map_err(|_| "invalid IPv6 literal")?;
        if ip.is_loopback() || ip.is_unspecified() || ip.is_multicast() {
            return Err("loopback, wildcard and multicast are forbidden");
        }
        if ip.to_ipv4().is_some_and(blocked_ipv4) {
            return Err("embedded IPv4 loopback or non-unicast address is forbidden");
        }
        return Ok(ip.to_string());
    }
    let (host, suffix) = authority.find(':').map_or((authority, ""), |i| {
        (&authority[..i], &authority[i..])
    });
    valid_port(suffix)?;
    if let Ok(ip) = host.parse::<Ipv4Addr>() {
        if blocked_ipv4(ip) {
            return Err("loopback, wildcard and non-unicast IPv4 are forbidden");
        }
        return Ok(ip.to_string());
    }
    let lower = host.to_ascii_lowercase();
    // Reject legacy numeric IPv4 spellings (one-integer, short, octal or hex)
    // rather than trying to normalize them differently from an HTTP client.
    let last_label = lower.rsplit('.').next().unwrap_or_default();
    let numeric_tail = !last_label.is_empty()
        && (last_label.bytes().all(|b| b.is_ascii_digit())
            || last_label.strip_prefix("0x").is_some_and(|s| {
                !s.is_empty() && s.bytes().all(|b| b.is_ascii_hexdigit())
            }));
    if lower.is_empty()
        || lower.len() > 253
        || numeric_tail
        || lower == "localhost"
        || lower.ends_with(".localhost")
        || lower.split('.').any(|label| {
            label.is_empty()
                || label.len() > 63
                || label.starts_with('-')
                || label.ends_with('-')
                || !label.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        })
    {
        return Err("invalid DNS host or forbidden numeric/localhost alias");
    }
    Ok(lower)
}

pub fn ordinary_absolute_path(path: &Path, directory: bool) -> Result<fs::Metadata, &'static str> {
    if !path.is_absolute() {
        return Err("path must be absolute");
    }
    let mut current = std::path::PathBuf::new();
    for component in path.components() {
        if matches!(component, Component::ParentDir | Component::CurDir) {
            return Err("dot path components are forbidden");
        }
        current.push(component.as_os_str());
        let metadata = fs::symlink_metadata(&current).map_err(|_| "path cannot be inspected")?;
        if metadata.file_type().is_symlink() {
            return Err("symlink path components are forbidden");
        }
    }
    let metadata = fs::symlink_metadata(path).map_err(|_| "path cannot be inspected")?;
    if (directory && !metadata.is_dir()) || (!directory && !metadata.is_file()) {
        return Err("path has an unexpected file type");
    }
    Ok(metadata)
}

pub fn read_bounded_config(path: &Path, limit: u64) -> Result<Vec<u8>, &'static str> {
    let before = ordinary_absolute_path(path, false)?;
    if before.len() == 0 || before.len() > limit {
        return Err("config is empty or exceeds byte budget");
    }
    let file = File::open(path).map_err(|_| "config cannot be opened")?;
    let opened = file.metadata().map_err(|_| "opened config cannot be inspected")?;
    if !opened.is_file() {
        return Err("opened config is not a regular file");
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if before.dev() != opened.dev() || before.ino() != opened.ino() {
            return Err("config changed while opening");
        }
    }
    let mut bytes = Vec::new();
    let cap = limit.checked_add(1).ok_or("invalid config budget")?;
    file.take(cap).read_to_end(&mut bytes).map_err(|_| "config read failed")?;
    if bytes.is_empty() || bytes.len() as u64 > limit {
        return Err("config is empty or exceeds byte budget");
    }
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_url_parser_differentials_and_non_unicast_destinations() {
        for value in [
            "https://[::]", "https://[::1]", "https://[0:0:0:0:0:0:0:1]",
            "https://[::ffff:127.0.0.1]", "https://[::127.0.0.1]",
            "https://[::ffff:0.0.0.0]", "https://[ff02::1]",
            "https://127.0.0.1", "https://0.0.0.0", "https://224.0.0.1",
            "https://255.255.255.255", "https://2130706433", "https://127.1",
            "https://0177.0.0.1", "https://0x7f000001", "https://0x7f.1",
            "https://localhost", "https://LOCALHOST", "https://api.localhost",
            "https://localhost.", "https://user:secret@api.example.com",
            "https://api.example.com?token=x", "https://api.example.com#x",
            "https://api.example.com\\@127.0.0.1", "https://api.example.com:0",
            "https://api.example.com:65536", "https://api.example.com:443x",
            "https://api.example.com:0443", "https://api.example.com:",
            "https://[2001:db8::1]junk", "https://[2001:db8::1]:",
            "https://2001:db8::1", "https://api..example.com", "https://-bad.example",
            "https://bad-.example", "https://api.example.com/%2e%2e/private",
            "https://api.example.com/../private", "https://api.example.com/./v1",
            "https://api.example.com/a b", "https://", "http://api.example.com",
        ] {
            assert!(endpoint_host(value).is_err(), "unexpected acceptance: {value}");
        }
    }

    #[test]
    fn normalizes_hosts_without_confusing_authority_and_port() {
        for (value, expected) in [
            ("https://NAKAMA.example.com:443/api/v1", "nakama.example.com"),
            ("https://nakama.svc.cluster.local/", "nakama.svc.cluster.local"),
            ("https://192.0.2.4:8443", "192.0.2.4"),
            ("https://[2001:db8::1]:8443/v1", "2001:db8::1"),
            ("https://[2001:0db8:0:0:0:0:0:1]/v1", "2001:db8::1"),
        ] {
            assert_eq!(endpoint_host(value).unwrap(), expected);
        }
    }

    #[test]
    fn host_aliases_do_not_bypass_service_separation() {
        assert_eq!(endpoint_host("https://API.example.com:443"), endpoint_host("https://api.example.com:8443"));
        assert_eq!(endpoint_host("https://[2001:db8::1]"), endpoint_host("https://[2001:0db8:0:0:0:0:0:1]"));
    }

    #[test]
    fn enforces_size_before_parsing_and_does_not_return_private_paths() {
        assert!(endpoint_host(&format!("https://{}", "a".repeat(513))).is_err());
        let error = read_bounded_config(Path::new("relative-secret.json"), 1024).unwrap_err();
        assert!(!error.contains("relative-secret"));
    }

    #[cfg(unix)]
    #[test]
    fn ancestor_symlinks_are_rejected_and_config_reads_are_bounded() {
        use std::os::unix::fs::symlink;
        use std::time::{SystemTime, UNIX_EPOCH};
        let nonce = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
        let dir = std::env::temp_dir().join(format!("world-preflight-{}-{nonce}", std::process::id()));
        fs::create_dir(&dir).unwrap();
        let real = dir.join("real");
        fs::create_dir(&real).unwrap();
        let file = real.join("config.json");
        fs::write(&file, b"{}").unwrap();
        symlink(&real, dir.join("alias")).unwrap();
        assert!(ordinary_absolute_path(&dir.join("alias/config.json"), false).is_err());
        assert_eq!(read_bounded_config(&file, 2).unwrap(), b"{}");
        assert!(read_bounded_config(&file, 1).is_err());
        fs::write(&file, b"").unwrap();
        assert!(read_bounded_config(&file, 2).is_err());
        fs::remove_dir_all(&dir).unwrap();
    }
}
