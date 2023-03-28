use protobuf::*;
use std::str;

//use protobuf_test_common::*;
//use protobuf_test_common::hex::decode_hex;
//use protobuf_test_common::hex::encode_hex;

use super::test_offsets_pb::*;

#[test]
fn test_str() {
    let mut pb_str = TestStr::new();
    pb_str.set_b("hello".to_string());

    let serialized = pb_str.write_to_bytes().unwrap();
    let parsed = parse_from_bytes::<TestStr>(&serialized).unwrap();

    let offset = parsed.get_b_offset();
    assert_eq!(*offset, 2); // I happen to know this, but more general would be nice
    
    let start = *offset as usize;
    let end = start + parsed.get_b().len();
    assert_eq!(str::from_utf8(&serialized[start..end]).unwrap(), "hello".to_string());
}

#[test]
fn test_bytes() {
    let mut pb_str = TestBytes::new();
    pb_str.set_b("hello".to_string().into());

    let serialized = pb_str.write_to_bytes().unwrap();
    let parsed = parse_from_bytes::<TestBytes>(&serialized).unwrap();

    let offset = parsed.get_b_offset();
    assert_eq!(*offset, 2); // I happen to know this, but more general would be nice
    
    let start = *offset as usize;
    let end = start + parsed.get_b().len();
    assert_eq!(str::from_utf8(&serialized[start..end]).unwrap(), "hello".to_string());
}

#[test]
fn test_multiple() {
    let mut pb_str = TestMultiple::new();
    pb_str.set_a("hello, ".to_string());
    pb_str.set_b("world".to_string());
    pb_str.set_c("!".to_string());

    let serialized = pb_str.write_to_bytes().unwrap();
    let parsed = parse_from_bytes::<TestMultiple>(&serialized).unwrap();

    let offset_a = parsed.get_a_offset();
    let offset_b = parsed.get_b_offset();
    let offset_c = parsed.get_c_offset();
    // calculate the offset with 2-byte VARINT plus the length of the string
    assert_eq!(*offset_a, 2);
    assert_eq!(*offset_b, 11);
    assert_eq!(*offset_c, 18);

    {
        let start = *offset_a as usize;
        let end = start + parsed.get_a().len();
        assert_eq!(str::from_utf8(&serialized[start..end]).unwrap(), "hello, ".to_string());
    }

    {
        let start = *offset_b as usize;
        let end = start + parsed.get_b().len();
        assert_eq!(str::from_utf8(&serialized[start..end]).unwrap(), "world".to_string());
    }

    {
        let start = *offset_c as usize;
        let end = start + parsed.get_c().len();
        assert_eq!(str::from_utf8(&serialized[start..end]).unwrap(), "!".to_string());
    }
}

#[test]
fn test_multiple_spaced() {
    let mut pb_str = TestMultipleSpaced::new();
    pb_str.set_a(10);
    pb_str.set_b("helloworld".to_string());
    pb_str.set_d(", okay!".to_string().into());

    let serialized = pb_str.write_to_bytes().unwrap();
    let parsed = parse_from_bytes::<TestMultipleSpaced>(&serialized).unwrap();

    let offset_b = parsed.get_b_offset();
    let offset_d = parsed.get_d_offset();
    // int fields are VARINT length 2 here
    assert_eq!(*offset_b, 4);
    assert_eq!(*offset_d, 16);

    {
        let start = *offset_b as usize;
        let end = start + parsed.get_b().len();
        assert_eq!(str::from_utf8(&serialized[start..end]).unwrap(), "helloworld".to_string());
    }

    {
        let start = *offset_d as usize;
        let end = start + parsed.get_d().len();
        assert_eq!(str::from_utf8(&serialized[start..end]).unwrap(), ", okay!".to_string());
    }
}

#[test]
fn test_like_entry() {
    let mut pb_str = LikeEntry::new();
    pb_str.set_term(10);
    pb_str.set_index(1);
    pb_str.set_data("helloworld".to_string().into());

    let serialized = pb_str.write_to_bytes().unwrap();
    let parsed = parse_from_bytes::<LikeEntry>(&serialized).unwrap();

    let offset_data = parsed.get_data_offset();
    assert_eq!(*offset_data, 20);

    let start = *offset_data as usize;
    let end = start + parsed.get_data().len();
    assert_eq!(str::from_utf8(&serialized[start..end]).unwrap(), "helloworld".to_string());
}

#[test]
fn test_nested_msg() {
    let mut pb_orig = NestedMsg::new();
    let mut pb_str = TestStr::new();
    pb_str.set_b("hello".to_string());
    pb_orig.set_s(pb_str);
    let serialized = pb_orig.write_to_bytes().unwrap();
    let parsed = parse_from_bytes::<NestedMsg>(&serialized).unwrap();
    let offset = parsed.get_s_offset();
    assert_eq!(*offset, 2);

    {
        let pb_str = parsed.get_s();
        let offset_str = pb_str.get_b_offset();
        assert_eq!(*offset_str, 4); // offsets are cumulative, good!
    }

}

#[test]
fn test_repeated_nested_msg() {
    let mut pb_orig = RepeatedNestMsg::new();
    let mut pb_str1 = TestStr::new();
    pb_str1.set_b("hello, ".to_string());

    let mut pb_str2 = TestStr::new();
    pb_str2.set_b("world".to_string());

    let mut pb_str3 = TestStr::new();
    pb_str3.set_b("! I parsed a protobuf".to_string());

    pb_orig.set_s(RepeatedField::from_slice(&[pb_str1, pb_str2, pb_str3]));

    let serialized = pb_orig.write_to_bytes().unwrap();
    let parsed = parse_from_bytes::<RepeatedNestMsg>(&serialized).unwrap();
    let offset = parsed.get_s_offset();
//    assert_eq!(*offset, 2);

    {
        let pb_strs = parsed.get_s();
        {
            let offset = (*pb_strs)[0].get_b_offset();
            assert_eq!(*offset, 4);
            let start = *offset as usize;
            let end = start + (*pb_strs)[0].get_b().len();
            assert_eq!(str::from_utf8(&serialized[start..end]).unwrap(), "hello, ".to_string());
        }

        {
            let offset = (*pb_strs)[1].get_b_offset();
            assert_eq!(*offset, 15);
            let start = *offset as usize;
            let end = start + (*pb_strs)[1].get_b().len();
            assert_eq!(str::from_utf8(&serialized[start..end]).unwrap(), "world".to_string());            
        }

        {
            let offset = (*pb_strs)[2].get_b_offset();
            assert_eq!(*offset, 24);
            let start = *offset as usize;
            let end = start + (*pb_strs)[2].get_b().len();
            assert_eq!(str::from_utf8(&serialized[start..end]).unwrap(), "! I parsed a protobuf".to_string());            
        }
    }
}


