fn main() {
    let targets = vec!["arm64-v8a", "armeabi-v7a", "x86", "x86_64"];
    for target in targets {
        println!("cargo:rustc-link-search=../../android/tokencore/build/intermediates/cmake/release/obj/{}/", target);
    }
}
