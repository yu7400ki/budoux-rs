/*
Copyright 2025 Google LLC

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use std::collections::HashMap;

pub struct Parser {
    model: HashMap<String, HashMap<String, f64>>,
    base_score: f64,
}

impl Parser {
    pub fn new(model: HashMap<String, HashMap<String, f64>>) -> Self {
        let base_score = -0.5 * model.values().flat_map(|group| group.values()).sum::<f64>();

        Parser { model, base_score }
    }

    pub fn parse<'a>(&self, sentence: &'a str) -> Vec<&'a str> {
        let boundaries = self.parse_boundaries(sentence);
        let mut result = Vec::new();
        let mut start = 0;

        for &boundary in &boundaries {
            result.push(sentence.substring(start, boundary));
            start = boundary;
        }
        result.push(sentence.substring(start, sentence.len()));

        result
    }

    pub fn parse_boundaries(&self, sentence: &str) -> Vec<usize> {
        let mut result = Vec::new();
        let chars = sentence.chars().collect::<Vec<_>>();

        for i in 1..chars.len() {
            let mut score = self.base_score;

            score += self.get_score(
                "UW1",
                sentence.substring(i.saturating_sub(3), i.saturating_sub(2)),
            );
            score += self.get_score(
                "UW2",
                sentence.substring(i.saturating_sub(2), i.saturating_sub(1)),
            );
            score += self.get_score("UW3", sentence.substring(i.saturating_sub(1), i));
            score += self.get_score("UW4", sentence.substring(i, i.saturating_add(1)));
            score += self.get_score(
                "UW5",
                sentence.substring(i.saturating_add(1), i.saturating_add(2)),
            );
            score += self.get_score(
                "UW6",
                sentence.substring(i.saturating_add(2), i.saturating_add(3)),
            );
            score += self.get_score("BW1", sentence.substring(i.saturating_sub(2), i));
            score += self.get_score(
                "BW2",
                sentence.substring(i.saturating_sub(1), i.saturating_add(1)),
            );
            score += self.get_score("BW3", sentence.substring(i, i.saturating_add(2)));
            score += self.get_score("TW1", sentence.substring(i.saturating_sub(3), i));
            score += self.get_score(
                "TW2",
                sentence.substring(i.saturating_sub(2), i.saturating_add(1)),
            );
            score += self.get_score(
                "TW3",
                sentence.substring(i.saturating_sub(1), i.saturating_add(2)),
            );
            score += self.get_score("TW4", sentence.substring(i, i.saturating_add(3)));

            if score > 0.0 {
                result.push(i);
            }
        }

        result
    }

    fn get_score(&self, key: &str, value: &str) -> f64 {
        self.model
            .get(key)
            .and_then(|map| map.get(value))
            .copied()
            .unwrap_or(0.0)
    }
}

trait Substring {
    fn substring(&self, start: usize, end: usize) -> &str;
}

impl Substring for str {
    #[inline]
    fn substring(&self, start: usize, end: usize) -> &str {
        let (start, end) = if start > end {
            (end, start)
        } else {
            (start, end)
        };

        let char_indices = self.char_indices().collect::<Vec<_>>();
        let start_byte = char_indices
            .get(start)
            .map(|(byte, _)| *byte)
            .unwrap_or(self.len());
        let end_byte = char_indices
            .get(end)
            .map(|(byte, _)| *byte)
            .unwrap_or(self.len());

        &self[start_byte..end_byte]
    }
}
