use askama::Template;
use serde::Deserialize;
use std::fs;

#[derive(Template)]
#[template(path = "index.html")]
pub struct PortfolioTemplate {
    name: String,
    tagline: String,
    about: String,
    projects: Vec<Project>,
    email: String,
    github: String,
    year: i32,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    name: String,
    description: String,
    github: String,
}

fn main() {
    let projects_json =
        fs::read_to_string("static/projects.json")
            .expect("failed to read projects.json");

    let projects: Vec<Project> =
        serde_json::from_str(&projects_json)
            .expect("failed to parse projects.json");

    let template = PortfolioTemplate {
        name: "Unika Valentine".to_string(),
        tagline: "Student and Rust developer".to_string(),
        about: "I am a University student and Rust developer specialising in systems programming and high-performance software. Experience working with low-level systems, memory-safe design, and performance-critical applications in Rust.".to_string(),
        projects,
        email: "bnnui@mailbox.org".to_string(),
        github: "https://github.com/Bnnu1".to_string(),
        year: 2026,
    };

    fs::create_dir_all("dist").expect("failed to create dist directory");

    fs::write(
        "dist/index.html",
        template.render().expect("failed to render template"),
    )
    .expect("failed to write index.html");

    copy_dir("static", "dist/static");

    println!("Site generated in ./dist");
}

fn copy_dir(src: &str, dst: &str) {
    fs::create_dir_all(dst).expect("failed to create destination directory");

    for entry in fs::read_dir(src).expect("failed to read source directory") {
        let entry = entry.expect("failed to read directory entry");
        let path = entry.path();
        let dest = format!("{}/{}", dst, entry.file_name().to_string_lossy());

        if path.is_dir() {
            copy_dir(path.to_str().unwrap(), &dest);
        } else {
            fs::copy(&path, &dest).expect("failed to copy file");
        }
    }
}