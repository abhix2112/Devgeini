use clap::{Arg, Command};
use dialoguer::{theme::ColorfulTheme, Select, Input, Confirm};
use serde_json::json;
use std::fs;
use std::path::Path;
use std::io::{self, Write};
use std::process::{Command as ShellCommand, Stdio};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use reqwest;
use tokio;

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

// GitHub API structures
#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    name: String,
    body: String,
    assets: Vec<GitHubAsset>,
    prerelease: bool,
}

#[derive(Debug, Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
    size: u64,
}

const GITHUB_REPO: &str = "abhix2112/devgeini"; // Replace with your actual GitHub repo
const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");


async fn show_welcome_menu() {
    println!("\n🚀 Welcome to Devgeini - Your Dev CLI Companion!");
    println!("=================================================");
    println!("This tool helps you scaffold your project setup faster.\n");
    
    // Check for updates in background (non-blocking)
    tokio::spawn(async {
        let _ = check_for_updates_silent().await;
    });

    let options = vec![
        "🎯 Create a new project",
        "🔄 Check for updates", 
        "📖 Show help",
        "🚪 Exit"
    ];

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .default(0)
        .items(&options)
        .interact()
        .unwrap();

    match selection {
        0 => {
            // Create new project
            println!("\n🛠️  Starting project creation...\n");
            let project_name = get_project_name();
            let config = get_project_config_interactive(project_name);
            
            if let Err(e) = create_project(&config) {
                eprintln!("❌ Error creating project: {}", e);
                std::process::exit(1);
            }

            println!("🎉 Project '{}' created successfully!", config.name);
            println!("📁 Navigate to your project: cd {}", config.name);
            show_next_steps(&config);
        }
        1 => {
            // Check for updates
            if let Err(e) = check_for_updates().await {
                eprintln!("❌ Failed to check for updates: {}", e);
            }
        }
        2 => {
            // Show help
            show_help_menu();
        }
        3 => {
            // Exit
            println!("👋 Thanks for using Devgeini! Happy coding!");
            std::process::exit(0);
        }
        _ => {}
    }
}

async fn handle_init_command(matches: &clap::ArgMatches) {
    println!("\n🚀 Welcome to Devgeini - Your Dev CLI Companion!");
    println!("-----------------------------------------------");
    println!("This tool helps you scaffold your project setup faster.\n");
    
    // Check for updates in background (non-blocking)
    tokio::spawn(async {
        let _ = check_for_updates_silent().await;
    });

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
        eprintln!("❌ Error creating project: {}", e);
        std::process::exit(1);
    }

    println!("🎉 Project '{}' created successfully!", config.name);
    println!("📁 Navigate to your project: cd {}", config.name);
    
    // Enhanced stack-specific instructions
    show_next_steps(&config);
}

fn show_help_menu() {
    println!("\n📖 Devgeini Help");
    println!("================");
    println!("Available commands:");
    println!();
    println!("🎯 devgeini init                    - Start creating a new project");
    println!("🎯 devgeini init --name <name>      - Create project with specific name");
    println!("🎯 devgeini init --interactive      - Run in full interactive mode");
    println!();
    println!("🔄 devgeini --update               - Update to latest version");
    println!("🔍 devgeini --check-update         - Check if updates are available");
    println!("❓ devgeini --help                 - Show this help message");
    println!("📋 devgeini --version              - Show current version");
    println!();
    println!("💡 Pro tip: Just run 'devgeini' to see the interactive menu!");
    println!();
    println!("Supported Project Types:");
    println!("• 🌐 Full-Stack Web Applications");
    println!("• 🎨 Frontend Applications (React, Vue, Angular, Svelte, Next.js)");
    println!("• ⚙️  Backend APIs (Node.js, Python, Rust, Go, Java, PHP)");
    println!("• 🛠️  CLI Tools (Rust)");
    println!("• 🧩 Browser Extensions");
    println!();
}

