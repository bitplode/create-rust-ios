# create-rust-ios

This is a code generator. Yes, the dreaded animal that takes (some) input and spits out code. In this case, we enter a configuration for a Rust lib project that will be used by a Swift iOS project. Let's see where this goes.

## Configuration

Location of file:
- Same directory as the `create-rust-ios` executable, or in an ancestral directory (up the tree)

Name of file:
- `create-rust-ios.config.json`

Content of file:

```
{
  "project_name": "rusty_ios_app",
  "output_dir": "./generated_rust_ios_apps",
  "rust_file_name": "rust_bridge_header",
  "rust_dir": "rust",
  "ios_dir": "~/Documents/MyIOSApp"
}
```

* project_name : The name of the project (NOT USED - so we could probably omit this!)
* output_dir : The directory in which the generated 

## Build

```
cargo build
```


## Run

After building...

```
cd target/debug/create-rust-ios
./create-rust-ios
```

or you can also

```
cargo run
```

Once you do this, there will be a generated folder based on the configuration.

```
cd generated_rust_ios_app
./install.sh
```

This will complete the last steps of writing the necessary changes to the iOS project.

## Tree

If you `tree` (on a Mac at least) in the `generated_rust_ios_app` directory, you will see the following:

```
$ tree
.
├── install.sh
└── rust
    ├── Cargo.lock
    ├── Cargo.toml
    ├── rust_bridge_header.h
    ├── src
    │   └── lib.rs
    └── target
        ├── CACHEDIR.TAG
        ├── aarch64-apple-ios
        │   ├── CACHEDIR.TAG
        │   └── release
        │       ├── build
        │       ├── deps
        │       │   ├── librust_bridge_header.a
        │       │   ├── librust_bridge_header.dylib
        │       │   └── rust_bridge_header.d
        │       ├── examples
        │       ├── incremental
        │       ├── librust_bridge_header.a
        │       ├── librust_bridge_header.d
        │       └── librust_bridge_header.dylib
        ├── release
        │   ├── build
        │   ├── deps
        │   ├── examples
        │   └── incremental
        ├── universal
        │   └── release
        │       └── librust_bridge_header.a
        └── x86_64-apple-ios
            ├── CACHEDIR.TAG
            └── release
                ├── build
                ├── deps
                │   ├── librust_bridge_header.a
                │   ├── librust_bridge_header.dylib
                │   └── rust_bridge_header.d
                ├── examples
                ├── incremental
                ├── librust_bridge_header.a
                ├── librust_bridge_header.d
                └── librust_bridge_header.dylib

23 directories, 21 files
```

## The iOS Side

On the iOS side of things. Assuming that we are using a vanilla SwiftUI project called `MyIOSApp`:

```
$ cd MyIOSApp
$ ls -l MyIOSApp
total 24
drwxr-xr-x  5 me  staff  160 Jun 23 23:11 Assets.xcassets
-rw-r--r--@ 1 me  staff  235 Jun 23 23:11 MyIOSAppApp.swift
-rw-r--r--@ 1 me  staff  482 Jun 23 23:11 ContentView.swift
drwxr-xr-x  3 me  staff   96 Jun 23 23:11 Preview Content
-rw-r--r--@ 1 me  staff  292 Jun 25 17:54 rust_bridge_header.h
```

Notice the `rust_bridge_header.h` was added to the `MyIOSApp` project's source files.

### The Rust Bridge Header File

Here is the content of the `rust_bridge_header.h` file:

```
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * # Safety
 *
 * This function should not be called too early!
 */
char *rust_greet(const char *to);

/**
 * # Safety
 *
 * This function should not be called too early!
 */
void rust_greet_free(char *s);
```

### The lib

#### General tab

1. In XCode, click on the `MyIOSApp` project and then click on the `General` tab.
2. Scroll down until you see the `Frameworks, Libraries and Embedded Content` section.
3. Underneath that, you will see `librust_bridge_header.a` (assuming your configuration is set the same as above).

The format for the `.a` file is `lib{rust_file_name}.a` where `{rust_file_name}` comes from the value of the `rust_file_name` property in the configuration.

#### Build Settings tab

##### Search Paths

1. In XCode, click on the `MyIOSApp` project and then click on the `Build Settings` tab.
2. Scroll down until you see the `Search Paths` section.
3. Underneath that, you will see:

The settings under `Library Search Paths`, i.e.

```
Debug
  Any Architecture | Any SDK
Release
  Any Architecture | Any SDK
```

will all be set to `/Users/me/Documents/MyIOSApp/libs`

##### Swift Compiler - General

1. In XCode, click on the `MyIOSApp` project and then click on the `Build Settings` tab.
2. Scroll down until you see the `Swift Compiler - General` section.
3. Underneath that, you will see:

The setting `Objective-C Bridging Header` is set to `MyIOSApp/rust_bridge_header.h`

## After setup

Once you've run the program and the `install.sh` script, you will be able to open the iOS project (e.g. `MyIOSApp`) in XCode and build from there.

For each Rust function you wish to expose to Swift, you will have to update the 