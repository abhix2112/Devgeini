use clap::{Arg, Command};
use dialoguer::{theme::ColorfulTheme, Select, Input, Confirm};
use serde_json::json;
use std::fs;
use std::path::Path;
use std::io::{self, Write};
use std::process::{Command as ShellCommand, Stdio};
use std::collections::HashMap;
mod config;
//import all emnum and structure from config module
use config::structure::{ProjectConfig, FrontendStack, BackendStack,ProjectType};
mod utils;
use utils::createproject::{get_project_name, get_project_config_interactive, select_project_type, select_frontend_stack, select_backend_stack, create_project};
mod templates;

use crate::templates::frontend::{
    create_react_project, create_react_ts_project, create_vue_project, create_vue_ts_project,
    create_nextjs_project, create_nextjs_ts_project, create_svelte_project, create_svelte_ts_project,
    create_vanilla_project, create_vanilla_ts_project, create_angular_project,
};
use crate::templates::backend::{
    create_nodejs_backend, create_nodejs_ts_backend, create_python_backend, create_rust_backend,
};
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
     println!("\n🚀 Welcome to Devgeini - Your Dev CLI Companion!");
     println!("-----------------------------------------------");
     println!("This tool helps you scaffold your project setup faster.\n");
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
    
    // Enhanced stack-specific instructions
    show_next_steps(&config);
}

fn show_next_steps(config: &ProjectConfig) {
    match config.project_type {
        ProjectType::FullStackWeb => {
            show_fullstack_instructions(config);
        }
        ProjectType::Frontend => {
            show_frontend_instructions(config);
        }
        ProjectType::Backend => {
            show_backend_instructions(config);
        }
        ProjectType::CliTool => {
            println!("🦀 Build project: cargo build");
            println!("🏃 Run project: cargo run");
            println!("🧪 Run tests: cargo test");
        }
        ProjectType::WebExtension => {
            println!("📦 Install dependencies: npm install");
            println!("🔧 Build extension: npm run build");
            println!("🔍 Load extension in browser for testing");
        }
    }
}

fn show_fullstack_instructions(config: &ProjectConfig) {
    // Backend instructions
    if let Some(backend) = &config.backend_stack {
        println!("\n🔧 Backend Setup:");
        match backend {
            BackendStack::NodeJs => {
                println!("📦 Install backend deps: cd backend && npm install");
                println!("🚀 Start backend: npm run dev (usually on port 3001)");
            }
            BackendStack::NodeJsTs => {
                println!("📦 Install backend deps: cd backend && npm install");
                println!("🚀 Start backend: npm run dev (TypeScript)");
            }
            BackendStack::Python => {
                println!("🐍 Setup virtual env: cd backend && python -m venv venv");
                println!("📦 Activate & install: source venv/bin/activate && pip install -r requirements.txt");
                println!("🚀 Start backend: python app.py");
            }
            BackendStack::Rust => {
                println!("🦀 Build backend: cd backend && cargo build");
                println!("🚀 Start backend: cargo run");
            }
            BackendStack::Go => {
                println!("📦 Install deps: cd backend && go mod tidy");
                println!("🚀 Start backend: go run main.go");
            }
            BackendStack::Java => {
                println!("☕ Build project: cd backend && mvn clean install");
                println!("🚀 Start backend: mvn spring-boot:run");
            }
            BackendStack::Php => {
                println!("🐘 Install deps: cd backend && composer install");
                println!("🚀 Start backend: php -S localhost:8000");
            }
        }
    }

    // Frontend instructions
    if let Some(frontend) = &config.frontend_stack {
        println!("\n🎨 Frontend Setup:");
        match frontend {
            FrontendStack::React => {
                println!("📦 Install frontend deps: cd frontend && npm install");
                println!("🚀 Start frontend: npm start (usually on port 3000)");
            }
            FrontendStack::ReactTs => {
                println!("📦 Install frontend deps: cd frontend && npm install");
                println!("🚀 Start frontend: npm start (React + TypeScript)");
            }
            FrontendStack::Vue => {
                println!("📦 Install frontend deps: cd frontend && npm install");
                println!("🚀 Start frontend: npm run serve");
            }
            FrontendStack::VueTs => {
                println!("📦 Install frontend deps: cd frontend && npm install");
                println!("🚀 Start frontend: npm run serve (Vue + TypeScript)");
            }
            FrontendStack::Angular => {
                println!("📦 Install frontend deps: cd frontend && npm install");
                println!("🚀 Start frontend: ng serve");
            }
            FrontendStack::Svelte => {
                println!("📦 Install frontend deps: cd frontend && npm install");
                println!("🚀 Start frontend: npm run dev");
            }
            FrontendStack::SvelteTs => {
                println!("📦 Install frontend deps: cd frontend && npm install");
                println!("🚀 Start frontend: npm run dev (Svelte + TypeScript)");
            }
            FrontendStack::NextJs => {
                println!("📦 Install frontend deps: cd frontend && npm install");
                println!("🚀 Start frontend: npm run dev");
            }
            FrontendStack::NextJsTs => {
                println!("📦 Install frontend deps: cd frontend && npm install");
                println!("🚀 Start frontend: npm run dev (Next.js + TypeScript)");
            }
            FrontendStack::Vanilla => {
                println!("📦 Install frontend deps: cd frontend && npm install");
                println!("🚀 Start frontend: npm run dev");
            }
            FrontendStack::VanillaTs => {
                println!("📦 Install frontend deps: cd frontend && npm install");
                println!("🚀 Start frontend: npm run dev (Vanilla + TypeScript)");
            }
        }
    }

    println!("\n💡 Pro tip: Run backend and frontend in separate terminals!");
}

