// ═══════════════════════════════════════════════════════════════
//  D.A.B. INDUSTRIES — HEAVY ENGINEERING DIVISION
//  Architect: Josh | Owner: Derik
//  Project: Helix Fluid Accelerator (CAD Phase, Feb 2026)
// ═══════════════════════════════════════════════════════════════
use crate::phi::{GOLDEN_ANGLE_DEG, GOLDEN_ANGLE_RAD, PHI, PHI_INV, MOTOR_RATIO_DELTA_FROM_PHI_INV};

// ───────────────────────────────────────────────────────────────
//  7-12 MOTOR GEOMETRY CONSTANTS
// ───────────────────────────────────────────────────────────────

/// Stator pole count — governs timing and electromagnetic kick.
pub const STATOR_POLES: u8 = 7;

/// Rotor magnet count — governs torque and rotational smoothness.
pub const ROTOR_MAGNETS: u8 = 12;

/// The core ratio: 7 / 12 ≈ 0.58333...
/// GCD(7, 12) = 1 — no shared factors.  This is why cogging is eliminated:
/// the poles and magnets never synchronise at a sub-cycle boundary.
pub const MOTOR_RATIO: f64 = STATOR_POLES as f64 / ROTOR_MAGNETS as f64;

/// Least Common Multiple of 7 and 12 = 84.
/// Stator and rotor achieve full angular alignment only once every 84
/// electrical cycles — producing an imperceptibly small cogging period.
pub const ALIGNMENT_CYCLE: u8 = 84; // LCM(7, 12)

// ───────────────────────────────────────────────────────────────
//  MOTOR GEOMETRY
// ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct MotorGeometry712 {
    /// Number of stator poles (7) — controls pulse timing / kick.
    pub stator_poles: u8,
    /// Number of rotor magnets (12) — controls torque and smoothness.
    pub rotor_magnets: u8,
    /// Pole-to-magnet ratio (7/12 ≈ 0.5833).
    pub ratio: f64,
    /// Full alignment period in electrical cycles (LCM = 84).
    /// Higher = smoother.  84 is excellent for vortex-driven systems.
    pub alignment_cycle: u8,
}

impl MotorGeometry712 {
    pub fn new() -> Self {
        Self {
            stator_poles:    STATOR_POLES,
            rotor_magnets:   ROTOR_MAGNETS,
            ratio:           MOTOR_RATIO,
            alignment_cycle: ALIGNMENT_CYCLE,
        }
    }

    /// Electrical frequency (Hz) for a given mechanical RPM.
    /// f_elec = (RPM / 60) × (rotor_magnets / 2)
    pub fn electrical_frequency_hz(&self, rpm: f64) -> f64 {
        (rpm / 60.0) * (self.rotor_magnets as f64 / 2.0)
    }

    /// At a given RPM, how many electrical cycles per second does the
    /// hypervisor need to track to keep phase-lock on the rotor?
    pub fn hypervisor_tracking_hz(&self, rpm: f64) -> f64 {
        self.electrical_frequency_hz(rpm) * self.stator_poles as f64
    }

    /// How close is the 7-12 ratio (0.5833) to 1/φ (0.6180)?
    /// Returns the delta: ~0.0347 — the closest achievable approximation
    /// to the golden ratio at small integer pole counts.
    pub fn phi_proximity(&self) -> f64 {
        MOTOR_RATIO_DELTA_FROM_PHI_INV
    }

    /// Ratio expressed relative to φ: how many percent of 1/φ is 7/12?
    pub fn ratio_as_phi_fraction(&self) -> f64 {
        self.ratio / PHI_INV // 0.5833 / 0.6180 ≈ 0.9439 (94.4% of 1/φ)
    }

