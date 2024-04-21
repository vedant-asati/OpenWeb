# Rust GET Request Project

This project demonstrates how to perform HTTP GET requests in Rust using the `reqwest` crate, with `anyhow` for simplified error handling.

## Getting Started

Follow these steps to get the project up and running on your local machine.

### Prerequisites

- Make sure you have [Rust installed](https://www.rust-lang.org/tools/install) on your machine.

### Dependencies

This project relies on the following crates:

- `reqwest`: A high-level HTTP client.
- `anyhow`: A crate for flexible error handling.

These dependencies are specified in the `Cargo.toml` file as follows:

```toml
[dependencies]
reqwest = { version = "0.10", features = ["blocking", "json"] }
anyhow = "1.0"
```
### Running the Project
1. Clone the repository to your local machine.
2. Navigate to the project directory.
3. Run the project using the following command:
```bash
cargo run
```
This will send a GET request to http://httpbin.org/get and print the response status, headers, and body to the console.

### Contributing
Feel free to fork the repository, create a new branch for your work, and open a pull request if you'd like to contribute.

### License
This project is open-source and available under the MIT License.
