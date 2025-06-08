use crate::config::structure::{ProjectConfig, FrontendStack, BackendStack, ProjectType};
use serde_json::json;
use std::fs;
use std::path::Path;
use crate::utils::createfiles::{
    create_frontend_files, create_backend_files
};

pub fn create_fullstack_project(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
    let base_path = Path::new(&config.name);
    
    // Create frontend directory
    fs::create_dir_all(base_path.join("frontend"))?;
    create_frontend_files(config, &base_path.join("frontend"))?;
    
    // Create backend directory
    fs::create_dir_all(base_path.join("backend"))?;
    create_backend_files(config, &base_path.join("backend"))?;
    
    // Create docker-compose for full stack
    create_docker_compose(config)?;
    
    // Root package.json for workspace management
    let package_json = json!({
        "name": config.name,
        "version": "1.0.0",
        "private": true,
        "workspaces": ["frontend", "backend"],
        "scripts": {
            "dev": "concurrently \"npm run dev:frontend\" \"npm run dev:backend\"",
            "dev:frontend": "cd frontend && npm run dev",
            "dev:backend": "cd backend && npm run dev",
            "build": "npm run build:frontend && npm run build:backend",
            "build:frontend": "cd frontend && npm run build",
            "build:backend": "cd backend && npm run build"
        },
        "devDependencies": {
            "concurrently": "^7.6.0"
        }
    });
    
    fs::write(
        base_path.join("package.json"),
        serde_json::to_string_pretty(&package_json)?
    )?;
    
    Ok(())
}

pub fn create_frontend_project(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
    let base_path = Path::new(&config.name);
    create_frontend_files(config, base_path)?;
    create_dockerfile_frontend(config)?;
    Ok(())
}

pub fn create_backend_project(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
    let base_path = Path::new(&config.name);
    create_backend_files(config, base_path)?;
    create_dockerfile_backend(config)?;
    Ok(())
}

pub fn create_cli_project(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
    let base_path = Path::new(&config.name);
    
    // Create Cargo.toml
    let cargo_toml = format!(r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = {{ version = "4.0", features = ["derive"] }}
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
tokio = {{ version = "1.0", features = ["full"] }}
"#, config.name);
    
    fs::write(base_path.join("Cargo.toml"), cargo_toml)?;
    
    // Create src directory and main.rs
    fs::create_dir_all(base_path.join("src"))?;
    let main_rs = format!(r#"use clap::{{Arg, Command}};

fn main() {{
    let matches = Command::new("{}")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("A CLI tool built with Rust")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Sets the input file to use")
        )
        .get_matches();

    if let Some(input) = matches.get_one::<String>("input") {{
        println!("Input file: {{}}", input);
    }} else {{
        println!("Hello from {}!");
    }}
}}
"#, config.name, config.name);
    
    fs::write(base_path.join("src").join("main.rs"), main_rs)?;
    
    Ok(())
}

pub fn create_extension_project(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
    let base_path = Path::new(&config.name);
    
    // Create manifest.json
    let manifest = json!({
        "manifest_version": 3,
        "name": config.name,
        "version": "1.0.0",
        "description": "A web extension",
        "permissions": ["activeTab"],
        "action": {
            "default_popup": "popup.html",
            "default_title": config.name
        },
        "content_scripts": [{
            "matches": ["<all_urls>"],
            "js": ["content.js"]
        }],
        "background": {
            "service_worker": "background.js"
        }
    });
    
    fs::write(
        base_path.join("manifest.json"), 
        serde_json::to_string_pretty(&manifest)?
    )?;
    
    // Create popup.html
    let popup_html = format!(r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <style>
        body {{
            width: 300px;
            padding: 20px;
        }}
        h1 {{
            font-size: 18px;
            margin-bottom: 10px;
        }}
    </style>
</head>
<body>
    <h1>{}</h1>
    <p>Welcome to your web extension!</p>
    <button id="action-btn">Click me</button>
    <script src="popup.js"></script>
</body>
</html>"#, config.name);
    
    fs::write(base_path.join("popup.html"), popup_html)?;
    
    // Create JavaScript files
    fs::write(base_path.join("popup.js"), 
        r#"document.getElementById('action-btn').addEventListener('click', () => {
    chrome.tabs.query({active: true, currentWindow: true}, (tabs) => {
        chrome.tabs.sendMessage(tabs[0].id, {action: 'hello'});
    });
});"#)?;
    
    fs::write(base_path.join("content.js"), 
        r#"chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
    if (request.action === 'hello') {
        console.log('Hello from content script!');
        alert('Hello from your extension!');
    }
});"#)?;
    
    fs::write(base_path.join("background.js"), 
        r#"chrome.runtime.onInstalled.addListener(() => {
    console.log('Extension installed');
});"#)?;
    
    // Create package.json for build tools
    let package_json = json!({
        "name": config.name,
        "version": "1.0.0",
        "description": "A web extension",
        "scripts": {
            "build": "echo 'Extension built successfully'",
            "zip": "zip -r extension.zip . -x node_modules/\\* .git/\\*"
        }
    });
    
    fs::write(
        base_path.join("package.json"),
        serde_json::to_string_pretty(&package_json)?
    )?;
    
    Ok(())
}


