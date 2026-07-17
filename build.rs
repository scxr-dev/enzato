fn main() {
    // Only compile resources when the target is Windows
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        // Use WindowsResource, which is the correct entry point for winres 0.1.x
        let mut res = winres::WindowsResource::new();
        
        // Define your official ownership and publisher records
        res.set("CompanyName", "R H A Ashan Imalka (scxr-dev)");
        res.set("LegalCopyright", "Copyright © 2026 R H A Ashan Imalka (scxr-dev)");
        
        // Set file description and naming schemas
        res.set("FileDescription", "Enzato - Minimalist Terminal Text Editor");
        res.set("ProductName", "Enzato Text Editor");
        res.set("OriginalFilename", "enzato.exe");
        
        // Setup official versioning numbers (1.0.0.0)
        res.set("ProductVersion", "1.0.0.0");
        
        // When cross-compiling from Linux/Unix to Windows, we need to locate the mingw toolchain.
        // On Arch Linux, the default cross-compiler package puts windres at '/usr/bin/x86_64-w64-mingw32-windres'
        #[cfg(not(target_os = "windows"))]
        {
            res.set_toolkit_path("/usr/bin");
            res.set_windres_path("x86_64-w64-mingw32-windres");
        }
        
        // Compile the resources into a COFF object file and link it to the compiler output
        if let Err(e) = res.compile() {
            eprintln!("Error compiling Windows resources: {}", e);
            std::process::exit(1);
        }
    }
}