use clap::{Arg, Command};
use dialoguer::{theme::ColorfulTheme, Select, Input, Confirm};
use serde_json::json;
use std::fs;
use std::path::Path;
use std::io::{self, Write};
use std::process::{Command as ShellCommand, Stdio};
use std::collections::HashMap;



#[derive(Debug, Clone)]
enum ProjectType {
    FullStackWeb,
    Frontend,
    Backend,
    CliTool,
    WebExtension,
}

#[derive(Debug, Clone)]
enum FrontendStack {
    React,
    ReactTs,
    Vue,
    VueTs,
    Angular,
    Svelte,
    SvelteTs,
    NextJs,
    NextJsTs,
    Vanilla,
    VanillaTs,
}

#[derive(Debug, Clone)]
enum BackendStack {
    NodeJs,
    NodeJsTs,
    Python,
    Rust,
    Go,
    Java,
    Php,
    
}

struct ProjectConfig {
    name: String,
    project_type: ProjectType,
    frontend_stack: Option<FrontendStack>,
    backend_stack: Option<BackendStack>,
}

fn main() {
    let matches = Command::new("devgeini")
        .version("1.0.0")
        .author("DevGeini Team")
        .about("Initialize development projects with proper structure and boilerplate")
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .value_name("PROJECT_NAME")
                .help("Sets the project name")
        )
        .arg(
            Arg::new("interactive")
                .short('i')
                .long("interactive")
                .action(clap::ArgAction::SetTrue)
                .help("Run in interactive mode")
        )
        .get_matches();

    let project_name = if let Some(name) = matches.get_one::<String>("name") {
        name.clone()
    } else {
        get_project_name()
    };

    let config = if matches.get_flag("interactive") || matches.get_one::<String>("name").is_none() {
        get_project_config_interactive(project_name)
    } else {
        // Quick mode - just get project type
        let project_type = select_project_type();
        let mut config = ProjectConfig {
            name: project_name,
            project_type: project_type.clone(),
            frontend_stack: None,
            backend_stack: None,
        };
        
        match project_type {
            ProjectType::FullStackWeb => {
                config.frontend_stack = Some(select_frontend_stack());
                config.backend_stack = Some(select_backend_stack());
            }
            ProjectType::Frontend => {
                config.frontend_stack = Some(select_frontend_stack());
            }
            ProjectType::Backend => {
                config.backend_stack = Some(select_backend_stack());
            }
            _ => {}
        }
        
        config
    };

    if let Err(e) = create_project(&config) {
        eprintln!("Error creating project: {}", e);
        std::process::exit(1);
    }

    println!("🎉 Project '{}' created successfully!", config.name);
    println!("📁 Navigate to your project: cd {}", config.name);
    
    match config.project_type {
        ProjectType::FullStackWeb | ProjectType::Frontend | ProjectType::Backend => {
            println!("📦 Install dependencies: npm install");
            println!("🚀 Start development: npm run dev");
        }
        ProjectType::CliTool => {
            println!("🦀 Build project: cargo build");
            println!("🏃 Run project: cargo run");
        }
        ProjectType::WebExtension => {
            println!("📦 Install dependencies: npm install");
            println!("🔧 Build extension: npm run build");
        }
    }
}
    


fn get_project_name() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter project name")
        .interact_text()
        .unwrap()
}

fn get_project_config_interactive(name: String) -> ProjectConfig {
    let project_type = select_project_type();
    
    let mut config = ProjectConfig {
        name,
        project_type: project_type.clone(),
        frontend_stack: None,
        backend_stack: None,
    };

    match project_type {
        ProjectType::FullStackWeb => {
            config.frontend_stack = Some(select_frontend_stack());
            config.backend_stack = Some(select_backend_stack());
        }
        ProjectType::Frontend => {
            config.frontend_stack = Some(select_frontend_stack());
        }
        ProjectType::Backend => {
            config.backend_stack = Some(select_backend_stack());
        }
        _ => {}
    }

    config
}

fn select_project_type() -> ProjectType {
    let options = vec![
        "Full Stack Web Application",
        "Frontend Only",
        "Backend Only", 
        "CLI Tool",
        "Web Extension",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select project type")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match selection {
        0 => ProjectType::FullStackWeb,
        1 => ProjectType::Frontend,
        2 => ProjectType::Backend,
        3 => ProjectType::CliTool,
        4 => ProjectType::WebExtension,
        _ => ProjectType::FullStackWeb,
    }
}

fn select_frontend_stack() -> FrontendStack {
    let options = vec![
        "React (TypeScript)",
        "Vue.js (TypeScript)", 
        "Svelte (TypeScript)",
        "Next.js (TypeScript)",
        "Vanilla JavaScript",
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select frontend technology")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match selection {
        0 => FrontendStack::ReactTs,
        1 => FrontendStack::React,
        2 => FrontendStack::VueTs,
        3 => FrontendStack::Vue,
        4 => FrontendStack::Angular,
        5 => FrontendStack::SvelteTs,
        6 => FrontendStack::Svelte,
        7 => FrontendStack::NextJsTs,
        8 => FrontendStack::NextJs,
        9 => FrontendStack::VanillaTs,
        10 => FrontendStack::Vanilla,
        _ => FrontendStack::ReactTs,
    }
}

