pub mod superiority_evaluator;
pub mod benchmark_ingest;

use crate::symbiosis::synthesis::{LogicSynthesizer, UniversalSynthesizer};
use anyhow::Result;
use rand::Rng;

/// EVOLUTION ENGINE (V-133.0)
/// Goal: Autonomously derive superior logic fragments through recursive genetic selection.
pub struct EvolutionEngine {
    entropy: f64, // Mutation rate [0.0 - 1.0]
}

#[derive(Clone, Debug)]
pub struct EvolvedLogic {
    pub payload: Vec<u8>,
    pub generation: u32,
    pub fitness: f64,
}

impl EvolutionEngine {
    pub fn new(entropy: f64) -> Self {
        Self { entropy }
    }

    /// V-134.0: Fitness-Weighted Crossover
    /// Parents with higher fitness contribute proportionally more logic material.
    /// weight_a = fitness_a / (fitness_a + fitness_b) * 200 (avoids 0-weight edge case)
    pub fn crossover_resonance(&self, parent_a: &[u8], parent_b: &[u8],
                               fitness_a: f64, fitness_b: f64) -> Vec<u8> {
        let total = fitness_a + fitness_b;
        let (wa, wb): (usize, usize) = if total < 1e-9 {
            (100, 100) // equal if both unscored
        } else {
            let wa = ((fitness_a / total) * 200.0).round() as usize;
            let wb = (200usize).saturating_sub(wa);
            (wa.max(10), wb.max(10)) // minimum 10 to ensure both parents contribute
        };
        let fragments: Vec<(&[u8], usize)> = vec![
            (parent_a, wa),
            (parent_b, wb),
        ];
        LogicSynthesizer::synthesize_generic(&fragments)
    }


    /// V-133.0: High-Velocity Mutation Strike
    /// Introduces bit-level entropy into a logic fragment to explore new logic space.
    pub fn mutate_resonance(&self, fragment: &mut [u8]) {
        let mut rng = rand::thread_rng();
        let num_mutations = (fragment.len() as f64 * self.entropy) as usize;

        for _ in 0..num_mutations {
            let byte_idx = rng.gen_range(0..fragment.len());
            let bit_idx = rng.gen_range(0..8);
            fragment[byte_idx] ^= 1 << bit_idx;
        }
    }

    /// V-134.0: Recursive Evolution Generation with Fitness Scoring.
    /// Crosses pairs of parents, mutates children, scores all candidates,
    /// and returns them sorted descending by fitness.
    pub fn evolve_generation(&self, parents: Vec<Vec<u8>>, parent_fitness: Vec<f64>, generation: u32) -> Result<Vec<EvolvedLogic>> {
        use crate::evolution::superiority_evaluator::SuperiorityEvaluator;
        let mut next_gen = Vec::new();

        println!("[ EVOLUTION ] Hatching Generation {} | Parents: {}", generation, parents.len());

        // Ensure fitness vec lines up with parents
        let default_fit = 0.5f64;
        let fit = |i: usize| -> f64 { parent_fitness.get(i).copied().unwrap_or(default_fit) };

        for i in 0..parents.len() {
            for j in (i + 1)..parents.len() {
                // 1. Fitness-weighted crossover
                let mut child_payload = self.crossover_resonance(
                    &parents[i], &parents[j],
                    fit(i), fit(j),
                );

                // 2. Mutation
                self.mutate_resonance(&mut child_payload);

                // 3. Score the child using first-principles fitness
                let fitness = SuperiorityEvaluator::score_logic_payload(&child_payload);

                // 4. Manifest scored child
                next_gen.push(EvolvedLogic {
                    payload: child_payload,
                    generation,
                    fitness,
                });
            }
        }

        // Fitness-ranked survival: top-100 only
        next_gen.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap_or(std::cmp::Ordering::Equal));
        next_gen.truncate(100);

        Ok(next_gen)
    }
}
