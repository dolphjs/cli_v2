use std::{error::Error, fs};

use crate::utils::{capitalize_first_letter, ensure_directory_exists};

use super::config_writers::find_base_directory;

/// A Socket Component bundles Socket Services together, the same role
/// `@Component` plays for controllers/services — without one, a generated
/// socket service is never actually wired into the app. `dolph generate
/// --socket <name>` writes both files; this one still needs to be passed
/// into `DolphFactory`'s second constructor argument by hand (`{ socketService:
/// SocketService, component: new XSocketComponent() }`), since that wiring
/// lives in the user's own server.ts.
pub fn write_socket_component(name: &str) -> Result<(), Box<dyn Error>> {
    let base_directory = find_base_directory().ok_or_else(|| "Could not find base directory")?;

    let shared_directory = base_directory.join("shared");

    ensure_directory_exists(&shared_directory)?;

    let socket_path = shared_directory.join("socket");

    ensure_directory_exists(&socket_path)?;

    let index_path = socket_path.join(format!("{}.socket.component.ts", name));

    let capitalized_name = capitalize_first_letter(name);

    let import_statement = format!(
        r#"import {{ Socket }} from "@dolphjs/dolph/decorators";
import {{ SocketComponent }} from "@dolphjs/dolph/packages";
import {{ {capitalized_name}SocketService }} from "./{name}.socket.service";"#
    );

    let other_file_content = format!(
        r#"@Socket({{
    services: [],
    socketServices: [{capitalized_name}SocketService],
}})
export class {capitalized_name}SocketComponent extends SocketComponent {{}}"#
    );

    let file_content = format!("{}\n\n{}\n", import_statement, other_file_content);

    match fs::write(&index_path, file_content) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", format!("Error writing socket component file: {}", e));
            Err(Box::new(e))
        }
    }
}
