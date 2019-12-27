//! CKB uses Molecule serialization format, Please follow this
//! document https://github.com/nervosnetwork/rfcs/blob/1ed0da3c1fee42650e6f385346f76cdc9c140c6e/rfcs/0008-serialization/0008-serialization.md#molecule
//! for more details
use byteorder::{ByteOrder, LittleEndian};

pub struct Serializer();

impl Serializer {
    fn calculate_offsets(element_lengths: &[u32]) -> (u32, Vec<u32>) {
        const FULL_LENGTH_SIZE: u32 = 4; // 4 bytes
        const OFFSET_SIZE: u32 = 4; // 4 bytes

        // offsets only used for dynamic vec in molecule, serialize all offsets of items as 32 bit unsigned integer in little-endian
        // The result like "|size| record 1 offset | record 2 offset | ...",
        let header_length = FULL_LENGTH_SIZE + OFFSET_SIZE * element_lengths.len() as u32;
        let mut offsets = vec![];
        let mut total = header_length;

        for i in 0..element_lengths.len() {
            offsets.push(total as u32);
            total = total + (element_lengths[i] as u32);
        }

        (total, offsets)
    }

    pub fn serialize_u32(value: u32) -> Vec<u8> {
        let mut buf = [0; 4];
        LittleEndian::write_u32(&mut buf, value);
        buf.to_vec()
    }

    pub fn serialize_u64(value: u64) -> Vec<u8> {
        let mut buf = [0; 8];
        LittleEndian::write_u64(&mut buf, value);
        buf.to_vec()
    }

    pub fn serialize_struct(values: &[&[u8]]) -> Vec<u8> {
        let mut ret: Vec<u8> = vec![];

        for item in values.iter() {
            ret.extend(*item);
        }

        ret
    }

    /// Serialize dynamic vector
    ///
    /// There are three steps of serializing a dynvec:
    ///
    /// Serialize the full size in bytes as a 32 bit unsigned integer in little-endian.
    /// Serialize all offset of items as 32 bit unsigned integer in little-endian.
    /// Serialize all items in it.
    ///
    /// call calculate_offsets method for get offsets of items
    ///
    pub fn serialize_dynamic_vec(values: &[&[u8]]) -> Vec<u8> {
        let mut body: Vec<u8> = vec![];
        let mut element_lengths: Vec<u32> = vec![];

        for item in values.iter() {
            element_lengths.push(item.len() as u32);
            body.extend(*item);
        }

        let mut ret: Vec<u8> = vec![];

        let offsets = Serializer::calculate_offsets(&element_lengths);

        ret.extend(Serializer::serialize_u32(offsets.0));

        offsets.1.iter().for_each(|item| {
            ret.extend(Serializer::serialize_u32(*item));
        });

        ret.extend(body);

        ret
    }

    pub fn serialize_fixed_vec(values: &[&[u8]]) -> Vec<u8> {
        let mut ret: Vec<u8> = vec![];
        let mut body: Vec<u8> = vec![];

        let mut total_size = 0 as u32;

        for item in values.iter() {
            total_size = total_size + item.len() as u32;

            body.extend(*item);
        }

        ret.extend(Serializer::serialize_u32(total_size));
        ret.extend(body);

        ret
    }
}

#[cfg(test)]
mod tests {
    use crate::serializer::Serializer;

    #[test]
    fn serialize_struct() {
        let bytes = Serializer::serialize_struct(&vec![
            vec![0x11u8, 0x13u8].as_slice(),
            vec![0x20u8, 0x17u8, 0x9u8].as_slice(),
        ]);
        assert_eq!(hex::encode(bytes), "1113201709");
    }

    #[test]
    fn serialize_fixed_vec() {
        let bytes = Serializer::serialize_fixed_vec(&vec![hex::decode("1234567890abcdef")
            .unwrap()
            .as_slice()]);
        assert_eq!(hex::encode(bytes), "080000001234567890abcdef");
    }

    #[test]
    fn serialize_dynamic_vec() {
        let bytes = Serializer::serialize_dynamic_vec(&vec![]);
        assert_eq!(hex::encode(bytes), "04000000");

        let bytes = Serializer::serialize_dynamic_vec(&vec![hex::decode("020000001234")
            .unwrap()
            .as_slice()]);
        assert_eq!(hex::encode(bytes), "0e00000008000000020000001234");

        let bytes = Serializer::serialize_dynamic_vec(&vec![
            hex::decode("020000001234").unwrap().as_slice(),
            hex::decode("00000000").unwrap().as_slice(),
            hex::decode("020000000567").unwrap().as_slice(),
            hex::decode("0100000089").unwrap().as_slice(),
            hex::decode("03000000abcdef").unwrap().as_slice(),
        ]);
        assert_eq!(hex::encode(bytes), "34000000180000001e00000022000000280000002d00000002000000123400000000020000000567010000008903000000abcdef");
    }
}
