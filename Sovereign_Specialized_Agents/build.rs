fn main() {
    println!("cargo:rerun-if-changed=../Sovereign_Vortex_Core.cpp");
    
    // Phase 42: Compile the Vortex Inference Engine
    cc::Build::new()
        .cpp(true)
        .file("../Sovereign_Vortex_Core.cpp")
        .flag_if_supported("-O3")
        .flag_if_supported("/O2") // MSVC
        .flag_if_supported("-mavx2")
        .flag_if_supported("-mfma")
        .compile("vortex_core");
}
