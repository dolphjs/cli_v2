use std::{error::Error, fs};

use crate::utils::{capitalize_first_letter, ensure_directory_exists};

use super::config_writers::find_base_directory;

pub fn write_spring_controller(name: &str) -> Result<(), Box<dyn Error>> {
    let base_directory = find_base_directory().ok_or_else(|| "Could not find base directory")?;

    let component_path = base_directory.join("components");

    ensure_directory_exists(&component_path)?;

    let controller_path = component_path.join(format!("{}", name));

    ensure_directory_exists(&controller_path)?;

    let index_path = controller_path.join(format!("{}.controller.ts", name));

    let capitalized_name = capitalize_first_letter(name);

    let import_statement = format!(
        r#"import {{ DolphControllerHandler }} from "@dolphjs/dolph/classes";
import {{
  Dolph,
  SuccessResponse,
  DRequest,
  DResponse
}} from "@dolphjs/dolph/common";
import {{ Get, Route }} from "@dolphjs/dolph/decorators";"#
    );

    let other_body = format!(
        r#"@Route('{name}')
export class {capitalized_name}Controller extends DolphControllerHandler<Dolph> {{
  constructor() {{
    super();
    }}

  @Get("greet")
  async greet (req: DRequest, res: DResponse) {{
    SuccessResponse({{ res, body: {{ message: "you've reached the {name} endpoint." }} }});
    }};
}}"#
    );

    let file_content = format!("{}\n\n{}\n", import_statement, other_body);

    match fs::write(&index_path, file_content) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", format!("Error writing controller file: {}", e));
            Err(Box::new(e))
        }
    }
}
