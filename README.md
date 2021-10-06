# MCC patcher

Patches MCLauncher APK

* Disable SSL (Use http instead of https)
* Change package name so official apk and mod can co-exist
* Remove `android:sharedUserId="android.uid.system"` from Manifest
* ...

Tested / works so far for:

```
# SHA256
60937c42a5f20d1ac3f7c70c8dbb0361c33a015bbe3bb1e884897cb40634e22f  MCLauncher-release-1.1.16-208.apk
9d73ad1b852079c2ba2427ec25a69d893b6499dd3f5ddd9ef96a7eba9b83bf10  MCLauncher-release-1.1.17-226.apk
9b8ba59334588f5f337a9731f6e17808150e84314b9cb63339b4a7db62491dfb  MCLauncher-release-1.1.18-231.apk
85a9f5a247676db80df051433c44a3fb8afd482b38595ed7f589f0bb81b6b801  MCLauncher-release-1.1.19-238.apk
263e252dc9e139cb0a93f84547ca1971b3c4566136d0205dcbcea3c7ef24e201  MCLauncher-release-1.1.22-248.apk
```

Dependency versions:

* apktool 2.6.0
* apksigner 0.9

## Dependencies

Following tools need to be found in your `$PATH`:

* apktool (https://ibotpeaches.github.io/Apktool/)
* apksigner
* zipalign
* keytool

apksigner, zipalign, keytool are part of the Android SDK

## Usage

```
mcc_patch 0.1.1

USAGE:
    mcc_patch [FLAGS] [OPTIONS] <apk-file>

FLAGS:
        --debug      
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -o, --output-file <output-file>              Output filepath
    -p, --patch-file <patch-file>                Path to YAML patchfile [default: patches.yml]
    -s, --signing-keystore <signing-keystore>    Signing keystore [default: keystore.jks]

ARGS:
    <apk-file>    Path to APK to patch
```

### Example run

```
$ ./mcc_patch -o patched_MCLauncher.apk ../android_packages/MCLauncher-release-1.1.22-248.apk
[+] Unpacking APK "MCLauncher-release-1.1.22-248.apk" ...
[*] APKTool reports
I: Using Apktool 2.6.0 on MCLauncher-release-1.1.22-248.apk
I: Loading resource table...
I: Decoding AndroidManifest.xml with resources...
I: Loading resource table from file: /home/user/.local/share/apktool/framework/1.apk
I: Regular manifest package...
I: Decoding file-resources...
I: Decoding values */* XMLs...
I: Baksmaling classes.dex...
I: Copying assets and libs...
I: Copying unknown files...
I: Copying original files...


[+] Applying patches from "patches.yml"
-> Patch: Change APK package name
* Match on file "MCLauncher-release-1.1.22-248.apk.out/smali/de/silpion/mc2/BuildConfig.smali"
* Match on file "MCLauncher-release-1.1.22-248.apk.out/smali/helper/ResourceHelper.smali"
* Match on file "MCLauncher-release-1.1.22-248.apk.out/AndroidManifest.xml"
-> Patch: Remove sharedUserId
* Match on file "MCLauncher-release-1.1.22-248.apk.out/AndroidManifest.xml"
-> Patch: Change displayname
* Match on file "MCLauncher-release-1.1.22-248.apk.out/res/values/strings.xml"
* Match on file "MCLauncher-release-1.1.22-248.apk.out/res/values-en/strings.xml"
* Match on file "MCLauncher-release-1.1.22-248.apk.out/res/values-es/strings.xml"
* Match on file "MCLauncher-release-1.1.22-248.apk.out/res/values-fr/strings.xml"
* Match on file "MCLauncher-release-1.1.22-248.apk.out/res/values-it/strings.xml"
* Match on file "MCLauncher-release-1.1.22-248.apk.out/res/values-nl/strings.xml"
* Match on file "MCLauncher-release-1.1.22-248.apk.out/res/values-pl/strings.xml"
* Match on file "MCLauncher-release-1.1.22-248.apk.out/res/values-pt/strings.xml"
-> Patch: Disable SSL (Part 1)
* Match on file "MCLauncher-release-1.1.22-248.apk.out/smali/mcapi/APIServiceFactory.smali"
* Match on file "MCLauncher-release-1.1.22-248.apk.out/smali/mcapi/McApi.smali"
* Match on file "MCLauncher-release-1.1.22-248.apk.out/smali/mcapi/McUsageApi.smali"
-> Patch: Disable SSL (Part 2)
* Match on file "MCLauncher-release-1.1.22-248.apk.out/smali/nm$b.smali"
* Match on file "MCLauncher-release-1.1.22-248.apk.out/smali/nm$c.smali"
* Match on file "MCLauncher-release-1.1.22-248.apk.out/smali/nm$d.smali"
* Match on file "MCLauncher-release-1.1.22-248.apk.out/smali/nm$e.smali"
* Match on file "MCLauncher-release-1.1.22-248.apk.out/smali/nm$f.smali"
-> Patch: Disable SSL (Part 3)
* Match on file "MCLauncher-release-1.1.22-248.apk.out/smali/nm$d.smali"
-> Patch: Patch in custom domain or host ip
* Match on file "MCLauncher-release-1.1.22-248.apk.out/smali/mcapi/APIServiceFactory.smali"
* Match on file "MCLauncher-release-1.1.22-248.apk.out/smali/mcapi/McApi.smali"
* Match on file "MCLauncher-release-1.1.22-248.apk.out/smali/mcapi/McUsageApi.smali"
[+] Building APK into "patched_MCLauncher.apk" ...
[*] APKTool reports
I: Using Apktool 2.6.0
I: Checking whether sources has changed...
I: Smaling smali folder into classes.dex...
I: Checking whether resources has changed...
I: Building resources...
I: Copying libs... (/lib)
I: Building apk file...
I: Copying unknown files/dir...
I: Built apk...
W: /MCLauncher-release-1.1.22-248.apk.out/AndroidManifest.xml:34: Tag <action> attribute name has invalid character ':'.
W: /MCLauncher-release-1.1.22-248.apk.out/AndroidManifest.xml:35: Tag <action> attribute name has invalid character ':'.


[+] Signing APK
```