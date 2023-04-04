use protobuf::*;

use super::test_enum_values_pb::*;

#[test]
fn test_enum_values() {
    let expected = [
        TestEnumValuesEnum::Unknown,
        TestEnumValuesEnum::Winter,
        TestEnumValuesEnum::Spring,
        TestEnumValuesEnum::Summer,
        TestEnumValuesEnum::Autumn,
    ];
    assert_eq!(expected, TestEnumValuesEnum::values());
}
