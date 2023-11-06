use std::{env, fs};
use std::path::{Path, PathBuf};

fn get_output_path() -> PathBuf {
    // TODO: find a better path to this stuff
    Path::new(&env::var("OUT_DIR").unwrap()).join("../../../deps")
}

#[cfg(target_os = "windows")]
fn link_and_load() -> String {
    println!("cargo:rerun-if-env-changed=NDI_SDK_DIR");
    let sdk_dir = env::var("NDI_SDK_DIR");
    if sdk_dir.is_err() {
        panic!("NDI SDK not found, please (re)install the SDK (NDI_SDK_DIR not defined)")
    }
    let sdk_dir = sdk_dir.unwrap();

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let arch = if target_arch == "x86_64" { "x64" } else { "x86" };

    println!("cargo:rustc-link-lib=Processing.NDI.Lib.{arch}");
    println!("cargo:rustc-link-search={sdk_dir}\\Lib\\{arch}");

    let dll_name = format!("Processing.NDI.Lib.{arch}.dll");
    let dll_path = format!("{sdk_dir}\\Bin\\{arch}\\{}", dll_name);

    let dst = Path::join(&get_output_path(), dll_name);
    println!("Copying {}", dll_path);
    fs::copy(dll_path, dst).unwrap(); // Or ensure path to DLL is in PATH env variable

    format!("{sdk_dir}\\Include")
}

#[cfg(target_os = "linux")]
fn link_and_load() -> String {
    const SDK_DIR: &str = "/usr"; // To do: generalize/make configurable.

    if !Path::new(&format!("{SDK_DIR}/include/Processing.NDI.Lib.h")).exists() {
        panic!("NDI SDK not found, please (re)install the SDK (/usr/include/Processing.NDI.Lib.h not found; NDI SDK must be installed in /usr)")
    }

    // See 'NDI SDK for Linux/lib' for other possible targets
    let lib_target = match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "x86_64" => "x84_64-linux-gnu",
        "x86" => "i686-linux-gnu",
        "aarch64" => "aarch64-rpi4-linux-gnueabi", // 64 bit ARM
        "arm" => "arm-rpi4-linux-gnueabihf", // 32 bit ARM
        t => panic!("Unsupported target ({t}).")
    };

    println!("cargo:rustc-link-lib=ndi");
    println!("cargo:rustc-link-search={SDK_DIR}/lib/{lib_target}");

    // copy dll to OUT_DIR
    // To do: is this necessary?
    // let out_path = get_output_path();
    // let src = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
    //     .join("thirdparty/Linux/Lib/libndi.so.4");
    // let dst = Path::join(&out_path, "libndi.so.4");
    // fs::copy(src, dst).unwrap();

    format!("{SDK_DIR}/include")
}

#[cfg(target_os = "macos")]
fn link_and_load() -> String {
    const SDK_DIR: &str = "/Library/NDI SDK for Apple";

    println!("cargo:warning=Support for macOS has not been tested");

    if !Path::new(&format!("{SDK_DIR}/include/Processing.NDI.Lib.h")).exists() {
        panic!("NDI SDK not found, please (re)install the SDK (couldn't find {{SDK dir}}/include/Processing.NDI.Lib.h)")
    }

    let library_dir = format!("{SDK_DIR}/lib/macOS");
    for file in fs::read_dir(&library_dir).expect("reading directory {{SDK dir}}/lib/macOS failed") {
        let file = file.unwrap();
        if file.path().extension().unwrap() == "dylib" {
            println!("cargo:rustc-link-lib={}", file.file_name().to_str().unwrap());
        }
    }

    println!("cargo:rustc-link-search={library_dir}");

    // maybe to do: copy dylibs to same directory as executable?

    format!("{SDK_DIR}/include")
}

fn generate_bindings(include_dir: String) {
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate bindings for.
        .header("ndi_wrapper.h")
        // Find the includes in this directory
        .clang_args(["-I", &include_dir].iter())
        .clang_arg("-fdeclspec")
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    let include_dir = link_and_load();

    generate_bindings(include_dir);
}
