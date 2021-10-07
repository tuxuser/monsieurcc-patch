# MCC patcher

Patches MCLauncher APK (de.silpion.mc2)

* Disable SSL (Use **http** instead of **https**)
* Change package name so official apk and mod can co-exist
* Change target domain name from **mc20.monsieur-cuisine.com** to **mcc.example.com** (can be adjusted in `patches.yml`)

Resulting APK will be signed with the publically available **Android Platform Key**.

>DN: EMAILADDRESS=android@android.com, CN=Android, OU=Android, O=Android, L=Mountain View, ST=California, C=US
>
>SHA-256 digest: c8a2e9bccf597c2fb6dc66bee293fc13f2fc47ec77bc6b2b0d52c11f51192ab8


Tested / works so far on following APKs:

```
# SHA256
60937c42a5f20d1ac3f7c70c8dbb0361c33a015bbe3bb1e884897cb40634e22f  MCLauncher-release-1.1.16-208.apk
9d73ad1b852079c2ba2427ec25a69d893b6499dd3f5ddd9ef96a7eba9b83bf10  MCLauncher-release-1.1.17-226.apk
9b8ba59334588f5f337a9731f6e17808150e84314b9cb63339b4a7db62491dfb  MCLauncher-release-1.1.18-231.apk
85a9f5a247676db80df051433c44a3fb8afd482b38595ed7f589f0bb81b6b801  MCLauncher-release-1.1.19-238.apk
263e252dc9e139cb0a93f84547ca1971b3c4566136d0205dcbcea3c7ef24e201  MCLauncher-release-1.1.22-248.apk
```

## Download

* Fetch latest release from [Github Releases](https://github.com/tuxuser/monsieurcc-patch/releases/latest).
* Obtain original `MCLauncher.apk` for your **Monsieur Cuisine Connect**.

## Usage

### Manual

Following tools need to be found in your `$PATH`:

* apktool (>= 2.6.0) (https://ibotpeaches.github.io/Apktool/install/)
* apksigner
* zipalign

`apksigner` and `zipalign` are part of the Android SDK, for recent Ubuntu/Debian distros they're
contained in package `google-android-build-tools-installer`.

Steps:
* Extract or build `mcc_patch`
* Copy `MCLauncher APK` next to `mcc_patch binary`
* Run `./mcc_patch -o patched_MCLauncher.apk <original.apk>`

## Docker

Copy `MCLauncher APK` in some directory to share it inside the Docker container.

```
docker run -v <local directory with apk>:/apk -it tuxuser/mcc_patch:latest ./mcc_patch -o /apk/patched.apk /apk/<original MCLauncher APK>
```


## Development

Requirements:
* Rust toolchain
* cargo

[rustup](https://rustup.rs) is the preferred way to manage the Rust development environment.

Build

```sh
# To build binary into target/<build mode>/
cargo build
# Or directly build & run
cargo run -- <program arguments>
```

Run test
```
cargo test
```

Run formatters & linters
```sh
# First, install
cargo install clippy
cargo install fmt

# Run
cargo fmt --all
cargo clippy --all
```
