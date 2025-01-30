// use std::error::Error;
// use tokio::process::Command as AsyncCommand;

// pub async fn start_with_bun(watch: bool) -> Result<(), Box<dyn Error>> {
//     let spawn_args = vec!["run", "--watch", "src/server.ts"];

//     if !watch {
//         spawn_args = vec!["run", "src/server.ts"];
//     }

//     let mut child = AsyncCommand::new("bun").args(&spawn_args).spawn()?;

//     // match child.wait().await {
//     //     Ok(status) => {
//     //         if !status.success() {
//     //             println!("{} {}", "[Dolph Error]: ", "Exciting compilation...");
//     //             return Err("Compilation error....".into());
//     //         }

//     //         println!("{} {}", "[Dolph Info]: ", "");
//     //     }

//     // }
// }
