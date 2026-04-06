//! gpt_gen.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::struct;

pub const IMG_PATH: &str = r"C:\Genlex_Linear\sarah_os.img";
pub const GPT_PATH: &str = r"C:\Genlex_Linear\sarah_os_gpt.img";
pub fn create_gpt_image() {
        size = 128 * 1024 * 1024;
        image = bytearray ( size );
        image [ 446 : 446 + 16 ] = struct . pack ( "<BBBBBBBBII" ,;
        0x00 , 0x00 , 0x02 , 0x00 , 0x EE , 0x FF , 0x FF , 0x FF , 0x00000001 , ( size / / 512 ) - 1 );
        image [ 510 ] = 0x55;
        image [ 511 ] = 0x AA;
        header_lba = 1;
        image [ 512 * header_lba : 512 * header_lba + 8 ] = b "EFI PART";
        image [ 512 * header_lba + 8 : 512 * header_lba + 12 ] = b "\x00\x00\x01\x00";
        image [ 512 * header_lba + 12 : 512 * header_lba + 16 ] = struct . pack ( "<I" , 92 );
        image [ 512 * header_lba + 24 : 512 * header_lba + 32 ] = struct . pack ( "<Q" , 1 );
        image [ 512 * header_lba + 32 : 512 * header_lba + 40 ] = struct . pack ( "<Q" , ( size / / 512 ) - 1 );
        image [ 512 * header_lba + 40 : 512 * header_lba + 48 ] = struct . pack ( "<Q" , 34 );
        image [ 512 * header_lba + 48 : 512 * header_lba + 56 ] = struct . pack ( "<Q" , ( size / / 512 ) - 34 );
        part_type_guid = b "\x28\x73\x2A\xC1\x1F\xF8\xD2\x11\xBA\x4B\x00\xA0\xC9\x3E\xC9\x3B";
        unique_guid = b "\x47\x45\x4E\x45\x53\x49\x53\x5F\x53\x4F\x56\x45\x52\x45\x49\x47";
        entry_offset = 512 * 2;
        image [ entry_offset : entry_offset + 16 ] = part_type_guid;
        image [ entry_offset + 16 : entry_offset + 32 ] = unique_guid;
        image [ entry_offset + 32 : entry_offset + 40 ] = struct . pack ( "<Q" , 2048 );
        image [ entry_offset + 40 : entry_offset + 48 ] = struct . pack ( "<Q" , ( size / / 512 ) - 2048 );
        with open ( GPT_PATH , "wb" ) as f ;
        f . write ( image );
        println!( f "SUCCESS: Created GPT-compliant image at {GPT_PATH}" );
        fn main() {
        create_gpt_image ( );
}

