/*
--- Day 2: Rock Paper Scissors ---

The Elves begin to set up camp on the beach. To decide whose tent gets to be
closest to the snack storage, a giant Rock Paper Scissors tournament is already
in progress.

Rock Paper Scissors is a game between two players. Each game contains many
rounds; in each round, the players each simultaneously choose one of Rock,
Paper, or Scissors using a hand shape. Then, a winner for that round is
selected: Rock defeats Scissors, Scissors defeats Paper, and Paper defeats Rock.
If both players choose the same shape, the round instead ends in a draw.

Appreciative of your help yesterday, one Elf gives you an encrypted strategy
guide (your puzzle input) that they say will be sure to help you win. "The first
column is what your opponent is going to play: A for Rock, B for Paper, and C
for Scissors. The second column--" Suddenly, the Elf is called away to help with
someone's tent.

The second column, you reason, must be what you should play in response: X for
Rock, Y for Paper, and Z for Scissors. Winning every time would be suspicious,
so the responses must have been carefully chosen.

The winner of the whole tournament is the player with the highest score. Your
total score is the sum of your scores for each round. The score for a single
round is the score for the shape you selected (1 for Rock, 2 for Paper, and 3
for Scissors) plus the score for the outcome of the round (0 if you lost, 3 if
the round was a draw, and 6 if you won).

Since you can't be sure if the Elf is trying to help you or trick you, you
should calculate the score you would get if you were to follow the strategy
guide.

For example, suppose you were given the following strategy guide:

A Y
B X
C Z

This strategy guide predicts and recommends the following:

    In the first round, your opponent will choose Rock (A), and you should choose Paper (Y). This ends in a win for you with a score of 8 (2 because you chose Paper + 6 because you won).
    In the second round, your opponent will choose Paper (B), and you should choose Rock (X). This ends in a loss for you with a score of 1 (1 + 0).
    The third round is a draw with both players choosing Scissors, giving you a score of 3 + 3 = 6.

In this example, if you were to follow the strategy guide, you would get a total
score of 15 (8 + 1 + 6).

What would your total score be if everything goes exactly according to your
strategy guide?
*/

use anyhow::anyhow;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::multispace1,
    combinator::{all_consuming, value},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

#[derive(Debug, Clone, Copy)]
pub enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
pub enum Outcome {
    Win,
    Loss,
    Draw,
}

pub fn parse_input(raw: &str) -> anyhow::Result<Vec<(Shape, Shape)>> {
    match all_consuming(separated_list1(multispace1, moves_parser))(raw.trim()) {
        Ok((_, lines)) => Ok(lines),
        Err(e) => Err(anyhow!("could not parse input [{}]: {}", e, raw)),
    }
}
fn moves_parser(input: &str) -> IResult<&str, (Shape, Shape)> {
    let (input, (p1, p2)) = separated_pair(move_parser, multispace1, move_parser)(input)?;
    Ok((input, (p1, p2)))
}
fn move_parser(input: &str) -> IResult<&str, Shape> {
    alt((
        value(Shape::Rock, tag("A")),
        value(Shape::Paper, tag("B")),
        value(Shape::Scissors, tag("C")),
        value(Shape::Rock, tag("X")),
        value(Shape::Paper, tag("Y")),
        value(Shape::Scissors, tag("Z")),
    ))(input)
}

pub fn score(input: &[(Shape, Shape)]) -> i32 {
    input
        .iter()
        .map(|&(theirs, mine)| score_round(theirs, mine))
        .sum()
}

fn score_round(theirs: Shape, mine: Shape) -> i32 {
    let outcome = decide_outcome(theirs, mine);
    score_shape(mine) + score_outcome(outcome)
}

fn decide_outcome(theirs: Shape, mine: Shape) -> Outcome {
    match (theirs, mine) {
        (Shape::Rock, Shape::Rock)
        | (Shape::Paper, Shape::Paper)
        | (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
        (Shape::Rock, Shape::Paper)
        | (Shape::Paper, Shape::Scissors)
        | (Shape::Scissors, Shape::Rock) => Outcome::Win,
        (Shape::Rock, Shape::Scissors)
        | (Shape::Paper, Shape::Rock)
        | (Shape::Scissors, Shape::Paper) => Outcome::Loss,
    }
}

fn score_shape(shape: Shape) -> i32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn score_outcome(outcome: Outcome) -> i32 {
    match outcome {
        Outcome::Win => 6,
        Outcome::Loss => 0,
        Outcome::Draw => 3,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_test() -> anyhow::Result<()> {
        let input = r#"
            A Y
            B X
            C Z
        "#;
        let input = parse_input(input)?;
        assert_eq!(score(&input), 15);
        Ok(())
    }

    #[test]
    fn part1() -> anyhow::Result<()> {
        let input = std::fs::read_to_string("data/day02.input")?;
        let input = parse_input(&input)?;
        assert_eq!(score(&input), 71023);
        Ok(())
    }
}
