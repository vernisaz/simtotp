/*
function hmac is
    input:
        key:        Bytes    // Array of bytes
        message:    Bytes    // Array of bytes to be hashed
        hash:       Function // The hash function to use (e.g. SHA-1)
        blockSize:  Integer  // The block size of the hash function (e.g. 64 bytes for SHA-1)

    // Compute the block sized key
    block_sized_key = computeBlockSizedKey(key, hash, blockSize)

    o_key_pad ← block_sized_key xor [0x5c blockSize]   // Outer padded key
    i_key_pad ← block_sized_key xor [0x36 blockSize]   // Inner padded key

    return  hash(o_key_pad ∥ hash(i_key_pad ∥ message))

function computeBlockSizedKey is
    input:
        key:        Bytes    // Array of bytes
        hash:       Function // The hash function to use (e.g. SHA-1)
        blockSize:  Integer  // The block size of the hash function (e.g. 64 bytes for SHA-1)

    // Keys longer than blockSize are shortened by hashing them
    if (length(key) > blockSize) then
        key = hash(key)

    // Keys shorter than blockSize are padded to blockSize by padding with zeros on the right
    if (length(key) < blockSize) then
        return  Pad(key, blockSize) // Pad key with zeros to make it blockSize bytes long

    return  key
*/
use crate::Sha1;
// TODO use one instance Sha1
pub fn hmac(key: &[u8], message: &[u8], block_size: usize) -> [u8; 20] {
    let mut sha1 = Sha1::new();
    let key = compute_block_sized_key(key, &mut Sha1::new(), block_size);

    // Generate inner and outer keys
    let mut inner_key = vec![0u8; block_size];
    let mut outer_key = vec![0u8; block_size];
    for i in 0..block_size {
        inner_key[i] = 0x36 ^ key[i];
        outer_key[i] = 0x5c ^ key[i];
    }

    // Append the inner_key
    let mut msg = vec![0u8; message.len() + block_size];
    msg[0..inner_key.len()].copy_from_slice(&inner_key[..]);
    msg[inner_key.len()..].copy_from_slice(message);

    // Has the previous message and append the outer_key
    let mut result = vec![0u8; block_size + 20];
    result[0..outer_key.len()].copy_from_slice(&outer_key[..]);
    result[outer_key.len()..outer_key.len() + 20].copy_from_slice(&Sha1::new().hash(&msg)[..20]);

    // Hash the previous message
    sha1.hash(&result)
}

fn compute_block_sized_key(key: &[u8], sha1: &mut Sha1, block_size: usize) -> Vec<u8> {
    let mut res = vec![0u8; block_size];
    if key.len() > block_size {
        res[0..20].copy_from_slice(&sha1.hash(key))
    }
    if key.len() <= block_size {
        res[0..key.len()].copy_from_slice(key)
    }
    res
}
