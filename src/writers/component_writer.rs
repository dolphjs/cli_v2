use std::{error::Error, fs, path::Path};

use crate::utils::{capitalize_first_letter, ensure_directory_exists};

use super::config_writers::find_base_directory;

/// Only lists what's in the folder, so `--component` works on its own and a
/// generated component always compiles.
fn component_source(name: &str, has_controller: bool, has_service: bool) -> String {
    let capitalized_name = capitalize_first_letter(name);

    let mut imports = vec![r#"import { Component } from "@dolphjs/dolph";"#.to_string()];
    let (mut controllers, mut services) = (String::new(), String::new());

    if has_controller {
        imports.push(format!(
            r#"import {{ {capitalized_name}Controller }} from "./{name}.controller";"#
        ));
        controllers = format!("{capitalized_name}Controller");
    }

    if has_service {
        imports.push(format!(
            r#"import {{ {capitalized_name}Service }} from "./{name}.service";"#
        ));
        services = format!("{capitalized_name}Service");
    }

    format!(
        "{}\n\n@Component({{ controllers: [{controllers}], services: [{services}] }})\nexport class {capitalized_name}Component {{}}\n",
        imports.join("\n")
    )
}

fn import_insert_position(source: &str) -> usize {
    let mut position = 0;
    let mut offset = 0;

    for line in source.split_inclusive('\n') {
        offset += line.len();

        if line.starts_with("import ") || line.starts_with("} from ") {
            position = offset;
        }
    }

    position
}

/// Returns the `]` closing the array passed to `new DolphFactory(`, with the
/// index just after its `[`. A first argument that isn't an array gives `None`.
fn component_array_bounds(source: &str) -> Option<(usize, usize)> {
    const CALL: &str = "new DolphFactory(";

    let after_call = source.find(CALL)? + CALL.len();
    let open = after_call + source[after_call..].find(|c: char| !c.is_whitespace())?;

    if !source[open..].starts_with('[') {
        return None;
    }

    let mut depth = 0;

    for (index, character) in source[open..].char_indices() {
        match character {
            '[' => depth += 1,
            ']' => {
                depth -= 1;
                if depth == 0 {
                    return Some((open + 1, open + index));
                }
            }
            _ => {}
        }
    }

    None
}

/// Adds a component to `new DolphFactory([...])` and imports it. Gives back the
/// source unchanged if it's already there, and `None` if the call can't be found,
/// because this edits a file the user owns and mustn't guess.
fn add_component_to_server(source: &str, name: &str) -> Option<String> {
    let component = format!("{}Component", capitalize_first_letter(name));
    let (start, end) = component_array_bounds(source)?;
    let items = &source[start..end];

    if items
        .split(|c: char| !c.is_alphanumeric() && c != '_')
        .any(|item| item == component)
    {
        return Some(source.to_string());
    }

    let content_end = start + items.trim_end().len();
    let addition = match items.trim_end().chars().last() {
        None => component.clone(),
        Some(',') => format!(" {component},"),
        Some(_) => format!(", {component}"),
    };

    let import = format!(
        "import {{ {component} }} from \"./components/{name}/{name}.component\";\n"
    );
    let import_position = import_insert_position(source);

    Some(format!(
        "{}{}{}{}{}",
        &source[..import_position],
        import,
        &source[import_position..content_end],
        addition,
        &source[content_end..]
    ))
}

fn wire_into_server(base_directory: &Path, name: &str) -> Result<(), Box<dyn Error>> {
    let server_path = base_directory.join("server.ts");
    let component = format!("{}Component", capitalize_first_letter(name));

    let updated = fs::read_to_string(&server_path)
        .ok()
        .and_then(|source| add_component_to_server(&source, name));

    match updated {
        Some(content) => fs::write(&server_path, content)?,
        None => println!(
            "Could not find `new DolphFactory([...])` in src/server.ts. Add {component} to it by hand."
        ),
    }

    Ok(())
}

pub fn write_spring_component(name: &str) -> Result<(), Box<dyn Error>> {
    let base_directory = find_base_directory().ok_or_else(|| "Could not find base directory")?;

    let component_path = base_directory.join("components");

    ensure_directory_exists(&component_path)?;

    let specific_component_path = component_path.join(format!("{}", name));

    ensure_directory_exists(&specific_component_path)?;

    let index_path = specific_component_path.join(format!("{}.component.ts", name));

    let content = component_source(
        name,
        specific_component_path.join(format!("{}.controller.ts", name)).exists(),
        specific_component_path.join(format!("{}.service.ts", name)).exists(),
    );

    if let Err(e) = fs::write(&index_path, content) {
        eprintln!("{}", format!("Error writing component file: {}", e));
        return Err(Box::new(e));
    }

    wire_into_server(&base_directory, name)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY_SERVER: &str = r#"import { DolphFactory } from "@dolphjs/dolph";

const dolph = new DolphFactory([]);

dolph.start();
"#;

    #[test]
    fn should_list_the_controller_and_service_that_exist() {
        let source = component_source("user", true, true);

        assert!(source.contains(r#"import { UserController } from "./user.controller";"#));
        assert!(source.contains(r#"import { UserService } from "./user.service";"#));
        assert!(source.contains("@Component({ controllers: [UserController], services: [UserService] })"));
    }

    #[test]
    fn should_not_import_files_that_do_not_exist() {
        let source = component_source("user", false, true);

        assert!(!source.contains("Controller"));
        assert!(source.contains("@Component({ controllers: [], services: [UserService] })"));
    }

    #[test]
    fn should_add_the_first_component() {
        let updated = add_component_to_server(EMPTY_SERVER, "user").unwrap();

        assert_eq!(
            updated,
            r#"import { DolphFactory } from "@dolphjs/dolph";
import { UserComponent } from "./components/user/user.component";

const dolph = new DolphFactory([UserComponent]);

dolph.start();
"#
        );
    }

    #[test]
    fn should_add_after_existing_components() {
        let once = add_component_to_server(EMPTY_SERVER, "user").unwrap();
        let twice = add_component_to_server(&once, "post").unwrap();

        assert!(twice.contains("new DolphFactory([UserComponent, PostComponent]);"));
        assert!(twice.contains(r#"import { PostComponent } from "./components/post/post.component";"#));
    }

    #[test]
    fn should_change_nothing_when_run_twice() {
        let once = add_component_to_server(EMPTY_SERVER, "user").unwrap();

        assert_eq!(add_component_to_server(&once, "user").unwrap(), once);
    }

    #[test]
    fn should_not_treat_a_longer_name_as_the_same_component() {
        let source = EMPTY_SERVER.replace("[]", "[AdminUserComponent]");
        let updated = add_component_to_server(&source, "user").unwrap();

        assert!(updated.contains("[AdminUserComponent, UserComponent]"));
    }

    #[test]
    fn should_keep_the_options_argument() {
        let source = EMPTY_SERVER.replace(
            "new DolphFactory([])",
            "new DolphFactory([], { middlewares: [cookieParser()] })",
        );
        let updated = add_component_to_server(&source, "user").unwrap();

        assert!(updated.contains("new DolphFactory([UserComponent], { middlewares: [cookieParser()] })"));
    }

    #[test]
    fn should_handle_a_multi_line_array_with_a_trailing_comma() {
        let source = EMPTY_SERVER.replace("[]", "[\n  PostComponent,\n]");
        let updated = add_component_to_server(&source, "user").unwrap();

        assert!(updated.contains("[\n  PostComponent, UserComponent,\n]"));
    }

    #[test]
    fn should_put_the_import_after_a_multi_line_import() {
        let source = EMPTY_SERVER.replace(
            r#"import { DolphFactory } from "@dolphjs/dolph";"#,
            "import {\n  DolphFactory,\n} from \"@dolphjs/dolph\";",
        );
        let updated = add_component_to_server(&source, "user").unwrap();

        assert!(updated.contains(
            "} from \"@dolphjs/dolph\";\nimport { UserComponent } from \"./components/user/user.component\";\n"
        ));
    }

    #[test]
    fn should_leave_a_server_it_cannot_read_alone() {
        assert_eq!(add_component_to_server("const app = createApp();\n", "user"), None);
        assert_eq!(
            add_component_to_server("const dolph = new DolphFactory(components);\n", "user"),
            None
        );
    }
}
