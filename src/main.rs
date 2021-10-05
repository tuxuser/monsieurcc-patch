use std::{
    error::Error,
    path::{Path, PathBuf},
    process::Command,
};

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

    /// Custom domain for API calls
    #[structopt(short, long)]
    domain: Option<String>,

    /// Custom port for API calls
    #[structopt(short, long)]
    port: Option<u16>,

    /// Signing keystore
    #[structopt(short, long, default_value = "keystore.jks")]
    signing_keystore: PathBuf,

    /// Disable SSL API connection
    #[structopt(short, long)]
    no_ssl: bool,
}

enum Patch<'a> {
    #[allow(dead_code)]
    Binary(&'a [u8], &'a [u8]),
    String(&'a str, &'a str),
    Truncation(&'a str, &'a str, Option<&'a str>),
}

fn get_domain_patches<'a>(target_domain: &'a str) -> Vec<(Vec<&'static str>, Vec<Patch<'a>>)> {
    vec![
        // Package name
        (
            vec!["smali/mcapi/McApi.smali", "smali/mcapi/McUsageApi.smali"],
            vec![Patch::String(
                "https://mc20.monsieur-cuisine.com",
                target_domain,
            )],
        ),
    ]
}

fn get_general_patches<'a>() -> Vec<(Vec<&'static str>, Vec<Patch<'a>>)> {
    vec![
        // Package name
        (
            vec![
                "smali/de/silpion/mc2/BuildConfig.smali",
                "smali/helper/ResourceHelper.smali",
                "AndroidManifest.xml",
            ],
            vec![Patch::String("de.silpion.mc2", "de.mcc_hack.mc2.mod")],
        ),
        (
            vec!["AndroidManifest.xml"],
            vec![
                // Remove android:sharedUserId="android.uid.system" so apps can co-exist
                Patch::String(r#"android:sharedUserId="android.uid.system" "#, ""),
                // App name
                Patch::String(
                    r#"<string name="app_name">MC2</string>"#,
                    r#"<string name="app_name">MC2 MOD</string>"#,
                ),
            ],
        ),
    ]
}

fn get_ssl_patches<'a>() -> Vec<(Vec<&'static str>, Vec<Patch<'a>>)> {
    vec![
        // No SSL factory
        (
            vec!["smali/ql$d.smali"],
            vec![
                // Do not set SSLContext on HttpURLConnection
                Patch::Truncation(
                    ".line 6\n    sget-object p2, Lql$d;->d:Ljavax/net/ssl/SSLContext;",
                    "invoke-virtual {p1, p2}, Ljavax/net/ssl/HttpsURLConnection;->setSSLSocketFactory(Ljavax/net/ssl/SSLSocketFactory;)V",
                    Some(".line 6\n    nop")
                ),
            ],
        ),
        // HttpsURLConnection -> HttpURLConnection
        (
            vec!["smali/ql$b.smali", "smali/ql$c.smali", "smali/ql$d.smali", "smali/ql$e.smali", "smali/ql$f.smali"],
            vec![Patch::String("Ljavax/net/ssl/HttpsURLConnection", "Ljava/net/HttpURLConnection")],
        ),
    ]
}

fn patch_buf(
    mut input: Vec<u8>,
    pattern: &[u8],
    replace: &[u8],
) -> Result<Vec<u8>, Box<dyn Error>> {
    loop {
        // Find position of pattern match
        let match_pos = input
            .windows(pattern.len())
            .position(|chunk| chunk == pattern);

        if let Some(pos) = match_pos {
            let end = pos + pattern.len();
            // Remove matched bytes from buffer
            if input.drain(pos..end).collect::<Vec<u8>>() != pattern {
                return Err("Unexpected pattern removed".into());
            }

            // Inject replacement bytes into buffer
            for (idx, b) in replace.iter().enumerate() {
                input.insert(pos + idx, *b);
            }
        } else {
            break;
        }
    }

    Ok(input)
}

