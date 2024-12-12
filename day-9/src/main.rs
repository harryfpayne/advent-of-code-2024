use std::time::Instant;

const INPUT: &str = include_str!("input.txt");

fn main() {
    let s = Instant::now();
    println!("{:?}", part_1(INPUT));
    println!("{:?}", part_2(INPUT));
    println!("Time: {:?}", s.elapsed());
}

fn part_1(input: &str) -> usize {
    let mut disk = parse(input);

    let mut f = 0usize;
    let mut b = disk.len() - 1;
    while f < b {
        if disk[f] != None {
            f += 1;
            continue
        }
        if disk[b] == None {
            b -= 1;
            continue
        }
        disk[f] = disk[b].clone();
        disk[b] = None
    }

    let mut checksum = 0;
    for (i, v) in disk.iter().enumerate() {
        if let Some(n) = v {
            checksum += n * i
        }
    }
    checksum
}


#[derive(Copy, Clone, Debug)]
struct File {
    id: usize,
    size: usize,
    disk_start_addr: usize,
    is_file: bool,
}
impl File {
    fn new(id: usize, size: usize, disk_start_addr: usize, is_file: bool) -> File {
        File{id, size, disk_start_addr, is_file }
    }
}

fn part_2(input: &str) -> usize {
    let mut disk = parse2(input);

    let mut current_addr = 0usize;
    while current_addr < disk.len()-1 {
        current_addr += 1; // Is ok because we always start with a file
        if disk[current_addr].is_file {
            continue;
        }

        let gap_size = disk[current_addr].size;

        let mut file_to_move = None;
        for i in (current_addr..disk.len()-1).rev() {
            let file = disk[i];
            if !file.is_file {
                continue
            }
            if file.size <= gap_size {
                file_to_move = Some(file.clone());
                break
            }
        }

        if file_to_move.is_none() {
            continue
        }
        let file_to_move = file_to_move.unwrap();

        for i in 0..file_to_move.size {
            disk[current_addr + i] = file_to_move;

            disk[file_to_move.disk_start_addr + i].id = 0;
            disk[file_to_move.disk_start_addr + i].is_file = false;
        }

        for i in file_to_move.size..gap_size {
            disk[current_addr + i].size = gap_size - file_to_move.size;
            disk[current_addr + i].disk_start_addr = current_addr + file_to_move.size;
        }
    }

    let mut checksum = 0;
    for (i, v) in disk.iter().enumerate() {
        if v.is_file {
            checksum += v.id * i
        }
    }
    checksum
}

fn parse2(input: &str) -> Vec<File> {
    let nums = input.trim().split("")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

    let mut vec = Vec::new();
    let mut c = 0;
    for (i, num) in nums.into_iter().enumerate() {
        let file_start_position = vec.len();
        for _ in 0..num {
            if i % 2 == 0 {
                vec.push(File::new(c, num, file_start_position, true));
            } else {
                vec.push(File::new(0, num, file_start_position, false));
            }
        }
        if i % 2 != 0 {
            c += 1
        }
    }


    vec
}

fn parse(input: &str) -> Vec<Option<usize>> {
    let nums = input.trim().split("")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

    let mut vec = Vec::new();
    let mut c = 0;
    for (i, num) in nums.into_iter().enumerate() {
        for _ in 0..num {
            if i % 2 == 0 {
                vec.push(Some(c));
            } else {
                vec.push(None);
            }
        }
        if i % 2 != 0 {
            c += 1
        }
    }


    vec
}
