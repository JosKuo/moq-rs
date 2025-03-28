use crate::coding::{Decode, DecodeError, Encode, EncodeError};
use crate::data::ObjectStatus;

#[derive(Clone, Debug)]
pub struct Datagram {
    // The subscribe ID.
    pub subscribe_id: u64,

    // The track alias.
    pub track_alias: u64,

    // The sequence number within the track.
    pub group_id: u64,

    // The object ID within the group.
    pub object_id: u64,

    // Publisher priority, where **smaller** values are sent first.
    pub publisher_priority: u8,

    // Object status
    pub object_status: ObjectStatus,

    // The payload length.
    pub payload_len: u64,

    // The payload.
    pub payload: bytes::Bytes,
}

impl Decode for Datagram {
    fn decode<R: bytes::Buf>(r: &mut R) -> Result<Self, DecodeError> {
        let subscribe_id = u64::decode(r)?;
        let track_alias = u64::decode(r)?;
        let group_id = u64::decode(r)?;
        let object_id = u64::decode(r)?;
        let publisher_priority = u8::decode(r)?;
        let object_status = ObjectStatus::decode(r)?;
        let payload_len = u64::decode(r)?;
        if payload_len != r.remaining() as u64 {
            return Err(DecodeError::InvalidLength(
                payload_len as usize,
                r.remaining(),
            ));
        }
        let payload = r.copy_to_bytes(r.remaining());

        Ok(Self {
            subscribe_id,
            track_alias,
            group_id,
            object_id,
            publisher_priority,
            object_status,
            payload_len,
            payload,
        })
    }
}

impl Encode for Datagram {
    fn encode<W: bytes::BufMut>(&self, w: &mut W) -> Result<(), EncodeError> {
        self.subscribe_id.encode(w)?;
        self.track_alias.encode(w)?;
        self.group_id.encode(w)?;
        self.object_id.encode(w)?;
        self.publisher_priority.encode(w)?;
        self.object_status.encode(w)?;
        self.payload_len.encode(w)?;
        Self::encode_remaining(w, self.payload.len())?;
        w.put_slice(&self.payload);

        Ok(())
    }
}