fn patch_truncate_buf(
    mut input: Vec<u8>,
    start_pattern: &[u8],
    end_pattern: &[u8],
    replacement: &[u8],
) -> Result<Vec<u8>, Box<dyn Error>> {
    // Find position of pattern match
    let pos = input
        .windows(start_pattern.len())
        .position(|chunk| chunk == start_pattern)
        .expect("Did not find pattern");

    let section_len = &input[pos..]
        .windows(end_pattern.len())
        .position(|chunk| chunk == end_pattern)
        .expect("Did not find end pattern");

    // Calculate absolute end offset
    let end = pos + section_len + end_pattern.len();

    // Remove matched bytes from buffer
    let matched = input.drain(pos..end).collect::<Vec<u8>>();

    /*
    let removed_chunk_str = String::from_utf8(matched.clone())?;
    println!("Removed chunk: {}\nStart Pattern: {}\nEnd Pattern: {}\n",
        removed_chunk_str,
        String::from_utf8(start_pattern.to_vec())?,
        String::from_utf8(end_pattern.to_vec())?
    );
    */

    if !matched.starts_with(start_pattern) {
        return Err("Start pattern not matching".into());
    } else if !matched.ends_with(end_pattern) {
        return Err("End pattern not matching".into());
    }

    // Inject replacement bytes into buffer
    for (idx, b) in replacement.iter().enumerate() {
        input.insert(pos + idx, *b);
    }

    Ok(input)
}

fn patch_string(input: &str, pattern: &str, replace: &str) -> Result<String, Box<dyn Error>> {
    Ok(input.replace(pattern, replace))
}

