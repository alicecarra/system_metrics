use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Clone, Default)]
pub struct Memory {
    pub total: u64,
    pub free: u64,
    pub buffers: u64,
    pub cached: u64,
    pub used: u64,
    pub shared: u64,
}

pub fn get_memory() -> Result<Memory, io::Error> {
    let info_file = File::open("/proc/meminfo")?;
    let file_buffer = BufReader::new(info_file);
    let mut lines = file_buffer.lines();

    let mut memory = Memory::default();

    while let Some(line) = lines.next() {
        let line = line?;
        let line_split = &mut line.split(":");

        let key = line_split.next();
        let value = line_split
            .next()
            .map(|value| value.split_whitespace().next())
            .flatten();

        // println!("{key:?}: {value:?}");

        match key {
            Some("MemTotal") => memory.total = value.unwrap().parse::<u64>().unwrap(),
            Some("MemFree") => memory.free = value.unwrap().parse::<u64>().unwrap(),
            Some("Buffers") => memory.buffers = value.unwrap().parse::<u64>().unwrap(),
            Some("Cached") => memory.cached = value.unwrap().parse::<u64>().unwrap(),
            Some("Shmem") => memory.shared = value.unwrap().parse::<u64>().unwrap(),
            _ => continue,
        }
    }

    memory.used = memory.total - (memory.free + memory.buffers + memory.cached);

    Ok(memory)
}
