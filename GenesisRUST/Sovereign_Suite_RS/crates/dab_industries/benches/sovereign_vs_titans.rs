/// SOVEREIGN VS TITANS — EVOLUTION_7 COMPREHENSIVE BENCHMARK
///
/// Head-to-head: Sovereign Genesis stack vs equivalent brute-force and ML-standard operations.
/// Every group tests our approach vs what a typical AI/ML system would use.
///
/// Groups:
///  1. KV-Cache: hit vs miss vs no-cache
///  2. Query routing: our O(1) tier classifier vs brute-force regex vs if-else cascade
///  3. String similarity: our phonetic skeleton vs Levenshtein vs exact contains()
///  4. Memory recall: our φ-decay × holographic vs brute-force linear scan
///  5. Deliberation: sequential 11-observer vs Rayon parallel vs single-observer
///  6. Math primitives: φ-table O(1) vs powi() vs linear approximation
///  7. Hypervector: holographic expansion vs naive dot-product similarity
///  8. Throughput flood: how many sovereign queries/sec at each tier

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId, black_box, Throughput};
use dab_industries::{
    phi::{phi_density_score, PHI, PHI_5},
    scheduler::query_depth_from_density,
    Bar, LyricPhase, DABIndustries,
};
use kv_cache_turbo::TurboQuantCache;
use sovereign_math::SovereignMath;
use theory_lab::{TheoryLab, TruthPillars};
use sovereign_constants::RECOVERY_DENSITY_THRESHOLD;
use rayon::prelude::*;

// ─── HELPER STRUCTURES ───────────────────────────────────────────────────────

/// Naive brute-force phonetic: strip to lowercase, check contains() on stem.
fn naive_exact_match(query: &str, keyword: &str) -> bool {
    query.to_lowercase().contains(&keyword.to_lowercase())
}

/// Levenshtein edit distance — O(m×n) — what most NLP pipelines use for fuzzy match.
fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let (m, n) = (a.len(), b.len());
    let mut dp = vec![vec![0usize; n + 1]; m + 1];
    for i in 0..=m { dp[i][0] = i; }
    for j in 0..=n { dp[0][j] = j; }
    for i in 1..=m {
        for j in 1..=n {
            dp[i][j] = if a[i-1] == b[j-1] { dp[i-1][j-1] }
                else { 1 + dp[i-1][j].min(dp[i][j-1]).min(dp[i-1][j-1]) };
        }
    }
    dp[m][n]
}

/// Phonetic skeleton — our approach: strip vowels, compare consonant frames.
fn consonant_skeleton(s: &str) -> String {
    s.chars().filter(|c| !"aeiou ".contains(*c)).collect::<String>().to_lowercase()
}

fn sovereign_phonetic_score(a: &str, b: &str) -> f64 {
    let sa = consonant_skeleton(a);
    let sb = consonant_skeleton(b);
    if sa.is_empty() || sb.is_empty() { return 0.0; }
    let d = levenshtein(&sa, &sb);
    let max = sa.len().max(sb.len());
    1.0 - (d as f64 / max as f64)
}

/// Naive dot-product similarity on f64 vectors — what most ML embeddings use.
fn naive_dot_product_similarity(a: &[f64], b: &[f64]) -> f64 {
    let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let na: f64  = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let nb: f64  = b.iter().map(|x| x * x).sum::<f64>().sqrt();
    if na == 0.0 || nb == 0.0 { 0.0 } else { dot / (na * nb) }
}

/// Linear powi() phi density — what we used BEFORE Evolution_5.
fn phi_density_old_powi(density: usize) -> f64 {
    if density == 0 { return 0.0; }
    let phi: f64 = PHI;
    phi.powi(density as i32) / phi.powi(20) // normalize to [0,1] range
}

