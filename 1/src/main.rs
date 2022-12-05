use std::str::FromStr;

fn main() {
	let input = parse(include_str!("input.txt"));
	println!("Part 1 result: {:?}", part1(&input));
	println!("Part 2 result: {:?}", part2(&input));
}

fn parse(input: &str) -> Vec<&str> {
	input.split_terminator("\n").collect()
}

fn calories_per_elf(input: &Vec<&str>) -> Vec<i32> {
	let mut elves: Vec<Vec<i32>> = vec![vec![]];
	let mut count = 0;
	for line in input {
		if line.to_owned() == "" {
			elves.push(vec![]);
			count += 1;
			continue;
		}
		elves[count].push(<i32 as FromStr>::from_str(line).unwrap());
	}
	elves.iter().map(|x| x.iter().sum::<i32>()).collect()
}

fn part1(input: &Vec<&str>) -> i32 {
	*calories_per_elf(input).iter().max().unwrap()
}

fn part2(input: &Vec<&str>) -> i32 {
	let mut calories = calories_per_elf(input);
	calories.sort_by(|a, b| b.cmp(a));
	calories.iter().take(3).sum::<i32>()
}

#[test]
fn test() {
	let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";
	assert_eq!(part1(&parse(input)), 24000);
	assert_eq!(part2(&parse(input)), 45000);
}
