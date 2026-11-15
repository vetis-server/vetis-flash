# vetis-flash

[![Crates.io downloads](https://img.shields.io/crates/d/vetis-flash)](https://crates.io/crates/vetis-flash) [![crates.io](https://img.shields.io/crates/v/vetis-flash?style=flat-square)](https://crates.io/crates/vetis-flash) [![Build Status](https://github.com/vetis-server/vetis-flash/actions/workflows/rust.yml/badge.svg?event=push)](https://github.com/vetis-server/vetis-flash/actions/workflows/rust.yml) ![Crates.io MSRV](https://img.shields.io/crates/msrv/vetis-flash) [![Documentation](https://docs.rs/vetis-flash/badge.svg)](https://docs.rs/vetis-flash/latest/vetis-flash) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/vetis-server/vetis-flash/blob/main/LICENSE.md)  [![codecov](https://codecov.io/gh/vetis-server/vetis-flash/graph/badge.svg?token=T0HSBAPVSI)](https://codecov.io/gh/vetis-server/vetis-flash)

Static file serving support for the vetis HTTP server framework. This module provides efficient static file serving with configurable caching, support for multiple async runtimes, and flexible path configuration.

## Features

- Serve static files with configurable extensions and directories

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
vetis-flash = "0.1.0"
```

### Runtime Features

Choose the appropriate runtime feature for your project:

- `runtime-tokio` (default): Use with Tokio runtime

## Configuration

### Using API

Configure static file serving using the builder pattern:

```rust,ignore
use vetis_flash::FlashHookConfig;
use vetis_matchers::request::{PathMatcher, PathMatcherConfig};

let config = FlashHookConfig::builder()
    .matcher(PathMatcher::new(
        PathMatcherConfig::builder()
            .add_path("/flash*")
            .build()?,
    ))
    .response(r"Hello from VeTiS!")
    .build()?;
```

### vetis.yaml

```yaml
type: flash
matcher:
  type: "path"
  paths:
    - "/flash*"
response: "Hello from VeTiS!"
```

### Configuration Options

#### FlashHookConfig

- **matcher**: If set, this hook will be only called if request meet the criteria defined by him, otherwise, all requests will call this hook (optional)
- **response**: Server response content

## Runtime-Specific Modules

Based on your selected runtime feature, import the appropriate module:

```rust
// For Tokio runtime
#[cfg(feature = "runtime-tokio")]
use vetis_static::tokio;

// For Smol runtime
#[cfg(feature = "runtime-smol")]
use vetis_static::smol;

// For Compio runtime
#[cfg(feature = "runtime-compio")]
use vetis_static::compio;
```

## License

Licensed under either of

- Apache License, Version 2.0
  (LICENSE-APACHE or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  (LICENSE-MIT or <https://opensource.org/licenses/MIT>)

at your option.

## Author

Rogerio Pereira Araujo <rogerio.araujo@gmail.com>
