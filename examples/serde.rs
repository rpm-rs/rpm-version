use rpm_version::{Evr, Nevra};

fn main() {
    // Serialize an Evr to JSON
    let evr = Evr::parse("1:2.3.4-5");
    let json = serde_json::to_string_pretty(&evr).unwrap();
    println!("Evr as JSON:\n{json}\n");

    // Deserialize it back
    let parsed: Evr = serde_json::from_str(&json).unwrap();
    assert_eq!(evr, parsed);
    println!("Round-tripped Evr: {parsed}\n");

    // Serialize a Nevra to JSON
    let nevra = Nevra::parse("bash-1:5.2.26-3.fc40.x86_64");
    let json = serde_json::to_string_pretty(&nevra).unwrap();
    println!("Nevra as JSON:\n{json}\n");

    // Deserialize it back
    let parsed: Nevra = serde_json::from_str(&json).unwrap();
    assert_eq!(nevra, parsed);
    println!("Round-tripped Nevra: {parsed}");
}
