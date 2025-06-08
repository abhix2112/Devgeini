use serde_json::json;
use std::fs;
use std::path::Path;
use crate::config::structure::{ProjectConfig, FrontendStack,BackendStack};


pub fn create_react_ts_project(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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


pub fn create_vue_project(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn create_nextjs_project(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn create_vanilla_project(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn create_svelte_project(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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


pub fn create_vue_ts_project(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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
pub fn create_nextjs_ts_project(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn create_svelte_ts_project(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("Currently TS not supported for Svelte");
    create_svelte_project(config, path)
}

pub fn create_angular_project(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("Currently TS not supported for Angular");
    create_angular_project(config, path)
}

pub fn create_vanilla_ts_project(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    println!("Currently TS not supported for Vanilla");
    create_vanilla_project(config, path)
}
