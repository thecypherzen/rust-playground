use regex::{Regex};
use serde::{Serialize};
use serde_wasm_bindgen::{to_value};
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

// Struct to hold analysis results
#[derive(Serialize, Debug)]
pub struct AnalysisResult {
	word_pos: HashMap<String, Vec<usize>>,
	word_freqs: HashMap<String, usize>,
	stats: HashMap<String, usize>,
}

// native rust analyser function
pub fn analyse_native(text: &str) -> AnalysisResult {
	let mut word_indices = HashMap::<String, Vec<usize>>::new();
	let mut word_frequencies: HashMap<String, usize> = HashMap::new();
	let mut statistics: HashMap<String, usize> = HashMap::new();

	// generate stats
	let reg_matches: Vec<_> = Regex::new(r"[A-Za-z_][A-Za-z0-9_.@-]*")
	.unwrap()
	.find_iter(text)
	.collect();

	for (i, wmatch) in reg_matches.iter().enumerate() {
		let string  = wmatch.as_str().to_lowercase();
		word_indices.entry(string.clone()).or_default().push(i);
		*word_frequencies.entry(string.clone()).or_insert(0) += 1;
	}
	statistics.insert("word_count".to_string(), reg_matches.len());
	statistics.insert("char_count".to_string(), text.chars().count());

	AnalysisResult {
		word_pos: word_indices,
		word_freqs: word_frequencies,
		stats: statistics,
	}
}

// wasm-bindgen wrapper function
#[wasm_bindgen]
pub fn analyse(text: &str) -> JsValue  {
	// get the analysis result
	let analysis_res = analyse_native(text);
	let json = serde_json::json!(analysis_res);
	to_value(&json).unwrap()
}