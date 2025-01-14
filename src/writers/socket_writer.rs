use std::{error::Error, fs};

use crate::utils::{capitalize_first_letter, ensure_directory_exists};

use super::config_writers::{find_base_directory};

pub fn write_socket_service(name: &str) -> Result<(), Box<dyn Error>>{
    let base_directory = find_base_directory().ok_or_else(|| "Could not find base directory")?;

    let shared_directory = base_directory.join("shared");

    ensure_directory_exists(&shared_directory)?;

    let socket_path = shared_directory.join("socket");

    ensure_directory_exists(&socket_path)?;

    let index_path = socket_path.join(format!("{}.socket.service.ts", name));

    let capitalized_name = capitalize_first_letter(name);

    let import_statement = format!(
    r#"import {{ DolphSocketServiceHandler }} from '@dolphjs/dolph/classes';
import {{ Dolph }} from '@dolphjs/dolph/common';
    "#);

    let other_file_content = format!(
    r#"export class {capitalized_name}SocketService extends DolphSocketServiceHandler<Dolph> {{
        constructor(){{
            super();
            this.socketService;
            this.handleEvents();
        }}

        private handleEvents(){{
            this.socket.on("connection", (socket) => {{
                socket.emit("connected", "connection successful");
            }});
        }}
    }}
    "#
);

    let file_content = format!("{}\n\n{}\n", import_statement, other_file_content);

    match fs::write(&index_path, file_content) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", format!("Error writing socket service file: {}", e));
            Err(Box::new(e))
        }
    }

}