#[tokio::main]
async fn main() {
    let matches = Command::new("devgeini")
        .version(env!("CARGO_PKG_VERSION"))
        .author("DevGeini Team")
        .about("Initialize development projects with proper structure and boilerplate")
        .subcommand(
            Command::new("init")
                .about("Initialize a new project")
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
        )
        .arg(
            Arg::new("update")
                .short('u')
                .long("update")
                .action(clap::ArgAction::SetTrue)
                .help("Update devgeini to the latest version")
        )
        .arg(
            Arg::new("check-update")
                .long("check-update")
                .action(clap::ArgAction::SetTrue)
                .help("Check if a new version is available")
        )
        .get_matches();

    // Handle update commands first
    if matches.get_flag("update") {
        if let Err(e) = handle_update().await {
            eprintln!("❌ Update failed: {}", e);
            std::process::exit(1);
        }
        return;
    }

    if matches.get_flag("check-update") {
        if let Err(e) = check_for_updates().await {
            eprintln!("❌ Failed to check for updates: {}", e);
            std::process::exit(1);
        }
        return;
    }

    // Handle subcommands
    match matches.subcommand() {
        Some(("init", sub_matches)) => {
            handle_init_command(sub_matches).await;
        }
        _ => {
            // No subcommand provided - show interactive menu
            show_welcome_menu().await;
        }
    }
}

async fn handle_update() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Checking for updates...");
    
    let latest_release = get_latest_release().await?;
    let latest_version = latest_release.tag_name.trim_start_matches('v');
    
    if version_compare(CURRENT_VERSION, latest_version) >= 0 {
        println!("✅ You're already running the latest version ({})", CURRENT_VERSION);
        return Ok(());
    }
    
    println!("🆕 New version available: {} -> {}", CURRENT_VERSION, latest_version);
    println!("📋 Release notes:\n{}", latest_release.body);
    
    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to update now?")
        .default(true)
        .interact()?;
    
    if !confirm {
        println!("Update cancelled.");
        return Ok(());
    }
    
    // Download and install update
    download_and_install_update(&latest_release).await?;
    
    println!("✅ Successfully updated to version {}!", latest_version);
    println!("🔄 Please restart your terminal or run 'devgeini --version' to verify the update.");
    
    Ok(())
}

async fn check_for_updates() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Checking for updates...");
    
    let latest_release = get_latest_release().await?;
    let latest_version = latest_release.tag_name.trim_start_matches('v');
    
    println!("📦 Current version: {}", CURRENT_VERSION);
    println!("🆕 Latest version: {}", latest_version);
    
    if version_compare(CURRENT_VERSION, latest_version) < 0 {
        println!("🎉 A new version is available!");
        println!("📋 Release notes:\n{}", latest_release.body);
        println!("🚀 Run 'devgeini --update' to update to the latest version.");
    } else {
        println!("✅ You're running the latest version!");
    }
    
    Ok(())
}

async fn check_for_updates_silent() -> Result<(), Box<dyn std::error::Error>> {
    let latest_release = get_latest_release().await?;
    let latest_version = latest_release.tag_name.trim_start_matches('v');
    
    if version_compare(CURRENT_VERSION, latest_version) < 0 {
        println!("💡 A new version ({}) is available! Run 'devgeini --update' to upgrade.", latest_version);
    }
    
    Ok(())
}

async fn get_latest_release() -> Result<GitHubRelease, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/repos/{}/releases/latest", GITHUB_REPO);
    
    let response = client
        .get(&url)
        .header("User-Agent", format!("devgeini/{}", CURRENT_VERSION))
        .send()
        .await?;
    
    if !response.status().is_success() {
        return Err(format!("GitHub API request failed: {}", response.status()).into());
    }
    
    let release: GitHubRelease = response.json().await?;
    Ok(release)
}

async fn download_and_install_update(release: &GitHubRelease) -> Result<(), Box<dyn std::error::Error>> {
    // Determine the correct asset for the current platform
    let target_asset = find_matching_asset(&release.assets)?;
    
    println!("📥 Downloading {} ({} bytes)...", target_asset.name, target_asset.size);
    
    // Download the asset
    let client = reqwest::Client::new();
    let response = client
        .get(&target_asset.browser_download_url)
        .header("User-Agent", format!("devgeini/{}", CURRENT_VERSION))
        .send()
        .await?;
    
    if !response.status().is_success() {
        return Err(format!("Failed to download update: {}", response.status()).into());
    }
    
    let bytes = response.bytes().await?;
    
    // Get current executable path
    let current_exe = std::env::current_exe()?;
    let backup_path = current_exe.with_extension("bak");
    
    // Create backup of current executable
    std::fs::copy(&current_exe, &backup_path)?;
    println!("📁 Created backup at: {}", backup_path.display());
    
    // Write new executable
    if target_asset.name.ends_with(".tar.gz") || target_asset.name.ends_with(".zip") {
        // Handle compressed archives
        extract_and_install_executable(&bytes, &current_exe, &target_asset.name).await?;
    } else {
        // Direct executable replacement
        std::fs::write(&current_exe, &bytes)?;
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut permissions = std::fs::metadata(&current_exe)?.permissions();
            permissions.set_mode(0o755);
            std::fs::set_permissions(&current_exe, permissions)?;
        }
    }
    
    println!("✅ Installation completed!");
    
    // Clean up backup on successful installation
    if let Err(e) = std::fs::remove_file(&backup_path) {
        println!("⚠️  Warning: Could not remove backup file: {}", e);
    }
    
    Ok(())
}

