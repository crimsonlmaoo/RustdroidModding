fn main() {
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();

    if target_os == "android" {
        let arch_dir = match target_arch.as_str() {
            "aarch64" => "arm64-v8a",
            "arm" => "armeabi-v7a",
            "x86_64" => "x86_64",
            "x86" => "x86",
            other => panic!("Unsupported Android target arch: {}", other),
        };

        let ndk_arch = match target_arch.as_str() {
            "aarch64" => "aarch64-linux-android",
            "arm" => "armv7a-linux-androideabi",
            "x86_64" => "x86_64-linux-android",
            "x86" => "i686-linux-android",
            other => panic!("Unsupported Android target arch: {}", other),
        };

        let ndk_home = std::env::var("ANDROID_NDK_HOME")
            .unwrap_or_else(|_| "/home/crimsonh/Android/Sdk/ndk/27.0.12077973".to_string());
        let toolchain = format!("{}/toolchains/llvm/prebuilt/linux-x86_64", ndk_home);

        let dobby_path = format!("dobby/{}", arch_dir);
        println!("cargo:rustc-link-search={}", dobby_path);
        println!("cargo:rustc-link-lib=static=dobby");

        let libcxx_path = format!("{}/sysroot/usr/lib/{}/21", toolchain, ndk_arch);
        println!("cargo:rustc-link-search={}", libcxx_path);
        println!("cargo:rustc-link-lib=static=c++_static");
        println!("cargo:rustc-link-lib=static=c++abi");

        println!("cargo:rustc-link-lib=dylib=log");
        println!("cargo:rustc-link-lib=dylib=dl");
    }
}