    /// Print geometry summary to stdout.
    pub fn report(&self) {
        println!("── 7-12 Motor Geometry ──────────────────────────");
        println!("  Stator poles    : {}", self.stator_poles);
        println!("  Rotor magnets   : {}", self.rotor_magnets);
        println!("  Ratio           : {:.6}", self.ratio);
        println!("  Alignment cycle : {} electrical cycles (LCM)", self.alignment_cycle);
        println!("  Cogging         : ELIMINATED (GCD=1, LCM=84)");
    }
}

impl Default for MotorGeometry712 {
    fn default() -> Self { Self::new() }
}

// ───────────────────────────────────────────────────────────────
//  HELIX FLUID ACCELERATOR
// ───────────────────────────────────────────────────────────────

/// Pitch mode of the accelerator helix.
#[derive(Debug, Clone, PartialEq)]
pub enum HelixPitch {
    /// Pitch varies along the helix length to match fluid velocity profile.
    Variable,
    /// Fixed pitch — simpler, lower-performance config.
    Fixed(f64),
}

/// CAD / prototyping phase of the accelerator build.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildPhase {
    ConceptualizationComplete,
    CadModellingActive,
    Prototyping,
    Testing,
    ProductionReady,
}

impl BuildPhase {
    pub fn label(&self) -> &'static str {
        match self {
            Self::ConceptualizationComplete => "Conceptualization Complete",
            Self::CadModellingActive        => "CAD Modelling Active (Feb 2026)",
            Self::Prototyping               => "Prototyping",
            Self::Testing                   => "Testing",
            Self::ProductionReady           => "Production Ready",
        }
    }
}

/// The primary D.A.B. Industries invention.
#[derive(Debug, Clone)]
pub struct HelixFluidAccelerator {
    pub name:        &'static str,
    pub phase:       BuildPhase,
    /// Helix Centrifuge with Magnetic Helix core.
    pub design_type: &'static str,
    /// Variable-pitch helix — matches fluid velocity profile at each stage.
    pub helix_pitch: HelixPitch,
    /// The 7-12 geometry that drives the accelerator.
    pub motor:       MotorGeometry712,
}

impl HelixFluidAccelerator {
    pub fn new() -> Self {
        Self {
            name:        "Helix Fluid Accelerator",
            phase:       BuildPhase::CadModellingActive,
            design_type: "Helix Centrifuge / Magnetic Helix (Vortex-Induced)",
            helix_pitch: HelixPitch::Variable,
            motor:       MotorGeometry712::new(),
        }
    }

    /// Given a target RPM, report the operating frequencies the Sarah
    /// Hypervisor must lock to for phase-stable control.
    pub fn hypervisor_lock_report(&self, rpm: f64) {
        let elec = self.motor.electrical_frequency_hz(rpm);
        let tracking = self.motor.hypervisor_tracking_hz(rpm);
        println!("── Hypervisor Lock @ {:.0} RPM ──────────────────", rpm);
        println!("  Electrical freq  : {:.2} Hz", elec);
        println!("  Tracking rate    : {:.2} Hz", tracking);
        println!("  Phase resolution : every {:.4} ms",
                 1_000.0 / tracking);
    }

    /// Recommended inter-turn angular offset for maximum fluid-path separation.
    /// Returns the Golden Angle: 360° / φ² ≈ 137.508°
    ///
    /// At this offset, each successive helix turn is maximally separated from
    /// every prior turn. Same principle used in sunflower seed packing, DNA
    /// pitch, and nautilus shell geometry. Eliminates resonant overlap between
    /// fluid passes — the rotational equivalent of cogging elimination.
    pub fn optimal_pitch_angle_deg(&self) -> f64 {
        GOLDEN_ANGLE_DEG
    }

    /// Same as above, in radians (for trig calculations).
    pub fn optimal_pitch_angle_rad(&self) -> f64 {
        GOLDEN_ANGLE_RAD
    }

    /// φ-scaled helix expansion factor.
    /// Each successive turn of the helix should expand in diameter by factor φ
    /// to produce a logarithmic spiral matching natural vortex geometry.
    pub fn phi_expansion_factor(&self) -> f64 {
        PHI
    }

