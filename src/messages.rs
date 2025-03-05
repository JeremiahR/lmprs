use std::collections::HashMap;

use crate::message_types::MessageTypeEnum;
use crate::serialization::DoesSerialize;
use crate::serialization::MessageTypeElement;
use crate::serialization::RemainderElement;
use crate::serialization::SerializableElement;
use crate::serialization::SerializableTypes;
use crate::serialization::U16SizedBytesElement;

pub struct MessageDecoder {}

#[derive(Debug)]
pub enum MessageDecodeError {
    Error,
}

pub type MessageStructurePair = (String, SerializableTypes);

// And a list (Vec) of such tuples.
pub type StructurePairList = Vec<MessageStructurePair>;

#[derive(Debug)]
pub struct Message {
    pub message_type: MessageTypeEnum,
    pub elements: HashMap<String, SerializableElement>,
}

impl MessageDecoder {
    pub fn get_structure(
        msg_type: u16,
    ) -> Result<(MessageTypeEnum, StructurePairList), MessageDecodeError> {
        match MessageTypeEnum::try_from(msg_type) {
            Ok(MessageTypeEnum::Init) => Ok((
                MessageTypeEnum::Init,
                vec![
                    ("type".to_string(), SerializableTypes::MessageType),
                    (
                        "globalfeatures".to_string(),
                        SerializableTypes::U16SizedBytes,
                    ),
                    (
                        "localfeatures".to_string(),
                        SerializableTypes::U16SizedBytes,
                    ),
                    // ("remainder".to_string(), SerializableTypes::Remainder),
                ],
            )),
            Ok(_) => Ok((MessageTypeEnum::Unknown, vec![])),
            Err(_) => {
                println!("Unknown message type");
                Err(MessageDecodeError::Error)
            }
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<(Message, &[u8]), MessageDecodeError> {
        let (m, _) = MessageTypeElement::from_bytes(bytes).unwrap();
        let (message_type, structure) = MessageDecoder::get_structure(m.id).unwrap();
        let mut x = HashMap::new();
        let mut bytes = bytes;
        for (key, enum_type) in &structure {
            let (obj, rem_bytes) = match enum_type {
                SerializableTypes::MessageType => {
                    let (obj, bytes) = MessageTypeElement::from_bytes(bytes).unwrap();
                    (SerializableElement::MessageType(obj), bytes)
                }
                SerializableTypes::U16SizedBytes => {
                    let (obj, bytes) = U16SizedBytesElement::from_bytes(bytes).unwrap();
                    (SerializableElement::U16SizedBytes(obj), bytes)
                }
                SerializableTypes::Remainder => {
                    let (obj, bytes) = RemainderElement::from_bytes(bytes).unwrap();
                    (SerializableElement::Remainder(obj), bytes)
                }
            };
            bytes = rem_bytes;
            x.insert(key.clone(), obj);
        }
        Ok((
            Message {
                message_type,
                elements: x,
            },
            bytes,
        ))
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_decode_init_message() {
        let bytes = hex::decode("001000021100000708a0880a8a59a1012006226e46111a0b59caaf126043eb5bbf28c34f3a5e332a1fc7b2b73cf188910f2d7ef99482067a1b72fe9e411d37be8c").unwrap();
        let (msg, _remainder) = MessageDecoder::from_bytes(&bytes).unwrap();
        assert_eq!(msg.message_type, MessageTypeEnum::Init);
        // check that "type" is contained in msg.elements
        assert!(msg.elements.contains_key("type"));
        assert!(msg.elements.contains_key("globalfeatures"));
        assert!(msg.elements.contains_key("localfeatures"));
        // assert_eq!(msg.msg_type, 16);
        // assert_eq!(msg.name, "init");
        // assert_eq!(true, false);
    }
}
