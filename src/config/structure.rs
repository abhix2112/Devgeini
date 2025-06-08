#[derive(Debug, Clone)]
pub enum ProjectType {
    FullStackWeb,
    Frontend,
    Backend,
    CliTool,
    WebExtension,
}

#[derive(Debug, Clone)]
pub enum FrontendStack {
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
pub enum BackendStack {
    NodeJs,
    NodeJsTs,
    Python,
    Rust,
    Go,
    Java,
    Php,
    
}

pub struct ProjectConfig {
    pub name: String,
    pub project_type: ProjectType,
    pub frontend_stack: Option<FrontendStack>,
    pub backend_stack: Option<BackendStack>,
}