    pub fn report(&self) {
        println!("── Helix Fluid Accelerator ──────────────────────");
        println!("  Name        : {}", self.name);
        println!("  Phase       : {}", self.phase.label());
        println!("  Design      : {}", self.design_type);
        println!("  Helix Pitch : {:?}", self.helix_pitch);
        self.motor.report();
    }
}

impl Default for HelixFluidAccelerator {
    fn default() -> Self { Self::new() }
}

// ───────────────────────────────────────────────────────────────
//  MATERIALS
// ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialGrade {
    AviationGrade,
    SuperAlloy,
    Custom,
}

#[derive(Debug, Clone)]
pub struct MaterialSpec {
    pub name:        &'static str,
    pub grade:       MaterialGrade,
    /// Where this material is used in the accelerator.
    pub application: &'static str,
}

/// Full D.A.B. Industries materials inventory.
#[derive(Debug, Clone)]
pub struct MaterialsInventory {
    pub specs: &'static [MaterialSpec],
}

/// All known materials in the D.A.B. build as a compile-time constant slice.
pub const DAB_MATERIALS: &[MaterialSpec] = &[
    MaterialSpec {
        name:        "High-Entropy Alloys",
        grade:       MaterialGrade::AviationGrade,
        application: "Impact resistance — outer shell and collision surfaces",
    },
    MaterialSpec {
        name:        "Titanium-Aluminide",
        grade:       MaterialGrade::AviationGrade,
        application: "Casing strength — primary structural housing",
    },
    MaterialSpec {
        name:        "Single-Crystal Superalloys",
        grade:       MaterialGrade::SuperAlloy,
        application: "Internal shaft — handles high RPM without grain-boundary creep",
    },
    MaterialSpec {
        name:        "Airplane-Grade Scrap Stock",
        grade:       MaterialGrade::AviationGrade,
        application: "Prototype fabrication — sourced aviation surplus",
    },
];

impl MaterialsInventory {
    pub fn new() -> Self {
        Self { specs: DAB_MATERIALS }
    }

    pub fn report(&self) {
        println!("── Materials Inventory ──────────────────────────");
        for m in self.specs {
            println!("  [{:?}] {} — {}", m.grade, m.name, m.application);
        }
    }
}

impl Default for MaterialsInventory {
    fn default() -> Self { Self::new() }
}

// ───────────────────────────────────────────────────────────────
//  MARINE DIVISION
// ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MarineAssetStatus {
    Prototype,
    InService,
    InDevelopment,
}

#[derive(Debug, Clone)]
pub struct MarineAsset {
    pub name:   &'static str,
    pub status: MarineAssetStatus,
}

pub const DAB_MARINE_ASSETS: &[MarineAsset] = &[
    MarineAsset { name: "Custom Rod Holders",          status: MarineAssetStatus::InDevelopment },
    MarineAsset { name: "Marine Flare Systems",        status: MarineAssetStatus::Prototype },
    MarineAsset { name: "Boat Propulsion Prototypes",  status: MarineAssetStatus::Prototype },
];

#[derive(Debug, Clone)]
pub struct MarineDivision {
    pub assets: &'static [MarineAsset],
}

impl MarineDivision {
    pub fn new() -> Self {
        Self { assets: DAB_MARINE_ASSETS }
    }

    pub fn report(&self) {
        println!("── Marine Division ──────────────────────────────");
        for a in self.assets {
            println!("  [{:?}] {}", a.status, a.name);
        }
    }
}

impl Default for MarineDivision {
    fn default() -> Self { Self::new() }
}

// ───────────────────────────────────────────────────────────────
//  VORTEX-INDUCED MOLECULAR BONDING ENGINE
// ───────────────────────────────────────────────────────────────

