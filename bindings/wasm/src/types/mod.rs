use core::{
    cell::RefCell,
    convert::TryFrom,
};
use iota_streams::{
    app::transport::tangle::client::{
        Client,
        SendOptions as ApiSendOptions,
    },
    app_channels::api::tangle::{
        Address as ApiAddress,
        ChannelType as ApiChannelType,
        MessageContent,
        UnwrappedMessage,
    },
    core::prelude::{
        Rc,
        String,
        ToString,
    },
    ddml::types::hex,
};
use wasm_bindgen::prelude::*;

use js_sys::Array;

pub type Result<T> = core::result::Result<T, JsValue>;
pub fn to_result<T, E: ToString>(r: core::result::Result<T, E>) -> Result<T> {
    r.map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub struct SendOptions {
    url: String,
    pub local_pow: bool,
}

impl From<SendOptions> for ApiSendOptions {
    fn from(options: SendOptions) -> Self {
        Self {
            url: options.url,
            local_pow: options.local_pow,
        }
    }
}

#[wasm_bindgen]
impl SendOptions {
    #[wasm_bindgen(constructor)]
    pub fn new(url: String, local_pow: bool) -> Self {
        Self { url, local_pow }
    }

    #[wasm_bindgen(setter)]
    pub fn set_url(&mut self, url: String) {
        self.url = url
    }

    #[wasm_bindgen(getter)]
    pub fn url(&self) -> String {
        self.url.clone()
    }

    #[wasm_bindgen]
    #[allow(clippy::should_implement_trait)]
    pub fn clone(&self) -> Self {
        SendOptions {
            url: self.url.clone(),
            local_pow: self.local_pow,
        }
    }
}

#[wasm_bindgen]
#[derive(Default, PartialEq)]
pub struct Address {
    addr_id: String,
    msg_id: String,
}

#[wasm_bindgen]
impl Address {
    #[wasm_bindgen(getter)]
    pub fn addr_id(&self) -> String {
        self.addr_id.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_addr_id(&mut self, addr_id: String) {
        self.addr_id = addr_id;
    }

    #[wasm_bindgen(getter)]
    pub fn msg_id(&self) -> String {
        self.msg_id.clone()
    }

    #[wasm_bindgen(setter)]
    pub fn set_msg_id(&mut self, msg_id: String) {
        self.msg_id = msg_id;
    }

    #[wasm_bindgen(static_method_of = Address)]
    pub fn from_string(link: String) -> Self {
        let link_vec: Vec<&str> = link
            .strip_prefix("<")
            .unwrap_or(&link)
            .strip_suffix(">")
            .unwrap_or(&link)
            .split(':')
            .collect();

        Address {
            addr_id: link_vec[0].to_string(),
            msg_id: link_vec[1].to_string(),
        }
    }

    #[wasm_bindgen]
    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        let mut link = String::new();
        link.push_str(&self.addr_id);
        link.push(':');
        link.push_str(&self.msg_id);
        link
    }

    pub fn copy(&self) -> Self {
        Address {
            addr_id: self.addr_id.clone(),
            msg_id: self.msg_id.clone(),
        }
    }
}

pub type ClientWrap = Rc<RefCell<Client>>;

impl TryFrom<Address> for ApiAddress {
    type Error = JsValue;
    fn try_from(addr: Address) -> Result<Self> {
        ApiAddress::from_str(&addr.addr_id, &addr.msg_id).map_err(|_err| JsValue::from_str("bad address"))
    }
}

pub fn get_message_contents(msgs: Vec<UnwrappedMessage>) -> Vec<UserResponse> {
    let mut payloads = Vec::new();
    for msg in msgs {
        match msg.body {
            MessageContent::SignedPacket {
                pk,
                public_payload: p,
                masked_payload: m,
            } => payloads.push(UserResponse::new(
                Address::from_string(msg.link.to_string()),
                None,
                Some(Message::new(Some(hex::encode(pk.to_bytes())), p.0, m.0)),
            )),
            MessageContent::TaggedPacket {
                public_payload: p,
                masked_payload: m,
            } => payloads.push(UserResponse::new(
                Address::from_string(msg.link.to_string()),
                None,
                Some(Message::new(None, p.0, m.0)),
            )),
            MessageContent::Sequence => (),
            _ => payloads.push(UserResponse::new(
                Address::from_string(msg.link.to_string()),
                None,
                None,
            )),
        };
    }
    payloads
}

#[wasm_bindgen]
pub enum ChannelType {
    SingleBranch,
    MultiBranch,
    SingleDepth,
}

impl From<ChannelType> for ApiChannelType {
    fn from(channel_type: ChannelType) -> Self {
        match channel_type {
            ChannelType::SingleBranch => ApiChannelType::SingleBranch,
            ChannelType::MultiBranch => ApiChannelType::MultiBranch,
            ChannelType::SingleDepth => ApiChannelType::SingleDepth,
        }
    }
}

#[wasm_bindgen]
pub struct UserResponse {
    link: Address,
    seq_link: Option<Address>,
    message: Option<Message>,
}

#[wasm_bindgen]
pub struct NextMsgId {
    pk: String,
    msgid: Address,
}

#[wasm_bindgen]
pub struct Message {
    pk: Option<String>,
    public_payload: Vec<u8>,
    masked_payload: Vec<u8>,
}

#[wasm_bindgen]
#[derive(Default)]
pub struct PskIds {
    ids: Vec<String>,
}

#[wasm_bindgen]
#[derive(Default)]
pub struct PublicKeys {
    pks: Vec<String>,
}

#[wasm_bindgen]
impl PskIds {
    pub fn add(&mut self, id: String) {
        self.ids.push(id);
    }

    pub fn get_ids(&self) -> Array {
        self.ids.iter().map(JsValue::from).collect()
    }
}

#[wasm_bindgen]
impl PublicKeys {
    pub fn add(&mut self, id: String) {
        self.pks.push(id);
    }

    pub fn get_pks(&self) -> Array {
        self.pks.iter().map(JsValue::from).collect()
    }
}

#[wasm_bindgen]
impl Message {
    pub fn default() -> Message {
        Self::new(None, Vec::new(), Vec::new())
    }

    pub fn new(pk: Option<String>, public_payload: Vec<u8>, masked_payload: Vec<u8>) -> Message {
        Message {
            pk,
            public_payload,
            masked_payload,
        }
    }

    pub fn get_pk(&self) -> String {
        self.pk.clone().unwrap_or_default()
    }

    pub fn get_public_payload(&self) -> Array {
        self.public_payload.clone().into_iter().map(JsValue::from).collect()
    }

    pub fn get_masked_payload(&self) -> Array {
        self.masked_payload.clone().into_iter().map(JsValue::from).collect()
    }
}

#[wasm_bindgen]
impl NextMsgId {
    pub fn new(pk: String, msgid: Address) -> Self {
        NextMsgId { pk, msgid }
    }

    pub fn get_pk(&self) -> String {
        self.pk.clone()
    }

    pub fn get_link(&self) -> Address {
        self.msgid.copy()
    }
}

#[wasm_bindgen]
impl UserResponse {
    pub fn new(link: Address, seq_link: Option<Address>, message: Option<Message>) -> Self {
        UserResponse {
            link,
            seq_link,
            message,
        }
    }

    pub fn from_strings(link: String, seq_link: Option<String>, message: Option<Message>) -> Self {
        let seq;
        if let Some(seq_link) = seq_link {
            seq = Some(Address::from_string(seq_link));
        } else {
            seq = None;
        }

        UserResponse {
            link: Address::from_string(link),
            seq_link: seq,
            message,
        }
    }

    pub fn copy(&self) -> Self {
        let mut seq = None;
        if !self.get_seq_link().eq(&Address::default()) {
            seq = Some(self.get_seq_link());
        }
        UserResponse::new(self.get_link(), seq, None)
    }

    pub fn get_link(&self) -> Address {
        let mut link = Address::default();
        link.set_addr_id(self.link.addr_id());
        link.set_msg_id(self.link.msg_id());
        link
    }

    pub fn get_seq_link(&self) -> Address {
        if self.seq_link.is_some() {
            let seq_link = self.seq_link.as_ref().unwrap();
            let mut link = Address::default();
            link.set_addr_id(seq_link.addr_id());
            link.set_msg_id(seq_link.msg_id());
            link
        } else {
            Address::default()
        }
    }

    pub fn get_message(&mut self) -> Message {
        if self.message.is_some() {
            let message = self.message.as_ref().unwrap();
            Message {
                pk: message.pk.clone(),
                public_payload: message.public_payload.clone(),
                masked_payload: message.masked_payload.clone(),
            }
        } else {
            Message::default()
        }
    }
}