// ═══════════════════════════════════════════════════════════════
//  D.A.B. INDUSTRIES — TITAN BENCHMARK SUITE
//  Measures all core D.A.B. functions under the load of
//  the 209-observer Titan Neural Lattice deliberation.
//
//  Run:  cargo bench -p dab_industries
//  HTML: target/criterion/report/index.html
// ═══════════════════════════════════════════════════════════════

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use dab_industries::{
    DABIndustries, Bar, LyricPhase, DABModel,
    phi::{
        PHI, PHI_INV, PHI_5, PHI_5_INV,
        SOVEREIGN_DENSITY_THRESHOLD, SOVEREIGN_MEMORY_CONFIDENCE,
        phi_density_score, memory_confidence, fibonacci, fibonacci_phi_approx,
    },
    scheduler::{query_depth_from_density, QueryDepth},
    engineering::{
        MotorGeometry712, HelixFluidAccelerator, DABEngineering,
        STATOR_POLES, ROTOR_MAGNETS, ALIGNMENT_CYCLE,
    },
};

// ───────────────────────────────────────────────────────────────
//  TEST BARS — from simple to Sovereign-density
// ───────────────────────────────────────────────────────────────

/// Bars of increasing percussion density for benchmark scaling tests.
const SPARSE_BAR:   &str = "soft melody flows into the air";            // density ~0
const STANDARD_BAR: &str = "breaking through the battleground";         // density ~3-4
const DEEP_BAR:     &str = "pick up the beat and take the stage";       // density ~6-7
const SOVEREIGN_BAR:&str = "pack the beat — trigger the peak, detonate the block"; // density ≥8

// ───────────────────────────────────────────────────────────────
//  GROUP 1 — PERCUSSION ENGINE
// ───────────────────────────────────────────────────────────────

fn bench_percussion_density(c: &mut Criterion) {
    let dab = DABIndustries::new();
    let mut group = c.benchmark_group("percussion_density");

    for (label, bar) in &[
        ("sparse",    SPARSE_BAR),
        ("standard",  STANDARD_BAR),
        ("deep",      DEEP_BAR),
        ("sovereign", SOVEREIGN_BAR),
    ] {
        group.bench_with_input(BenchmarkId::new("bar", label), bar, |b, bar| {
            b.iter(|| dab.protocols.percussion_density(black_box(bar)));
        });
    }
    group.finish();
}

// ───────────────────────────────────────────────────────────────
//  GROUP 2 — BAR VALIDATION (phi-curve scoring)
// ───────────────────────────────────────────────────────────────

fn bench_validate_bar(c: &mut Criterion) {
    let dab = DABIndustries::new();
    let mut group = c.benchmark_group("validate_bar");

    let bars = vec![
        ("Observation",    Bar { text: SPARSE_BAR.to_string(),    phase: LyricPhase::Observation }),
        ("Reaction",       Bar { text: STANDARD_BAR.to_string(),  phase: LyricPhase::Reaction    }),
        ("Action_Deep",    Bar { text: DEEP_BAR.to_string(),      phase: LyricPhase::Action      }),
        ("Action_Sovereign",Bar { text: SOVEREIGN_BAR.to_string(),phase: LyricPhase::Action      }),
    ];

    for (label, bar) in &bars {
        group.bench_with_input(BenchmarkId::new("phase", label), bar, |b, bar| {
            b.iter(|| dab.validate_bar(black_box(bar)));
        });
    }
    group.finish();
}

// ───────────────────────────────────────────────────────────────
//  GROUP 3 — phi COMPUTATIONS
// ───────────────────────────────────────────────────────────────

fn bench_phi_scoring(c: &mut Criterion) {
    let mut group = c.benchmark_group("phi");

    // phi_density_score at each density tier
    for density in [0usize, 2, 5, 7, 8, 12, 20] {
        group.bench_with_input(
            BenchmarkId::new("density_score", density),
            &density,
            |b, &d| b.iter(|| phi_density_score(black_box(d))),
        );
    }

    group.bench_function("PHI_5_computation", |b| {
        b.iter(|| black_box(5.0 * PHI))
    });

    group.bench_function("fibonacci_30", |b| {
        b.iter(|| fibonacci(black_box(30)))
    });

    group.bench_function("fibonacci_phi_approx_50", |b| {
        b.iter(|| fibonacci_phi_approx(black_box(50)))
    });

    group.finish();
}

// ───────────────────────────────────────────────────────────────
//  GROUP 4 — QUERY DEPTH CLASSIFICATION
// ───────────────────────────────────────────────────────────────

fn bench_query_depth(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_depth");

    group.throughput(Throughput::Elements(1));

    for density in [0usize, 2, 3, 5, 6, 7, 8, 20] {
        group.bench_with_input(
            BenchmarkId::new("classify", density),
            &density,
            |b, &d| b.iter(|| query_depth_from_density(black_box(d))),
        );
    }
    group.finish();
}

// ───────────────────────────────────────────────────────────────
//  GROUP 5 — MOTOR GEOMETRY (7-12)
// ───────────────────────────────────────────────────────────────

