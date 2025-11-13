pub trait ToBytes {
    fn to_bytes(&self, big_endian: bool) -> Vec<u8>;
}

macro_rules! impl_to_bytes {
    ($($t:ty),*) => {
        $(
            impl ToBytes for $t {
                fn to_bytes(&self, big_endian: bool) -> Vec<u8> {
                    let mut vec = Vec::new();
                    if (big_endian) {
                        vec.extend_from_slice(self.to_be_bytes().as_ref());
                    } else {
                        vec.extend_from_slice(self.to_le_bytes().as_ref());
                    }
                    vec
                }
            }
        )*
    };
}

impl_to_bytes!(u8, u16, u32, u64);

pub trait Serializable {
    fn serialize(&self, big_endian: bool) -> Vec<u8>;

    fn serialized_length(&self) -> usize {
        self.serialize(true).len()
    }
}

pub fn add_bytes<T: ToBytes>(vec: &mut Vec<u8>, val: T, be: bool) {
    vec.append(&mut val.to_bytes(be));
}

impl Serializable for String {
    fn serialize(&self, _: bool) -> Vec<u8> {
        let mut vec = Vec::from(self.as_bytes());

        vec.push(0x00);

        vec
    }

    fn serialized_length(&self) -> usize {
        self.len() + 1
    }
}

impl<T: Serializable> Serializable for Vec<T> {
    fn serialize(&self, be: bool) -> Vec<u8> {
        self.iter().flat_map(|x| x.serialize(be)).collect()
    }
}
