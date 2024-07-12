use std::fs;
use crate::graph::ArgumentationFramework;
use crate::cli::Format;
use memchr::memchr;
use memmap2::Mmap;

pub fn get_input(file_path : &str, format : Format) -> ArgumentationFramework {
    match format {
        Format::Apx => reading_apx(file_path),
        //Format::Cnf => _reading_cnf(file_path),
        //Format::Cnf => _reading_cnf_f(file_path),
        //Format::Cnf => reading_cnf_perf(file_path),
        Format::Cnf => reading_cnf_perf2(file_path),
    }
}

pub fn _reading_cnf( file_path : &str) -> ArgumentationFramework {
    let contents = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
    let mut content_iter = contents.trim().split('\n');
    let first_line = content_iter.next().unwrap();
    let iter: Vec<&str> = first_line.split_ascii_whitespace().collect();
    let nb_arg = iter[2].parse::<usize>().unwrap();
    let mut af = ArgumentationFramework::new(nb_arg);
    for line in content_iter {
        if !line.is_empty() && !line.starts_with('#') {
            let (mut attacker,mut target) = _parse_cnfattack_line(line);
            attacker-=1;
            target-=1;
            af.add_attack(attacker, target);
        }
    }
    af
}
pub fn _reading_cnf_f( file_path : &str) -> ArgumentationFramework {
    let content = fs::read_to_string(file_path)
    .expect("Should have been able to read the file");
    let mut data = content.as_bytes();
    let Some(separator) = memchr(b' ', data) else {panic!("oups")};
    data = &data[separator+1..];
    let Some(separator) = memchr(b' ', data) else {panic!("oups")};
    data = &data[separator+1..];
    let end = memchr(b'\n', data).unwrap();
    let nb_arg = data[.. end].iter().take(12).fold(0, |acc, b| acc * 10 + (b & 0x0f) as u32) as usize;
    let mut af = ArgumentationFramework::new(nb_arg);
    data = &data[end + 1..];
    loop {
        unsafe {
            if *data.get_unchecked(0) == b'#' {
                let Some(end) = memchr(b'\n', data) else {break;};
                data = &data[end+1..];
            }
            else { break; }
        }
    }
    loop {
        let Some(separator) = memchr(b' ', data) else { break; };
        let Some(end) = memchr(b'\n', &data[separator..]) else { break; };
        let att = _bytes_to_int(&data[..separator]);
        let target = _bytes_to_int(&data[separator + 1..separator + end]);
        af.add_attack(att, target);
        data = &data[separator + end + 1..];
    }
    af
}
#[inline(always)]
fn _bytes_to_int(bytes: &[u8]) -> u32 {
    let mut acc = 0;
    for a in bytes {
        acc = acc*10  + (*a & 0x0f) as u32;
    }
    acc-1
}
pub fn _reading_cnf_perf( file_path : &str) -> ArgumentationFramework{
    let mmap: Mmap;
    let mut data;
    {
        let file = std::fs::File::open(file_path).unwrap();
        mmap = unsafe { Mmap::map(&file).unwrap() };
        data = &*mmap;
    }
    let Some(separator) = memchr(b' ', data) else {panic!("oups")};
    data = &data[separator+1..];
    let Some(separator) = memchr(b' ', data) else {panic!("oups")};
    data = &data[separator+1..];
    let end = memchr(b'\n', data).unwrap();
    let nb_arg = data[.. end].iter().take(12).fold(0, |acc, b| acc * 10 + (b & 0x0f) as u32) as usize;
    let mut af = ArgumentationFramework::new(nb_arg);
    data = &data[end + 1..];
    loop {
        unsafe {
            if *data.get_unchecked(0) == b'#' {
                let Some(end) = memchr(b'\n', data) else {break;};
                data = &data.get_unchecked(end+1..);
            }
            else { break; }
        }
    }
    loop {
        unsafe {
            let Some(separator) = memchr(b' ', data) else { break; };
            let mut att = 0;
            for a in data.get_unchecked(..separator).iter() { att = att*10  + (*a & 0x0f) as u32; }
            data = &data.get_unchecked(separator+1..);
            let Some(end) = memchr(b'\n', data) else { break; };
            let mut target = 0;
            for a in data.get_unchecked(..end).iter() { target = target*10  + (*a & 0x0f) as u32; }
            af.add_attack(att-1, target-1);
            data = &data.get_unchecked(end + 1..);
        }
    }
    af
}
pub fn reading_cnf_perf2( file_path : &str) -> ArgumentationFramework{
    let mmap: Mmap;
    let mut data;
    {
        let file = std::fs::File::open(file_path).unwrap();
        mmap = unsafe { Mmap::map(&file).unwrap() };
        data = &*mmap;
    }
    let Some(separator) = memchr(b' ', data) else {panic!("oups")};
    data = &data[separator+1..];
    let Some(separator) = memchr(b' ', data) else {panic!("oups")};
    data = &data[separator+1..];
    let end = memchr(b'\n', data).unwrap();
    let nb_arg = data[.. end].iter().take(12).fold(0, |acc, b| acc * 10 + (b & 0x0f) as u32) as usize;
    let mut af = ArgumentationFramework::new(nb_arg);
    data = &data[end + 1..];
    loop {
        unsafe {
            if *data.get_unchecked(0) == b'#' {
                let Some(end) = memchr(b'\n', data) else {break;};
                data = &data.get_unchecked(end+1..);
            }
            else { break; }
        }
    }
    let mut i = 0;
    'block: loop  {
        unsafe {
            if data.len() == i {
                break;
            }
            let mut att = 0;
            loop {
                if *data.get_unchecked(i) == b' ' {
                    i+=1;
                    break;
                }
                #[allow(unused_assignments)]
                if *data.get_unchecked(i) == b'\n' {
                    i+=1;
                    break 'block;
                }
                att = att*10 + (data.get_unchecked(i) & 0x0f) as u32;
                i += 1;
            }
            let mut target = 0;
            loop {
                if *data.get_unchecked(i) == b'\n' {
                    i+=1;
                    break;
                }
                target = target*10 + (data.get_unchecked(i) & 0x0f) as u32;
                i += 1;
            }
            af.add_attack(att-1, target-1);
        }
    }
    af
}

fn find_number_argument(file_path : &str) -> i32 {
    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let a = contents.trim().split('\n');
    let mut nb_arg = 0;
    for line in a {
        if line.starts_with("arg") { nb_arg +=1; }
        else { break; }
    }
    nb_arg
}
#[inline(always)]
fn _parse_cnfattack_line (line : &str) -> (u32,u32) {
    let mut a = line.split_ascii_whitespace();
    let att = a.next().unwrap().parse::<u32>().unwrap();
    let targ = a.next().unwrap().parse::<u32>().unwrap();
    (att,targ)
}

pub fn reading_apx( file_path : &str) -> ArgumentationFramework {
    let nb_arg = find_number_argument(file_path);
    let af = ArgumentationFramework::new(nb_arg as usize);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");
    let a = contents.trim().split('\n');

    for line in a {
        if !line.starts_with('#') && (!line.trim().eq("")) {
            //af.add
        }
    }
    
    af
}