pub fn create_gitignore(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut gitignore_content = String::new();
    
    // Common ignores
    gitignore_content.push_str("# Dependencies\nnode_modules/\n");
    gitignore_content.push_str("# Environment variables\n.env\n.env.local\n.env.*.local\n");
    gitignore_content.push_str("# Logs\n*.log\nlogs/\n");
    gitignore_content.push_str("# OS generated files\n.DS_Store\nThumbs.db\n");
    gitignore_content.push_str("# IDE files\n.vscode/\n.idea/\n*.swp\n*.swo\n");
    
    match config.project_type {
        ProjectType::CliTool | ProjectType::Backend if matches!(config.backend_stack, Some(BackendStack::Rust)) => {
            gitignore_content.push_str("\n# Rust\ntarget/\nCargo.lock\n");
        }
        ProjectType::Frontend | ProjectType::FullStackWeb => {
            gitignore_content.push_str("\n# Build outputs\ndist/\nbuild/\n.next/\n");
            gitignore_content.push_str("# Coverage\ncoverage/\n");
        }
        ProjectType::Backend if matches!(config.backend_stack, Some(BackendStack::Python)) => {
            gitignore_content.push_str("\n# Python\n__pycache__/\n*.pyc\n*.pyo\n*.pyd\nvenv/\n.venv/\n");
        }
        ProjectType::WebExtension => {
            gitignore_content.push_str("\n# Extension builds\nextension.zip\ndist/\n");
        }
        _ => {}
    }
    
    fs::write(Path::new(&config.name).join(".gitignore"), gitignore_content)?;
    Ok(())
}

