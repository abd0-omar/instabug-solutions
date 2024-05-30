fn main() {
    let mut t = String::new();
    std::io::stdin().read_line(&mut t).unwrap();
    let t = t.trim().parse::<i32>().unwrap();

    for _ in 0..t {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut line = line.split_whitespace();
        let ram = line.next().unwrap().parse::<i64>().unwrap();
        let cpu = line.next().unwrap().parse::<i64>().unwrap();
        let disk = line.next().unwrap().parse::<i64>().unwrap();

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut line = line.split_whitespace();
        let ram_max = line.next().unwrap().parse::<i64>().unwrap();
        let cpu_max = line.next().unwrap().parse::<i64>().unwrap();
        let disk_max = line.next().unwrap().parse::<i64>().unwrap();

        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut line = line.split_whitespace();
        let ram_price = line.next().unwrap().parse::<i64>().unwrap();
        let cpu_price = line.next().unwrap().parse::<i64>().unwrap();
        let disk_price = line.next().unwrap().parse::<i64>().unwrap();

        let mut budget = String::new();
        std::io::stdin().read_line(&mut budget).unwrap();
        let budget = budget.trim().parse::<i64>().unwrap();

        let ans = max_machines_possible(
            ram, cpu, disk, ram_max, cpu_max, disk_max, ram_price, cpu_price, disk_price, budget,
        );
        println!("{}", ans);
    }
}

fn max_machines_possible(
    ram: i64,
    cpu: i64,
    disk: i64,
    ram_max: i64,
    cpu_max: i64,
    disk_max: i64,
    ram_price: i64,
    cpu_price: i64,
    disk_price: i64,
    budget: i64,
) -> i64 {
    let mut left = 0;
    let mut right = 1_000_000_000;
    let mut ans = 0;
    while left <= right {
        let mid = left + (right - left) / 2;

        // if I can do 100 machines
        // then I can do 90 machines 80 machines and so one
        // so find the max possible
        if possible(
            ram, cpu, disk, ram_max, cpu_max, disk_max, ram_price, cpu_price, disk_price, budget,
            mid,
        ) {
            left = mid + 1;
            ans = mid;
        } else {
            right = mid - 1;
        }
    }
    ans
}

/*
2   5  3 -> req. to make one machine
11 14  6 -> max available size
2   2  5 -> prices
70 -> budget

let's say that we want to see if we could make 5 machines

ram eg.
5 * 2 - 11
-1 we don't need any more ram to buy
then we make it 0
*/

fn possible(
    ram: i64,
    cpu: i64,
    disk: i64,
    ram_max: i64,
    cpu_max: i64,
    disk_max: i64,
    ram_price: i64,
    cpu_price: i64,
    disk_price: i64,
    budget: i64,
    number_of_machines: i64,
) -> bool {
    let extra_ram_to_buy = 0.max(number_of_machines * ram - ram_max);
    let extra_cpu_to_buy = 0.max(number_of_machines * cpu - cpu_max);
    let extra_disk_to_buy = 0.max(number_of_machines * disk - disk_max);

    let mut budget_after_buying = budget;
    budget_after_buying -= extra_ram_to_buy * ram_price;
    budget_after_buying -= extra_cpu_to_buy * cpu_price;
    budget_after_buying -= extra_disk_to_buy * disk_price;
    if budget_after_buying < 0 {
        false
    } else {
        true
    }
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
        let disk_max = 6;
        let ram_price = 2;
        let cpu_price = 2;
        let disk_price = 5;
        let budget = 70;
        let expected_output = 5;

        let result = max_machines_possible(
            ram, cpu, disk, ram_max, cpu_max, disk_max, ram_price, cpu_price, disk_price, budget,
        );
        assert_eq!(result, expected_output);
    }
}
