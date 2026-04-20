/// Supports marking as uncacheable.
pub const UNCACHEABLE: u64 = 0x1;
/// Supports write-combining.
pub const WRITE_COMBINE: u64 = 0x2;
/// Supports write-through.
pub const WRITE_THROUGH: u64 = 0x4;
/// Support write-back.
pub const WRITE_BACK: u64 = 0x8;
/// Supports marking as uncacheable, exported and
/// supports the "fetch and add" semaphore mechanism.
pub const UNCACHABLE_EXPORTED: u64 = 0x10;
/// Supports write-protection.
pub const WRITE_PROTECT: u64 = 0x1000;
/// Supports read-protection.
pub const READ_PROTECT: u64 = 0x2000;
/// Supports disabling code execution.
pub const EXECUTE_PROTECT: u64 = 0x4000;
/// Persistent memory.
pub const NON_VOLATILE: u64 = 0x8000;
/// This memory region is more reliable than other memory.
pub const MORE_RELIABLE: u64 = 0x10000;
/// This memory range can be set as read-only.
pub const READ_ONLY: u64 = 0x20000;
/// This memory is earmarked for specific purposes such as for specific
/// device drivers or applications. This serves as a hint to the OS to
/// avoid this memory for core OS data or code that cannot be relocated.
pub const SPECIAL_PURPOSE: u64 = 0x4_0000;
/// This memory region is capable of being protected with the CPU's memory
/// cryptography capabilities.
pub const CPU_CRYPTO: u64 = 0x8_0000;
/// This memory must be mapped by the OS when a runtime service is called.
pub const RUNTIME: u64 = 0x8000_0000_0000_0000;
/// This memory region is described with additional ISA-specific memory
/// attributes as specified in `MemoryAttribute::ISA_MASK`.
pub const ISA_VALID: u64 = 0x4000_0000_0000_0000;
/// These bits are reserved for describing optional ISA-specific cache-
/// ability attributes that are not covered by the standard UEFI Memory
/// Attribute cacheability bits such as `UNCACHEABLE`, `WRITE_COMBINE`,
/// `WRITE_THROUGH`, `WRITE_BACK`, and `UNCACHEABLE_EXPORTED`.
///
/// See Section 2.3 "Calling Conventions" in the UEFI Specification
/// for further information on each ISA that takes advantage of this.
pub const ISA_MASK: u64 = 0x0FFF_F000_0000_0000;
