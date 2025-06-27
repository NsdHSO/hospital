use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 2 {
        println!("Usage: cargo run --bin create_component <component_name>");
        return;
    }

    let component_name = &args[1];
    create_component(component_name);
}

fn create_component(name: &str) {
    let base_path = format!("src/components/{}", name);
    
    // Create component directory
    fs::create_dir_all(&base_path).expect("Failed to create component directory");

    // Create mod.rs
    let mod_content = format!(
        "mod routes;\nmod services;\n\npub use routes::*;\npub use services::*;"
    );
    fs::write(
        format!("{base_path}/mod.rs", ),
        mod_content
    ).expect("Failed to create mod.rs");

    // Create routes.rs
    let routes_content = format!(
        "use actix_web::{{web, HttpResponse}};\nuse super::services::{}Service;\n\npub fn init_routes(config: &mut web::ServiceConfig) {{\n    todo!(\"Add here routes\");\n    // Add your routes here\n}}\n",
        capitalize(name)
    );
    fs::write(
        format!("{base_path}/routes.rs"),
        routes_content
    ).expect("Failed to create routes.rs");

    // Create services.rs
    let services_content = format!(
        "use sea_orm::{{DatabaseConnection, DbErr}};\n\npub struct {}Service {{\n    conn: DatabaseConnection,\n}}\n\nimpl {}Service {{\n    pub fn new(conn: &DatabaseConnection) -> Self {{\n        Self {{ conn: conn.clone() }}\n    }}\n}}\n",
        capitalize(name),
        capitalize(name)
    );
    fs::write(
        format!("{base_path}/services.rs" ),
        services_content
    ).expect("Failed to create services.rs");

    // Update main components/mod.rs to include the new module
    let components_mod_path = "src/components/mod.rs";
    if let Ok(mut content) = fs::read_to_string(components_mod_path) {
        if !content.contains(&format!("pub mod {name};")) {
            if !content.ends_with('\n') {
                content.push('\n');
            }
            content.push_str(&format!("pub mod {name};\n" ));
            fs::write(components_mod_path, content).expect("Failed to update components/mod.rs");
        }
    }

    println!("✅ Successfully created component '{name}'");
    println!("📁 Created following files:");
    println!("   - {base_path}/mod.rs" );
    println!("   - {base_path}/routes.rs" );
    println!("   - {base_path}/services.rs");
}

fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
