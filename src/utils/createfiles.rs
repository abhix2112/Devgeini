use std::path::Path;
use crate::config::structure::{ProjectConfig, FrontendStack,BackendStack};

use crate::templates::frontend::{
    create_react_project, create_react_ts_project, create_vue_project, create_vue_ts_project,
    create_nextjs_project, create_nextjs_ts_project, create_svelte_project, create_svelte_ts_project,
    create_vanilla_project, create_vanilla_ts_project, create_angular_project,
};
use crate::templates::backend::{
    create_nodejs_backend, create_nodejs_ts_backend, create_python_backend, create_rust_backend,
};
pub fn create_frontend_files(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn create_backend_files(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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

