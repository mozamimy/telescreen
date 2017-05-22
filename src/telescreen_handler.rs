use regex::{Captures, Regex};
use router::{Router, Rule};
use slack::{Event, EventHandler, Message, RtmClient};

pub struct TelescreenHandler {
    router: Router,
    username_regex: Regex,
}

impl TelescreenHandler {
    pub fn new(router: Router) -> TelescreenHandler {
        TelescreenHandler {
            router: router,
            username_regex: Regex::new(r"<@(.+)>").unwrap(),
        }
    }

    pub fn send_message(&self,
                        cli: &RtmClient,
                        source_channel_id: &str,
                        source_user_id: &str,
                        source_text: &str) {
        let channel_name = self.get_channel_name_from_id(cli, source_channel_id);
        let unwrapped_channel_name = match channel_name {
            None => {
                warn!("No channel: {:?}", source_channel_id);
                return;
            }
            Some(c) => c,
        };

        let user_name = self.get_user_name_from_id(cli, source_user_id);
        let unwrapped_user_name = match user_name {
            None => {
                warn!("No user: {:?}", user_name);
                return;
            }
            Some(t) => t,
        };

        let rules: &Vec<Rule> = self.router.rules.as_ref();
        for rule in rules {
            if rule.regex.is_match(unwrapped_channel_name) {
                let dest_channel_id = cli.start_response()
                    .channels
                    .as_ref()
                    .and_then(|channels| {
                                  channels
                                      .iter()
                                      .find(|chan| match chan.name {
                                                None => false,
                                                Some(ref name) => name == &(rule.destination),
                                            })
                              })
                    .and_then(|chan| chan.id.as_ref());

                let dest_channel_id_unwrap = match dest_channel_id {
                    None => {
                        warn!("No channel: {:?}", dest_channel_id);
                        return;
                    }
                    Some(c) => c,
                };

                if unwrapped_channel_name != &(rule.destination) {
                    let replaced_source_text = self.username_regex
                        .replace_all(source_text, |caps: &Captures| {
                            format!("@{}",
                                    self.get_user_name_from_id(cli, &caps[1])
                                        .unwrap_or(&String::from("unknown")))
                        });
                    let message = format!("{:} [ <#{}> ]:\n{:}",
                                          unwrapped_user_name,
                                          source_channel_id,
                                          replaced_source_text);
                    info!("MESSAGE: {:?}", message);
                    let _ = cli.sender().send_message(&dest_channel_id_unwrap, &message);
                }
            }
        }
    }

    fn get_channel_name_from_id<'a>(&self,
                                    cli: &'a RtmClient,
                                    channel_id: &str)
                                    -> Option<&'a String> {
        cli.start_response()
            .channels
            .as_ref()
            .and_then(|channels| {
                          channels
                              .iter()
                              .find(|chan| match chan.id {
                                        None => false,
                                        Some(ref id) => id == channel_id,
                                    })
                      })
            .and_then(|chan| chan.name.as_ref())
    }

    fn get_user_name_from_id<'a>(&self, cli: &'a RtmClient, user_id: &str) -> Option<&'a String> {
        cli.start_response()
            .users
            .as_ref()
            .and_then(|users| {
                          users
                              .iter()
                              .find(|user| match user.id {
                                        None => false,
                                        Some(ref name) => name.to_string() == user_id,
                                    })
                      })
            .and_then(|user| user.name.as_ref())
    }
}

#[allow(unused_variables)]
impl EventHandler for TelescreenHandler {
    fn on_event(&mut self, cli: &RtmClient, event: Event) {
        debug!("EVENT: {:?}", event);

        match event {
            Event::Message(event) => {
                match *event {
                    Message::Standard(message) => {
                        let source_channel_id = match message.channel {
                            None => {
                                warn!("No channel: {:?}", message.channel);
                                return;
                            }
                            Some(c) => c,
                        };
                        let source_user_id = match message.user {
                            None => {
                                warn!("No user: {:?}", message.user);
                                return;
                            }
                            Some(u) => u,
                        };
                        let source_text = match message.text {
                            None => {
                                warn!("No text: {:?}", message.text);
                                return;
                            }
                            Some(t) => t,
                        };

                        self.send_message(cli, &source_channel_id, &source_user_id, &source_text);
                    }
                    _ => { /* noop */ }
                }
            }
            _ => { /* noop */ }
        }
    }

    fn on_close(&mut self, cli: &RtmClient) {
        info!("Disconnected");
    }

    fn on_connect(&mut self, cli: &RtmClient) {
        info!("Connected");
    }
}