pub fn create_readme(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut readme_content = format!("# {}\n\n", config.name);
    
    match config.project_type {
        ProjectType::FullStackWeb => {
            readme_content.push_str(&format!(
                "A full-stack web application built with {} and {}.\n\n",
                match config.frontend_stack.as_ref().unwrap() {
                    FrontendStack::React => "React",
                    FrontendStack::ReactTs => "React (TypeScript)",
                    FrontendStack::Vue => "Vue.js",
                    FrontendStack::VueTs => "Vue.js (TypeScript)",
                    FrontendStack::Angular => "Angular",
                    FrontendStack::Svelte => "Svelte",
                    FrontendStack::SvelteTs => "Svelte (TypeScript)",
                    FrontendStack::NextJs => "Next.js",
                    FrontendStack::NextJsTs => "Next.js (TypeScript)",
                    FrontendStack::Vanilla => "Vanilla JavaScript",
                    FrontendStack::VanillaTs => "Vanilla TypeScript",
                },
                match config.backend_stack.as_ref().unwrap() {
                    BackendStack::NodeJs => "Node.js",
                    BackendStack::NodeJsTs => "Node.js (TypeScript)",
                    BackendStack::Python => "Python/FastAPI",
                    BackendStack::Rust => "Rust/Actix",
                    BackendStack::Go => "Go",
                    BackendStack::Java => "Java",
                    BackendStack::Php => "PHP",
                }
            ));
            readme_content.push_str("## Project Structure\n\n");
            readme_content.push_str("```\n");
            readme_content.push_str("├── frontend/     # Frontend application\n");
            readme_content.push_str("├── backend/      # Backend API\n");
            readme_content.push_str("├── docker-compose.yml\n");
            readme_content.push_str("└── README.md\n");
            readme_content.push_str("```\n\n");
            readme_content.push_str("## Getting Started\n\n");
            readme_content.push_str("1. Install dependencies:\n");
            readme_content.push_str("   ```bash\n   npm install\n   ```\n\n");
            readme_content.push_str("2. Start development servers:\n");
            readme_content.push_str("   ```bash\n   npm run dev\n   ```\n\n");
            readme_content.push_str("3. Or use Docker:\n");
            readme_content.push_str("   ```bash\n   docker-compose up\n   ```\n\n");
        }
        ProjectType::Frontend => {
            readme_content.push_str(&format!(
                "A frontend application built with {}.\n\n",
                match config.frontend_stack.as_ref().unwrap() {
                    FrontendStack::React => "React",
                    FrontendStack::ReactTs => "React (TypeScript)",
                    FrontendStack::Vue => "Vue.js",
                    FrontendStack::VueTs => "Vue.js (TypeScript)",
                    FrontendStack::Angular => "Angular",
                    FrontendStack::Svelte => "Svelte",
                    FrontendStack::SvelteTs => "Svelte (TypeScript)",
                    FrontendStack::NextJs => "Next.js",
                    FrontendStack::NextJsTs => "Next.js (TypeScript)",
                    FrontendStack::Vanilla => "Vanilla JavaScript",
                    FrontendStack::VanillaTs => "Vanilla TypeScript",
                }
            ));
            readme_content.push_str("## Getting Started\n\n");
            readme_content.push_str("1. Install dependencies:\n");
            readme_content.push_str("   ```bash\n   npm install\n   ```\n\n");
            readme_content.push_str("2. Start development server:\n");
            readme_content.push_str("   ```bash\n   npm run dev\n   ```\n\n");
            readme_content.push_str("3. Build for production:\n");
            readme_content.push_str("   ```bash\n   npm run build\n   ```\n\n");
        }
        ProjectType::Backend => {
            readme_content.push_str(&format!(
                "A backend API built with {}.\n\n",
                match config.backend_stack.as_ref().unwrap() {
                    BackendStack::NodeJs => "Node.js/Express",
                    BackendStack::NodeJsTs => "Node.js/Express (TypeScript)",
                    BackendStack::Python => "Python/FastAPI",
                    BackendStack::Rust => "Rust/Actix",
                    BackendStack::Go => "Go",
                    BackendStack::Java => "Java",
                    BackendStack::Php => "PHP",
                }
            ));
            readme_content.push_str("## Getting Started\n\n");
            match config.backend_stack.as_ref().unwrap() {
                BackendStack::NodeJs | BackendStack::NodeJsTs => {
                    readme_content.push_str("1. Install dependencies:\n");
                    readme_content.push_str("   ```bash\n   npm install\n   ```\n\n");
                    readme_content.push_str("2. Start development server:\n");
                    readme_content.push_str("   ```bash\n   npm run dev\n   ```\n\n");
                }
                BackendStack::Python => {
                    readme_content.push_str("1. Create virtual environment:\n");
                    readme_content.push_str("   ```bash\n   python -m venv venv\n   source venv/bin/activate  # On Windows: venv\\Scripts\\activate\n   ```\n\n");
                    readme_content.push_str("2. Install dependencies:\n");
                    readme_content.push_str("   ```bash\n   pip install -r requirements.txt\n   ```\n\n");
                    readme_content.push_str("3. Start development server:\n");
                    readme_content.push_str("   ```bash\n   uvicorn main:app --reload\n   ```\n\n");
                }
                BackendStack::Rust => {
                    readme_content.push_str("1. Build and run:\n");
                    readme_content.push_str("   ```bash\n   cargo run\n   ```\n\n");
                    readme_content.push_str("2. Build for production:\n");
                    readme_content.push_str("   ```bash\n   cargo build --release\n   ```\n\n");
                }
                _ => {}
            }
        }
        ProjectType::CliTool => {
            readme_content.push_str("A command-line tool built with Rust.\n\n");
            readme_content.push_str("## Getting Started\n\n");
            readme_content.push_str("1. Build the project:\n");
            readme_content.push_str("   ```bash\n   cargo build\n   ```\n\n");
            readme_content.push_str("2. Run the tool:\n");
            readme_content.push_str("   ```bash\n   cargo run\n   ```\n\n");
            readme_content.push_str("3. Install globally:\n");
            readme_content.push_str("   ```bash\n   cargo install --path .\n   ```\n\n");
        }
        ProjectType::WebExtension => {
            readme_content.push_str("A web browser extension.\n\n");
            readme_content.push_str("## Getting Started\n\n");
            readme_content.push_str("1. Install dependencies (if any):\n");
            readme_content.push_str("   ```bash\n   npm install\n   ```\n\n");
            readme_content.push_str("2. Load the extension in your browser:\n");
            readme_content.push_str("   - Open Chrome/Edge and go to `chrome://extensions/`\n");
            readme_content.push_str("   - Enable \"Developer mode\"\n");
            readme_content.push_str("   - Click \"Load unpacked\" and select this directory\n\n");
            readme_content.push_str("3. Build for distribution:\n");
            readme_content.push_str("   ```bash\n   npm run zip\n   ```\n\n");
        }
    }
    
    readme_content.push_str("## Environment Variables\n\n");
    readme_content.push_str("Copy `.env.example` to `.env` and configure your environment variables.\n\n");
    
    readme_content.push_str("## Contributing\n\n");
    readme_content.push_str("1. Fork the repository\n");
    readme_content.push_str("2. Create your feature branch (`git checkout -b feature/amazing-feature`)\n");
    readme_content.push_str("3. Commit your changes (`git commit -m 'Add some amazing feature'`)\n");
    readme_content.push_str("4. Push to the branch (`git push origin feature/amazing-feature`)\n");
    readme_content.push_str("5. Open a Pull Request\n\n");
    
    readme_content.push_str("## License\n\n");
    readme_content.push_str("This project is licensed under the MIT License.\n");
    
    fs::write(Path::new(&config.name).join("README.md"), readme_content)?;
    Ok(())
}

