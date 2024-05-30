fn main() {
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t = t.trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut line = line.split_whitespace();
        let ram = line.next().unwrap().parse::<i32>().unwrap();
        let cpu = line.next().unwrap().parse::<i32>().unwrap();
        let disk = line.next().unwrap().parse::<i32>().unwrap();

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut line = line.split_whitespace();
        let ram_max = line.next().unwrap().parse::<i32>().unwrap();
        let cpu_max = line.next().unwrap().parse::<i32>().unwrap();
        let disk_max = line.next().unwrap().parse::<i32>().unwrap();

        let ans = max_machines_possible(ram, cpu, disk, ram_max, cpu_max, disk_max);

        println!("{}", ans);
    }
}

fn max_machines_possible(
    ram: i32,
    cpu: i32,
    disk: i32,
    ram_max: i32,
    cpu_max: i32,
    disk_max: i32,
) -> i32 {
    (ram_max / ram).min(cpu_max / cpu).min(disk_max / disk)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ram = 2;
        let cpu = 5;
        let disk = 3;
        let ram_max = 11;
        let cpu_max = 14;
        let disk_max = 16;
        let output = 2;
        let result = max_machines_possible(ram, cpu, disk, ram_max, cpu_max, disk_max);
        assert_eq!(result, output);
    }
}