/// Sequential 11-observer deliberation (brute force, no Rayon).
fn sequential_deliberation(theory_lab: &TheoryLab, anomaly_crate: &str) -> (f64, bool) {
    const FIB: [usize; 11] = [1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
    const GOLDEN_ANGLE: f64 = 137.50776405003785;
    let mut total = 0.0;
    let mut agrees = 0usize;
    for (i, &obs) in FIB.iter().enumerate() {
        let phase = ((i + 1) as f64 * GOLDEN_ANGLE) % 360.0;
        let p = TruthPillars {
            who:            format!("HIVE_{:03}_PHI{:.1}", obs, phase),
            what:           format!("DATA_AUDIT:{}", anomaly_crate),
            where_context:  anomaly_crate.to_string(),
            when_frequency: "1.092777 Hz".to_string(),
            why_intent:     "QUANTUM_CONSENSUS".to_string(),
            how_method:     "SPECTRAL_PROJECTION".to_string(),
            evolutionary:   [
                "0.0".to_string(), format!("{:03}", obs),
                "DEMO".to_string(), format!("{:.2}", phase),
                "BENCH".to_string(),
            ],
        };
        let d = theory_lab.weigh_truth(&p);
        total += d;
        if d > RECOVERY_DENSITY_THRESHOLD { agrees += 1; }
    }
    (total / 11.0, agrees >= 7)
}

/// Rayon parallel 11-observer deliberation — our Evolution_7 approach.
fn parallel_deliberation(theory_lab: &TheoryLab, anomaly_crate: &str) -> (f64, bool) {
    const FIB: [usize; 11] = [1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144];
    const GOLDEN_ANGLE: f64 = 137.50776405003785;
    let results: Vec<(f64, bool)> = FIB.par_iter()
        .enumerate()
        .map(|(i, &obs)| {
            let phase = ((i + 1) as f64 * GOLDEN_ANGLE) % 360.0;
            let p = TruthPillars {
                who:            format!("HIVE_{:03}_PHI{:.1}", obs, phase),
                what:           format!("DATA_AUDIT:{}", anomaly_crate),
                where_context:  anomaly_crate.to_string(),
                when_frequency: "1.092777 Hz".to_string(),
                why_intent:     "QUANTUM_CONSENSUS".to_string(),
                how_method:     "SPECTRAL_PROJECTION".to_string(),
                evolutionary:   [
                    "0.0".to_string(), format!("{:03}", obs),
                    "DEMO".to_string(), format!("{:.2}", phase),
                    "BENCH".to_string(),
                ],
            };
            let d = theory_lab.weigh_truth(&p);
            (d, d > RECOVERY_DENSITY_THRESHOLD)
        })
        .collect();
    let total: f64 = results.iter().map(|(d, _)| d).sum::<f64>() / 11.0;
    let agrees = results.iter().filter(|(_, a)| *a).count();
    (total, agrees >= 7)
}

/// Single-observer deliberation — what a naive AI agent would do.
fn single_observer_deliberation(theory_lab: &TheoryLab, anomaly_crate: &str) -> (f64, bool) {
    let p = TruthPillars {
        who:            "SINGLE_OBSERVER".to_string(),
        what:           format!("DATA_AUDIT:{}", anomaly_crate),
        where_context:  anomaly_crate.to_string(),
        when_frequency: "1.0 Hz".to_string(),
        why_intent:     "LINEAR_CHECK".to_string(),
        how_method:     "DIRECT".to_string(),
                evolutionary:   std::array::from_fn(|_| "0".to_string()),
    };
    let d = theory_lab.weigh_truth(&p);
    (d, d > RECOVERY_DENSITY_THRESHOLD)
}

// ─── BENCHMARKS ──────────────────────────────────────────────────────────────

fn bench_kv_cache(c: &mut Criterion) {
    let mut group = c.benchmark_group("KV_Cache");
    let queries = &[
        "what is the pulse frequency",
        "cadence and beat structure in bars",
        "SOVEREIGN_PURITY_STATUS",
        "how many nodes in the hive",
    ];

    group.bench_function("cache_miss_then_hit", |b| {
        b.iter(|| {
            let mut cache = TurboQuantCache::new();
            // Miss
            let _ = cache.get(black_box("sovereign_query"));
            // Store
            cache.insert(black_box("sovereign_query"), "SOVEREIGN_RESPONSE: density=8".to_string(), 0.95);
            // Hit
            let _ = cache.get(black_box("sovereign_query"));
        });
    });

    group.bench_function("cache_hit_only", |b| {
        let mut cache = TurboQuantCache::new();
        for q in queries {
            cache.insert(q, format!("response_{}", q.len()), 0.8);
        }
        b.iter(|| {
            for q in queries {
                black_box(cache.get(black_box(q)));
            }
        });
    });

    group.bench_function("no_cache_vault_search", |b| {
        // Simulate what handle_inquiry does WITHOUT cache: percussion density check
        let dab = DABIndustries::new();
        b.iter(|| {
            for q in queries {
                let density = dab.protocols.percussion_density(black_box(q));
                black_box(query_depth_from_density(density));
            }
        });
    });

    group.finish();
}

fn bench_query_routing(c: &mut Criterion) {
    let mut group = c.benchmark_group("Query_Routing");

    let queries = vec![
        "what",
        "tell me about the bars and cadence",
        "deep analysis of phonetic percussion density in sovereign bars with staccato beat",
        "SOVEREIGN_DEEP: critical percussion density analysis with DAB protocol validation",
    ];

    group.bench_function("sovereign_o1_percussion_table", |b| {
        let dab = DABIndustries::new();
        b.iter(|| {
            for q in &queries {
                let d = dab.protocols.percussion_density(black_box(q));
                black_box(query_depth_from_density(d));
            }
        });
    });

    group.bench_function("brute_force_if_else_cascade", |b| {
        b.iter(|| {
            for q in &queries {
                let q = black_box(q);
                // What most systems do: check length, then keyword lists with contains()
                let tier = if q.len() < 5 {
                    "shallow"
                } else if q.contains("deep") || q.contains("analysis") || q.contains("critical") {
                    "deep"
                } else if q.contains("sovereign") || q.contains("percussion") {
                    "sovereign"
                } else {
                    "standard"
                };
                black_box(tier);
            }
        });
    });

    group.bench_function("regex_style_linear_scan", |b| {
        let keywords_shallow  = ["what", "who", "when"];
        let keywords_deep     = ["deep", "analysis", "critical", "phonetic"];
        let keywords_sovereign= ["sovereign", "percussion", "cadence", "density"];
        b.iter(|| {
            for q in &queries {
                let q_low = q.to_lowercase();
                let tier = if keywords_sovereign.iter().any(|k| q_low.contains(k)) {
                    "sovereign"
                } else if keywords_deep.iter().any(|k| q_low.contains(k)) {
                    "deep"
                } else if keywords_shallow.iter().any(|k| q_low.contains(k)) {
                    "shallow"
                } else {
                    "standard"
                };
                black_box(tier);
            }
        });
    });

    group.finish();
}

fn bench_string_similarity(c: &mut Criterion) {
    let mut group = c.benchmark_group("String_Similarity");

    let pairs = vec![
        ("cadence", "cadanse"),
        ("percussion", "percushun"),
        ("sovereign", "sovreign"),
        ("staccato", "stacato"),
    ];

    group.bench_function("sovereign_phonetic_skeleton", |b| {
        b.iter(|| {
            for (a, b_str) in &pairs {
                black_box(sovereign_phonetic_score(black_box(a), black_box(b_str)));
            }
        });
    });

    group.bench_function("levenshtein_raw", |b| {
        b.iter(|| {
            for (a, b_str) in &pairs {
                black_box(levenshtein(black_box(a), black_box(b_str)));
            }
        });
    });

    group.bench_function("naive_exact_contains", |b| {
        b.iter(|| {
            for (a, b_str) in &pairs {
                black_box(naive_exact_match(black_box(a), black_box(b_str)));
            }
        });
    });

    group.finish();
}

fn bench_deliberation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Deliberation");
    let theory_lab = TheoryLab::new();

    group.bench_function("sovereign_parallel_11_fibonacci", |b| {
        b.iter(|| {
            black_box(parallel_deliberation(&theory_lab, black_box("dab_industries")));
        });
    });

    group.bench_function("sovereign_sequential_11_fibonacci", |b| {
        b.iter(|| {
            black_box(sequential_deliberation(&theory_lab, black_box("dab_industries")));
        });
    });

    group.bench_function("naive_single_observer", |b| {
        b.iter(|| {
            black_box(single_observer_deliberation(&theory_lab, black_box("dab_industries")));
        });
    });

    group.finish();
}