fn select_backend_stack() -> BackendStack {
    let options = vec![
        "Node.js (TypeScript)",
        "Node.js (JavaScript)",
        "Python (FastAPI)",
        "Rust (Actix)",
        "Go"
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select backend technology")
        .items(&options)
        .default(0)
        .interact()
        .unwrap();

    match selection {
        0 => BackendStack::NodeJsTs,
        1 => BackendStack::NodeJs,
        2 => BackendStack::Python,
        3 => BackendStack::Rust,
        4 => BackendStack::Go,
        5 => BackendStack::Java,
        6 => BackendStack::Php,
        _ => BackendStack::NodeJsTs,
    }
}

fn create_project(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
    let project_path = Path::new(&config.name);
    
    if project_path.exists() {
        let overwrite = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Project directory already exists. Overwrite?")
            .interact()?;
        
        if !overwrite {
            return Ok(());
        }
        
        fs::remove_dir_all(project_path)?;
    }

    fs::create_dir_all(project_path)?;

    // Create project structure
    match config.project_type {
        ProjectType::FullStackWeb => create_fullstack_project(config)?,
        ProjectType::Frontend => create_frontend_project(config)?,
        ProjectType::Backend => create_backend_project(config)?,
        ProjectType::CliTool => create_cli_project(config)?,
        ProjectType::WebExtension => create_extension_project(config)?,
    }

    // Create common files
    create_gitignore(config)?;
    create_readme(config)?;
    create_env_file(config)?;

    // 🎯 NEW: Auto-install dependencies
    println!("\n🔧 Setting up project dependencies...");
    
    

    Ok(())
}


fn create_fullstack_project(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
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

fn create_frontend_project(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
    let base_path = Path::new(&config.name);
    create_frontend_files(config, base_path)?;
    create_dockerfile_frontend(config)?;
    Ok(())
}

fn create_backend_project(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
    let base_path = Path::new(&config.name);
    create_backend_files(config, base_path)?;
    create_dockerfile_backend(config)?;
    Ok(())
}

fn create_cli_project(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
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

fn create_extension_project(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
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

fn create_frontend_files(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let frontend_stack = config.frontend_stack.as_ref().unwrap();
    
    match frontend_stack {
        FrontendStack::React => create_react_project(config, path)?,
        FrontendStack::ReactTs => create_react_ts_project(config, path)?,
        FrontendStack::Vue => create_vue_project(config, path)?,
        FrontendStack::VueTs => create_vue_ts_project(config, path)?,
        FrontendStack::NextJs => create_nextjs_project(config, path)?,
        FrontendStack::NextJsTs => create_nextjs_ts_project(config, path)?,
        FrontendStack::Svelte => create_svelte_project(config, path)?,
        FrontendStack::SvelteTs => create_svelte_ts_project(config, path)?,
        FrontendStack::Vanilla => create_vanilla_project(config, path)?,
        FrontendStack::VanillaTs => create_vanilla_ts_project(config, path)?,
        FrontendStack::Angular => create_angular_project(config, path)?,
    }
    
    Ok(())
}

pub fn create_react_project(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Create package.json
    let package_json = json!({
        "name": config.name,
        "version": "1.0.0",
        "private": true,
        "dependencies": {
            "react": "^18.2.0",
            "react-dom": "^18.2.0"
        },
        "devDependencies": {
            "@vitejs/plugin-react": "^4.0.0",
            "vite": "^4.4.0"
        },
        "scripts": {
            "dev": "vite",
            "build": "vite build",
            "preview": "vite preview"
        }
    });
    
    fs::write(path.join("package.json"), serde_json::to_string_pretty(&package_json)?)?;
    
    // Create src directory and files
    fs::create_dir_all(path.join("src"))?;
    fs::create_dir_all(path.join("public"))?;
    
    // Create index.html
    let index_html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{}</title>
</head>
<body>
    <div id="root"></div>
    <script type="module" src="/src/main.jsx"></script>
</body>
</html>"#, config.name);
    
    fs::write(path.join("index.html"), index_html)?;
    
    // Create main.jsx
    fs::write(path.join("src").join("main.jsx"), 
        r#"import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.jsx'
import './index.css'

ReactDOM.createRoot(document.getElementById('root')).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)"#)?;
    
    // Create App.jsx
    let app_jsx = format!(r#"import {{ useState }} from 'react'
import './App.css'

function App() {{
  const [count, setCount] = useState(0)

  return (
    <div className="App">
      <h1>{}</h1>
      <div className="card">
        <button onClick={{() => setCount((count) => count + 1)}}>
          count is {{count}}
        </button>
        <p>
          Edit <code>src/App.jsx</code> and save to test HMR
        </p>
      </div>
    </div>
  )
}}

export default App"#, config.name);
    
    fs::write(path.join("src").join("App.jsx"), app_jsx)?;
    
    // Create CSS files
    fs::write(path.join("src").join("App.css"), 
        r#"#root {
  max-width: 1280px;
  margin: 0 auto;
  padding: 2rem;
  text-align: center;
}

.card {
  padding: 2em;
}

button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  background-color: #1a1a1a;
  cursor: pointer;
  transition: border-color 0.25s;
}

button:hover {
  border-color: #646cff;
}

button:focus,
button:focus-visible {
  outline: 4px auto -webkit-focus-ring-color;
}"#)?;
    
    fs::write(path.join("src").join("index.css"), 
        r#"body {
  margin: 0;
  display: flex;
  place-items: center;
  min-width: 320px;
  min-height: 100vh;
  font-family: Inter, system-ui, Avenir, Helvetica, Arial, sans-serif;
  line-height: 1.5;
  font-weight: 400;
}

h1 {
  font-size: 3.2em;
  line-height: 1.1;
}"#)?;
    
    // Create vite.config.js
    fs::write(path.join("vite.config.js"), 
        r#"import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

export default defineConfig({
  plugins: [react()],
})"#)?;
    
    Ok(())
}


fn create_vue_project(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let package_json = json!({
        "name": config.name,
        "version": "1.0.0",
        "private": true,
        "dependencies": {
            "vue": "^3.3.0"
        },
        "devDependencies": {
            "@vitejs/plugin-vue": "^4.2.0",
            "vite": "^4.4.0"
        },
        "scripts": {
            "dev": "vite",
            "build": "vite build",
            "preview": "vite preview"
        }
    });
    
    fs::write(path.join("package.json"), serde_json::to_string_pretty(&package_json)?)?;
    
    fs::create_dir_all(path.join("src"))?;
    
    let index_html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{}</title>
</head>
<body>
    <div id="app"></div>
    <script type="module" src="/src/main.js"></script>
</body>
</html>"#, config.name);
    
    fs::write(path.join("index.html"), index_html)?;
    
    fs::write(path.join("src").join("main.js"), 
        r#"import { createApp } from 'vue'
import App from './App.vue'

createApp(App).mount('#app')"#)?;
    
    let app_vue = format!(r#"<template>
  <div id="app">
    <h1>{}</h1>
    <button @click="count++">Count: {{ count }}</button>
  </div>
</template>

<script>
import {{ ref }} from 'vue'

export default {{
  name: 'App',
  setup() {{
    const count = ref(0)
    return {{ count }}
  }}
}}
</script>

<style>
#app {{
  font-family: Avenir, Helvetica, Arial, sans-serif;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}}
</style>"#, config.name);
    
    fs::write(path.join("src").join("App.vue"), app_vue)?;
    
    fs::write(path.join("vite.config.js"), 
        r#"import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [vue()]
})"#)?;
    
    Ok(())
}

fn create_nextjs_project(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let package_json = json!({
        "name": config.name,
        "version": "1.0.0",
        "private": true,
        "dependencies": {
            "next": "13.4.19",
            "react": "^18.2.0",
            "react-dom": "^18.2.0"
        },
        "scripts": {
            "dev": "next dev",
            "build": "next build",
            "start": "next start",
            "lint": "next lint"
        }
    });
    
    fs::write(path.join("package.json"), serde_json::to_string_pretty(&package_json)?)?;
    
    fs::create_dir_all(path.join("pages"))?;
    fs::create_dir_all(path.join("public"))?;
    
    let index_js = format!(r#"export default function Home() {{
  return (
    <div>
      <h1>Welcome to {}</h1>
      <p>Built with Next.js</p>
    </div>
  )
}}"#, config.name);
    
    fs::write(path.join("pages").join("index.js"), index_js)?;
    
    Ok(())
}

fn create_vanilla_project(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let package_json = json!({
        "name": config.name,
        "version": "1.0.0",
        "private": true,
        "devDependencies": {
            "vite": "^4.4.0"
        },
        "scripts": {
            "dev": "vite",
            "build": "vite build",
            "preview": "vite preview"
        }
    });
    
    fs::write(path.join("package.json"), serde_json::to_string_pretty(&package_json)?)?;
    
    let index_html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{}</title>
    <link rel="stylesheet" href="style.css">
</head>
<body>
    <div id="app">
        <h1>{}</h1>
        <button id="counter">Count: 0</button>
    </div>
    <script type="module" src="main.js"></script>
</body>
</html>"#, config.name, config.name);
    
    fs::write(path.join("index.html"), index_html)?;
    
    fs::write(path.join("main.js"), 
        r#"let count = 0;
const button = document.getElementById('counter');

button.addEventListener('click', () => {
    count++;
    button.textContent = `Count: ${count}`;
});"#)?;
    
    fs::write(path.join("style.css"), 
        r#"body {
    font-family: Arial, sans-serif;
    margin: 0;
    padding: 2rem;
    text-align: center;
}

#app {
    max-width: 800px;
    margin: 0 auto;
}

button {
    padding: 10px 20px;
    font-size: 16px;
    cursor: pointer;
}"#)?;
    
    Ok(())
}

fn create_backend_files(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let backend_stack = config.backend_stack.as_ref().unwrap();
    
    match backend_stack {
        BackendStack::NodeJs => create_nodejs_backend(config, path)?,
        BackendStack::NodeJsTs => create_nodejs_ts_backend(config, path)?,
        BackendStack::Python => create_python_backend(config, path)?,
        BackendStack::Rust => create_rust_backend(config, path)?,
        _ => create_nodejs_ts_backend(config, path)?, // Default to Node.js TS
    }
    
    Ok(())
}

fn create_nodejs_ts_backend(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let package_json = json!({
        "name": format!("{}-backend", config.name),
        "version": "1.0.0",
        "description": "Backend API with TypeScript",
        "main": "dist/index.js",
        "scripts": {
            "build": "tsc",
            "start": "node dist/index.js",
            "dev": "ts-node-dev --respawn --transpile-only src/index.ts",
            "watch": "tsc --watch"
        },
        "dependencies": {
            "express": "^4.18.0",
            "cors": "^2.8.5",
            "dotenv": "^16.0.0",
            "helmet": "^7.0.0",
            "compression": "^1.7.4"
        },
        "devDependencies": {
            "@types/node": "^20.0.0",
            "@types/express": "^4.17.0",
            "@types/cors": "^2.8.0",
            "@types/compression": "^1.7.0",
            "typescript": "^5.0.0",
            "ts-node-dev": "^2.0.0",
            "nodemon": "^3.0.0"
        }
    });
    
    fs::write(path.join("package.json"), serde_json::to_string_pretty(&package_json)?)?;
    
    // Create TypeScript config
    let tsconfig = json!({
        "compilerOptions": {
            "target": "ES2020",
            "module": "commonjs",
            "lib": ["ES2020"],
            "outDir": "./dist",
            "rootDir": "./src",
            "strict": true,
            "esModuleInterop": true,
            "skipLibCheck": true,
            "forceConsistentCasingInFileNames": true,
            "resolveJsonModule": true,
            "declaration": true,
            "declarationMap": true,
            "sourceMap": true
        },
        "include": ["src/**/*"],
        "exclude": ["node_modules", "dist"]
    });
    
    fs::write(path.join("tsconfig.json"), serde_json::to_string_pretty(&tsconfig)?)?;
    
    // Create source directory structure
    fs::create_dir_all(path.join("src"))?;
    fs::create_dir_all(path.join("src/routes"))?;
    fs::create_dir_all(path.join("src/controllers"))?;
    fs::create_dir_all(path.join("src/middleware"))?;
    fs::create_dir_all(path.join("src/types"))?;
    fs::create_dir_all(path.join("src/utils"))?;
    fs::create_dir_all(path.join("dist"))?;
    
    // Main TypeScript entry point
    let index_ts = r#"import express, { Request, Response, NextFunction } from 'express';
import cors from 'cors';
import helmet from 'helmet';
import compression from 'compression';
import dotenv from 'dotenv';
import { errorHandler } from './middleware/errorHandler';
import { logger } from './middleware/logger';
import healthRoutes from './routes/health';

dotenv.config();

const app = express();
const PORT = process.env.PORT || 3001;

// Security middleware
app.use(helmet());
app.use(compression());

// CORS configuration
app.use(cors({
    origin: process.env.FRONTEND_URL || 'http://localhost:3000',
    credentials: true
}));

// Body parsing middleware
app.use(express.json({ limit: '10mb' }));
app.use(express.urlencoded({ extended: true, limit: '10mb' }));

// Logging middleware
app.use(logger);

// Routes
app.get('/', (req: Request, res: Response) => {
    res.json({ 
        message: 'TypeScript API is running!',
        version: '1.0.0',
        timestamp: new Date().toISOString()
    });
});

app.use('/api/health', healthRoutes);

// 404 handler
app.use('*', (req: Request, res: Response) => {
    res.status(404).json({ 
        error: 'Route not found',
        path: req.originalUrl 
    });
});

// Error handling middleware
app.use(errorHandler);

app.listen(PORT, () => {
    console.log(`🚀 Server is running on port ${PORT}`);
    console.log(`📱 Environment: ${process.env.NODE_ENV || 'development'}`);
});"#;
    
    fs::write(path.join("src/index.ts"), index_ts)?;
    
    // Error handler middleware
    let error_handler = r#"import { Request, Response, NextFunction } from 'express';

export interface AppError extends Error {
    statusCode?: number;
    status?: string;
    isOperational?: boolean;
}

export const errorHandler = (
    err: AppError, 
    req: Request, 
    res: Response, 
    next: NextFunction
) => {
    err.statusCode = err.statusCode || 500;
    err.status = err.status || 'error';

    if (process.env.NODE_ENV === 'development') {
        res.status(err.statusCode).json({
            status: err.status,
            error: err,
            message: err.message,
            stack: err.stack
        });
    } else {
        // Production error response
        if (err.isOperational) {
            res.status(err.statusCode).json({
                status: err.status,
                message: err.message
            });
        } else {
            console.error('ERROR 💥', err);
            res.status(500).json({
                status: 'error',
                message: 'Something went wrong!'
            });
        }
    }
};"#;
    
    fs::write(path.join("src/middleware/errorHandler.ts"), error_handler)?;
    
    // Logger middleware
    let logger_middleware = r#"import { Request, Response, NextFunction } from 'express';

export const logger = (req: Request, res: Response, next: NextFunction) => {
    const start = Date.now();
    
    res.on('finish', () => {
        const duration = Date.now() - start;
        const timestamp = new Date().toISOString();
        
        console.log(
            `[${timestamp}] ${req.method} ${req.originalUrl} - ${res.statusCode} - ${duration}ms`
        );
    });
    
    next();
};"#;
    
    fs::write(path.join("src/middleware/logger.ts"), logger_middleware)?;
    
    // Health check routes
    let health_routes = r#"import { Router, Request, Response } from 'express';

const router = Router();

router.get('/', (req: Request, res: Response) => {
    res.json({
        status: 'OK',
        timestamp: new Date().toISOString(),
        uptime: process.uptime(),
        memory: process.memoryUsage(),
        version: process.version
    });
});

router.get('/ready', (req: Request, res: Response) => {
    // Add any readiness checks here (database connections, etc.)
    res.json({
        status: 'ready',
        timestamp: new Date().toISOString()
    });
});

router.get('/live', (req: Request, res: Response) => {
    res.json({
        status: 'alive',
        timestamp: new Date().toISOString()
    });
});

export default router;"#;
    
    fs::write(path.join("src/routes/health.ts"), health_routes)?;
    
    // Types definition file
    let types_file = r#"export interface ApiResponse<T = any> {
    success: boolean;
    data?: T;
    message?: string;
    error?: string;
}

export interface User {
    id: string;
    email: string;
    name: string;
    createdAt: Date;
    updatedAt: Date;
}

export interface RequestWithUser extends Request {
    user?: User;
}"#;
    
    fs::write(path.join("src/types/index.ts"), types_file)?;
    
    // Utility functions
    let utils_file = r#"export const asyncHandler = (fn: Function) => (req: any, res: any, next: any) => {
    Promise.resolve(fn(req, res, next)).catch(next);
};

export const createResponse = <T>(
    success: boolean, 
    data?: T, 
    message?: string
) => ({
    success,
    data,
    message,
    timestamp: new Date().toISOString()
});

export const validateEmail = (email: string): boolean => {
    const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
    return emailRegex.test(email);
};

export const generateId = (): string => {
    return Math.random().toString(36).substring(2) + Date.now().toString(36);
};"#;
    
    fs::write(path.join("src/utils/index.ts"), utils_file)?;
    
    // Environment variables template
    let env_example = r#"# Server Configuration
PORT=3001
NODE_ENV=development

# CORS Configuration
FRONTEND_URL=http://localhost:3000

# Database Configuration (uncomment when needed)
# DATABASE_URL=postgresql://username:password@localhost:5432/database_name
# REDIS_URL=redis://localhost:6379

# JWT Configuration (uncomment when needed)
# JWT_SECRET=your-super-secret-jwt-key
# JWT_EXPIRES_IN=7d

# Email Configuration (uncomment when needed)
# SMTP_HOST=smtp.gmail.com
# SMTP_PORT=587
# SMTP_USER=your-email@gmail.com
# SMTP_PASS=your-app-password"#;
    
    fs::write(path.join(".env.example"), env_example)?;
    
    // Create .env file
    fs::write(path.join(".env"), "PORT=3001\nNODE_ENV=development\nFRONTEND_URL=http://localhost:3000\n")?;
    
    // Create .gitignore
    let gitignore = r#"# Dependencies
node_modules/
npm-debug.log*
yarn-debug.log*
yarn-error.log*

# Runtime data
pids
*.pid
*.seed
*.pid.lock

# Directory for instrumented libs generated by jscoverage/JSCover
lib-cov

# Coverage directory used by tools like istanbul
coverage/

# Compiled binary addons
build/Release

# Dependency directories
node_modules/
jspm_packages/

# TypeScript output
dist/
*.tsbuildinfo

# Optional npm cache directory
.npm

# Optional eslint cache
.eslintcache

# Output of 'npm pack'
*.tgz

# Yarn Integrity file
.yarn-integrity

# dotenv environment variables file
.env
.env.local
.env.development.local
.env.test.local
.env.production.local

# Logs
logs
*.log

# Runtime data
pids
*.pid
*.seed
*.pid.lock

# Stores VSCode versions used for testing VSCode extensions
.vscode-test

# OS generated files
.DS_Store
.DS_Store?
._*
.Spotlight-V100
.Trashes
ehthumbs.db
Thumbs.db"#;
    
    fs::write(path.join(".gitignore"), gitignore)?;
    
    // Create README
    let readme = format!(r#"# {}-backend

TypeScript Node.js backend API server.

## Getting Started

1. Install dependencies:
```bash
npm install
```

2. Copy environment variables:
```bash
cp .env.example .env
```

3. Start development server:
```bash
npm run dev
```

4. Build for production:
```bash
npm run build
npm start
```

## API Endpoints

- `GET /` - API status
- `GET /api/health` - Health check
- `GET /api/health/ready` - Readiness check
- `GET /api/health/live` - Liveness check

## Project Structure

```
src/
├── controllers/     # Route controllers
├── middleware/      # Custom middleware
├── routes/         # API routes
├── types/          # TypeScript type definitions
├── utils/          # Utility functions
└── index.ts        # Application entry point
```

## Scripts

- `npm run dev` - Start development server with hot reload
- `npm run build` - Build TypeScript to JavaScript
- `npm start` - Start production server
- `npm run watch` - Watch for TypeScript changes
```"#, config.name);
    
    fs::write(path.join("README.md"), readme)?;
    
    Ok(())
}

fn create_nodejs_backend(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let package_json = json!({
        "name": format!("{}-backend", config.name),
        "version": "1.0.0",
        "description": "Backend API",
        "main": "index.js",
        "dependencies": {
            "express": "^4.18.0",
            "cors": "^2.8.5",
            "dotenv": "^16.0.0"
        },
        "devDependencies": {
            "nodemon": "^2.0.0"
        },
        "scripts": {
            "start": "node index.js",
            "dev": "nodemon index.js"
        }
    });
    
    fs::write(path.join("package.json"), serde_json::to_string_pretty(&package_json)?)?;
    
    let index_js = r#"const express = require('express');
const cors = require('cors');
require('dotenv').config();

const app = express();
const PORT = process.env.PORT || 3001;

app.use(cors());
app.use(express.json());

app.get('/', (req, res) => {
    res.json({ message: 'API is running!' });
});

app.get('/api/health', (req, res) => {
    res.json({ status: 'OK', timestamp: new Date().toISOString() });
});

app.listen(PORT, () => {
    console.log(`Server is running on port ${PORT}`);
});"#;
    
    fs::write(path.join("index.js"), index_js)?;
    
    fs::create_dir_all(path.join("routes"))?;
    fs::create_dir_all(path.join("controllers"))?;
    fs::create_dir_all(path.join("middleware"))?;
    
    Ok(())
}

fn create_python_backend(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Create requirements.txt
    fs::write(path.join("requirements.txt"), 
        r#"fastapi==0.104.1
uvicorn==0.24.0
python-dotenv==1.0.0
pydantic==2.5.0"#)?;
    
    // Create main.py
    let main_py = r#"from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
import os
from dotenv import load_dotenv

load_dotenv()

app = FastAPI()

app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

@app.get("/")
async def root():
    return {"message": "API is running!"}

@app.get("/api/health")
async def health():
    return {"status": "OK"}

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)"#;
    
    fs::write(path.join("main.py"), main_py)?;
    
    fs::create_dir_all(path.join("routers"))?;
    fs::create_dir_all(path.join("models"))?;
    
    Ok(())
}

fn create_rust_backend(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let cargo_toml = format!(r#"[package]
name = "{}-backend"
version = "0.1.0"
edition = "2021"

[dependencies]
actix-web = "4.4"
tokio = {{ version = "1.0", features = ["full"] }}
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
dotenv = "0.15"
"#, config.name);
    
    fs::write(path.join("Cargo.toml"), cargo_toml)?;
    
    fs::create_dir_all(path.join("src"))?;
    
    let main_rs = r#"use actix_web::{web, App, HttpResponse, HttpServer, Result, middleware::Logger};
use serde_json::json;

async fn hello() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({"message": "API is running!"})))
}

async fn health() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(json!({
        "status": "OK",
        "timestamp": chrono::Utc::now()
    })))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    
    println!("Starting server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/", web::get().to(hello))
            .route("/api/health", web::get().to(health))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}"#;
    
    fs::write(path.join("src").join("main.rs"), main_rs)?;
    
    Ok(())
}

fn create_gitignore(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
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

fn create_readme(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
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

fn create_env_file(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
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

fn create_dockerfile_frontend(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
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

fn create_dockerfile_backend(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
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

fn create_docker_compose(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
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

fn create_svelte_project(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Create package.json
    let package_json = json!({
        "name": config.name,
        "version": "1.0.0",
        "private": true,
        "dependencies": {
            "svelte": "^4.0.5"
        },
        "devDependencies": {
            "@sveltejs/adapter-auto": "^2.0.0",
            "@sveltejs/kit": "^1.20.4",
            "vite": "^4.4.2"
        },
        "scripts": {
            "dev": "vite",
            "build": "vite build",
            "preview": "vite preview"
        }
    });
    fs::write(path.join("package.json"), serde_json::to_string_pretty(&package_json)?)?;
    // Create src directory and main Svelte file
    fs::create_dir_all(path.join("src"))?;
    let app_svelte = format!(r#"<script>
    let count = 0;
</script>

<main>
    <h1>{}</h1>
    <button on:click={{() => count++}}>Count: {{count}}</button>
</main>

<style>
    main {{
        text-align: center;
        padding: 2rem;
        margin: 0 auto;
    }}
    button {{
        padding: 10px 20px;
        font-size: 16px;
        cursor: pointer;
    }}
</style>
"#, config.name);
    fs::write(path.join("src").join("App.svelte"), app_svelte)?;
    // Create main.js
    fs::write(path.join("src").join("main.js"), r#"import App from './App.svelte';

const app = new App({
    target: document.body
});

export default app;
"#)?;
    // Create index.html
    let index_html = format!(r#"<!DOCTYPE html>
<html lang=\"en\">
<head>
    <meta charset=\"UTF-8\" />
    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />
    <title>{}</title>
</head>
<body>
    <script type=\"module\" src=\"/src/main.js\"></script>
</body>
</html>"#, config.name);
    fs::write(path.join("index.html"), index_html)?;
    Ok(())
}

fn create_react_ts_project(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Create package.json with TypeScript deps
    let package_json = json!({
        "name": config.name,
        "version": "1.0.0",
        "private": true,
        "dependencies": {
            "react": "^18.2.0",
            "react-dom": "^18.2.0"
        },
        "devDependencies": {
            "@types/react": "^18.2.0",
            "@types/react-dom": "^18.2.0",
            "@typescript-eslint/eslint-plugin": "^6.0.0",
            "@typescript-eslint/parser": "^6.0.0",
            "@vitejs/plugin-react": "^4.0.0",
            "eslint": "^8.45.0",
            "eslint-plugin-react-hooks": "^4.6.0",
            "eslint-plugin-react-refresh": "^0.4.0",
            "typescript": "^5.0.2",
            "vite": "^4.4.0"
        },
        "scripts": {
            "dev": "vite",
            "build": "tsc && vite build",
            "lint": "eslint . --ext ts,tsx --report-unused-disable-directives --max-warnings 0",
            "preview": "vite preview"
        }
    });
    
    fs::write(path.join("package.json"), serde_json::to_string_pretty(&package_json)?)?;
    
    // Create src directory and files
    fs::create_dir_all(path.join("src"))?;
    fs::create_dir_all(path.join("public"))?;
    
    // Create index.html
    let index_html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/vite.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{}</title>
</head>
<body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
</body>
</html>"#, config.name);
    
    fs::write(path.join("index.html"), index_html)?;
    
    // Create main.tsx
    fs::write(path.join("src").join("main.tsx"), 
        r#"import React from 'react'
import ReactDOM from 'react-dom/client'
import App from './App.tsx'
import './index.css'

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
)"#)?;
    
    // Create App.tsx
    let app_tsx = format!(r#"import {{ useState }} from 'react'
import './App.css'

function App(): JSX.Element {{
  const [count, setCount] = useState<number>(0)

  return (
    <div className="App">
      <h1>{}</h1>
      <div className="card">
        <button onClick={{() => setCount((count) => count + 1)}}>
          count is {{count}}
        </button>
        <p>
          Edit <code>src/App.tsx</code> and save to test HMR
        </p>
      </div>
    </div>
  )
}}

export default App"#, config.name);
    
    fs::write(path.join("src").join("App.tsx"), app_tsx)?;
    
    // Create CSS files (same as JS version)
    fs::write(path.join("src").join("App.css"), 
        r#"#root {
  max-width: 1280px;
  margin: 0 auto;
  padding: 2rem;
  text-align: center;
}

.card {
  padding: 2em;
}

button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  background-color: #1a1a1a;
  cursor: pointer;
  transition: border-color 0.25s;
}

button:hover {
  border-color: #646cff;
}

button:focus,
button:focus-visible {
  outline: 4px auto -webkit-focus-ring-color;
}"#)?;
    
    fs::write(path.join("src").join("index.css"), 
        r#"body {
  margin: 0;
  display: flex;
  place-items: center;
  min-width: 320px;
  min-height: 100vh;
  font-family: Inter, system-ui, Avenir, Helvetica, Arial, sans-serif;
  line-height: 1.5;
  font-weight: 400;
}

h1 {
  font-size: 3.2em;
  line-height: 1.1;
}"#)?;
    
    // Create tsconfig.json
    let tsconfig = json!({
        "compilerOptions": {
            "target": "ES2020",
            "useDefineForClassFields": true,
            "lib": ["ES2020", "DOM", "DOM.Iterable"],
            "module": "ESNext",
            "skipLibCheck": true,
            "moduleResolution": "bundler",
            "allowImportingTsExtensions": true,
            "resolveJsonModule": true,
            "isolatedModules": true,
            "noEmit": true,
            "jsx": "react-jsx",
            "strict": true,
            "noUnusedLocals": true,
            "noUnusedParameters": true,
            "noFallthroughCasesInSwitch": true
        },
        "include": ["src"],
        "references": [{ "path": "./tsconfig.node.json" }]
    });
    
    fs::write(path.join("tsconfig.json"), serde_json::to_string_pretty(&tsconfig)?)?;
    
    // Create tsconfig.node.json
    let tsconfig_node = json!({
        "compilerOptions": {
            "composite": true,
            "skipLibCheck": true,
            "module": "ESNext",
            "moduleResolution": "bundler",
            "allowSyntheticDefaultImports": true
        },
        "include": ["vite.config.ts"]
    });
    
    fs::write(path.join("tsconfig.node.json"), serde_json::to_string_pretty(&tsconfig_node)?)?;
    
    // Create vite.config.ts
    fs::write(path.join("vite.config.ts"), 
        r#"import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
})"#)?;
    
    Ok(())
}

fn create_vue_ts_project(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let package_json = json!({
        "name": config.name,
        "version": "1.0.0",
        "private": true,
        "dependencies": {
            "vue": "^3.3.0"
        },
        "devDependencies": {
            "@vitejs/plugin-vue": "^4.2.0",
            "@vue/tsconfig": "^0.4.0",
            "typescript": "^5.0.0",
            "vue-tsc": "^1.4.2",
            "vite": "^4.4.0"
        },
        "scripts": {
            "dev": "vite",
            "build": "vue-tsc && vite build",
            "preview": "vite preview"
        }
    });
    
    fs::write(path.join("package.json"), serde_json::to_string_pretty(&package_json)?)?;
    
    fs::create_dir_all(path.join("src"))?;
    
    let index_html = format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>{}</title>
</head>
<body>
    <div id="app"></div>
    <script type="module" src="/src/main.ts"></script>
</body>
</html>"#, config.name);
    
    fs::write(path.join("index.html"), index_html)?;
    
    fs::write(path.join("src").join("main.ts"), 
        r#"import { createApp } from 'vue'
import App from './App.vue'

createApp(App).mount('#app')"#)?;
    
    let app_vue = format!(r#"<template>
  <div id="app">
    <h1>{}</h1>
    <button @click="increment">Count: {{ count }}</button>
  </div>
</template>

<script setup lang="ts">
import {{ ref }} from 'vue'

const count = ref<number>(0)

const increment = (): void => {{
  count.value++
}}
</script>

<style>
#app {{
  font-family: Avenir, Helvetica, Arial, sans-serif;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}}
</style>"#, config.name);
    
    fs::write(path.join("src").join("App.vue"), app_vue)?;
    
    // Create tsconfig.json
    let tsconfig = json!({
        "extends": "@vue/tsconfig/tsconfig.dom.json",
        "include": ["env.d.ts", "src/**/*", "src/**/*.vue"],
        "exclude": ["src/**/__tests__/*"],
        "compilerOptions": {
            "composite": true,
            "baseUrl": ".",
            "paths": {
                "@/*": ["./src/*"]
            }
        }
    });
    
    fs::write(path.join("tsconfig.json"), serde_json::to_string_pretty(&tsconfig)?)?;
    
    // Create env.d.ts
    fs::write(path.join("src").join("env.d.ts"), 
        r#"/// <reference types="vite/client" />
"#)?;
    
    fs::write(path.join("vite.config.ts"), 
        r#"import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [vue()]
})"#)?;
    
    Ok(())
}
fn create_nextjs_ts_project(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let package_json = json!({
        "name": config.name,
        "version": "1.0.0",
        "private": true,
        "dependencies": {
            "next": "13.4.19",
            "react": "^18.2.0",
            "react-dom": "^18.2.0"
        },
        "devDependencies": {
            "@types/node": "^20",
            "@types/react": "^18",
            "@types/react-dom": "^18",
            "eslint": "^8",
            "eslint-config-next": "13.4.19",
            "typescript": "^5"
        },
        "scripts": {
            "dev": "next dev",
            "build": "next build",
            "start": "next start",
            "lint": "next lint"
        }
    });
    
    fs::write(path.join("package.json"), serde_json::to_string_pretty(&package_json)?)?;
    
    fs::create_dir_all(path.join("pages"))?;
    fs::create_dir_all(path.join("public"))?;
    
    let index_tsx = format!(r#"import type {{ NextPage }} from 'next'
import Head from 'next/head'

const Home: NextPage = () => {{
  return (
    <div>
      <Head>
        <title>{}</title>
        <meta name="description" content="Generated by DevGeini" />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main>
        <h1>Welcome to {}</h1>
        <p>Built with Next.js and TypeScript</p>
      </main>
    </div>
  )
}}

export default Home"#, config.name, config.name);
    
    fs::write(path.join("pages").join("index.tsx"), index_tsx)?;
    
    // Create tsconfig.json
    let tsconfig = json!({
        "compilerOptions": {
            "target": "es5",
            "lib": ["dom", "dom.iterable", "es6"],
            "allowJs": true,
            "skipLibCheck": true,
            "strict": true,
            "forceConsistentCasingInFileNames": true,
            "noEmit": true,
            "esModuleInterop": true,
            "module": "esnext",
            "moduleResolution": "node",
            "resolveJsonModule": true,
            "isolatedModules": true,
            "jsx": "preserve",
            "incremental": true,
            "baseUrl": ".",
            "paths": {
                "@/*": ["./*"]
            }
        },
        "include": ["next-env.d.ts", "**/*.ts", "**/*.tsx"],
        "exclude": ["node_modules"]
    });
    
    fs::write(path.join("tsconfig.json"), serde_json::to_string_pretty(&tsconfig)?)?;
    
    // Create next-env.d.ts
    fs::write(path.join("next-env.d.ts"), 
        r#"/// <reference types="next" />
/// <reference types="next/image-types/global" />

// NOTE: This file should not be edited
// see https://nextjs.org/docs/basic-features/typescript for more information.
"#)?;
    
    Ok(())
}

fn create_svelte_ts_project(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("Currently TS not supported for Svelte");
    create_svelte_project(config, path)
}

fn create_angular_project(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("Currently TS not supported for Angular");
    create_angular_project(config, path)
}

fn create_vanilla_ts_project(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("Currently TS not supported for Vanilla");
    create_vanilla_project(config, path)
}



