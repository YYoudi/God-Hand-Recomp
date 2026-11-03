//! Read-only ISO 9660 root access, implemented from the ECMA-119 record layout.
//! No game executable is decoded or executed. No data is written or extracted.
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

pub mod afs;
pub mod dat;
pub mod fst;
pub mod scr;
pub mod tim2;

const SECTOR: u64 = 2048;
const MAX_DIRECTORY: u64 = 16 * 1024 * 1024;

fn invalid(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

fn both_u32(bytes: &[u8]) -> io::Result<u32> {
    if bytes.len() < 8 {
        return Err(invalid("truncated ISO number"));
    }
    let le = u32::from_le_bytes(bytes[..4].try_into().unwrap());
    let be = u32::from_be_bytes(bytes[4..8].try_into().unwrap());
    if le != be {
        return Err(invalid("inconsistent ISO number byte orders"));
    }
    Ok(le)
}

#[derive(Debug, Clone)]
pub struct Entry {
    pub name: String,
    pub size: u32,
    sector: u32,
    directory: bool,
}

fn record(bytes: &[u8]) -> io::Result<Entry> {
    if bytes.len() < 34 || usize::from(bytes[0]) != bytes.len() {
        return Err(invalid("invalid ISO directory record"));
    }
    let name_end = 33 + usize::from(bytes[32]);
    if name_end > bytes.len() {
        return Err(invalid("truncated ISO name"));
    }
    // Extended attribute blocks and interleaving are intentionally unsupported.
    if bytes[1] != 0 || bytes[26] != 0 || bytes[27] != 0 {
        return Err(invalid("unsupported ISO extended or interleaved record"));
    }
    if bytes[25] & 0x80 != 0 {
        return Err(invalid("multi-extent ISO files are unsupported"));
    }
    Ok(Entry {
        name: String::from_utf8_lossy(&bytes[33..name_end]).into_owned(),
        sector: both_u32(&bytes[2..10])?,
        size: both_u32(&bytes[10..18])?,
        directory: bytes[25] & 2 != 0,
    })
}

pub struct Disc<R> {
    source: R,
    length: u64,
    root: Entry,
    pub volume_id: String,
}

/// A read-only view whose relative seeks and reads cannot escape an ISO file extent.
pub struct FileSlice<'a, R> {
    source: &'a mut R,
    start: u64,
    length: u64,
    position: u64,
}

impl<R: Read + Seek> Read for FileSlice<'_, R> {
    fn read(&mut self, output: &mut [u8]) -> io::Result<usize> {
        let count = (self.length - self.position).min(output.len() as u64) as usize;
        if count == 0 {
            return Ok(0);
        }
        self.source
            .seek(SeekFrom::Start(self.start + self.position))?;
        let read = self.source.read(&mut output[..count])?;
        self.position += read as u64;
        Ok(read)
    }
}

impl<R> Seek for FileSlice<'_, R> {
    fn seek(&mut self, origin: SeekFrom) -> io::Result<u64> {
        let position = match origin {
            SeekFrom::Start(value) => i128::from(value),
            SeekFrom::End(delta) => i128::from(self.length) + i128::from(delta),
            SeekFrom::Current(delta) => i128::from(self.position) + i128::from(delta),
        };
        if !(0..=i128::from(self.length)).contains(&position) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "seek outside ISO file extent",
            ));
        }
        self.position = position as u64;
        Ok(self.position)
    }
}

impl Disc<File> {
    pub fn open(path: impl AsRef<Path>) -> io::Result<Self> {
        // File::open is read-only; never use create/open with write permissions here.
        Self::read(File::open(path)?)
    }
}

impl<R: Read + Seek> Disc<R> {
    pub fn read(mut source: R) -> io::Result<Self> {
        let length = source.seek(SeekFrom::End(0))?;
        source.seek(SeekFrom::Start(16 * SECTOR))?;
        let mut pvd = [0u8; SECTOR as usize];
        source.read_exact(&mut pvd)?;
        if pvd[0] != 1 || &pvd[1..6] != b"CD001" || pvd[6] != 1 {
            return Err(invalid("ISO 9660 primary volume descriptor not found"));
        }
        if pvd[128..132] != [0, 8, 8, 0] {
            return Err(invalid("only 2048-byte ISO blocks are supported"));
        }
        let root_end = 156 + usize::from(pvd[156]);
        if root_end > pvd.len() {
            return Err(invalid("truncated ISO root record"));
        }
        let root = record(&pvd[156..root_end])?;
        if !root.directory {
            return Err(invalid("ISO root is not a directory"));
        }
        let volume_id = String::from_utf8_lossy(&pvd[40..72]).trim().to_owned();
        let disc = Self {
            source,
            length,
            root,
            volume_id,
        };
        disc.check_range(&disc.root)?;
        Ok(disc)
    }

