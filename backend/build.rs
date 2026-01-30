use std::path::Path;
use std::fs;

fn main() {

    println!("cargo:rerun-if-changed=csharp/DataProcessor.cs");
    println!("cargo:rerun-if-changed=csharp/MathTransforms.cs");
    println!("cargo:rerun-if-changed=csharp/RuleEngine.cs");

    let processor_path = Path::new("bin/DeterministicProcessor");

    if !processor_path.exists() {
        panic!(
            "\n[build.rs error]\n\
            Required prebuilt C# processor not found.\n\n\
            Expected path:\n\
              backend/bin/DeterministicProcessor\n\n\
            Please build the C# project on macOS with:\n\
              dotnet publish -c Release -r linux-x64 \\\n\
              --self-contained true \\\n\
              -p:PublishSingleFile=true \\\n\
              -o backend/bin\n"
        );
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let metadata = fs::metadata(processor_path)
            .expect("Failed to read metadata of DeterministicProcessor");

        let permissions = metadata.permissions();
        let mode = permissions.mode();

        if mode & 0o111 == 0 {
            panic!(
                "\n[build.rs error]\n\
                DeterministicProcessor exists but is not executable.\n\n\
                Please fix with:\n\
                  chmod +x backend/bin/DeterministicProcessor\n"
            );
        }
    }

}
