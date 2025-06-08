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
    










