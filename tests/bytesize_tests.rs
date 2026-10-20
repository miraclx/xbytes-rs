use xbytes::sizes::{BIT, BYTE, MEBI_BYTE};
use xbytes::{ByteSize, Mode, sizes};

#[test]
fn an_explicit_bit_unit_denominates_a_byte_once() {
    let size = ByteSize::of_int(1, BYTE);
    assert_eq!(size.repr_as(BIT).to_string(), "8 b");
    assert_eq!(size.repr_as(BIT).no_space().to_string(), "8b");
    assert_eq!(size.repr_as(BIT), size.iec().pin(BIT));
}

#[test]
fn explicit_units_agree_with_pinning_in_both_denominations() {
    for size in [
        ByteSize::of_int(0, BYTE),
        ByteSize::of_int(1, BYTE),
        ByteSize::of_int(1536, BYTE),
        ByteSize::of_int(7, MEBI_BYTE),
    ] {
        for unit in sizes::ALL {
            assert_eq!(
                size.repr_as(unit),
                size.repr(Mode::NoPrefix).pin(unit),
                "explicit and pinned representations disagree for {size:?} at {unit}"
            );
        }
    }
}
