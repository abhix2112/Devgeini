# 🚀 Devgeini - Modern CLI to Initialize Your Development Projects

**Devgeini** is a powerful Rust-based command-line tool designed to help developers quickly scaffold project directories and boilerplates across frontend, backend, and full-stack environments.

---

## 📦 Features

- 🔧 Initialize full-stack project structures in seconds
- 🧱 Support for multiple frontend & backend stacks (React, Next.js, Node.js, etc.)
- ⚙️ Extensible configuration via TOML
- 🚀 Fast and efficient (built with Rust)
- 🐳 Easy containerization with Docker

---

## 🛠️ Installation

### 🔧 Manual Build

```bash
git clone https://github.com/yourusername/devgeini.git
cd devgeini
cargo build --release
./target/release/devgeini init
```

---

## 🐳 Docker Build

```bash
docker build -t devgeini .
docker run --rm -v $(pwd):/app devgeini init
The init command will prompt you for configuration and create your project structure accordingly.
```

---

## 🧰 Usage

```bash
devgeini init
You will be guided through a CLI wizard to select:

Project type: frontend, backend, or fullstack

Stack preferences (e.g. React, Next.js, Express.js, Rust, etc.)

Folder naming conventions

Optional integrations (Docker, CI/CD, etc.)

```

---

## 🗂️ Example Output

```bash
my-app/
├── backend/
│   ├── src/
│   └── Cargo.toml
├── frontend/
│   ├── src/
│   └── package.json
├── .gitignore
└── README.md
```

---

###📄 Configuration
A project_config.toml file is generated automatically. You can edit it to tweak defaults for:

toml

```bash
project_name = "my-app"
project_type = "fullstack"
frontend_stack = "nextjs"
backend_stack = "rust"
```

### 🧑‍💻 Contributing

## Contributions are welcome! Please fork the repo, create a new branch, and open a PR.

### 📜 License

## MIT © 2025 Abhishek Aggarwal

### 🙌 Acknowledgments

Built with ❤️ in Rust for devs who care about clean project structures and speed.
