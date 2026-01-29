use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let csharp_dir = PathBuf::from("csharp");
    
    println!("cargo:rerun-if-changed=csharp/DataProcessor.cs");
    println!("cargo:rerun-if-changed=csharp/MathTransforms.cs");
    println!("cargo:rerun-if-changed=csharp/RuleEngine.cs");
    
    let output = Command::new("dotnet")
        .args(&["publish", "-c", "Release", "-o", "../target/csharp"])
        .current_dir(&csharp_dir)
        .output()
        .expect("Failed to compile C# code. Ensure .NET SDK is installed.");
    
    if !output.status.success() {
        panic!(
            "C# compilation failed:\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
    
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:rustc-link-search=native={}", out_dir);
}