fn bench_math_primitives(c: &mut Criterion) {
    let mut group = c.benchmark_group("Math_Primitives");

    group.bench_function("sovereign_phi_table_o1", |b| {
        b.iter(|| {
            for density in 0usize..=20 {
                black_box(phi_density_score(black_box(density)));
            }
        });
    });

    group.bench_function("legacy_phi_powi", |b| {
        b.iter(|| {
            for density in 0usize..=20 {
                black_box(phi_density_old_powi(black_box(density)));
            }
        });
    });

    group.bench_function("5phi_constant", |b| {
        b.iter(|| {
            black_box(PHI_5);
        });
    });

    group.bench_function("linear_phi_approximation", |b| {
        // What a naive approach might do: precomputed sum rather than true phi^n
        b.iter(|| {
            for density in 0usize..=20 {
                let approx = (1.0 + density as f64 * 0.618) / 21.618;
                black_box(approx);
            }
        });
    });

    group.finish();
}

fn bench_holographic_vs_dot_product(c: &mut Criterion) {
    let mut group = c.benchmark_group("Vector_Similarity");
    let math = SovereignMath::new();

    group.bench_function("sovereign_holographic_expansion", |b| {
        let test_intents = vec![
            "SOVEREIGN_QUERY_BARS",
            "PHONETIC_ANALYSIS",
            "DEEP_CADENCE_SCAN",
        ];
        b.iter(|| {
            for intent in &test_intents {
                let hv = math.holographic_expand(black_box(intent));
                black_box(hv);
            }
        });
    });

    group.bench_function("naive_64d_dot_product_similarity", |b| {
        let va: Vec<f64> = (0..64).map(|i| (i as f64 * PHI) % 1.0).collect();
        let vb: Vec<f64> = (0..64).map(|i| ((i + 3) as f64 * PHI) % 0.9).collect();
        b.iter(|| {
            black_box(naive_dot_product_similarity(black_box(&va), black_box(&vb)));
        });
    });

    group.bench_function("sovereign_volumetric_expand", |b| {
        b.iter(|| {
            for intent in &["BARS", "CADENCE", "DAB", "SOVEREIGN", "PHONETIC"] {
                let ctx = math.expand(black_box(intent));
                let refracted = math.refract(black_box(&ctx));
                black_box(refracted);
            }
        });
    });

    group.finish();
}

