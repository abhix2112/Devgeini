use clap::{Arg, Command};
use std::path::Path;
use dialoguer::{theme::ColorfulTheme, Select, Input, Confirm};
use serde_json::json;
use std::fs;

use crate::config::structure::{ProjectConfig, FrontendStack, BackendStack, ProjectType};
use crate::utils::projecttype::{
    create_fullstack_project, create_frontend_project, create_backend_project,
    create_cli_project, create_extension_project, create_gitignore, create_readme, create_env_file,
};


pub fn get_project_name() -> String {
    Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Enter project name")
        .interact_text()
        .unwrap()
}

pub fn get_project_config_interactive(name: String) -> ProjectConfig {
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

pub fn select_project_type() -> ProjectType {
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

pub fn select_frontend_stack() -> FrontendStack {
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

pub fn select_backend_stack() -> BackendStack {
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

pub fn create_project(config: &ProjectConfig) -> Result<(), Box<dyn std::error::Error>> {
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
