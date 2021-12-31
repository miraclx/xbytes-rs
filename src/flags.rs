#![allow(non_upper_case_globals)]

use bitflags::bitflags;

bitflags! {
    #[derive(Default)]
    pub struct Mode: u8 {
        const Default  = 0 << 0;
        const Bits     = 1 << 0;
        const Decimal  = 1 << 1;
        const NoPrefix = 1 << 2;
    }
}

bitflags! {
    #[derive(Default)]
    pub struct Format: u16 {
        const Default                = 0 << 0; // 1 B, 2.13 KB, 1024.43 MiB

        const Initials               = 1 << 0; // 1 B, 2.13 KB, 1024.43 MB
        const Condensed              = 1 << 1; // 1 B, 2.13 K, 1024.43 M
        const Long                   = 1 << 2; // 1 Byte, 2.13 KiloBytes, 1024.43 MebiBytes

        // (requires Long)
        const NoPlural               = 1 << 3; // 1 Byte, 2.13 KiloByte, 1024.43 MebiByte
        const ForcePlural            = 1 << 4; // 1 Bytes, 2.13 KiloBytes, 1024.43 MebiBytes

        // (requires Long)
        const NoMultiCaps            = 1 << 5; // 1 Byte, 2.13 Kilobytes, 1024.43 Mebibytes

        const LowerCaps              = 1 << 6; // 1 b, 2.13 kb, 1024.43 mib
        const UpperCaps              = 1 << 7; // 1 B, 2.13 KB, 1024.43 MIB

        const NoFraction             = 1 << 8; // 1 B, 2 KB, 1024 MiB
        const ForceFraction          = 1 << 9; // 1.00 B, 2.13 KB, 1024.43 MiB

        const ShowThousandsSeparator = 1 << 10; // 1 B, 2.13 KB, 1,024.43 MiB
        const NoSpace                = 1 << 11; // 1B, 2.13KB, 1024.43MiB
    }
}