    fn check_range(&self, entry: &Entry) -> io::Result<()> {
        let start = u64::from(entry.sector) * SECTOR;
        let end = start
            .checked_add(u64::from(entry.size))
            .ok_or_else(|| invalid("ISO extent overflow"))?;
        if end > self.length {
            return Err(invalid("ISO extent outside image"));
        }
        Ok(())
    }

    fn contents(&mut self, entry: &Entry, limit: u64) -> io::Result<Vec<u8>> {
        self.check_range(entry)?;
        if u64::from(entry.size) > limit {
            return Err(invalid("ISO record exceeds read limit"));
        }
        self.source
            .seek(SeekFrom::Start(u64::from(entry.sector) * SECTOR))?;
        let mut data = vec![0u8; entry.size as usize];
        self.source.read_exact(&mut data)?;
        Ok(data)
    }

    pub fn root_entries(&mut self) -> io::Result<Vec<Entry>> {
        let data = self.contents(&self.root.clone(), MAX_DIRECTORY)?;
        let mut offset = 0;
        let mut result = Vec::new();
        while offset < data.len() {
            let len = usize::from(data[offset]);
            if len == 0 {
                offset = (offset / SECTOR as usize + 1) * SECTOR as usize;
                continue;
            }
            let end = offset + len;
            if end > data.len() || offset / SECTOR as usize != (end - 1) / SECTOR as usize {
                return Err(invalid("ISO directory record crosses boundary"));
            }
            let entry = record(&data[offset..end])?;
            self.check_range(&entry)?;
            if entry.name != "\0" && entry.name != "\u{1}" {
                result.push(entry);
            }
            offset = end;
        }
        Ok(result)
    }

