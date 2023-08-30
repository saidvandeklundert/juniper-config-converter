# JCC: Juniper Config Converter

Convert Juniper configurations.

Takes a Juniper configuration as displayed using `show configuration` and transforms it to one as displayed using `show configuration | display set`.

Example:

```
policy-options {
    policy-statement directs {
        term Lo0 {
            from {
                protocol direct;
                route-filter 192.168.100.0/24 orlonger;
            }
            then accept;
        }
    }   
}
```

Will get transformed into:

```
set policy-options policy-statement directs term Lo0 from protocol direct
set policy-options policy-statement directs term Lo0 from route-filter 192.168.100.0/24 orlonger
set policy-options policy-statement directs term Lo0 then accept
```


The `jcc` crate exposes the function `convert` so you can use it inside your own application. The `jcc-cli` crate offers a CLI tool to convert Juniper configurations from the CLI.

Example usage:

```
# cargo install jcc-cli


# cat /tmp/config_1.txt 
policy-options {
    policy-statement directs {
        term Lo0 {
            from {
                protocol direct;
                route-filter 192.168.100.0/24 orlonger;
            }
            then accept;
        }
    }   
}


# jcc-cli --file /tmp/config_1.txt
set policy-options policy-statement directs term Lo0 from protocol direct
set policy-options policy-statement directs term Lo0 from route-filter 192.168.100.0/24 orlonger
set policy-options policy-statement directs term Lo0 then accept
```


Currently in Beta.

Future plans:
- polish the package
- also convert configurations into useful data structures



## For Windows users:

In case the linker is not found when using cargo install, you can read [this](https://stackoverflow.com/questions/55603111/unable-to-compile-rust-hello-world-on-windows-linker-link-exe-not-found) and/or try this:
```
rustup uninstall toolchain stable-x86_64-pc-windows-msvc
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
cargo install jcc-cli
```