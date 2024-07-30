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
}

pub struct Node {
    pub id: String,
    pub node_ids: Vec<String>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            id: String::new(),
            node_ids: vec![],
        }
    }

    pub fn handle(&mut self, msg: Message) -> Option<Message> {
        match msg.body {
            Body::Echo { msg_id, echo } => Some(Message {
                src: self.id.to_string(),
                dest: msg.src,
                body: Body::EchoOk {
                    msg_id: msg_id + 1,
                    echo,
                    in_reply_to: msg_id,
                },
            }),
            Body::Init {
                msg_id,
                node_id,
                node_ids,
            } => {
                self.id = node_id;
                self.node_ids = node_ids;
                Some(Message {
                    src: self.id.to_string(),
                    dest: msg.src,
                    body: Body::InitOk {
                        in_reply_to: msg_id,
                    },
                })
            }
            _ => None,
        }
    }
}
