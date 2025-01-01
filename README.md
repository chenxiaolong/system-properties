# system-properties

This is a repackaging of the `system_properties` module from AOSP's librustutils so that it is usable outside of the AOSP build system.

The source is taken from the `android-15.0.0_r9` tag, unmodified. The only new code in this repo is `build.rs` and `lib.rs`. Due to [Cargo caching issues with submodules](https://github.com/rust-lang/cargo/issues/7987), the upstream files are copied into this repo instead of being added as a submodule.

## License

android-properties is licensed under Apache 2.0, the same license as the original AOSP library. Please see [`LICENSE`](./LICENSE) for the full license text.
