use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("build-x86-image") => build_x86_image(),
        _ => {
            eprintln!("Usage: xtask <command>");
            eprintln!();
            eprintln!("Commands:");
            eprintln!("  build-x86-image   Build bootable x86_64 BIOS disk image");
            std::process::exit(1);
        }
    }
}

fn build_x86_image() {
    let kernel_path = PathBuf::from("target/x86_64-unknown-none/release/arch_x86_64");

    if !kernel_path.exists() {
        eprintln!(
            "Error: kernel ELF not found at {}",
            kernel_path.display()
        );
        eprintln!("Run: cargo build -p arch_x86_64 --target x86_64-unknown-none --release");
        std::process::exit(1);
    }

    let out_dir = PathBuf::from("dist");
    std::fs::create_dir_all(&out_dir).expect("Failed to create dist/");

    let bios_image_path = out_dir.join("os-x86_64-bios.img");

    let bios_boot = bootloader::BiosBoot::new(&kernel_path);
    bios_boot
        .create_disk_image(&bios_image_path)
        .expect("Failed to create BIOS disk image");

    println!("[xtask] wrote {}", bios_image_path.display());
}