fn bench_throughput_flood(c: &mut Criterion) {
    let mut group = c.benchmark_group("Throughput_Flood");
    let dab = DABIndustries::new();

    // How many tier classifications per second?
    let batch_sizes = [100usize, 1000, 10_000];
    let test_query = "sovereign percussion beat analysis cadence";

    for &n in &batch_sizes {
        group.throughput(Throughput::Elements(n as u64));
        group.bench_with_input(
            BenchmarkId::new("sovereign_tier_classify_per_sec", n),
            &n,
            |b, &n| {
                b.iter(|| {
                    for _ in 0..n {
                        let d = dab.protocols.percussion_density(black_box(test_query));
                        black_box(query_depth_from_density(d));
                    }
                });
            }
        );

        group.bench_with_input(
            BenchmarkId::new("kv_cache_hit_per_sec", n),
            &n,
            |b, &n| {
                let mut cache = TurboQuantCache::new();
                cache.insert(test_query, "SOVEREIGN_RESPONSE".to_string(), 0.95);
                b.iter(|| {
                    for _ in 0..n {
                        black_box(cache.get(black_box(test_query)));
                    }
                });
            }
        );
    }

    group.finish();
}

fn bench_bar_validation(c: &mut Criterion) {
    let mut group = c.benchmark_group("Bar_Validation");
    let dab = DABIndustries::new();

    let bars = vec![
        Bar { text: "simple verse".to_string(),               phase: LyricPhase::Observation },
        Bar { text: "perception deception direction".to_string(), phase: LyricPhase::Reaction },
        Bar { text: "blazing percussion past your perception breaking defenses combustion".to_string(), phase: LyricPhase::Action },
        Bar { text: "sovereign dominion beyond the horizon divine cognition positioned precision decision derision".to_string(), phase: LyricPhase::Action },
    ];

    for bar in &bars {
        let density = dab.protocols.percussion_density(&bar.text);
        let word_count = bar.text.split_whitespace().count();
        group.bench_with_input(
            BenchmarkId::new("validate_bar", format!("d{}w{}", density, word_count)),
            bar,
            |b, bar| {
                b.iter(|| { black_box(dab.validate_bar(black_box(bar))); });
            }
        );
    }

    group.finish();
}

criterion_group!(
    sovereign_vs_titans,
    bench_kv_cache,
    bench_query_routing,
    bench_string_similarity,
    bench_deliberation,
    bench_math_primitives,
    bench_holographic_vs_dot_product,
    bench_throughput_flood,
    bench_bar_validation,
);
criterion_main!(sovereign_vs_titans);
