use rpm_version::{Evr, EvrSortKey};

fn main() {
    // Create sort keys from individual components
    let key_a = EvrSortKey::from_values("1", "2.0", "3.fc40");
    let key_b = EvrSortKey::from_values("1", "3.0", "1.fc40");
    assert!(key_a < key_b);
    println!("{key_a:?} < {key_b:?}");

    // Or parse from an EVR string
    let key_c = EvrSortKey::parse("1:2.0-3.fc40");
    assert_eq!(key_a, key_c);
    println!(
        "from_values and parse produce identical keys: {}",
        key_a == key_c
    );

    // Sort keys encode RPM version ordering into raw bytes, so standard
    // byte comparison produces correct results — useful for database indexes.
    // So long as the database can implement memcmp semantics over an array
    // of bytes, you can perform in-database sorting.
    let mut keys: Vec<EvrSortKey> = ["2.0-1", "1:0.1-1", "1.0~rc1-1", "1.0-1"]
        .iter()
        .map(|s| EvrSortKey::parse(s))
        .collect();

    keys.sort();
    println!("\nSorted by sort key (byte order):");
    for k in &keys {
        println!("  {k:?}");
    }

    // The sort key order matches Evr's Ord implementation
    let mut evrs: Vec<Evr> = ["2.0-1", "1:0.1-1", "1.0~rc1-1", "1.0-1"]
        .iter()
        .map(|s| Evr::parse(s))
        .collect();

    evrs.sort();
    println!("\nSorted by Evr comparison:");
    for e in &evrs {
        println!("  {e}");
    }

    // Convert to/from Vec<u8> for storage
    let bytes: Vec<u8> = key_a.into();
    let restored = EvrSortKey::from(bytes);
    assert_eq!(key_c, restored);
}
