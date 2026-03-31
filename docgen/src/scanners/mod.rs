

pub enum ProjectType {
    Rust,
    Python,
    JavaScript,
    Generic,
}

use std::path::Path;
use walkdir::WalkDir;

pub fn detect_project_type(path: &str) -> ProjectType{
    let root = Path::new(path);

    //detection Rust 
    if root.join("Cargo.toml").exists(){
        return ProjectType::Rust;
    }
    //detection Node.js
    if root.join("package.json").exists(){
        return ProjectType::JavaScript;
    }    
    if root.join("requirements.txt").exists() || root.join("pyproject.toml").exists() {
        return ProjectType::Python;
    }
    for entry in WalkDir::new(path).into_iter().flatten(){
        if let Some(ext) = entry.path().extension(){
            match ext.to_str().unwrap_or(""){
                "py" => return ProjectType::Python,
                "js" | "ts" => return ProjectType::JavaScript,
                _ => {}
            }
        } 
    }
    //detection generic
    return ProjectType::Generic;
}