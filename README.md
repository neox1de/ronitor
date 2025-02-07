# Ronitor

Ronitor is a lightweight system monitoring and management utility written in Rust. It uses Actix Web to expose RESTful endpoints that provide real-time system metrics, network usage, kernel logs, process information, and more powerful features to come.

## Features

- **Health Check:** Verify that the API is alive.
- **System Metrics:** Retrieve detailed CPU, memory, and disk information.
- **Network Information:** List interfaces, IP addresses, MAC addresses, and network usage.
- **Kernel Information:** Get kernel version, release, architecture, uptime, kernel parameters, loaded modules, and recent logs.
- **Process Information:** List running processes along with their resource usage.
- **Server Control:** Issue shutdown and reboot commands (requires proper privileges).
- **More to Come:** Future functionalities beyond real-time monitoring.

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed.
2. Clone the repository:
   ```bash
   git clone https://github.com/neox1de/ronitor.git
   cd ronitor
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```

## Running the Server

Before starting the server, optionally set the server port using the `RONITOR_PORT` environment variable. By default, the service runs on port `3301`.

```bash
export RONITOR_PORT=3301
cargo run
```

## API Endpoints

- **GET** `/health`  
  Checks if the API is alive.

- **GET** `/metrics`  
  Returns detailed system metrics (CPU, Memory, Disks).

- **GET** `/network`  
  Provides network interface details and usage data.

- **GET** `/kernel`  
  Returns kernel information including logs, parameters, and uptime.

- **GET** `/processes`  
  Lists active processes and their resource usage.

- **POST** `/control/shutdown`  
  Issues a shutdown command to the system (requires privileges).

- **POST** `/control/reboot`  
  Issues a reboot command to the system (requires privileges).

## Future Enhancements

Ronitor is under active development. Upcoming features will include advanced management capabilities, enhanced analytics, and greater customization options.

## Donations

Your support is appreciated!

- **BITCOIN:** bc1q97rfy9qkllth8tzpxfskkw6rqp7tsqkpgrkamk  
- **Ethereum:** 0xA569D9c1D3C1f839029ced839b1a2599592344eE

## Contributing

Contributions are welcome! Please fork the repository and submit pull requests for improvements or bug fixes. For major changes, open an issue to discuss your ideas.

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Acknowledgements

- Built with [Actix Web](https://actix.rs/)
- System metrics via [sysinfo](https://docs.rs/sysinfo/)
- Logging using [env_logger](https://docs.rs/env_logger/)
- And many other great open source libraries.
