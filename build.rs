// Copyright (c) Microsoft Corporation.
// Licensed under the MIT license.

use std::process::Command;
use std::path::Path;

fn main() {
    built::write_built_file().expect("Failed to acquire build-time information");
    
    // Build the frontend if it exists and Node.js is available
    let frontend_dir = Path::new("frontend");
    if frontend_dir.exists() {
        println!("cargo:rerun-if-changed=frontend/src");
        println!("cargo:rerun-if-changed=frontend/package.json");
        
        // Check if Node.js is available
        if Command::new("node").arg("--version").output().is_ok() {
            println!("Building frontend with Node.js...");
            
            // Install dependencies if node_modules doesn't exist
            let node_modules = frontend_dir.join("node_modules");
            if !node_modules.exists() {
                let install_output = Command::new("npm")
                    .arg("install")
                    .current_dir(frontend_dir)
                    .output();
                    
                match install_output {
                    Ok(output) if output.status.success() => {
                        println!("Frontend dependencies installed successfully");
                    }
                    Ok(output) => {
                        println!("cargo:warning=Failed to install frontend dependencies: {}", 
                               String::from_utf8_lossy(&output.stderr));
                    }
                    Err(e) => {
                        println!("cargo:warning=Failed to run npm install: {}", e);
                    }
                }
            }
            
            // Build the frontend
            let build_output = Command::new("npm")
                .arg("run")
                .arg("build")
                .current_dir(frontend_dir)
                .output();
                
            match build_output {
                Ok(output) if output.status.success() => {
                    println!("Frontend built successfully");
                }
                Ok(output) => {
                    println!("cargo:warning=Failed to build frontend: {}", 
                           String::from_utf8_lossy(&output.stderr));
                }
                Err(e) => {
                    println!("cargo:warning=Failed to run npm build: {}", e);
                }
            }
        } else {
            println!("cargo:warning=Node.js not found, skipping frontend build");
        }
    }
}