fn find_matching_asset(assets: &[GitHubAsset]) -> Result<&GitHubAsset, Box<dyn std::error::Error>> {
    let target_os = std::env::consts::OS;
    let target_arch = std::env::consts::ARCH;
    
    // Define platform-specific patterns
    let patterns = match (target_os, target_arch) {
        ("windows", "x86_64") => vec!["windows", "win64", "x86_64-pc-windows"],
        ("windows", "x86") => vec!["windows", "win32", "i686-pc-windows"],
        ("macos", "x86_64") => vec!["macos", "darwin", "x86_64-apple-darwin"],
        ("macos", "aarch64") => vec!["macos", "darwin", "aarch64-apple-darwin"],
        ("linux", "x86_64") => vec!["linux", "x86_64-unknown-linux"],
        ("linux", "aarch64") => vec!["linux", "aarch64-unknown-linux"],
        _ => vec!["universal"],
    };
    
    // Find matching asset
    for asset in assets {
        let asset_name_lower = asset.name.to_lowercase();
        for pattern in &patterns {
            if asset_name_lower.contains(pattern) {
                return Ok(asset);
            }
        }
    }
    
    // Fallback to first asset if no specific match found
    assets.first()
        .ok_or_else(|| "No suitable release asset found for your platform".into())
}

async fn extract_and_install_executable(
    bytes: &[u8], 
    target_path: &Path, 
    filename: &str
) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::Read;
    
    if filename.ends_with(".tar.gz") {
        // Handle tar.gz files
        let tar = flate2::read::GzDecoder::new(bytes);
        let mut archive = tar::Archive::new(tar);
        
        for entry in archive.entries()? {
            let mut entry = entry?;
            let path = entry.path()?;
            
            // Look for the executable (usually named 'devgeini' or similar)
            if path.file_name().and_then(|s| s.to_str()).map_or(false, |s| s.starts_with("devgeini")) {
                let mut buffer = Vec::new();
                entry.read_to_end(&mut buffer)?;
                std::fs::write(target_path, buffer)?;
                
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let mut permissions = std::fs::metadata(target_path)?.permissions();
                    permissions.set_mode(0o755);
                    std::fs::set_permissions(target_path, permissions)?;
                }
                
                return Ok(());
            }
        }
    } else if filename.ends_with(".zip") {
        // Handle zip files (Windows typically)
        let reader = std::io::Cursor::new(bytes);
        let mut archive = zip::ZipArchive::new(reader)?;
        
        for i in 0..archive.len() {
            let mut file = archive.by_index(i)?;
            if file.name().ends_with(".exe") || file.name().ends_with("devgeini") {
                let mut buffer = Vec::new();
                std::io::copy(&mut file, &mut buffer)?;
                std::fs::write(target_path, buffer)?;
                return Ok(());
            }
        }
    }
    
    Err("Could not find executable in archive".into())
}

fn version_compare(current: &str, latest: &str) -> i32 {
    let current_parts: Vec<u32> = current.split('.').filter_map(|s| s.parse().ok()).collect();
    let latest_parts: Vec<u32> = latest.split('.').filter_map(|s| s.parse().ok()).collect();
    
    let max_len = current_parts.len().max(latest_parts.len());
    
    for i in 0..max_len {
        let current_part = current_parts.get(i).unwrap_or(&0);
        let latest_part = latest_parts.get(i).unwrap_or(&0);
        
        match current_part.cmp(latest_part) {
            std::cmp::Ordering::Less => return -1,
            std::cmp::Ordering::Greater => return 1,
            std::cmp::Ordering::Equal => continue,
        }
    }
    
    0
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