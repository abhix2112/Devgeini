use serde_json::json;
use std::fs;
use std::path::Path;
use crate::config::structure::{ProjectConfig, FrontendStack,BackendStack};


pub fn create_nodejs_ts_backend(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn create_nodejs_backend(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn create_python_backend(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn create_rust_backend(config: &ProjectConfig, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
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
