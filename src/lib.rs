mod des;

use anyhow::{anyhow, Result};
use byteorder::{LittleEndian, ReadBytesExt};
use flate2::bufread::ZlibDecoder;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, Cursor, Read, Seek},
    path::Path,
};

const GRF_MAGIC: &str = "Master of Magic";
const GRF_HEADER_SIZE: usize = 46;
const GRF_VERSION: usize = 0x200;

enum EntryType {
    Unknown,
    Normal,
    EncryptedMixed,
    EncryptedHeader,
}

impl From<u8> for EntryType {
    fn from(item: u8) -> Self {
        match item {
            0x01 => Self::Normal,
            0x02 => Self::EncryptedMixed,
            0x04 => Self::EncryptedHeader,
            _ => Self::Unknown,
        }
    }
}

impl Into<u8> for EntryType {
    fn into(self) -> u8 {
        match self {
            EntryType::Unknown => 0x00,
            EntryType::Normal => 0x01,
            EntryType::EncryptedMixed => 0x02,
            EntryType::EncryptedHeader => 0x04,
        }
    }
}

#[derive(Debug)]
struct Header {
    pub magic: [u8; 15],
    pub key: [u8; 15],
    pub file_table_offset: u32,
    pub seed: u32,
    pub file_count: u32,
    pub version: u32,
}

#[derive(Debug)]
struct Entry {
    pub path: String,
    pub size_packed: u32,
    pub size_aligned: u32,
    pub size: u32,
    pub flag: u8,
    pub offset: u32,
}

#[derive(Debug)]
struct EntryTable {
    pub size_packed: u32,
    pub size: u32,
    pub entries: HashMap<String, Entry>,
}

#[derive(Debug)]
pub struct Archive<R: Read + Seek> {
    file: R,
    header: Header,
    entry_table: EntryTable,
}

impl<F> Archive<F>
where
    F: Seek + Read,
{
    pub fn from_reader(mut reader: F) -> Result<Archive<F>> {
        let header = parse_header(&mut reader)?;
        let entry_table = parse_file_table(
            &mut reader,
            header.file_table_offset as usize,
            header.file_count as usize,
        )?;

        Ok(Archive {
            file: reader,
            header,
            entry_table,
        })
    }

    pub fn list_entries(&self) -> Result<Vec<String>> {
        let entries: Vec<_> = self
            .entry_table
            .entries
            .keys()
            .map(|key| key.clone())
            .collect();

        Ok(entries)
    }

    pub fn read_entry(&mut self, entry_name: &str) -> Result<Vec<u8>> {
        let entry = self
            .entry_table
            .entries
            .get(entry_name)
            .ok_or(anyhow!("File entry does not exist"))?;

        let normal_type: u8 = EntryType::Normal.into();
        if (entry.flag & normal_type) == 0 {
            return Err(anyhow!(
                "File path is not a file, more than likely this is a folder"
            ));
        }

        self.file.seek(io::SeekFrom::Start(
            entry.offset as u64 + GRF_HEADER_SIZE as u64,
        ))?;

        let mut data = vec![0; entry.size_packed as usize];
        self.file.read(&mut data)?;

        if entry.size == entry.size_packed {
            return Ok(data);
        }

        decode_entry(&mut data, entry.flag, entry.size_aligned, entry.size_packed)
    }
}

impl Archive<File> {
    pub fn from_file_path(path: impl AsRef<Path>) -> Result<Archive<File>> {
        let file = File::open(path)?;

        Archive::from_reader(file)
    }
}

fn decode_entry(
    data: &mut [u8],
    flags: u8,
    aligned_size: u32,
    packed_size: u32,
) -> Result<Vec<u8>> {
    let encrypted_mixed_type: u8 = EntryType::EncryptedMixed.into();
    let encrypted_header_type: u8 = EntryType::EncryptedHeader.into();

    if (flags & encrypted_mixed_type) > 0 {
        des::decode_entry(data, aligned_size as usize, packed_size as usize);
    } else if (flags & encrypted_header_type) > 0 {
        des::decode_header(data, aligned_size as usize);
    }

    let mut decoder = ZlibDecoder::new(&data[..]);

    let mut unpacked_buf = vec![];
    decoder.read(&mut unpacked_buf)?;

    Ok(unpacked_buf)
}

fn parse_file_table<R: Read + Seek>(
    data: &mut R,
    offset: usize,
    file_count: usize,
) -> Result<EntryTable> {
    data.seek(io::SeekFrom::Start(GRF_HEADER_SIZE as u64 + offset as u64))?;

    let size_packed = data.read_u32::<LittleEndian>()?;
    let size_unpacked = data.read_u32::<LittleEndian>()?;

    let mut packed_buffer = vec![0; size_packed as usize];
    data.read(&mut packed_buffer)?;

    let mut table_decoded = ZlibDecoder::new(&packed_buffer[..]);

    let mut table_data = vec![0; size_unpacked as usize];
    table_decoded.read(&mut table_data)?;

    let mut table_cursor = Cursor::new(&table_data);

    let mut entries = HashMap::new();

    for _ in 1..file_count {
        let mut path_buf = vec![];

        while let Ok(character) = table_cursor.read_u8() {
            path_buf.push(character);

            if character == 0x00 {
                break;
            }
        }

        let path = String::from_utf8_lossy(&path_buf).trim().to_string();

        let entry_size_packed = table_cursor.read_u32::<LittleEndian>()?;
        let entry_size_aligned = table_cursor.read_u32::<LittleEndian>()?;
        let entry_size = table_cursor.read_u32::<LittleEndian>()?;
        let entry_flag = table_cursor.read_u8()?;
        let entry_offset = table_cursor.read_u32::<LittleEndian>()?;

        let entry = Entry {
            path: path.clone(),
            size_packed: entry_size_packed,
            size_aligned: entry_size_aligned,
            size: entry_size,
            flag: entry_flag,
            offset: entry_offset,
        };

        entries.insert(path, entry);
    }

    Ok(EntryTable {
        size_packed,
        size: size_unpacked,
        entries,
    })
}

fn parse_header<R: Read + Seek>(data: &mut R) -> Result<Header> {
    let mut header_data = [0; GRF_HEADER_SIZE];
    data.read(&mut header_data)?;

    let mut header_cursor = Cursor::new(header_data);

    let mut magic = [0; 15];
    header_cursor.read(&mut magic)?;

    let magic_string = std::str::from_utf8(&magic)?;
    if magic_string != GRF_MAGIC {
        return Err(anyhow!("Invalid GRF Archive, identifier not correct"));
    }

    let mut key = [0; 15];
    header_cursor.read(&mut key)?;

    let file_table_offset = header_cursor.read_u32::<LittleEndian>()?;
    let seed = header_cursor.read_u32::<LittleEndian>()?;
    let file_count = header_cursor.read_u32::<LittleEndian>()?;
    let version = header_cursor.read_u32::<LittleEndian>()?;

    if version != GRF_VERSION as u32 {
        return Err(anyhow!("Invalid GRF Archive, version != 0x200"));
    }

    let header = Header {
        magic,
        key,
        file_table_offset,
        seed,
        file_count: file_count - seed - 7,
        version,
    };

    Ok(header)
}
