use core::fmt::Display;

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MemoryType(pub u32);

impl MemoryType {
    pub const RESERVED: Self = Self(0);
    pub const LOADER_CODE: Self = Self(1);
    pub const LOADER_DATA: Self = Self(2);
    pub const BOOT_SERVICES_CODE: Self = Self(3);
    pub const BOOT_SERVICES_DATA: Self = Self(4);
    pub const RUNTIME_SERVICES_CODE: Self = Self(5);
    pub const RUNTIME_SERVICES_DATA: Self = Self(6);
    pub const CONVENTIONAL: Self = Self(7);
    pub const UNUSABLE: Self = Self(8);
    pub const ACPI_RECLAIM: Self = Self(9);
    pub const ACPI_NON_VOLATILE: Self = Self(10);
    pub const MMIO: Self = Self(11);
    pub const MMIO_PORT_SPACE: Self = Self(12);
    pub const PAL_CODE: Self = Self(13);
    pub const PERSISTENT_MEMORY: Self = Self(14);
    pub const UNACCEPTED: Self = Self(15);
    pub const KERNEL_MEMORY: Self = Self(0x80000000);

    pub fn is_conventional_memory(self) -> bool {
        matches!(
            self,
            Self::CONVENTIONAL | Self::BOOT_SERVICES_CODE | Self::BOOT_SERVICES_DATA
        )
    }
}
impl Display for MemoryType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let name = match *self {
            Self::RESERVED => "RESERVED",
            Self::LOADER_CODE => "LOADER_CODE",
            Self::LOADER_DATA => "LOADER_DATA",
            Self::BOOT_SERVICES_CODE => "BOOT_SERVICES_CODE",
            Self::BOOT_SERVICES_DATA => "BOOT_SERVICES_DATA",
            Self::RUNTIME_SERVICES_CODE => "RUNTIME_SERVICES_CODE",
            Self::RUNTIME_SERVICES_DATA => "RUNTIME_SERVICES_DATA",
            Self::CONVENTIONAL => "CONVENTIONAL",
            Self::UNUSABLE => "UNUSABLE",
            Self::ACPI_RECLAIM => "ACPI_RECLAIM",
            Self::ACPI_NON_VOLATILE => "ACPI_NON_VOLATILE",
            Self::MMIO => "MMIO",
            Self::MMIO_PORT_SPACE => "MMIO_PORT_SPACE",
            Self::PAL_CODE => "PAL_CODE",
            Self::PERSISTENT_MEMORY => "PERSISTENT_MEMORY",
            Self::UNACCEPTED => "UNACCEPTED",
            Self::KERNEL_MEMORY => "KERNEL_MEMORY",
            Self(v) => return write!(f, "UNKNOWN({:#x})", v),
        };
        f.write_str(name)
    }
}
