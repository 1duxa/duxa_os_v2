use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct UefiMemAttrs: u64 {
        /// Supports marking as uncacheable.
        const UNCACHEABLE = 0x1;
        /// Supports write-combining.
        const WRITE_COMBINE = 0x2;
        /// Supports write-through.
        const WRITE_THROUGH = 0x4;
        /// Support write-back.
        const WRITE_BACK = 0x8;
        /// Supports marking as uncacheable, exported and
        /// supports the "fetch and add" semaphore mechanism.
        const UNCACHABLE_EXPORTED = 0x10;
        /// Supports write-protection.
        const WRITE_PROTECT = 0x1000;
        /// Supports read-protection.
        const READ_PROTECT = 0x2000;
        /// Supports disabling code execution.
        const EXECUTE_PROTECT = 0x4000;
        /// Persistent memory.
        const NON_VOLATILE = 0x8000;
        /// This memory region is more reliable than other memory.
        const MORE_RELIABLE = 0x10000;
        /// This memory range can be set as read-only.
        const READ_ONLY = 0x20000;
        /// This memory is earmarked for specific purposes such as for specific
        /// device drivers or applications. This serves as a hint to the OS to
        /// avoid this memory for core OS data or code that cannot be relocated.
        const SPECIAL_PURPOSE = 0x4_0000;
        /// This memory region is capable of being protected with the CPU's memory
        /// cryptography capabilities.
        const CPU_CRYPTO = 0x8_0000;
        /// This memory must be mapped by the OS when a runtime service is called.
        const RUNTIME = 0x8000_0000_0000_0000;
        /// This memory region is described with additional ISA-specific memory
        /// attributes as specified in `MemoryAttribute::ISA_MASK`.
        const ISA_VALID = 0x4000_0000_0000_0000;
        /// These bits are reserved for describing optional ISA-specific cache-
        /// ability attributes that are not covered by the standard UEFI Memory
        /// Attribute cacheability bits such as `UNCACHEABLE`, `WRITE_COMBINE`,
        /// `WRITE_THROUGH`, `WRITE_BACK`, and `UNCACHEABLE_EXPORTED`.
        ///
        /// See Section 2.3 "Calling Conventions" in the UEFI Specification
        /// for further information on each ISA that takes advantage of this.
        const ISA_MASK= 0x0FFF_F000_0000_0000;
    }
}
