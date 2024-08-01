# Bitcoin-Privacy

This repository contains the source code for the Bitcoin-Privacy project. It includes the following features:

- [api] for accessing Bitcoin-Privacy functionality
- [frontend] for user interaction
- [shared] reused module for api and frontend

## Getting Started

To get started with the project, follow these steps:

### Prerequisites

Before you begin, ensure you have met the following requirements:

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/) (version 16.x or later)
- [Yarn](https://classic.yarnpkg.com/en/docs/install) (version 1.x)
- [Tauri prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)
- [cargo-watch](https://github.com/watchexec/cargo-watch#installation): Install via Cargo
  ```bash
  cargo install cargo-watch
  ```
- [cargo-shuttle](https://docs.rs/crate/cargo-shuttle/latest): Install via Cargo
  ```bash
  cargo install cargo-shuttle
  ```

### Step 1: Clone the Repository

Clone the repository to your local machine using the following command:

```bash
git clone https://github.com/your-username/bitcoin-privacy.git
cd bitcoin-privacy
```

### Step 2: Install Dependencies

Navigate to the frontend directory and install the necessary dependencies using Yarn:

```bash
make frontend-setup
```

### Step 3: Build the Project

To build the entire project, use the following commands:

#### Build the API and Shared Modules

```bash
cd api
cargo build --release

cd ../shared
cargo build --release
```

#### Build the Frontend

```bash
make frontend-setup
```

### Step 4: Run the Project

After building the project, you can run the application using the provided `make` commands.

#### Run the Frontend

```bash
make fe
```

This command will start the Tauri development environment for the frontend.

#### Run the Backend

```bash
make be
```

This command will start the backend using `cargo watch` and `shuttle`.

#### Run Both Frontend and Backend Together

```bash
make start
```

This command will start both the frontend and backend in a `tmux` session.