/// The engine concept that sits above the accelerator.
/// Uses vortex dynamics to organise (not simply mix) fluid molecules
/// at high velocity with minimal heat or friction loss.
#[derive(Debug, Clone)]
pub struct VortexBondingEngine {
    pub name:        &'static str,
    pub description: &'static str,
    /// The accelerator that feeds this engine.
    pub accelerator: HelixFluidAccelerator,
}

impl VortexBondingEngine {
    pub fn new() -> Self {
        Self {
            name: "Vortex-Induced Molecular Bonding Engine",
            description: "Organises fluid molecules into a stable vortex at high \
                          velocity using the 7-12 harmonic frequency. Eliminates \
                          energy loss through heat and friction. Distinct from a \
                          blender (which mashes); this system arranges.",
            accelerator: HelixFluidAccelerator::new(),
        }
    }

    pub fn report(&self) {
        println!("── Vortex-Induced Molecular Bonding Engine ──────");
        println!("  Name        : {}", self.name);
        println!("  Description : {}", self.description);
        self.accelerator.report();
    }
}

impl Default for VortexBondingEngine {
    fn default() -> Self { Self::new() }
}

// ───────────────────────────────────────────────────────────────
//  D.A.B. ENGINEERING DIVISION — TOP-LEVEL
// ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct DABEngineering {
    pub lead_engineers: &'static [&'static str],
    pub engine:         VortexBondingEngine,
    pub materials:      MaterialsInventory,
    pub marine:         MarineDivision,
}

impl DABEngineering {
    pub fn new() -> Self {
        Self {
            lead_engineers: &["Derik", "Josh (The Architect)"],
            engine:         VortexBondingEngine::new(),
            materials:      MaterialsInventory::new(),
            marine:         MarineDivision::new(),
        }
    }

    /// Full diagnostic — every sub-system reports to stdout.
    pub fn full_diagnostic(&self) {
        println!("╔══════════════════════════════════════════════════╗");
        println!("║   D.A.B. INDUSTRIES — ENGINEERING DIVISION       ║");
        println!("╚══════════════════════════════════════════════════╝");
        println!("  Lead Engineers: {}", self.lead_engineers.join(", "));
        println!();
        self.engine.report();
        println!();
        self.materials.report();
        println!();
        self.marine.report();
        println!();
        // Hypervisor lock at representative operating speed
        self.engine.accelerator.hypervisor_lock_report(3_500.0);
    }
}

impl Default for DABEngineering {
    fn default() -> Self { Self::new() }
}

// ───────────────────────────────────────────────────────────────
//  TESTS
// ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn motor_ratio_correct() {
        let m = MotorGeometry712::new();
        // 7/12 ≈ 0.5833...
        assert!((m.ratio - (7.0 / 12.0)).abs() < f64::EPSILON);
    }

    #[test]
    fn alignment_cycle_is_lcm() {
        // LCM(7, 12) = 84
        assert_eq!(ALIGNMENT_CYCLE, 84);
    }

    #[test]
    fn gcd_is_one_so_cogging_eliminated() {
        fn gcd(a: u8, b: u8) -> u8 { if b == 0 { a } else { gcd(b, a % b) } }
        assert_eq!(gcd(STATOR_POLES, ROTOR_MAGNETS), 1);
    }

    #[test]
    fn electrical_frequency_at_3500rpm() {
        let m = MotorGeometry712::new();
        let f = m.electrical_frequency_hz(3_500.0);
        // (3500/60) * 6 = 350 Hz
        assert!((f - 350.0).abs() < 0.01);
    }

    #[test]
    fn four_materials_in_inventory() {
        assert_eq!(DAB_MATERIALS.len(), 4);
    }

    #[test]
    fn three_marine_assets() {
        assert_eq!(DAB_MARINE_ASSETS.len(), 3);
    }

    #[test]
    fn engineering_division_has_two_lead_engineers() {
        let eng = DABEngineering::new();
        assert_eq!(eng.lead_engineers.len(), 2);
    }
}