pub fn create_env_file(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut env_content = String::new();
    
    match config.project_type {
        ProjectType::FullStackWeb | ProjectType::Backend => {
            env_content.push_str("# Database\n");
            env_content.push_str("DATABASE_URL=mongodb://localhost:27017/myapp\n");
            env_content.push_str("# or DATABASE_URL=postgresql://user:password@localhost:5432/myapp\n\n");
            
            env_content.push_str("# API Configuration\n");
            env_content.push_str("PORT=3001\n");
            env_content.push_str("NODE_ENV=development\n\n");
            
            env_content.push_str("# Authentication\n");
            env_content.push_str("JWT_SECRET=your-super-secret-jwt-key-change-this-in-production\n");
            env_content.push_str("JWT_EXPIRES_IN=7d\n\n");
            
            env_content.push_str("# External APIs\n");
            env_content.push_str("API_KEY=your-api-key-here\n");
            env_content.push_str("# STRIPE_SECRET_KEY=sk_test_...\n");
            env_content.push_str("# SENDGRID_API_KEY=SG...\n\n");
        }
        ProjectType::Frontend => {
            env_content.push_str("# API Configuration\n");
            env_content.push_str("VITE_API_URL=http://localhost:3001\n");
            env_content.push_str("VITE_APP_NAME=");
            env_content.push_str(&config.name);
            env_content.push_str("\n\n");
            
            env_content.push_str("# External Services\n");
            env_content.push_str("VITE_GOOGLE_ANALYTICS_ID=G-XXXXXXXXXX\n");
            env_content.push_str("# VITE_STRIPE_PUBLIC_KEY=pk_test_...\n\n");
        }
        _ => {
            env_content.push_str("# Environment Variables\n");
            env_content.push_str("# Add your environment variables here\n");
            env_content.push_str("DEBUG=true\n");
        }
    }
    
    fs::write(Path::new(&config.name).join(".env.example"), &env_content)?;
    fs::write(Path::new(&config.name).join(".env"), env_content)?;
    
    Ok(())
}

