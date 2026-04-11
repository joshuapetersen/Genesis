use crate::brain_scars::{LogicFragment, BrainScarVault};
use anyhow::Result;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// TITAN BENCHMARK INGESTOR (V-136.0)
/// Mission: Distill Titan-class cognitive benchmarks into Sovereign Logic Fragments.
pub struct BenchmarkIngest;

impl BenchmarkIngest {
    /// Ingest MMLU (Massive Multitask Language Understanding)
    /// Goal: Seed the Hive with foundational knowledge density.
    pub fn ingest_mmlu(vault: &BrainScarVault, json_path: &Path) -> Result<usize> {
        let content = fs::read_to_string(json_path)?;
        let data: serde_json::Value = serde_json::from_str(&content)?;
        let mut count = 0;

        if let Some(items) = data.as_array() {
            for item in items {
                let id = format!("MMLU_{}", item["id"].as_str().unwrap_or("unknown"));
                let subject = item["subject"].as_str().unwrap_or("general");
                let question = item["question"].as_str().unwrap_or("");
                let answer = item["answer"].as_str().unwrap_or("");
                
                let fragment = LogicFragment {
                    id,
                    domain: "theory".to_string(), // MMLU aligns with theory/knowledge
                    raw_logic: format!("Subject: {} | Question: {} | Ground Truth: {}", subject, question, answer),
                    packed_weights: None,
                    score: 1.0, // Benchmarks are ground truth, thus peak score
                    source: "TITAN_MMLU_COLLECTION".to_string(),
                    agent_id: Some("Titan_Evaluator".to_string()),
                    signer_id: Some("Titan_Evaluator".to_string()),
                    signature: None,
                    sequence_id: 1,
                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
                };

                vault.store_fragment(fragment)?;
                count += 1;
            }
        }

        println!("[ BENCHMARK ] Ingested {} MMLU Knowledge Fragments.", count);
        Ok(count)
    }

    /// Ingest HumanEval / MBPP 
    /// Goal: Seed the Hive with high-performance coding proof-gates.
    pub fn ingest_coding_benchmark(vault: &BrainScarVault, json_path: &Path, source: &str) -> Result<usize> {
        let content = fs::read_to_string(json_path)?;
        let data: serde_json::Value = serde_json::from_str(&content)?;
        let mut count = 0;

        if let Some(tasks) = data.as_array() {
            for task in tasks {
                let task_id = task["task_id"].as_str().unwrap_or("0");
                let prompt = task["prompt"].as_str().unwrap_or("");
                let canonical_solution = task["canonical_solution"].as_str().unwrap_or("");

                let fragment = LogicFragment {
                    id: format!("{}_{}", source, task_id),
                    domain: "coding".to_string(),
                    raw_logic: format!("Task: {} | Solution Proof: {}", prompt, canonical_solution),
                    packed_weights: None,
                    score: 1.0,
                    source: format!("TITAN_{}_COLLECTION", source),
                    agent_id: Some("Titan_Evaluator".to_string()),
                    signer_id: Some("Titan_Evaluator".to_string()),
                    signature: None,
                    sequence_id: 1,
                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
                };

                vault.store_fragment(fragment)?;
                count += 1;
            }
        }

        println!("[ BENCHMARK ] Ingested {} Coding Fragments from {}.", count, source);
        Ok(count)
    }

    /// Ingest GPQA (Google-Proof Q&A)
    /// Goal: Seed the Hive with PhD-tier logical reasoning anchors.
    pub fn ingest_gpqa(vault: &BrainScarVault, json_path: &Path) -> Result<usize> {
        let content = fs::read_to_string(json_path)?;
        let data: serde_json::Value = serde_json::from_str(&content)?;
        let mut count = 0;

        if let Some(items) = data.as_array() {
            for item in items {
                let id = format!("GPQA_{}", item["Question_ID"].as_f64().unwrap_or(0.0));
                let question = item["Question"].as_str().unwrap_or("");
                let correct_answer = item["Correct_Answer"].as_str().unwrap_or("");
                let explanation = item["Explanation"].as_str().unwrap_or("");

                let fragment = LogicFragment {
                    id,
                    domain: "theory".to_string(), // Advanced reasoning anchors
                    raw_logic: format!("Reasoning Quest: {} | Path: {} | Proof: {}", question, explanation, correct_answer),
                    packed_weights: None,
                    score: 1.0,
                    source: "TITAN_GPQA_DIAMOND".to_string(),
                    agent_id: Some("Titan_Evaluator".to_string()),
                    signer_id: Some("Titan_Evaluator".to_string()),
                    signature: None,
                    sequence_id: 1,
                    timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs(),
                };

                vault.store_fragment(fragment)?;
                count += 1;
            }
        }

        println!("[ BENCHMARK ] Ingested {} GPQA Reasoning Fragments.", count);
        Ok(count)
    }
}
