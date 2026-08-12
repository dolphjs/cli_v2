use std::{error::Error, fs};

use crate::utils::{capitalize_first_letter, ensure_directory_exists};

use super::config_writers::find_base_directory;

/// Writes a controller spec and a service spec, co-located with the
/// generated `{name}.controller.ts` / `{name}.service.ts` files. The
/// bodies are written to match those templates exactly (same class names,
/// same `greet` handler, same constructor shape), so a freshly scaffolded
/// component has a passing test from the start rather than an empty stub.
pub fn write_spring_test(name: &str) -> Result<(), Box<dyn Error>> {
    let base_directory = find_base_directory().ok_or_else(|| "Could not find base directory")?;

    let component_path = base_directory.join("components");

    ensure_directory_exists(&component_path)?;

    let test_path = component_path.join(format!("{}", name));

    ensure_directory_exists(&test_path)?;

    let capitalized_name = capitalize_first_letter(name);

    write_controller_spec(&test_path, name, &capitalized_name)?;
    write_service_spec(&test_path, name, &capitalized_name)?;

    Ok(())
}

fn write_controller_spec(
    dir: &std::path::Path,
    name: &str,
    capitalized_name: &str,
) -> Result<(), Box<dyn Error>> {
    let index_path = dir.join(format!("{}.controller.spec.ts", name));

    let file_content = format!(
        r#"import {{ {capitalized_name}Controller }} from "./{name}.controller";
import {{ DResponse }} from "@dolphjs/dolph/common";

// Tier 2 (unit): the controller is a plain class, so it's constructed
// directly here rather than through `@Component` — no Express, no DI.
// `res` is mocked because this handler writes to it directly; swap to
// asserting a return value instead if you switch to auto-return handlers.
describe("{capitalized_name}Controller", () => {{
  it("responds to GET /{name}/greet", async () => {{
    const controller = new {capitalized_name}Controller();

    const res = {{
      set: jest.fn().mockReturnThis(),
      status: jest.fn().mockReturnThis(),
      json: jest.fn().mockReturnThis(),
      send: jest.fn().mockReturnThis(),
    }} as unknown as DResponse;

    await controller.greet({{}} as any, res);

    expect(res.status).toHaveBeenCalledWith(200);
    expect(res.json).toHaveBeenCalledWith({{ message: "you've reached the {name} endpoint." }});
  }});
}});
"#
    );

    match fs::write(&index_path, file_content) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", format!("Error writing controller spec file: {}", e));
            Err(Box::new(e))
        }
    }
}

fn write_service_spec(
    dir: &std::path::Path,
    name: &str,
    capitalized_name: &str,
) -> Result<(), Box<dyn Error>> {
    let index_path = dir.join(format!("{}.service.spec.ts", name));

    let file_content = format!(
        r#"import {{ {capitalized_name}Service }} from "./{name}.service";

// Tier 1 (unit): services never need `@Component` to be constructed —
// replace this with real behaviour assertions as the service grows,
// passing mocked collaborators straight into the constructor.
describe("{capitalized_name}Service", () => {{
  it("is defined", () => {{
    const service = new {capitalized_name}Service();
    expect(service).toBeDefined();
  }});
}});
"#
    );

    match fs::write(&index_path, file_content) {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", format!("Error writing service spec file: {}", e));
            Err(Box::new(e))
        }
    }
}
