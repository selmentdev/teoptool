use elf::abi::*;
use elf::ElfBytes;
use elf::endian::AnyEndian;
use iced_x86::*;

fn main() {
    let path = std::path::PathBuf::from("examples/TestNumerics");
    let file_data = std::fs::read(path).expect("Could not read file.");
    let slice = file_data.as_slice();
    let file = ElfBytes::<AnyEndian>::minimal_parse(slice).expect("Open test1");

    let common = file.find_common_data().expect("shdrs should parse");
    let (dynsyms, strtab) = (common.symtab.unwrap(), common.symtab_strs.unwrap());

    let load_segments:Vec<_> = file.segments().unwrap().iter().filter(|x| {
        x.p_type == PT_LOAD
    }).collect();

    let headers = file.section_headers().expect("elf without sections?");

    for sym in dynsyms.iter() {
        let symbol_type = sym.st_symtype();
        let symbol_name = strtab.get(sym.st_name as usize);

        if symbol_type == STT_FUNC {
            if sym.st_shndx != SHN_XINDEX {
                if let Ok(name) = symbol_name {
                    let section = headers.get(sym.st_shndx as usize).unwrap();
                    if section.sh_type == SHT_PROGBITS {
                        let segment = load_segments[0];
                        let offset_first = (segment.p_offset + sym.st_value) as usize;
                        let offset_last = offset_first + sym.st_size as usize;
                        let bytes = &slice[offset_first..offset_last];

                        // Decode function code
                        let mut decoder = Decoder::new(64, bytes, DecoderOptions::NONE);
                        let mut total_count : usize = 0;
                        let mut scalar_floats : usize = 0;
                        let mut packed_floats : usize = 0;

                        // Collect stats about instructions
                        for insn in decoder.iter() {
                            total_count += 1;

                            match insn.memory_size().element_type() {
                                MemorySize::Float16 | MemorySize::Float32 | MemorySize::Float64 | MemorySize::Float128 => {
                                    if insn.memory_size().is_packed() {
                                        packed_floats += 1;
                                    } else {
                                        scalar_floats += 1;
                                    }
                                },
                                _ => {}
                            }
                        }

                        // Report SIMD usage
                        let total_floats = scalar_floats + packed_floats;
                        let pct_floats = total_floats as f64 /  total_count as f64;
                        let pct_packed_floats = packed_floats as f64 / total_floats as f64;
                        let pct_scalar_floats = scalar_floats as f64 / total_floats as f64;

                        let limit_float_insn = 0.3;
                        let limit_float_scalars = 0.3;

                        if (pct_floats > limit_float_insn) && (pct_scalar_floats > limit_float_scalars) {
                            println!("insn: {:>10}, floats {:>10} ({:>6.2}%), packed: {:>10} ({:>6.2}%), scalar: {:>10} ({:>6.2}%), name: {}",
                                     total_count,
                                     scalar_floats + packed_floats,
                                     pct_floats * 100.0,
                                     packed_floats,
                                     pct_packed_floats * 100.0,
                                     scalar_floats,
                                     pct_scalar_floats * 100.0,
                                name
                            );
                        }
                    }
                }
            }
        }
    }
}
