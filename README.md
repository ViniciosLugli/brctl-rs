# brctl-rs

This project is a Rust wrapper for the `brctl` command-line tool used for Ethernet bridge administration on Linux systems. It provides a clean, programmatic interface for creating, managing, and inspecting network bridges, allowing Rust developers to interact with `brctl` and related networking utilities(like `ip`) in a more user-friendly manner.

## Features

- **Bridge Creation and Deletion**: Easily create and delete network bridges with intuitive Rust functions.
- **Interface Management**: Add or remove network interfaces from bridges programmatically.
- **Bridge Properties**: Query and modify bridge properties, such as Spanning Tree Protocol (STP), hairpin mode, and bridge ageing time.
- **System Readiness Check**: Verify system readiness by ensuring `brctl` and `ip` utilities are available and properly configured.
- **List and Inspect Bridges**: Retrieve a list of all bridges on the system and inspect bridge-specific details, including connected interfaces and bridge IDs.

## Dependencies / System Requirements

- **Linux**: This wrapper is designed for Linux systems, as `brctl` is a Linux-specific utility.
	- **brctl**: The tool for managing Ethernet bridges, part of the `bridge-utils` package on most Linux distributions.
	- **ip**: The command used for network interface manipulation, typically provided by `iproute2`.

	Ensure both utilities are installed on your system for full functionality of this wrapper.

### Example Usage

You can find this example on [examples/basic.rs](examples/basic.rs).

```rust
use brctl::{BridgeController, CommandError};

fn main() -> Result<(), CommandError> {
    // Initialize logging, if you want to see logs, set the RUST_LOG environment variable
    env_logger::init();

    // Check if the system is ready for bridge operations
    match BridgeController::check() {
        Ok((brctl_version, interfaces)) => {
            println!("brctl version: {}", brctl_version);
            println!("Available interfaces: {:?}", interfaces);
        }
        Err(e) => eprintln!("System not ready: {}", e),
    }

    // Create a new bridge
    let mybridge = BridgeController::create_bridge("mybridge")?;
    mybridge.add_interface("eth0")?;
    mybridge.add_interface("eth1")?;

    // List all bridges
    let bridges = BridgeController::list_bridges()?;
    println!("Available bridges:");
    for bridge in bridges {
        println!("  - {}", bridge);
    }

    // Retrieve bridge details
    let bridge_name = mybridge.get_name();
    println!("Bridge Name: {}", bridge_name);
    println!("Bridge ID: {}", mybridge.get_id()?);
    println!("Spanning Tree Protocol: {}", mybridge.get_stp()?);

    // Delete the bridge
    mybridge.delete()?;
    println!("Bridge deleted successfully");

    Ok(())
}
```

## Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
brctl = "1.0.0"
```

now you can use the crate in your project 🦀!

## License

This project is licensed under the **GPL-3.0**. For more information, see the [LICENSE](LICENSE) file.

## Contributing

Contributions are welcome! Please submit issues or pull requests to the repository at: [GitHub - ViniciosLugli/brctl-rs](https://github.com/ViniciosLugli/brctl-rs)

## Authors

- **Vinicios Lugli** - [vinicioslugli@gmail.com](mailto:vinicioslugli@gmail.com)