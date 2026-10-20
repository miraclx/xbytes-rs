use xbytes::sizes::{BIT, BYTE, MEBI_BYTE};
use xbytes::{ByteSize, Int, Mode, sizes};

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

#[test]
#[cfg(not(feature = "bits"))]
fn integer_byte_construction_preserves_the_full_destination_range() {
    const ABOVE_BIT_CEILING: ByteSize = ByteSize::of_int(Int::MAX / 8 + 1, BYTE);
    const MAX_BYTES: ByteSize = ByteSize::of_int(Int::MAX, BYTE);
    assert_eq!(ABOVE_BIT_CEILING.byte_count(), Int::MAX / 8 + 1);
    assert_eq!(MAX_BYTES.byte_count(), Int::MAX);

    for unit in sizes::ALL {
        let unit_bits = unit.effective_value();
        if unit_bits == 1 {
            continue;
        }
        assert_eq!(unit_bits % 8, 0, "{unit} must scale to whole bytes");
        let unit_bytes = unit_bits / 8;
        let last_whole = Int::MAX / unit_bytes;
        for value in [0, 1, last_whole, last_whole.saturating_add(1), Int::MAX] {
            assert_eq!(
                ByteSize::of_int(value, unit).byte_count(),
                value.saturating_mul(unit_bytes),
                "{value} {unit} must saturate only at the byte-store ceiling"
            );
        }
    }
}

#[test]
#[cfg(feature = "bits")]
fn integer_construction_saturates_only_at_the_bit_store_ceiling() {
    const MAX_BYTES: ByteSize = ByteSize::of_int(Int::MAX, BYTE);
    assert_eq!(MAX_BYTES.bit_count(), Int::MAX);

    for unit in sizes::ALL {
        let unit_bits = unit.effective_value();
        let last_whole = Int::MAX / unit_bits;
        for value in [0, 1, last_whole, last_whole.saturating_add(1), Int::MAX] {
            assert_eq!(
                ByteSize::of_int(value, unit).bit_count(),
                value.saturating_mul(unit_bits),
                "{value} {unit} must saturate at the bit-store ceiling"
            );
        }
    }
}

#[test]
fn truncating_construction_and_lossy_counts_keep_sub_byte_flooring() {
    const ONE_BYTE: Int = ByteSize::of_int(9, BIT).byte_count_lossy();
    assert_eq!(ONE_BYTE, 1);

    for bits in [0, 1, 7, 8, 9, 16] {
        let integer = ByteSize::of_int(bits, BIT);
        let scalar = ByteSize::of(bits as f64, BIT);
        let parsed: ByteSize = format!("{bits} b").parse().unwrap();
        assert_eq!(integer.byte_count_lossy(), bits / 8);
        assert_eq!(scalar, integer);
        assert_eq!(parsed, integer);
    }
    assert_eq!(
        ByteSize::of_int(Int::MAX, BIT).byte_count_lossy(),
        Int::MAX / 8
    );
}
