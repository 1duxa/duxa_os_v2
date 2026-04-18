#![no_main]
#![no_std]

use log::info;
use uefi::{
    Identify,
    boot::{MemoryType, SearchType},
    data_types::Align,
    mem::memory_map::MemoryMap,
    prelude::*,
    proto::media::{
        file::{Directory, File, FileAttribute, FileInfo, FileType},
        fs::SimpleFileSystem,
    },
};
use uefi_bootinfo::BootInfo;


#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    info!("Bootloader started");

    let mut dir = get_fs();

    let file = dir.open(
        cstr16!("EFI\\BOOT\\KERNEL.EFI"),
        uefi::proto::media::file::FileMode::Read,
        FileAttribute::empty(),
    );
    let mut file = match file.unwrap().into_type().unwrap() {
        FileType::Regular(f) => f,
        _ => panic!("Expected a regular file"),
    };

    let align = <FileInfo as Align>::alignment();
    let mut raw = [0u8; 512 + 8];
    let offset = raw.as_mut_ptr().align_offset(align);
    let buffer = &mut raw[offset..];
    let file_info = file.get_info::<FileInfo>(buffer).unwrap();
    let file_size = file_info.file_size() as usize;

    let pool = boot::allocate_pool(MemoryType::LOADER_DATA, file_size).unwrap();
    let kernel_buf = unsafe { core::slice::from_raw_parts_mut(pool.as_ptr(), file_size) };
    let mut total = 0;
    while total < file_size {
        let read = file.read(&mut kernel_buf[total..]).unwrap();
        if read == 0 {
            break;
        }
        total += read;
    }
    info!("Read {} bytes", total);
    info!("First 4 bytes: {:02x?}", &kernel_buf[..4]);

    let ehdr = unsafe { &*(kernel_buf.as_ptr() as *const Elf64Header) };
    assert_eq!(&ehdr.magic, b"\x7FELF", "Bad ELF magic");
    assert_eq!(ehdr.class, 2, "Need 64-bit ELF");
    assert_eq!(ehdr.endianness, 1, "Need little-endian");
    assert_eq!(ehdr.machine, 0x3E, "Need x86-64");

    let entry_point = ehdr.entry;
    info!("Entry point: {:#x}", entry_point);
    assert!(
        entry_point != 0,
        "Entry point is 0 — missing #[no_mangle] or linker script"
    );

    let phoff = ehdr.phoff as usize;
    let phentsize = ehdr.phentsize as usize;
    let phnum = ehdr.phnum as usize;

    let mut min_vaddr = usize::MAX;
    let mut max_vaddr = 0usize;

    for i in 0..phnum {
        let phdr =
            unsafe { &*(kernel_buf.as_ptr().add(phoff + i * phentsize) as *const Elf64Phdr) };
        if phdr.ptype != PT_LOAD || phdr.memsz == 0 {
            continue;
        }
        let start = phdr.vaddr as usize;
        let end = start + phdr.memsz as usize;
        if start < min_vaddr {
            min_vaddr = start;
        }
        if end > max_vaddr {
            max_vaddr = end;
        }
    }

    let page_start = min_vaddr & !0xFFF;
    let pages = (max_vaddr - page_start + 0xFFF) / 0x1000;
    boot::allocate_pages(
        boot::AllocateType::Address(page_start as u64),
        MemoryType::LOADER_DATA,
        pages,
    )
    .expect("Failed to allocate kernel memory");

    for i in 0..phnum {
        let phdr =
            unsafe { &*(kernel_buf.as_ptr().add(phoff + i * phentsize) as *const Elf64Phdr) };
        if phdr.ptype != PT_LOAD || phdr.memsz == 0 {
            continue;
        }

        let file_bytes = phdr.filesz as usize;
        let mem_bytes = phdr.memsz as usize;
        let vaddr = phdr.vaddr as usize;
        let file_offset = phdr.offset as usize;

        let dest = unsafe { core::slice::from_raw_parts_mut(vaddr as *mut u8, mem_bytes) };
        dest[..file_bytes].copy_from_slice(&kernel_buf[file_offset..file_offset + file_bytes]);
        dest[file_bytes..].fill(0);

        info!(
            "Loaded segment @ {:#x} filesz={} memsz={}",
            vaddr, file_bytes, mem_bytes
        );
    }
    info!("All segments loaded successfully");

    let stack_pages = 16u64;
    let stack_ptr = boot::allocate_pages(
        boot::AllocateType::AnyPages,
        MemoryType::LOADER_DATA,
        stack_pages as usize,
    )
    .expect("Failed to allocate stack");

    info!("Exiting boot services...");
    drop(dir);
    let memory_map = unsafe { boot::exit_boot_services(Some(MemoryType::LOADER_DATA)) };

    let stack_top = stack_ptr.as_ptr() as u64 + (stack_pages as u64) * 4096;
    let boot_info = BootInfo {
        mmap_ptr: memory_map.get(0).unwrap() as *const _ as u64,
        mmap_len: memory_map.len(),
        mmap_desc_size: memory_map.meta().desc_size,
        stack_top
    };

    let entry_point = ehdr.entry;

    info!(
        "Jumping to kernel at {:#x} with stack at {:#x}",
        entry_point, stack_top
    );

    unsafe {
        core::arch::asm!(
            "mov rsp, {stack}",
            "xor rbp, rbp",
            "mov rdi, {boot_info}",
            "jmp {entry}",
            stack = in(reg) boot_info.stack_top,
            boot_info = in(reg) &boot_info as *const BootInfo,
            entry = in(reg) entry_point,
            options(noreturn)
        );
    }
}

const PT_LOAD: u32 = 1;

#[repr(C)]
struct Elf64Header {
    magic: [u8; 4],
    class: u8,
    endianness: u8,
    version: u8,
    osabi: u8,
    _pad: [u8; 8],
    etype: u16,
    machine: u16,
    version2: u32,
    entry: u64,
    phoff: u64,
    shoff: u64,
    flags: u32,
    ehsize: u16,
    phentsize: u16,
    phnum: u16,
    shentsize: u16,
    shnum: u16,
    shstrndx: u16,
}

#[repr(C)]
struct Elf64Phdr {
    ptype: u32,
    flags: u32,
    offset: u64,
    vaddr: u64,
    paddr: u64,
    filesz: u64,
    memsz: u64,
    align: u64,
}

pub fn get_fs() -> Directory {
    let handles = boot::locate_handle_buffer(SearchType::ByProtocol(&SimpleFileSystem::GUID))
        .expect("Failed to locate FS handles");
    for handle in handles.iter() {
        if let Ok(mut fs) = boot::open_protocol_exclusive::<SimpleFileSystem>(*handle) {
            if let Ok(mut dir) = fs.open_volume() {
                if dir
                    .open(
                        cstr16!("EFI\\BOOT\\KERNEL.EFI"),
                        uefi::proto::media::file::FileMode::Read,
                        FileAttribute::empty(),
                    )
                    .is_ok()
                {
                    return dir;
                }
            }
        }
    }
    panic!("No suitable filesystem found");
}
