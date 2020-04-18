use serde::{Serialize, Deserialize};
use client::Args;
use std::mem::{size_of_val, size_of};

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    pub usize block_number;
    pub bool is_last;
    pub u32 hop_limit;
    pub bytes[] content;
}

impl Packet {
    pub new(hop_limit: u32, content: bytes[]) -> Vec<Self> {
        let content_block = 4096 - (size_of<u32> + size_of<bool>() + size_of<usize>());
        let mut packets = Vec::new();
        let mut iter = content.chunks_exact(content_block);
        let mut id = 0;
        for chunk in iter.next() {
            packets.push(Packet { dest: dest, block_id: id++, is_last: false, hop_limit: hop_limit, content: chunk } );
        };
        packets.push(Packet { dest: dest, block_id: id, is_last: true, hop_limit: hop_limit, content: iter.reminder() });
        packets
    }
}