fn show_frontend_instructions(config: &ProjectConfig) {
    if let Some(frontend) = &config.frontend_stack {
        match frontend {
            FrontendStack::React => {
                println!("📦 Install dependencies: npm install");
                println!("🚀 Start development: npm start");
                println!("🏗️  Build for production: npm run build");
            }
            FrontendStack::ReactTs => {
                println!("📦 Install dependencies: npm install");
                println!("🚀 Start development: npm start (React + TypeScript)");
                println!("🏗️  Build for production: npm run build");
                println!("🔧 Type check: npm run type-check");
            }
            FrontendStack::Vue => {
                println!("📦 Install dependencies: npm install");
                println!("🚀 Start development: npm run serve");
                println!("🏗️  Build for production: npm run build");
            }
            FrontendStack::VueTs => {
                println!("📦 Install dependencies: npm install");
                println!("🚀 Start development: npm run serve (Vue + TypeScript)");
                println!("🏗️  Build for production: npm run build");
                println!("🔧 Type check: npm run type-check");
            }
            FrontendStack::Angular => {
                println!("📦 Install dependencies: npm install");
                println!("🚀 Start development: ng serve");
                println!("🏗️  Build for production: ng build");
                println!("🧪 Run tests: ng test");
            }
            FrontendStack::Svelte => {
                println!("📦 Install dependencies: npm install");
                println!("🚀 Start development: npm run dev");
                println!("🏗️  Build for production: npm run build");
            }
            FrontendStack::SvelteTs => {
                println!("📦 Install dependencies: npm install");
                println!("🚀 Start development: npm run dev (Svelte + TypeScript)");
                println!("🏗️  Build for production: npm run build");
                println!("🔧 Type check: npm run check");
            }
            FrontendStack::NextJs => {
                println!("📦 Install dependencies: npm install");
                println!("🚀 Start development: npm run dev");
                println!("🏗️  Build for production: npm run build");
                println!("🌐 Start production: npm start");
            }
            FrontendStack::NextJsTs => {
                println!("📦 Install dependencies: npm install");
                println!("🚀 Start development: npm run dev (Next.js + TypeScript)");
                println!("🏗️  Build for production: npm run build");
                println!("🌐 Start production: npm start");
                println!("🔧 Type check: npm run type-check");
            }
            FrontendStack::Vanilla => {
                println!("📦 Install dependencies: npm install");
                println!("🚀 Start development: npm run dev");
                println!("🏗️  Build for production: npm run build");
            }
            FrontendStack::VanillaTs => {
                println!("📦 Install dependencies: npm install");
                println!("🚀 Start development: npm run dev (Vanilla + TypeScript)");
                println!("🏗️  Build for production: npm run build");
                println!("🔧 Type check: tsc --noEmit");
            }
        }
    }
}

fn show_backend_instructions(config: &ProjectConfig) {
    if let Some(backend) = &config.backend_stack {
        match backend {
            BackendStack::NodeJs => {
                println!("📦 Install dependencies: npm install");
                println!("🚀 Start development: npm run dev");
                println!("🏗️  Start production: npm start");
            }
            BackendStack::NodeJsTs => {
                println!("📦 Install dependencies: npm install");
                println!("🚀 Start development: npm run dev (Node.js + TypeScript)");
                println!("🏗️  Build project: npm run build");
                println!("🌐 Start production: npm start");
                println!("🔧 Type check: npm run type-check");
            }
            BackendStack::Python => {
                println!("🐍 Create virtual environment: python -m venv venv");
                println!("📦 Activate and install: source venv/bin/activate && pip install -r requirements.txt");
                println!("🚀 Start development: python app.py");
                println!("🧪 Run tests: pytest");
            }
            BackendStack::Rust => {
                println!("📦 Build dependencies: cargo build");
                println!("🚀 Start development: cargo run");
                println!("🧪 Run tests: cargo test");
                println!("🏗️  Build release: cargo build --release");
            }
            BackendStack::Go => {
                println!("📦 Install dependencies: go mod tidy");
                println!("🚀 Start development: go run main.go");
                println!("🏗️  Build binary: go build");
                println!("🧪 Run tests: go test");
            }
            BackendStack::Java => {
                println!("📦 Install dependencies: mvn clean install");
                println!("🚀 Start development: mvn spring-boot:run");
                println!("🏗️  Build project: mvn clean package");
                println!("🧪 Run tests: mvn test");
            }
            BackendStack::Php => {
                println!("📦 Install dependencies: composer install");
                println!("🚀 Start development: php -S localhost:8000");
                println!("🧪 Run tests: vendor/bin/phpunit");
                println!("📋 Check syntax: composer run-script lint");
            }
        }
    }
}
 










