fn main() {
    //println!("cargo:rerun-if-env-changed=API_ROOT");
    println!("cargo:rerun-if-changed=../../.env");
}
