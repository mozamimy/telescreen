use slack::{Event, RtmClient, EventHandler, Message};
use router::{Router,Rule};

pub struct TelescreenHandler {
    router: Router,
}

impl TelescreenHandler {
    pub fn new(router: Router) -> TelescreenHandler {
        TelescreenHandler { router: router }
    }
}

#[allow(unused_variables)]
impl EventHandler for TelescreenHandler {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        match event {
            Event::Message(event) => {
                match *event {
                    Message::Standard(message) => {
                        let source_user_id = match message.user {
                            None => { println!("No user: {:?}", message.user); return },
                            Some(u) => u,
                        };
                        let source_channel_id = match message.channel {
                            None => { println!("No channel: {:?}", message.channel); return },
                            Some(c) => c,
                        };
                        let source_text = match message.text {
                            None => { println!("No text: {:?}", message.text); return },
                            Some(t) => t,
                        };

                        let source_user_name = cli.start_response()
                            .users
                            .as_ref()
                            .and_then(|users| {
                                users.iter().find(|user| match user.id {
                                    None => false,
                                    Some(ref name) => name.to_string() == source_user_id,
                                })
                            }).and_then(|user| user.name.as_ref());

                        let unwrapped_source_user_name = match source_user_name {
                            None => { println!("No user: {:?}", source_user_name); return },
                            Some(t) => t,
                        };

                        let channel_name = cli.start_response().channels.as_ref()
                            .and_then(|channels| {
                                channels.iter().find(|chan| match chan.id {
                                    None => false,
                                    Some(ref id) => { id == &source_channel_id },
                                })
                            })
                            .and_then(|chan| chan.name.as_ref());

                        let unwrapped_channel_name = match channel_name {
                            None => { println!("No channel: {:?}", source_channel_id); return },
                            Some(c) => c,
                        };

                        let rules: &Vec<Rule> = self.router.rules.as_ref();
                        for rule in rules {
                            if rule.regex.is_match(unwrapped_channel_name) {
                                let dest_channel_id = cli.start_response().channels.as_ref()
                                    .and_then(|channels| {
                                        channels.iter().find(|chan| match chan.name {
                                            None => false,
                                            Some(ref name) => name == &(rule.destination),
                                        })
                                    })
                                    .and_then(|chan| chan.id.as_ref());

                                let dest_channel_id_unwrap = match dest_channel_id {
                                    None => { println!("No channel: {:?}", dest_channel_id); return },
                                    Some(c) => c,
                                };

                                if unwrapped_channel_name != &(rule.destination) {
                                    let message = format!("{:} (#{}): {:}", unwrapped_source_user_name, unwrapped_channel_name, source_text);
                                    let _ = cli.sender().send_message(&dest_channel_id_unwrap, &message);
                                }
                            }
                        }
                    },
                    _ => { /* noop */ },
                }
            },
            _ => { /* noop */ },
        }
    }

    fn on_close(&mut self, cli: &RtmClient) {
        println!("Disconnected");
    }

    fn on_connect(&mut self, cli: &RtmClient) {
        println!("Connected");
    }
}
