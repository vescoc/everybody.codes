fn main() {
    println!("cargo::rerun-if-changed=data/part_1");
    println!("cargo::rerun-if-changed=data/part_2");
    println!("cargo::rerun-if-changed=data/part_3");
    
    everybody_codes::fetch_parts("data").unwrap();
}
