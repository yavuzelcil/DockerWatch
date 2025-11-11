# DockWatch 🐳

A powerful and lightweight CLI tool for monitoring and managing Docker containers and images, written in Rust.

## Features ✨

- 📋 **List Containers**: View all running or stopped containers
- 🖼️ **List Images**: Display all Docker images on your system
- ▶️ **Start Containers**: Start containers with a simple command
- ⏹️ **Stop Containers**: Gracefully stop running containers
- ⬇️ **Pull Images**: Download Docker images from registries

## Installation 🚀

### From Source

```bash
git clone https://github.com/yavuzelcil/DockerWatch.git
cd DockerWatch
cargo install --path .
```

After installation, the binary will be available as `dw`.

## Usage 📖

### List Containers

```bash
# List only running containers
dw list containers

# List all containers (including stopped)
dw list containers --all
dw list containers -a
```

### List Images

```bash
dw list images
```

### Start a Container

```bash
dw start --container <container_id>
dw start -c <container_id>
```

### Stop a Container

```bash
dw stop --container <container_id>
dw stop -c <container_id>
```

### Pull an Image

```bash
dw pull --image nginx:latest
dw pull -i postgres:16
```

## Requirements 📦

- Rust 1.70 or higher
- Docker daemon running
- Docker socket accessible at `~/.docker/run/docker.sock`

## Tech Stack 🛠️

- **[Bollard](https://github.com/fussybeaver/bollard)**: Docker API client for Rust
- **[Clap](https://github.com/clap-rs/clap)**: Command-line argument parser
- **[Tokio](https://tokio.rs/)**: Asynchronous runtime

## Development 💻

```bash
# Build the project
cargo build

# Run tests
cargo test

# Run with cargo
cargo run -- list containers --all
```

## Contributing 🤝

Contributions are welcome! Feel free to open issues or submit pull requests.

## License 📄

This project is open source and available under the MIT License.

---

Built with Rust 🦀 • In the name of freedom 🕊️