fn patch_files(
    base_path: &Path,
    patches: Vec<(Vec<&'static str>, Vec<Patch>)>,
    debug_output: bool,
) -> Result<(), Box<dyn Error>> {
    for (files, patches) in patches {
        for file in files {
            let mut path = base_path.to_path_buf();
            path.push(file);

            let mut filebuf =
                std::fs::read(&path).unwrap_or_else(|_| panic!("Failed to read file: {:?}", &path));

            println!("[*] Patching {:?}", &path);
            for patch in &patches {
                match patch {
                    Patch::String(orig, replace) => {
                        if debug_output {
                            println!(
                                "###\n*STRING*\nOrig: \'{}\'\nReplacement: \'{}\'\n###",
                                orig, replace
                            );
                        }

                        let mut filebuf_str = String::from_utf8(filebuf)?;
                        filebuf_str = patch_string(&filebuf_str, orig, replace)?;
                        filebuf = filebuf_str.as_bytes().to_vec();
                    }
                    Patch::Binary(orig, replace) => {
                        if debug_output {
                            println!(
                                "###\n*BYTE*\nOrig: \'{:?}\'\nReplacement: \'{:?}\'\n###",
                                orig, replace
                            );
                        }
                        filebuf = patch_buf(filebuf, orig, replace)?;
                    }
                    Patch::Truncation(start, end, replacement) => {
                        if debug_output {
                            println!(
                                "###\n*TRUNC*\nStart: {:?}\nEnd: {:?}\nReplacement: \'{:?}\'\n###')",
                                start, end, replacement
                            );
                        }
                        filebuf = patch_truncate_buf(
                            filebuf,
                            start.as_bytes(),
                            end.as_bytes(),
                            replacement.unwrap_or("").as_bytes(),
                        )?;
                    }
                }
            }

            std::fs::write(&path, filebuf)
                .unwrap_or_else(|_| panic!("Failed to write back file: {:?}", &path));
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Opt::from_args();

    if !args.apk_file.exists() {
        return Err("Input APK File does not exist".into());
    }

    let input_filename = args.apk_file.file_name().expect("Input APK is not a file");

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

    let target_domain = {
        // Fallback to stock URL if none is provided via cmdline
        let mut domain = match args.domain {
            Some(d) => {
                if d.starts_with("http") || d.starts_with("https") {
                    return Err("Please provide domain without http/https prefix".into());
                }
                d
            }
            None => "mc20.monsieur-cuisine.com".to_string(),
        };

        // Append port to domain, if provided
        domain = match args.port {
            Some(port) => format!("{}:{}", domain, port),
            None => domain,
        };

        // Prefix with http or https
        let protocol = args
            .no_ssl
            .then(|| "http://")
            .unwrap_or("https://")
            .to_owned();
        domain = protocol + &domain;

        // No change in API URL needed
        match domain.eq("https://mc20.monsieur-cuisine.com") {
            true => None,
            false => Some(domain),
        }
    };

    // Ensure to clear apktool's cache, for safety
    Command::new("apktool")
        .arg("empty-framework-dir")
        .output()
        .expect("Failed to execute apktool");

    println!("[+] Unpacking APK {:?} ...", &input_filename);
    let output = Command::new("apktool")
        .arg("d") // decode
        .arg("-o")
        .arg(&tmp_patch_dir)
        .arg(args.apk_file)
        .output()
        .expect("Failed to execute apktool");

    println!(
        "[*] APKTool reports\n{}{}\n",
        String::from_utf8(output.stdout).expect("Found invalid UTF-8 in stdout..."),
        String::from_utf8(output.stderr).expect("Found invalid UTF-8 in stderr..."),
    );

    // Patch domains, if desired
    match target_domain {
        Some(domain) => {
            println!("[*] Patching domain to {}", &domain);
            let patches = get_domain_patches(&domain);
            patch_files(&tmp_patch_dir, patches, args.debug).expect("Failed to patch domains");
        }
        None => println!("[-] Not patching domain"),
    }

    // Apply other patches
    patch_files(&tmp_patch_dir, get_general_patches(), args.debug)
        .expect("Failed to apply general patches");

    // SSL Patches
    match args.no_ssl {
        true => {
            println!("[*] Removing API SSL interaction");
            patch_files(&tmp_patch_dir, get_ssl_patches(), args.debug)
                .expect("Failed to apply SSL patches");
        }
        false => {
            println!("[*] Leaving API SSL interaction as-is");
        }
    }

    // Repack patched APK
    println!("[+] Packing APK into {:?} ...", output_file);
    let output = Command::new("apktool")
        .arg("b") // decode
        .arg("-o")
        .arg(&output_file)
        .arg(&tmp_patch_dir)
        .output()
        .expect("Failed to execute apktool");

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
        .expect("Failed to execute zipalign");

    if !output.status.success() {
        println!(
            "[*] zipalign reports\n{}{}\n",
            String::from_utf8(output.stdout).expect("Found invalid UTF-8 in stdout..."),
            String::from_utf8(output.stderr).expect("Found invalid UTF-8 in stderr..."),
        );
    }

    // Move aligned apk to output filepath
    std::fs::rename("aligned.apk", &output_file)?;

    // Optionally, generate fresh siging key
    if !args.signing_keystore.exists() {
        println!("[!] Generating fresh signing keystore");
        let output = Command::new("keytool")
            .arg("-genkey")
            .arg("-noprompt")
            .arg("-v")
            .arg("-keystore")
            .arg(&args.signing_keystore)
            .arg("-keyalg")
            .arg("RSA")
            .arg("-keysize")
            .arg("2048")
            .arg("-validity")
            .arg("10000")
            .arg("-storepass")
            .arg("MCC1234567890")
            .arg("-keypass")
            .arg("MCC0987654321")
            .arg("-alias")
            .arg("MCCHack")
            .arg("-dname")
            .arg("CN=mcc_hack, OU=Main, O=MCC_HACK, L=Munich, S=Bavaria, C=DE")
            .output()
            .expect("Failed to execute keytool");

        println!(
            "[*] keytool reports\n{}{}\n",
            String::from_utf8(output.stdout).expect("Found invalid UTF-8 in stdout..."),
            String::from_utf8(output.stderr).expect("Found invalid UTF-8 in stderr..."),
        );
    }

    // Sign APK
    println!("[+] Signing APK");
    let output = Command::new("apksigner")
        .arg("sign")
        .arg("--ks")
        .arg(&args.signing_keystore)
        .arg("--ks-pass")
        .arg("pass:MCC1234567890")
        .arg("--key-pass")
        .arg("pass:MCC0987654321")
        .arg(&output_file)
        .output()
        .expect("Failed to execute apksigner");

    if !output.status.success() {
        println!(
            "[*] apksigner reports\n{}{}\n",
            String::from_utf8(output.stdout).expect("Found invalid UTF-8 in stdout..."),
            String::from_utf8(output.stderr).expect("Found invalid UTF-8 in stderr..."),
        );
    }

    Ok(())
}
