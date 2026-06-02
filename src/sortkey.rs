//! Encodes an EVR (epoch, version, release) as a memcmp-sortable binary key.
//! The returned `Vec<u8>` produces correct RPM version ordering when compared
//! byte-by-byte (memcmp-style), matching the semantics of `rpmvercmp()`.
//!
//! Unlike the `Evr` struct which uses a traditional procedural approach to
//! determining the order, this approach produces a single value that can be
//! utilized to order RPM versions from any programming language or database that
//! can use memcmp semantics on byte array values.
//!
//! It also tends to be faster in e.g. Python because it can completely avoid FFI
//! overhead.
//!
//! The key is structured as: [4-byte epoch] [version sortkey] [release sortkey]
//!
//! Epoch is encoded as a 4-byte big-endian unsigned integer. Because memcmp
//! compares left-to-right, the epoch bytes are always compared first, and a
//! higher epoch always dominates regardless of version/release — matching RPM
//! semantics. NULL epochs are coalesced to 0.
//!
//! Only ASCII alphanumeric characters participate in comparison - non-ASCII
//! characters and non-alphanumeric ASCII characters (except ~ and ^) are
//! treated as segment separators and skipped (matching RPM).
//!
//! Each version/release sortkey uses six byte markers, chosen so their numeric
//! order matches the required RPM sort order:
//!
//!   \x00  — segment delimiter (terminates alpha and numeric segments)
//!   \x01  — tilde (~), sorts lowest, before end-of-version
//!   \x02  — end-of-version (appended once at the end of the key)
//!   \x03  — caret (^), sorts after end but before real segments
//!   \x04  — alpha segment prefix (followed by the segment text + \x00 delimiter)
//!   \x05  — numeric segment prefix (followed by length byte + digits + \x00)
//!
//! This order directly encodes the RPM rule: ~ < end < ^ < alpha < numeric.
//!
//! Segment encoding details:
//!
//!   Alpha segments: \x04 + raw ASCII text + \x00
//!     Lexicographic byte comparison gives correct alphabetical ordering.
//!
//!   Numeric segments: \x05 + length_byte + stripped_digits + \x00
//!     Leading zeros are stripped. The length byte sorts first, so longer digit
//!     strings (= larger numbers) always sort after shorter ones. Within the
//!     same length, digit characters compare lexicographically, which is correct
//!     for same-length decimal integers.
//!
//! Each version/release component ends with \x02 (end-of-version). When two
//! versions are equal, the comparison naturally falls through the end marker
//! into the release bytes. This is why no separator is needed between the
//! version and release sortkeys — the \x02 end marker serves as a delimiter.
//!
//! Example: EVR "2:1.0~rc1-3.fc40" encodes as:
//!
//!   epoch (4-byte big-endian):
//!     \x00 \x00 \x00 \x02            — epoch 2
//!
//!   version sortkey ("1.0~rc1"):
//!     \x05 \x01 1 \x00               — numeric segment "1"
//!     \x05 \x00 \x00                  — numeric segment "0" (leading zeros stripped, length=0)
//!     \x01                            — tilde
//!     \x04 rc \x00                    — alpha segment "rc"
//!     \x05 \x01 1 \x00               — numeric segment "1"
//!     \x02                            — end-of-version
//!
//!   release sortkey ("3.fc40"):
//!     \x05 \x01 3 \x00               — numeric segment "3"
//!     \x04 fc \x00                    — alpha segment "fc"
//!     \x05 \x02 40 \x00              — numeric segment "40"
//!     \x02                            — end-of-version
//!
//! Comparing "1.0~rc1" vs "1.0": after the shared prefix "1.0", the first key
//! has \x01 (tilde) while the second has \x02 (end). Since \x01 < \x02,
//! "1.0~rc1" correctly sorts before "1.0".

use crate::Evr;

/// Encode an EVR (epoch, version, release) as a memcmp-sortable binary key.
///
/// The returned `Vec<u8>` produces correct RPM version ordering when compared
/// byte-by-byte, matching the semantics of `rpmvercmp()`.
///
/// This property is extremely useful, because it means that the value can be stored
/// in a database and used to perform in-database RPM version comparisons, provided
/// the database is capable of memcmp-style order comparisons across collections
/// of raw bytes.
///
/// The key layout is: `[4-byte big-endian epoch] [version sortkey] [release sortkey]`
///
/// Empty or unparseable epochs are treated as 0, matching RPM semantics.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EvrSortKey(Vec<u8>);

impl EvrSortKey {
    /// Create a sortkey from individual epoch, version, and release strings.
    pub fn from_values(epoch: &str, version: &str, release: &str) -> Self {
        let epoch_int: u32 = if epoch.is_empty() {
            0
        } else {
            epoch.parse().unwrap_or(0)
        };

        let ver_key = version_sortkey(version);
        let rel_key = version_sortkey(release);

        let mut result = Vec::with_capacity(4 + ver_key.len() + rel_key.len());
        result.extend_from_slice(&epoch_int.to_be_bytes());
        result.extend(ver_key);
        result.extend(rel_key);
        EvrSortKey(result)
    }

    /// Parse an EVR string (e.g. `"1:2.3.4-5"`) and create a sortkey.
    pub fn parse(evr: &str) -> Self {
        let (epoch, version, release) = Evr::parse_values(evr);
        Self::from_values(epoch, version, release)
    }

    /// Return a reference to the underlying bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }
}

impl From<Vec<u8>> for EvrSortKey {
    fn from(v: Vec<u8>) -> Self {
        EvrSortKey(v)
    }
}

impl From<EvrSortKey> for Vec<u8> {
    fn from(k: EvrSortKey) -> Self {
        k.0
    }
}

impl AsRef<[u8]> for EvrSortKey {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// Encode a version or release string as a memcmp-sortable binary key.
///
/// The returned `Vec<u8>` can be compared with standard byte comparison
/// (`memcmp` / `Ord for [u8]`) to produce correct RPM version ordering.
///
/// See [`evr_sortkey`] for the full EVR encoding.
pub fn version_sortkey(ver: &str) -> Vec<u8> {
    if ver.is_empty() {
        return vec![0x02];
    }

    let bytes = ver.as_bytes();
    let len = bytes.len();
    let mut result = Vec::with_capacity(len * 2);
    let mut pos = 0;

    while pos < len {
        // Skip non-alphanumeric, non-tilde, non-caret (ASCII-only)
        while pos < len {
            let b = bytes[pos];
            if b.is_ascii_alphanumeric() || b == b'~' || b == b'^' {
                break;
            }
            pos += 1;
        }

        if pos >= len {
            break;
        }

        let b = bytes[pos];

        if b == b'~' {
            result.push(0x01);
            pos += 1;
            continue;
        }

        if b == b'^' {
            result.push(0x03);
            pos += 1;
            continue;
        }

        if b.is_ascii_digit() {
            let start = pos;
            while pos < len && bytes[pos].is_ascii_digit() {
                pos += 1;
            }
            let segment = &bytes[start..pos];
            let stripped = segment
                .iter()
                .position(|&c| c != b'0')
                .map(|i| &segment[i..])
                .unwrap_or(b"");
            result.push(0x05);
            result.push(stripped.len() as u8);
            result.extend_from_slice(stripped);
            result.push(0x00);
        } else {
            let start = pos;
            while pos < len && bytes[pos].is_ascii_alphabetic() {
                pos += 1;
            }
            result.push(0x04);
            result.extend_from_slice(&bytes[start..pos]);
            result.push(0x00);
        }
    }

    result.push(0x02);
    result
}
