# system-properties

This is a repackaging of the `system_properties` module from AOSP's librustutils so that it is usable outside of the AOSP build system.

The source is taken from the `android-17.0.0_r1` tag, unmodified. The only new code in this repo is `build.rs` and `lib.rs`. Due to [Cargo caching issues with submodules](https://github.com/rust-lang/cargo/issues/7987), the upstream files are copied into this repo instead of being added as a submodule.

## Contributing

([AI policy](https://github.com/chenxiaolong/chenxiaolong/blob/master/AI_POLICY.md))

Only bug fix contributions to the cargo metadata or build script are accepted because this is intended to use AOSP's code unmodified.

## License

android-properties is licensed under Apache 2.0, the same license as the original AOSP library. Please see [`LICENSE`](./LICENSE) for the full license text.
