/*
--- Day 3: Rucksack Reorganization ---

One Elf has the important job of loading all of the rucksacks with supplies for
the jungle journey. Unfortunately, that Elf didn't quite follow the packing
instructions, and so a few items now need to be rearranged.

Each rucksack has two large compartments. All items of a given type are meant to
go into exactly one of the two compartments. The Elf that did the packing failed
to follow this rule for exactly one item type per rucksack.

The Elves have made a list of all of the items currently in each rucksack (your
puzzle input), but they need your help finding the errors. Every item type is
identified by a single lowercase or uppercase letter (that is, a and A refer to
different types of items).

The list of items for each rucksack is given as characters all on a single line.
A given rucksack always has the same number of items in each of its two
compartments, so the first half of the characters represent items in the first
compartment, while the second half of the characters represent items in the
second compartment.

For example, suppose you have the following list of contents from six rucksacks:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

    The first rucksack contains the items vJrwpWtwJgWrhcsFMMfFFhFp, which means its first compartment contains the items vJrwpWtwJgWr, while the second compartment contains the items hcsFMMfFFhFp. The only item type that appears in both compartments is lowercase p.
    The second rucksack's compartments contain jqHRNqRjqzjGDLGL and rsFMfFZSrLrFZsSL. The only item type that appears in both compartments is uppercase L.
    The third rucksack's compartments contain PmmdzqPrV and vPwwTWBwg; the only common item type is uppercase P.
    The fourth rucksack's compartments only share item type v.
    The fifth rucksack's compartments only share item type t.
    The sixth rucksack's compartments only share item type s.

To help prioritize item rearrangement, every item type can be converted to a
priority:

    Lowercase item types a through z have priorities 1 through 26.
    Uppercase item types A through Z have priorities 27 through 52.

In the above example, the priority of the item type that appears in both
compartments of each rucksack is 16 (p), 38 (L), 42 (P), 22 (v), 20 (t), and 19
(s); the sum of these is 157.

Find the item type that appears in both compartments of each rucksack. What is
the sum of the priorities of those item types?
*/

/*
As you finish identifying the misplaced items, the Elves come to you with
another issue.

For safety, the Elves are divided into groups of three. Every Elf carries a
badge that identifies their group. For efficiency, within each group of three
Elves, the badge is the only item type carried by all three Elves. That is, if a
group's badge is item type B, then all three Elves will have item type B
somewhere in their rucksack, and at most two of the Elves will be carrying any
other item type.

The problem is that someone forgot to put this year's updated authenticity
sticker on the badges. All of the badges need to be pulled out of the rucksacks
so the new authenticity stickers can be attached.

Additionally, nobody wrote down which item type corresponds to each group's
badges. The only way to tell which item type is the right one is by finding the
one item type that is common between all three Elves in each group.

Every set of three lines in your list corresponds to a single group, but each
group can have a different badge item type. So, in the above example, the first
group's rucksacks are the first three lines:

vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg

And the second group's rucksacks are the next three lines:

wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw

In the first group, the only item type that appears in all three rucksacks is
lowercase r; this must be their badges. In the second group, their badge item
type must be Z.

Priorities for these items must still be found to organize the sticker
attachment efforts: here, they are 18 (r) for the first group and 52 (Z) for the
second group. The sum of these is 70.
*/

use std::collections::BTreeSet;

use anyhow::anyhow;
use nom::{
    character::complete::{alpha1, multispace1},
    combinator::all_consuming,
    multi::separated_list1,
    IResult,
};

#[derive(Debug)]
pub struct Rucksack(String);
pub fn parse_input(raw: &str) -> anyhow::Result<Vec<Rucksack>> {
    match all_consuming(separated_list1(multispace1, rucksack_parser))(raw.trim()) {
        Ok((_, lines)) => Ok(lines),
        Err(e) => Err(anyhow!("could not parse input [{}]: {}", e, raw)),
    }
}
fn rucksack_parser(input: &str) -> IResult<&str, Rucksack> {
    let (input, v) = alpha1(input)?;
    Ok((input, Rucksack(v.to_owned())))
}

pub fn solve1(rucksacks: &[Rucksack]) -> i32 {
    rucksacks
        .iter()
        .map(|r| {
            let (c1, c2) = r.0.as_bytes().split_at(r.0.len() / 2);
            let c1: BTreeSet<u8> = c1.iter().copied().collect();
            let c2: BTreeSet<u8> = c2.iter().copied().collect();
            let common: Vec<u8> = c1.intersection(&c2).copied().collect();
            assert_eq!(
                common.len(),
                1,
                "{} has the wrong number of common items",
                r.0
            );
            priority(common[0]) as i32
        })
        .sum()
}

pub fn solve2(rucksacks: &[Rucksack]) -> i32 {
    rucksacks
        .chunks_exact(3)
        .map(|group| {
            let mut iter = group.iter();
            let mut common: BTreeSet<u8> =
                iter.next().unwrap().0.as_bytes().iter().copied().collect();
            for r in group {
                let items = r.0.as_bytes().iter().copied().collect();
                common = common.intersection(&items).copied().collect();
            }
            assert_eq!(
                common.len(),
                1,
                "{:?} has the wrong number of badges",
                group
            );
            let badge = common.iter().next().unwrap();
            priority(*badge) as i32
        })
        .sum()
}

fn priority(item: u8) -> u8 {
    match item {
        b'a'..=b'z' => item - b'a' + 1,
        b'A'..=b'Z' => item - b'A' + 27,
        _ => unreachable!("invalid item: {}", item),
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_test() -> anyhow::Result<()> {
        let input = r#"
            vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw
        "#;
        let input = parse_input(input)?;
        assert_eq!(solve1(&input), 157);
        Ok(())
    }

    #[test]
    fn part1() -> anyhow::Result<()> {
        let input = std::fs::read_to_string("data/day03.input")?;
        let input = parse_input(&input)?;
        assert_eq!(solve1(&input), 7428);
        Ok(())
    }

    #[test]
    fn part2() -> anyhow::Result<()> {
        let input = std::fs::read_to_string("data/day03.input")?;
        let input = parse_input(&input)?;
        assert_eq!(solve2(&input), 2650);
        Ok(())
    }
}
