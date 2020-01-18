// Made by Jacob Shin 2020
// Original Example Code: Copyright 2014-2016 the slack-rs authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
// `cargo run <api_key>`
//
//

use slack;
use slack::{Message, Event, RtmClient};

struct MyHandler;

#[allow(unused_variables)]
impl slack::EventHandler for MyHandler {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {

        //println!("{:?}", event);

        let mut joined = false;
        let mut username = String::new();

        match event {
            Event::Message(message) => {
                //println!("{:?}", message);

                match *message {
                    Message::ChannelJoin(m) => {
                        joined = true;

                        username.push_str(&m.user.unwrap());

                        println!("The new user {} has joined the channel.", username);
                    }
                    _ => (),

                }
            }
            Event::Hello => {
                println!("Bot has successfully connected.");
            }
            _ => (),
        }

        if joined {

            // find the general IM channel id from the `StartResponse`
            let general_channel_id = cli.start_response()
                .ims
                .as_ref()
                .and_then(|ims| {
                    ims
                    .iter()
                    .find(|im| match im.user {
                        None => false,
                        Some(ref user) => user == &*username,
                    })
                })
            .and_then(|im| im.id.as_ref())
            .expect("general channel not found");

            // Send a message over the real time api websocket
            let mut welcome = format!("Welcome <@{}>\n", username);
            welcome.push_str("Welcome to the CCExtractor Slack Community!\nIf you're here for Google Code-In 2019 (GCI) you can go to https://gci2019.ccextractor.org/.\nIf you're here for Google Summer of Code, you can visit https://www.ccextractor.org/public:gsoc:google_summer_of_code_2019.\nFinally, if you're just looking to contribute or need help using CCExtractor, feel free to stick around, ask questions, or visit https://www.ccextractor.org/.");

            println!("IM ID: {}", general_channel_id);
            let _ = cli.sender().send_message(&general_channel_id, &welcome);

            //self.on_close(cli);
        }
}

    fn on_close(&mut self, cli: &RtmClient) {
        println!("Closed connection.");
    }

    fn on_connect(&mut self, cli: &RtmClient) {
        println!("Connected.");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let api_key = match args.len() {
        0 | 1 => panic!("No api-key in args! Usage: cargo run <api-key>"),
        x => args[x - 1].clone(),
    };
    let mut handler = MyHandler;
    let r = RtmClient::login_and_run(&api_key, &mut handler);
    match r {
        Ok(_) => {}
        Err(err) => panic!("Error: {}", err),
    }
}