    fn root_file_entry(&mut self, name: &str) -> io::Result<Entry> {
        // Names are matched as records, never used as output paths.
        let entries = self.root_entries()?;
        let mut matching = entries.iter().filter(|e| {
            !e.directory
                && e.name
                    .split(';')
                    .next()
                    .unwrap_or("")
                    .eq_ignore_ascii_case(name)
        });
        let entry = matching
            .next()
            .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "ISO root file missing"))?;
        if matching.next().is_some() {
            return Err(invalid("ambiguous ISO root filename"));
        }
        Ok(entry.clone())
    }

    pub fn read_root_file(&mut self, name: &str, limit: u64) -> io::Result<Vec<u8>> {
        let entry = self.root_file_entry(name)?;
        self.contents(&entry, limit)
    }

    pub fn open_root_file(&mut self, name: &str) -> io::Result<FileSlice<'_, R>> {
        let entry = self.root_file_entry(name)?;
        self.check_range(&entry)?;
        Ok(FileSlice {
            source: &mut self.source,
            start: u64::from(entry.sector) * SECTOR,
            length: u64::from(entry.size),
            position: 0,
        })
    }

    pub fn validate_godhand_usa(&mut self) -> io::Result<()> {
        let config = self.read_root_file("SYSTEM.CNF", 64 * 1024)?;
        let config =
            std::str::from_utf8(&config).map_err(|_| invalid("invalid SYSTEM.CNF encoding"))?;
        let boot = config.lines().find_map(|line| {
            let (key, value) = line.split_once('=')?;
            key.trim()
                .eq_ignore_ascii_case("BOOT2")
                .then(|| value.trim())
        });
        if !boot.is_some_and(|b| b.eq_ignore_ascii_case("cdrom0:\\SLUS_215.03;1")) {
            return Err(invalid("expected God Hand USA boot serial SLUS_215.03"));
        }
        if !self
            .root_entries()?
            .iter()
            .any(|e| !e.directory && e.name.eq_ignore_ascii_case("SLUS_215.03;1"))
        {
            return Err(invalid("retail boot executable missing"));
        }
        // Serial recognition is a format check, not proof of an authentic retail dump.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn num(value: u32) -> [u8; 8] {
        let mut result = [0; 8];
        result[..4].copy_from_slice(&value.to_le_bytes());
        result[4..].copy_from_slice(&value.to_be_bytes());
        result
    }
    fn entry(name: &[u8], sector: u32, size: u32, directory: bool) -> Vec<u8> {
        let len = (33 + name.len() + 1) & !1;
        let mut r = vec![0; len];
        r[0] = len as u8;
        r[2..10].copy_from_slice(&num(sector));
        r[10..18].copy_from_slice(&num(size));
        r[25] = if directory { 2 } else { 0 };
        r[32] = name.len() as u8;
        r[33..33 + name.len()].copy_from_slice(name);
        r
    }
    fn fixture(serial: &str) -> Vec<u8> {
        let mut image = vec![0; 24 * SECTOR as usize];
        let p = 16 * SECTOR as usize;
        image[p..p + 7].copy_from_slice(b"\x01CD001\x01");
        image[p + 128..p + 132].copy_from_slice(&[0, 8, 8, 0]);
        let root = entry(&[0], 20, 2048, true);
        image[p + 156..p + 156 + root.len()].copy_from_slice(&root);
        let config = format!("BOOT2 = cdrom0:\\{serial};1\r\n");
        let cnf = entry(b"SYSTEM.CNF;1", 21, config.len() as u32, false);
        let boot = entry(format!("{serial};1").as_bytes(), 22, 4, false);
        let dir = 20 * SECTOR as usize;
        image[dir..dir + cnf.len()].copy_from_slice(&cnf);
        image[dir + cnf.len()..dir + cnf.len() + boot.len()].copy_from_slice(&boot);
        let cfg = 21 * SECTOR as usize;
        image[cfg..cfg + config.len()].copy_from_slice(config.as_bytes());
        image
    }
    #[test]
    fn accepts_synthetic_serial_and_reads_without_extraction() {
        let mut disc = Disc::read(Cursor::new(fixture("SLUS_215.03"))).unwrap();
        disc.validate_godhand_usa().unwrap();
        assert_eq!(disc.root_entries().unwrap().len(), 2);
    }
    #[test]
    fn rejects_other_serial() {
        let mut disc = Disc::read(Cursor::new(fixture("FAKE_000.00"))).unwrap();
        assert!(disc.validate_godhand_usa().is_err());
    }
    #[test]
    fn rejects_truncated_and_wrong_signature() {
        assert!(Disc::read(Cursor::new(vec![0; 100])).is_err());
        let mut image = fixture("SLUS_215.03");
        image[16 * SECTOR as usize + 1] = b'X';
        assert!(Disc::read(Cursor::new(image)).is_err());
    }
    #[test]
    fn rejects_extent_past_eof() {
        let mut image = fixture("SLUS_215.03");
        let offset = 16 * SECTOR as usize + 156 + 2;
        image[offset..offset + 8].copy_from_slice(&num(u32::MAX));
        assert!(Disc::read(Cursor::new(image)).is_err());
    }
    #[test]
    fn rejects_mismatched_endian_and_oversized_reads() {
        assert!(both_u32(&[1, 0, 0, 0, 0, 0, 0, 2]).is_err());
        let mut disc = Disc::read(Cursor::new(fixture("SLUS_215.03"))).unwrap();
        assert!(disc.read_root_file("SYSTEM.CNF", 1).is_err());
        assert!(disc.read_root_file("../SYSTEM.CNF", 4096).is_err());
    }

    #[test]
    fn file_view_caps_reads_and_rejects_outside_seeks() {
        let mut image = fixture("SLUS_215.03");
        let start = 22 * SECTOR as usize;
        image[start..start + 4].copy_from_slice(b"ABCD");
        let mut disc = Disc::read(Cursor::new(image)).unwrap();
        let mut view = disc.open_root_file("SLUS_215.03").unwrap();
        let mut bytes = [0; 8];
        assert_eq!(view.read(&mut bytes).unwrap(), 4);
        assert_eq!(&bytes[..4], b"ABCD");
        assert_eq!(view.read(&mut bytes).unwrap(), 0);
        assert!(view.seek(SeekFrom::Start(5)).is_err());
        assert!(view.seek(SeekFrom::End(-5)).is_err());
        view.seek(SeekFrom::End(-1)).unwrap();
        assert_eq!(view.read(&mut bytes).unwrap(), 1);
        assert_eq!(bytes[0], b'D');
    }

    #[test]
    fn rejects_multi_extent_records() {
        let mut bytes = entry(b"TEST;1", 1, 4, false);
        bytes[25] |= 0x80;
        assert!(record(&bytes).is_err());
    }
}
