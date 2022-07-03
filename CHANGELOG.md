# 0.3.0

- Update to CLAP version 1.0.3.
- Fix typo in `clap_gui_resize_hints` field name (`preseve_aspect_ratio` to `preserve_aspect_ratio`).
- Derive Debug for all types.
- Use `CStr` instead of `*const c_char` for string constants.
- Wrap every function pointer type in an `Option`.

# 0.2.0

- Update to CLAP version 1.0.1.
- Fix typo in `clap_event_param_mod` field name (`cooke` to `cookie`).

# 0.1.0

- First release, implementing CLAP API version 1.0.0.
