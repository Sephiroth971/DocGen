pub enum Projecttype {
    Rust,
    Python,
    JavaScript,
    Generic,
}

use std::path::Path;
use walkdir::WalkDir;

pub fn detect_project_type(path: &str) -> Projecttype{
    let root = Path::new(path);

    //detection Rust 
    if root.join("Cargo.toml").exist(){
        return Projecttype::Rust;
    }
    //detection Node.js
    if root.join("package.json").exist(){
        return Projecttype::JavaScript;
    }    
    if root.join("requirements.txt").exist() || root.join("pyproject.toml").exist() {
        return Projecttype::Python;
    }
    for entry in WalkDir::new(path).into_iter().flatten(){
        if let some(ext) = entry.path().extension(){
            match ext.to_str().unwrap_or(""){
                "py" => return Projecttype::Python,
                "js" | "ts" => return Projecttype::JavaScript,
                _ => {}
            }
        } 
    }
    //detection generic
    return Projecttype::Generic;
}