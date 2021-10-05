# MCC patcher

Patches MCLauncher APK

* Disable SSL (Use http instead of https)
* Change package name so official apk and mod can co-exist
* Remove `android:sharedUserId="android.uid.system"` from Manifest
* ...

Tested / works so far for:

* MCLauncher v1.19

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
mcc_patch 0.1.0

USAGE:
    mcc_patch [FLAGS] [OPTIONS] <apk-file>

FLAGS:
        --debug      
    -h, --help       Prints help information
    -n, --no-ssl     Disable SSL API connection
    -V, --version    Prints version information

OPTIONS:
    -d, --domain <domain>                        Custom domain for API calls
    -o, --output-file <output-file>              Output filepath
    -p, --port <port>                            Custom port for API calls
    -s, --signing-keystore <signing-keystore>    Signing keystore [default: keystore.jks]

ARGS:
    <apk-file>    Path to APK to patch
```

### Example run

```
$ ./mcc_patch --no-ssl -d "plain_http_domain.com" MCLauncher.apk
[+] Unpacking APK "MCLauncher.apk" ...
[*] APKTool reports
I: Using Apktool 2.6.0 on MCLauncher.apk
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


[*] Patching domain to http://plain_http_domain.com
[*] Patching "MCLauncher.apk.out/smali/mcapi/McApi.smali"
[*] Patching "MCLauncher.apk.out/smali/mcapi/McUsageApi.smali"
[*] Patching "MCLauncher.apk.out/smali/de/silpion/mc2/BuildConfig.smali"
[*] Patching "MCLauncher.apk.out/smali/helper/ResourceHelper.smali"
[*] Patching "MCLauncher.apk.out/AndroidManifest.xml"
[*] Patching "MCLauncher.apk.out/AndroidManifest.xml"
[*] Removing API SSL interaction
[*] Patching "MCLauncher.apk.out/smali/ql$d.smali"
[*] Patching "MCLauncher.apk.out/smali/ql$b.smali"
[*] Patching "MCLauncher.apk.out/smali/ql$c.smali"
[*] Patching "MCLauncher.apk.out/smali/ql$d.smali"
[*] Patching "MCLauncher.apk.out/smali/ql$e.smali"
[*] Patching "MCLauncher.apk.out/smali/ql$f.smali"
[+] Packing APK into "patched_MCLauncher.apk" ...
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
W: /tmp/MCLauncher.apk.out/AndroidManifest.xml:34: Tag <action> attribute name has invalid character ':'.
W: /tmp/MCLauncher.apk.out/AndroidManifest.xml:35: Tag <action> attribute name has invalid character ':'.


[+] Signing APK
```