pub fn create_dockerfile_frontend(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
    let dockerfile_content = r#"# Build stage
FROM node:18-alpine AS builder

WORKDIR /app
COPY package*.json ./
RUN npm ci --only=production

COPY . .
RUN npm run build

# Production stage
FROM nginx:alpine

COPY --from=builder /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/nginx.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]"#;
    
    fs::write(Path::new(&config.name).join("Dockerfile"), dockerfile_content)?;
    
    // Create nginx.conf
    let nginx_conf = r#"events {
    worker_connections 1024;
}

http {
    include       /etc/nginx/mime.types;
    default_type  application/octet-stream;
    
    server {
        listen 80;
        server_name localhost;
        root /usr/share/nginx/html;
        index index.html;
        
        location / {
            try_files $uri $uri/ /index.html;
        }
        
        location /api {
            proxy_pass http://backend:3001;
            proxy_set_header Host $host;
            proxy_set_header X-Real-IP $remote_addr;
        }
    }
}"#;
    
    fs::write(Path::new(&config.name).join("nginx.conf"), nginx_conf)?;
    
    Ok(())
}

pub fn create_dockerfile_backend(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
    let dockerfile_content = match config.backend_stack.as_ref().unwrap() {
        BackendStack::NodeJs => {
            r#"FROM node:18-alpine

WORKDIR /app

COPY package*.json ./
RUN npm ci --only=production

COPY . .

EXPOSE 3001

CMD ["npm", "start"]"#
        }
        BackendStack::Python => {
            r#"FROM python:3.11-slim

WORKDIR /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

COPY . .

EXPOSE 8000

CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8000"]"#
        }
        BackendStack::Rust => {
            r#"# Build stage
FROM rust:1.70 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/app /usr/local/bin/app

EXPOSE 8080

CMD ["app"]"#
        }
        _ => {
            r#"FROM node:18-alpine

WORKDIR /app

COPY package*.json ./
RUN npm ci --only=production

COPY . .

EXPOSE 3001

CMD ["npm", "start"]"#
        }
    };
    
    fs::write(Path::new(&config.name).join("Dockerfile"), dockerfile_content)?;
    
    Ok(())
}

pub fn create_docker_compose(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
    let compose_content = r#"version: '3.8'

services:
  frontend:
    build: ./frontend
    ports:
      - "3000:80"
    depends_on:
      - backend
    environment:
      - VITE_API_URL=http://localhost:3001

  backend:
    build: ./backend
    ports:
      - "3001:3001"
    environment:
      - NODE_ENV=development
      - PORT=3001
    depends_on:
      - database

  database:
    image: postgres:15-alpine
    environment:
      - POSTGRES_DB=myapp
      - POSTGRES_USER=user
      - POSTGRES_PASSWORD=password
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data

  redis:
    image: redis:7-alpine
    ports:
      - "6379:6379"

volumes:
  postgres_data:"#;
    
    fs::write(Path::new(&config.name).join("docker-compose.yml"), compose_content)?;
    
    Ok(())
}