fn bench_motor_geometry(c: &mut Criterion) {
    let motor = MotorGeometry712::new();
    let mut group = c.benchmark_group("motor_geometry_712");

    group.bench_function("new", |b| {
        b.iter(|| MotorGeometry712::new())
    });

    for rpm in [1000.0f64, 3500.0, 7200.0, 12000.0] {
        group.bench_with_input(
            BenchmarkId::new("electrical_freq_hz", rpm as u64),
            &rpm,
            |b, &rpm| b.iter(|| motor.electrical_frequency_hz(black_box(rpm))),
        );

        group.bench_with_input(
            BenchmarkId::new("hypervisor_tracking_hz", rpm as u64),
            &rpm,
            |b, &rpm| b.iter(|| motor.hypervisor_tracking_hz(black_box(rpm))),
        );
    }

    group.bench_function("phi_proximity", |b| {
        b.iter(|| motor.phi_proximity())
    });

    group.finish();
}

// ───────────────────────────────────────────────────────────────
//  GROUP 6 — TITAN DELIBERATION SIMULATION
//  This is the crown benchmark.
//  Simulates what sarah_reasoning::HiveAssembly does:
//  validate a bar through 209 observers, each with a DAB model tag.
//  Measures total throughput of the Titan Neural Lattice.
// ───────────────────────────────────────────────────────────────

fn bench_titan_deliberation(c: &mut Criterion) {
    let dab          = DABIndustries::new();
    let dab_models   = DABModel::all();
    const OBSERVERS: usize = 209; // Titan Lattice count

    let mut group = c.benchmark_group("titan_deliberation");
    group.sample_size(50); // fewer samples — this is a heavy benchmark

    // Benchmark: 209-observer deliberation on a single bar at each depth tier.
    for (label, bar_text, phase) in &[
        ("sparse_obs",    SPARSE_BAR,    LyricPhase::Observation),
        ("standard_react",STANDARD_BAR,  LyricPhase::Reaction   ),
        ("deep_action",   DEEP_BAR,      LyricPhase::Action     ),
        ("sovereign",     SOVEREIGN_BAR, LyricPhase::Action     ),
    ] {
        let bar = Bar { text: bar_text.to_string(), phase: *phase };

        group.bench_with_input(
            BenchmarkId::new("209_observers", label),
            &bar,
            |b, bar| {
                b.iter(|| {
                    let mut agreement = 0u32;
                    let mut total_score = 0u32;

                    // Primary: Sarah high-authority pass (weight 10)
                    let sarah_score = dab.validate_bar(black_box(bar)) as u32;
                    total_score += sarah_score * 10;
                    if sarah_score > 50 { agreement += 10; }

                    // 209 Titan observers — every 10th gets a DAB model tag
                    for i in 1..=OBSERVERS {
                        let model = dab_models[(i % dab_models.len())];
                        let _ = model.tag(); // tag lookup — matches sarah_reasoning pattern
                        let score = dab.validate_bar(black_box(bar)) as u32;
                        total_score += score;
                        if score > 50 { agreement += 1; }
                    }

                    let total_votes = (OBSERVERS as u32) + 10;
                    let consensus = agreement as f64 / total_votes as f64;
                    let _ = query_depth_from_density(
                        dab.protocols.percussion_density(&bar.text)
                    );
                    black_box((consensus, total_score))
                });
            },
        );
    }

    // Sovereign sovereign: does density ≥ 8 correctly trigger?
    group.bench_function("sovereign_threshold_detection", |b| {
        let bar = Bar { text: SOVEREIGN_BAR.to_string(), phase: LyricPhase::Action };
        b.iter(|| {
            let density = dab.protocols.percussion_density(black_box(&bar.text));
            let depth   = query_depth_from_density(density);
            black_box(depth == QueryDepth::Sovereign)
        })
    });

    // Full system instantiation — how fast can a new DABEngineering core spin up?
    group.bench_function("dab_engineering_full_boot", |b| {
        b.iter(|| black_box(DABEngineering::new()))
    });

    group.finish();
}

// ───────────────────────────────────────────────────────────────
//  GROUP 7 — PHI vs LINEAR scoring comparison
//  Shows exactly how much faster/slower the phi-curve is vs the
//  old linear hard-cap formula.
// ───────────────────────────────────────────────────────────────

fn bench_phi_vs_linear(c: &mut Criterion) {
    let mut group = c.benchmark_group("phi_vs_linear_scoring");

    let densities: Vec<usize> = (0..=15).collect();

    group.bench_function("phi_curve", |b| {
        b.iter(|| {
            let mut total = 0u32;
            for &d in &densities {
                total += phi_density_score(black_box(d)) as u32;
            }
            black_box(total)
        })
    });

    group.bench_function("linear_old", |b| {
        b.iter(|| {
            let mut total = 0u32;
            for &d in &densities {
                // Old formula: hard cap at 5, linear scale
                let score = ((d.min(5) as f64 / 5.0) * 80.0) as u32;
                total += score;
            }
            black_box(total)
        })
    });

    group.finish();
}

// ───────────────────────────────────────────────────────────────
//  REGISTER ALL GROUPS
// ───────────────────────────────────────────────────────────────

criterion_group!(
    benches,
    bench_percussion_density,
    bench_validate_bar,
    bench_phi_scoring,
    bench_query_depth,
    bench_motor_geometry,
    bench_titan_deliberation,
    bench_phi_vs_linear,
);
criterion_main!(benches);
