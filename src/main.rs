mod patch;

use anyhow::{Context, Result};
use std::{error::Error, path::PathBuf, process::Command};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(long)]
    debug: bool,

    /// Path to APK to patch
    apk_file: PathBuf,

    /// Output filepath
    #[structopt(short, long)]
    output_file: Option<PathBuf>,

    /// Path to YAML patchfile
    #[structopt(short, long, default_value = "patches.yml")]
    patch_file: PathBuf,

    /// Signing keystore
    #[structopt(short, long, default_value = "keystore.jks")]
    signing_keystore: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Opt::from_args();

    if !args.apk_file.exists() {
        return Err("Input APK file does not exist".into());
    } else if !args.patch_file.exists() {
        return Err("Patches YAML file does not exist".into());
    } else if !args.signing_keystore.exists() {
        return Err("Keystore file does not exist".into());
    }

    let input_filename = args
        .apk_file
        .file_name()
        .context("APK input argument is no file")?;

    let patches = {
        let yaml =
            std::fs::read_to_string(&args.patch_file).context("Failed to read Patches YAML")?;

        patch::deserialize_patches(&yaml).context("Failed to deserialize patches")?
    };

    let output_file = match args.output_file {
        Some(out) => out,
        None => PathBuf::from(format!("patched_{}", input_filename.to_str().unwrap())),
    };

    let tmp_patch_dir = {
        let p = PathBuf::from(format!("{}.out", input_filename.to_str().unwrap()));

        if p.exists() {
            eprintln!("[!] Removing leftover tmp patch dir: {:?}", &p);
            let _ = std::fs::remove_dir_all(&p);
        }

        p
    };

    // Ensure to clear apktool's cache, for safety
    Command::new("apktool")
        .arg("empty-framework-dir")
        .output()
        .context("Failed to execute apktool (cmd: empty-framework-dir)")?;

    println!("[+] Unpacking APK {:?} ...", &input_filename);
    let output = Command::new("apktool")
        .arg("d") // decode
        .arg("-o")
        .arg(&tmp_patch_dir) // Extraction output dir
        .arg(args.apk_file) // APK input
        .output()
        .context("Failed to execute apktool (cmd: decode)")?;

    println!(
        "[*] APKTool reports\n{}{}\n",
        String::from_utf8(output.stdout).expect("Found invalid UTF-8 in stdout..."),
        String::from_utf8(output.stderr).expect("Found invalid UTF-8 in stderr..."),
    );

    println!("[+] Applying patches from {:?}", args.patch_file);
    patch::apply_patches(&tmp_patch_dir, patches).context("Failed to apply patches")?;

    // Repack patched APK
    println!("[+] Building APK into {:?} ...", output_file);
    let output = Command::new("apktool")
        .arg("b") // build
        .arg("-o")
        .arg(&output_file) // Packed APK file output
        .arg(&tmp_patch_dir) // Unpacked, patched APK data dir
        .output()
        .context("Failed to execute apktool (cmd: build)")?;

    println!(
        "[*] APKTool reports\n{}{}\n",
        String::from_utf8(output.stdout).expect("Found invalid UTF-8 in stdout..."),
        String::from_utf8(output.stderr).expect("Found invalid UTF-8 in stderr..."),
    );

    // Zipalign
    let output = Command::new("zipalign")
        .arg("-f")
        .arg("-p")
        .arg("4")
        .arg(&output_file)
        .arg("aligned.apk")
        .output()
        .context("Failed to execute zipalign")?;

    if !output.status.success() {
        println!(
            "[*] zipalign reports\n{}{}\n",
            String::from_utf8(output.stdout).expect("Found invalid UTF-8 in stdout..."),
            String::from_utf8(output.stderr).expect("Found invalid UTF-8 in stderr..."),
        );
    }

    // Move aligned apk to output filepath
    std::fs::rename("aligned.apk", &output_file)?;

    // Sign APK
    println!("[+] Signing APK");
    let output = Command::new("apksigner")
        .arg("sign")
        .arg("--ks")
        .arg(&args.signing_keystore)
        .arg("--ks-pass")
        .arg("pass:MCC0987654321")
        .arg(&output_file)
        .output()
        .context("Failed to execute apksigner")?;

    if !output.status.success() {
        println!(
            "[*] apksigner reports\n{}{}\n",
            String::from_utf8(output.stdout).expect("Found invalid UTF-8 in stdout..."),
            String::from_utf8(output.stderr).expect("Found invalid UTF-8 in stderr..."),
        );
    }

    Ok(())
}
