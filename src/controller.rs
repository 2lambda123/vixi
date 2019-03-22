use std::char;
use std::collections::HashMap;

use ncurses::*;
use xi_rpc::Peer;

#[derive(Default)]
pub struct Controller {
    view_id: String,
    size_y: i32,
    size_x: i32,
}

#[derive(Deserialize, Debug)]
pub struct ConfigMap(HashMap<String, String>);

impl Controller {
    pub fn open_file(&mut self, core: Box<dyn Peer>, file_path: &str) {
        let mut xi_config_dir = dirs::config_dir().expect("failed to retrieve your config dir");
        xi_config_dir.push("xi");

        core.send_rpc_notification(
            "client_started",
            &json!({ "config_dir": xi_config_dir.to_str().unwrap(), }),
        );

        let view_id = core
            .send_rpc_request("new_view", &json!({ "file_path": file_path }))
            .expect("failed to create the new view");

        self.view_id = view_id.as_str().unwrap().to_string();

        getmaxyx(stdscr(), &mut self.size_y, &mut self.size_x);
        core.send_rpc_notification(
            "edit",
            &json!({
                "method": "resize",
                "view_id": self.view_id,
                "params": {
                    "width": self.size_x,
                    "height": self.size_y,
                }
            }),
        );

        core.send_rpc_notification(
            "edit",
            &json!({
            "method": "scroll",
            "view_id": self.view_id,
            "params": [0 , self.size_y]
            }),
        );

        //core.send_rpc_notification(
        //"plugin",
        //&json!({
        //"command": "start",
        //"view_id": self.view_id,
        //"plugin_name": "syntect",
        //}),
        //);
    }

    pub fn start_keyboard_event_loop(&self, core: Box<dyn Peer>) {
        loop {
            let ch = getch();
            match ch {
                KEY_F1 => break,
                KEY_UP => {
                    core.send_rpc_notification(
                        "edit",
                        &json!({ "method": "move_up", "view_id": self.view_id}),
                    );
                }
                KEY_DOWN => {
                    core.send_rpc_notification(
                        "edit",
                        &json!({ "method": "move_down", "view_id": self.view_id}),
                    );
                }
                KEY_LEFT => {
                    core.send_rpc_notification(
                        "edit",
                        &json!({ "method": "move_left", "view_id": self.view_id}),
                    );
                }
                KEY_RIGHT => {
                    core.send_rpc_notification(
                        "edit",
                        &json!({ "method": "move_right", "view_id": self.view_id}),
                    );
                }
                _ => (),
            }

            match char::from_u32(ch as u32).expect("Invalid char") {
                'i' => {
                    core.send_rpc_notification(
                        "edit",
                        &json!({
                            "method": "insert",
                            "params": {
                                "chars": "!",
                            },
                            "view_id": self.view_id}),
                    );
                }
                _ => (),
            }
        }
    }
}