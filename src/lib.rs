use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Body,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Body {
    #[serde(rename = "echo")]
    Echo { msg_id: usize, echo: String },
    #[serde(rename = "echo_ok")]
    EchoOk {
        msg_id: usize,
        echo: String,
        in_reply_to: usize,
    },
    #[serde(rename = "init")]
    Init {
        msg_id: usize,
        node_id: String,
        node_ids: Vec<String>,
    },
    #[serde(rename = "init_ok")]
    InitOk { in_reply_to: usize },

    #[serde(rename = "generate")]
    Generate { msg_id: usize },
    #[serde(rename = "generate_ok")]
    GenerateOk { id: String, in_reply_to: usize },
    #[serde(rename = "broadcast")]
    Broadcast { msg_id: Option<usize>, message: usize },
    #[serde(rename = "broadcast_ok")]
    BroadcastOk { in_reply_to: usize },
    #[serde(rename = "read")]
    Read { msg_id: usize },
    #[serde(rename = "read_ok")]
    ReadOk {
        in_reply_to: usize,
        messages: HashSet<usize>,
    },
    #[serde(rename = "topology")]
    Topology {
        msg_id: usize,
        topology: HashMap<String, Vec<String>>,
    },
    #[serde(rename = "topology_ok")]
    TopologyOk { in_reply_to: usize },
}

pub struct Node {
    pub id: String,
    pub node_ids: Vec<String>,
    pub messages: HashSet<usize>,
}

impl Default for Node {
    fn default() -> Self {
        Self::new()
    }
}

impl Node {
    pub fn new() -> Self {
        Node {
            id: String::new(),
            node_ids: vec![],
            messages: HashSet::new(),
        }
    }

    pub fn handle(&mut self, msg: Message) -> Vec<Message> {
        match msg.body {
            Body::Echo { msg_id, echo } => vec![Message {
                src: self.id.to_string(),
                dest: msg.src,
                body: Body::EchoOk {
                    msg_id: msg_id + 1,
                    echo,
                    in_reply_to: msg_id,
                },
            }],
            Body::Init {
                msg_id,
                node_id,
                node_ids,
            } => {
                self.id = node_id;
                self.node_ids = node_ids;
                vec![Message {
                    src: self.id.to_string(),
                    dest: msg.src,
                    body: Body::InitOk {
                        in_reply_to: msg_id,
                    },
                }]
            }
            Body::Generate { msg_id } => {
                let id = format!("{}-{}", self.id, msg_id);
                vec![Message {
                    src: self.id.to_string(),
                    dest: msg.src,
                    body: Body::GenerateOk {
                        id,
                        in_reply_to: msg_id,
                    },
                }]
            }
            Body::Broadcast { msg_id, message } => {
                let mut responses = vec![];

                if !self.messages.contains(&message) {
                    self.messages.insert(message);

                    for neighbor in &self.node_ids {
                        responses.push(Message {
                            src: self.id.to_string(),
                            dest: neighbor.to_string(),
                            body: Body::Broadcast {
                                msg_id: None,
                                message,
                            },
                        });
                    }

                    if let Some(id) = msg_id {
                        responses.push(Message {
                            src: self.id.to_string(),
                            dest: msg.src,
                            body: Body::BroadcastOk {
                                in_reply_to: id,
                            },
                        });
                    }
                }

                responses
            }
            Body::Read { msg_id } => vec![Message {
                src: self.id.to_string(),
                dest: msg.src,
                body: Body::ReadOk {
                    in_reply_to: msg_id,
                    messages: self.messages.clone(),
                },
            }],
            Body::Topology { msg_id, topology } => {
                self.node_ids.clone_from(topology.get(&self.id).unwrap());
                vec![Message {
                    src: self.id.to_string(),
                    dest: msg.src,
                    body: Body::TopologyOk {
                        in_reply_to: msg_id,
                    },
                }]
            }
            _ => vec![],
        }
    }
}
