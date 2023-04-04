use protobuf::ProtobufEnum;

use super::test_enum_alias_pb::*;

use protobuf_test_common::*;

#[test]
fn test_enum() {
    assert_eq!(10, EnumWithAlias::A.value());
    assert_eq!(10, EnumWithAlias::AAgain.value());
    assert_eq!(
        &[
            EnumWithAlias::Unknown,
            EnumWithAlias::A,
            EnumWithAlias::B,
            EnumWithAlias::AAgain,
        ],
        EnumWithAlias::values()
    );
    assert_eq!(EnumWithAlias::A, EnumWithAlias::AAgain);
}

#[test]
fn test_enum_in_message() {
    let mut m = TestEnumWithAlias::new();
    m.set_en(EnumWithAlias::A);
    test_serialize_deserialize("08 0a", &m);
}
