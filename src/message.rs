pub enum MessageId {
    Choke = 0,
    UnChoke = 1,
    Interested = 2,
    NotInterested = 3,
    Have = 4,
    Bitfield = 5,
    Request = 6,
    Piece = 7,
    Cancel = 8,
    Port = 9,
    KeepAlive,
    Invalid = 999,
}

impl MessageId {
    pub fn new(id: u32) -> MessageId {
        match id {
            0 => MessageId::Choke,
            1 => MessageId::UnChoke,
            2 => MessageId::Interested,
            3 => MessageId::NotInterested,
            4 => MessageId::Have,
            5 => MessageId::Bitfield,
            6 => MessageId::Request,
            7 => MessageId::Piece,
            8 => MessageId::Cancel,
            9 => MessageId::Port,
            _ => MessageId::Invalid,
        }
    }
}

pub fn serialize_message(id: MessageId, payload: Option<Vec<u8>>) -> Vec<u8> {
    let mut message = Vec::new();

    let length = match id {
        MessageId::Bitfield | MessageId::Piece => {
            if let Some(payload) = &payload {
                payload.len() as u32
            } else {
                panic!("BitField and Piece must have a payload")
            }
        }
        _ => length_prefix(&id),
    };

    message.append(&mut length.to_be_bytes().to_vec());
    message.push(id as u8);
    if let Some(payload) = payload {
        message.append(&mut payload.to_vec());
    }

    message
}

fn length_prefix(id: &MessageId) -> u32 {
    match id {
        MessageId::Choke
        | MessageId::UnChoke
        | MessageId::Interested
        | MessageId::NotInterested => 1,

        MessageId::Have => 5,
        MessageId::Request | MessageId::Cancel => 13,
        MessageId::Port => 3,

        _ => 0,
    }
}
