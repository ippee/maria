//! A module related to Markov chain and its model generation.

use rand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
/// Markov model structure
pub struct MarkovModel<T> {
    elements: Vec<T>,
    cm_dist: Vec<Vec<f32>>, // cumulative distribution of transition probabilities
    pre_index: usize,
}

impl<T> MarkovModel<T>
where
    T: Clone,
    T: Eq,
    T: Ord,
    T: PartialOrd,
    T: PartialEq,
{
    /// Creates a new instance of [`MarkovModel`].
    fn new(elements: Vec<T>, cm_dist: Vec<Vec<f32>>, pre_index: usize) -> MarkovModel<T> {
        MarkovModel {
            elements: elements,
            cm_dist: cm_dist,
            pre_index: pre_index,
        }
    }

    /// Builds a new model from [`Vec<T>`].
    pub fn from(elements: Vec<T>) -> MarkovModel<T> {
        let mut non_dup_elements = elements.clone();
        non_dup_elements.sort();
        non_dup_elements.dedup();

        let elements_len = non_dup_elements.len();

        let mut state_freq = vec![vec![0; elements_len]; elements_len];
        let mut pre_index: Option<usize> = None;
        for token in elements {
            let cur_index = non_dup_elements
                .iter()
                .position(|t| token == *t)
                .expect("There is no token that should exist.");
            if let Some(i) = pre_index {
                state_freq[i][cur_index] += 1;
            }
            pre_index = Some(cur_index);
        }

        let mut cm_dist = vec![vec![0.0; elements_len]; elements_len];
        for (i, vector) in state_freq.iter().enumerate() {
            let row_sum = vector.iter().fold(0, |acc, cur| acc + cur);
            let mut cumulative_p = 0.0;
            for (j, count) in vector.iter().enumerate() {
                if row_sum != 0 {
                    cumulative_p = cumulative_p + (*count as f32 / row_sum as f32);
                    cm_dist[i][j] = cumulative_p;
                }
            }
        }

        MarkovModel::new(non_dup_elements, cm_dist, elements_len)
    }

    /// Returns the next possible element.
    ///
    /// The first element will be determined randomly, and the next one will be chosen
    /// by its state space.
    ///
    /// If you want to reset the chain of elements, use [`initialize()`](#method.initialize) methods.
    pub fn next(&mut self) -> &T {
        let mut rng = rand::thread_rng();

        let row_index = {
            let mut i;
            loop {
                if self.pre_index != self.elements.len() {
                    i = self.pre_index;
                } else {
                    i = rng.gen::<usize>() % self.elements.len()
                }
                let row_sum = self.cm_dist[i].iter().fold(0.0, |acc, cur| acc + cur);
                if row_sum == 0.0 {
                    self.initialize();
                } else {
                    break;
                }
            }
            i
        };

        let f = rng.gen::<f32>();
        let cur_index: usize = {
            let mut res = self.cm_dist[row_index].len() - 1;
            for (i, p) in self.cm_dist[row_index].iter().enumerate() {
                if f <= *p {
                    res = i;
                    break;
                }
            }
            res
        };

        self.pre_index = cur_index;
        self.elements
            .get(cur_index)
            .expect("There is no token that should exist.")
    }

    /// Resets the information of the element generated by the previous
    /// [`next()`](#method.next) method.
    pub fn initialize(&mut self) {
        self.pre_index = self.elements.len();
    }
}

#[cfg(test)]
mod markov_test {
    use crate::markov::MarkovModel;

    #[test]
    fn make_markov_model() {
        let actual = MarkovModel::from(vec!["すもも", "も", "もも", "も", "もも", "の", "うち"]);

        let expected = MarkovModel {
            elements: vec!["うち", "すもも", "の", "も", "もも"],
            wa_table: vec![
                vec![
                    [(4, 0), (0, 0)],
                    [(3, 0), (0, 0)],
                    [(2, 0), (0, 0)],
                    [(1, 0), (0, 0)],
                    [(0, 0), (0, 0)],
                ],
                vec![
                    [(4, 0), (3, 1)],
                    [(2, 0), (3, 1)],
                    [(1, 0), (3, 1)],
                    [(0, 0), (3, 1)],
                    [(3, 1), (0, 0)],
                ],
                vec![
                    [(4, 0), (0, 1)],
                    [(3, 0), (0, 1)],
                    [(2, 0), (0, 1)],
                    [(1, 0), (0, 1)],
                    [(0, 1), (0, 0)],
                ],
                vec![
                    [(3, 0), (4, 4)],
                    [(2, 0), (4, 4)],
                    [(1, 0), (4, 4)],
                    [(0, 0), (4, 4)],
                    [(4, 4), (0, 0)],
                ],
                vec![
                    [(4, 0), (3, 4)],
                    [(1, 0), (3, 4)],
                    [(3, 2), (2, 2)],
                    [(0, 0), (2, 4)],
                    [(2, 4), (0, 0)],
                ],
            ],
            pre_index: 5,
        };

        assert_eq!(actual, expected)
    }
}
