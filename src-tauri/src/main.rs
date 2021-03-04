#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use crate::cmd::{GetServersResponse, StatusAndServers};

mod cmd;


fn main() {
  tauri::AppBuilder::new()
    .invoke_handler(|_webview, arg| {
      use cmd::Cmd::*;
      match serde_json::from_str(arg) {
        Err(e) => {
          println!("{}", e.to_string());
          Err(e.to_string())
        }
        Ok(command) => {

          match command {
            // definitions for your custom commands from Cmd here
            GetServersCommand { callback, error } => tauri::execute_promise(
              _webview,
              move || {
                let body = reqwest::blocking::get("https://mcservers.jp/api/v1/server/list")?.text()?;

                let parsed = serde_json::from_str::<StatusAndServers>(&*body)?;

                let response = GetServersResponse {
                  servers: parsed.servers
                };
                Ok(response)
              },
              callback,
              error
            ),
          }
          Ok(())
        }
      }
    })
    .build()
    .run();
}
