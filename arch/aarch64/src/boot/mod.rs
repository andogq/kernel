use core::{arch::naked_asm, marker::PhantomData};

// #[naked]
// #[no_mangle]
// pub unsafe extern "C" fn _start() -> ! {
//     naked_asm!(
//         include_str!("boot.s"),
//         CONST_CURRENTEL_EL2 = const 0x8,
//         CONST_CORE_ID_MASK = const 0b11,
//         CONST_BOOT_CORE_ID = const BOOT_CORE_ID,
//     );
// }

pub struct S<SInfo> {
    _phantom: PhantomData<SInfo>,
}

impl<SInfo: Info> S<SInfo> {
    const BOOT_CORE_ID: usize = SInfo::BOOT_CORE_ID;

    pub const fn new() -> Self {
        Self {
            _phantom: PhantomData,
        }
    }

    #[naked]
    #[no_mangle]
    pub unsafe extern "C" fn _start() -> ! {
        naked_asm!(
            include_str!("boot.s"),
            CONST_CURRENTEL_EL2 = const 0x8,
            CONST_CORE_ID_MASK = const 0b11,
            CONST_BOOT_CORE_ID = const Self::BOOT_CORE_ID,
        )
    }
}

pub trait Info {
    const BOOT_CORE_ID: usize;
}
