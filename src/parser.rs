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

use crate::models::Model;

#[cfg(feature = "ja")]
use crate::models::JA_MODEL;

#[cfg(feature = "zh-hans")]
use crate::models::ZH_HANS_MODEL;

#[cfg(feature = "zh-hant")]
use crate::models::ZH_HANT_MODEL;

#[cfg(feature = "th")]
use crate::models::TH_MODEL;

/// A parser for BudouX that provides semantic chunking functionality.
pub struct Parser {
    /// BudouX model data
    model: Model,
    /// Base score for boundary determination
    base_score: i64,
}

impl Parser {
    /// Constructs a BudouX parser.
    ///
    /// # Arguments
    ///
    /// * `model` - A model containing scoring data for boundary determination.
    pub fn new(model: Model) -> Self {
        let s = model.values().flat_map(|group| group.values()).sum::<i64>();
        let base_score = -((s + 1) / 2);

        Parser { model, base_score }
    }

    /// Parses the input sentence and returns a list of semantic chunks.
    ///
    /// # Arguments
    ///
    /// * `sentence` - An input sentence.
    ///
    /// # Returns
    ///
    /// The retrieved chunks.
    pub fn parse<'a>(&self, sentence: &'a str) -> Vec<&'a str> {
        if sentence.is_empty() {
            return Vec::new();
        }

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

    /// Parses the input sentence and returns a list of boundaries.
    ///
    /// # Arguments
    ///
    /// * `sentence` - An input sentence.
    ///
    /// # Returns
    ///
    /// The list of boundary positions.
    pub fn parse_boundaries(&self, sentence: &str) -> Vec<usize> {
        let mut result = Vec::new();
        let chars = sentence.chars().collect::<Vec<_>>();

        for i in 1..chars.len() {
            let mut score = self.base_score;

            score += self.get_score("UW1", sentence.substring(i.saturating_sub(3), i.saturating_sub(2)));
            score += self.get_score("UW2", sentence.substring(i.saturating_sub(2), i.saturating_sub(1)));
            score += self.get_score("UW3", sentence.substring(i.saturating_sub(1), i));
            score += self.get_score("UW4", sentence.substring(i, i.saturating_add(1)));
            score += self.get_score("UW5", sentence.substring(i.saturating_add(1), i.saturating_add(2)));
            score += self.get_score("UW6", sentence.substring(i.saturating_add(2), i.saturating_add(3)));
            score += self.get_score("BW1", sentence.substring(i.saturating_sub(2), i));
            score += self.get_score("BW2", sentence.substring(i.saturating_sub(1), i.saturating_add(1)));
            score += self.get_score("BW3", sentence.substring(i, i.saturating_add(2)));
            score += self.get_score("TW1", sentence.substring(i.saturating_sub(3), i));
            score += self.get_score("TW2", sentence.substring(i.saturating_sub(2), i.saturating_add(1)));
            score += self.get_score("TW3", sentence.substring(i.saturating_sub(1), i.saturating_add(2)));
            score += self.get_score("TW4", sentence.substring(i, i.saturating_add(3)));

            if score > 0 {
                result.push(i);
            }
        }

        result
    }

    /// Gets the score for a given key and value from the model.
    ///
    /// # Arguments
    ///
    /// * `key` - The model feature group key.
    /// * `value` - The specific substring to score.
    ///
    /// # Returns
    ///
    /// The score value or 0 if not found.
    fn get_score(&self, key: &str, value: &str) -> i64 {
        self.model.get(key).and_then(|map| map.get(value)).copied().unwrap_or(0)
    }

    /// Loads a parser equipped with the default Japanese model.
    ///
    /// # Returns
    ///
    /// A parser with the default Japanese model.
    #[cfg(feature = "ja")]
    pub fn load_default_japanese_parser() -> Self {
        Self::new(JA_MODEL.to_owned())
    }

    /// Loads a parser equipped with the default Simplified Chinese model.
    ///
    /// # Returns
    ///
    /// A parser with the default Simplified Chinese model.
    #[cfg(feature = "zh-hans")]
    pub fn load_default_simplified_chinese_parser() -> Self {
        Self::new(ZH_HANS_MODEL.to_owned())
    }

    /// Loads a parser equipped with the default Traditional Chinese model.
    ///
    /// # Returns
    ///
    /// A parser with the default Traditional Chinese model.
    #[cfg(feature = "zh-hant")]
    pub fn load_default_traditional_chinese_parser() -> Self {
        Self::new(ZH_HANT_MODEL.to_owned())
    }

    /// Loads a parser equipped with the default Thai model.
    ///
    /// # Returns
    ///
    /// A parser with the default Thai model.
    #[cfg(feature = "th")]
    pub fn load_default_thai_parser() -> Self {
        Self::new(TH_MODEL.to_owned())
    }
}

trait Substring {
    fn substring(&self, start: usize, end: usize) -> &str;
}

impl Substring for str {
    #[inline]
    fn substring(&self, start: usize, end: usize) -> &str {
        let char_indices = self.char_indices().collect::<Vec<_>>();
        let start_byte = char_indices.get(start).map(|(byte, _)| *byte).unwrap_or(self.len());
        let end_byte = char_indices.get(end).map(|(byte, _)| *byte).unwrap_or(self.len());

        &self[start_byte..end_byte]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    const TEST_SENTENCE: &str = "abcdeabcd";

    #[test]
    fn should_separate_if_a_strong_feature_item_supports() {
        let mut model = HashMap::new();
        let mut uw4 = HashMap::new();
        uw4.insert("a".to_string(), 10000);
        model.insert("UW4".to_string(), uw4);

        let parser = Parser::new(model);
        let result = parser.parse(TEST_SENTENCE);

        assert_eq!(result, vec!["abcde", "abcd"]);
    }

    #[test]
    fn should_separate_even_if_it_makes_a_phrase_of_one_character() {
        let mut model = HashMap::new();
        let mut uw4 = HashMap::new();
        uw4.insert("b".to_string(), 10000);
        model.insert("UW4".to_string(), uw4);

        let parser = Parser::new(model);
        let result = parser.parse(TEST_SENTENCE);

        assert_eq!(result, vec!["a", "bcdea", "bcd"]);
    }

    #[test]
    fn should_return_an_empty_list_when_the_input_is_a_blank_string() {
        let model = HashMap::new();
        let parser = Parser::new(model);
        let result = parser.parse("");

        assert_eq!(result, Vec::<String>::new());
    }
}
