use std::fs;
use std::collections::HashMap;
#[derive(Clone, Debug)]
struct Cards{ cards:Vec<usize>, score:usize, pairs:HashMap<usize,usize>}

fn extract_cards(file_data: &String, problem_2: bool ) -> Vec<Cards> {
    let card_map: HashMap<char, usize> = if problem_2 {
            HashMap::from([
                ('A', 14),
                ('K', 13),
                ('Q', 12),
                ('J', 1), // Changed to 1 value for Joker
                ('T', 10),
                ('9', 9),
                ('8', 8),
                ('7', 7),
                ('6', 6),
                ('5', 5),
                ('4', 4),
                ('3', 3),
                ('2', 2),
            ])

        } else{
            HashMap::from([
                ('A', 14),
                ('K', 13),
                ('Q', 12),
                ('J', 11),
                ('T', 10),
                ('9', 9),
                ('8', 8),
                ('7', 7),
                ('6', 6),
                ('5', 5),
                ('4', 4),
                ('3', 3),
                ('2', 2),
            ])
    };
       
    let lines = file_data.lines()
                .map(String::from)
                .collect::<Vec<String>>();
    
    let mut final_cards = Vec::new();

    for line  in lines{
        let split = line.split(" ").map(String::from).collect::<Vec<String>>();
        
        let line_cards = split[0].chars()
                        .map(|face| card_map.get(&face).copied().unwrap() )
                        .collect::<Vec<usize>>();

        let mut occurences: HashMap<usize, usize> = HashMap::new();
        for card in &line_cards{
            *occurences.entry(*card).or_insert(0) += 1 as usize ; 
        }

        final_cards.push(Cards{cards: line_cards, score: split[1].parse().unwrap(), pairs: occurences});
    }

    return final_cards
    
    
}

fn score(mut cards: Vec<Cards>, problem_2: bool) -> usize {
    let mut ordered_cards = vec![cards.remove(0)];

    for card in cards {
        let card_max_kind = max_kind(&card, problem_2);
        let card_pair_sum = pair_sum(&card, problem_2);

        let mut inserted = false;
        for (index, ordered_card) in ordered_cards.iter().enumerate() {
            let ordered_max_kind = max_kind(ordered_card, problem_2);
            let ordered_pair_sum = pair_sum(ordered_card, problem_2);

            if card_max_kind > ordered_max_kind 
                || (card_max_kind == ordered_max_kind && card_pair_sum > ordered_pair_sum)
                || (card_max_kind == ordered_max_kind && card_pair_sum == ordered_pair_sum && is_higher_card(&card, ordered_card)) 
            {
                ordered_cards.insert(index, card.clone());
                inserted = true;
                break;
            }
        }

        if !inserted {
            ordered_cards.push(card);
        }
    }

    ordered_cards.iter().rev().enumerate().map(|(i, card)| (i + 1) * card.score).sum()
}

fn max_kind(card: &Cards, problem_2: bool) -> usize {
    if problem_2 {
        card.pairs
            .keys()
            .filter(|&&k| k > 1)
            .filter_map(|k| card.pairs.get(k))
            .max()
            .copied()
            .unwrap_or(0) + card.pairs.get(&1).copied().unwrap_or(0)
    } else {
        *card.pairs.values().max().unwrap()
    }
}

fn pair_sum(card: &Cards, problem_2: bool) -> usize {
    if problem_2 {
        if *card.pairs.get(&1).unwrap_or(&0) == 5 {
            5
        } else {
            card.pairs
                .keys()
                .filter(|&&k| k > 1)
                .filter_map(|k| card.pairs.get(k))
                .filter(|&&c| c > 1)
                .sum::<usize>()
                .max(1) + card.pairs.get(&1).copied().unwrap_or(0)
        }
    } else {
        card.pairs.values().filter(|&&c| c > 1).sum()
    }
}

fn is_higher_card(card: &Cards, ordered_card: &Cards) -> bool {
    for (&c1, &c2) in card.cards.iter().zip(ordered_card.cards.iter()) {
        if c1 > c2 {
            return true;
        } else if c1 < c2 {
            return false;
        }
    }
    false
}
fn main(){
    let file_data: String = fs::read_to_string("./puzzle_inputs/day_07.txt").expect("attempting to read file");
    
    let cards = extract_cards(&file_data, false);
    let answer_1 = score(cards, false);
    assert_eq!(answer_1, 248559379);
    println!("Score 1 {:?}", answer_1);

    // Problem 2
    let cards_2 = extract_cards(&file_data, true);
    let answer_2 = score(cards_2, true);
    assert_eq!(answer_2, 249631254);
    println!("Score 2 {:?}", answer_2);
}

