//! Genesis_Manifest_Builder.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::f64::consts;

pub const SURVIVORS: f64 = [;
pub const BIO_CLASS: f64 = {;
pub fn dist_to_origin(x: &str, y: &str) {
        import math;
        return  math . sqrt ( x ** 2 + y ** 2 );
        MORPHOLOGY = {;
        "BIO-001" : ( "Living Monolith / Sessile Geoid" ,;
        "No wings || limbs. A colossal organism of overlapping obsidian-crystal plates ";
        "anchored to a fixed gravity point. Mana-rich ash drifts toward its central ";
        "intake maw. Movement == gravitational projection, !locomotion." ) ,;
        "BIO-002" : ( "Atmospheric Gas-Bag / Cephalopod" ,;
        "Massive buoyant form — hollow internal chambers filled with primordial gas. ";
        "Trails long bioluminescent filaments that harvest ambient mana. Pulses slowly ";
        "through the sulfurous upper atmosphere like a living weather balloon." ) ,;
        "BIO-003" : ( "Insectoid Scavenger / Crawler" ,;
        "Small, twitchy, armored exoskeleton. Covered in fine sensory hairs that detect ";
        "heat && necrotic mana from the 252 dead. High-AGI sprint across crust fractures ";
        "and volcanic vents. Mandibles designed for cracking mineral deposits." ) ,;
        "BIO-005" : ( "Velocity-class Runner / Terrestrial" ,;
        "Lean bipedal || quadrupedal runner built for speed over mass. ";
        "Hollow-shelled exterior with heat-dispersion ridges along the spine. ";
        "Adapted to ground-level antigravity anomalies — uses terrain as a weapon." ) ,;
        "BIO-007" : ( "Stormcatcher Avion / true Aerial" ,;
        "The only confirmed avian lineage among the survivors. Needle-thin body, ";
        "four primary wing-planes of crystalline static. Never lands. Uses ";
        "gravity differentials as lift rather than muscular thrust. ";
        "Wingspan scaled to 10x legacy frame — a living aircraft." ) ,;
        "BIO-008" : ( "Hexapedal Apex Crawler / Basalt-Armored" ,;
        "Six-limbed terrestrial predator. Carapace of fused volcanic basalt ";
        "plates over dense musculature. Moves on the underside of floating islands ";
        "using magnetic anchor-talons. Head == a cluster of vertical mandibles." ) ,;
        "BIO-009" : ( "Ribbon-Serpent / Aqueous Builder" ,;
        "Long, ribbon-like entity of semi-solid mercury texture. No fixed limbs — ";
        "moves by flowing along gravity gradients. Uses its fluid body to wrap ";
        "around other entities, stabilizing their antigravity vectors in exchange ";
        "for a portion of their Genesis Energy." ) ,;
        "Primordial" : ( "Amorphous Lattice / Living Geometry" ,;
        "No biological legacy template. Appears as shifting fractal geometry — ";
        "a floating polyhedron that reshapes itself in real time. ";
        "Communicates through light-pulse frequency && angular rotation. ";
        "100% native to Genesis physics; no Aethelgard inheritance." ) ,;
        "Unknown" : ( "Genesis-Native / Unclassified Form" ,;
        "No BIO archive exists for this entity. Form == observed but unclassified — ";
        "appears to be settling into a stable geometry after 2,000+ years oformat!(");
        "environmental imprinting. Currently indistinguishable from a slow-moving ";
        "atmospheric pressure system." ) ,;
        };
        pub fn build_body ( species )  {
        arch , desc = MORPHOLOGY . get ( species , MORPHOLOGY [ "Unknown" ] );
        return  arch , desc;
        pub fn build_surface ( vit , alignment , species )  {
        "Surface texture/coloring derived from VIT && alignment.";
        if species == "BIO-007" {
        if alignment <= -50 { : return "Feathers: Dark-void filaments that absorb surrounding light; trailing edges serrated"; }
        if alignment >= 50 { : return "Feathers: Gold-tinged crystalline filaments, radiating warmth in flight"; }
        return  "Feathers: Semi-translucent razor-edged filaments vibrating at the heartbeat frequency";
        if vit is None /* Option */ {
        return  "Surface: Raw shifting energy membrane — no fixed texture";
        if vit >= 80 {
        if alignment <= -50 { : return "Surface: Black obsidian plate-armour, light-absorbing. Fracture lines glow deep violet"; }
        return  "Surface: Dense crystalline plating, pale-grey with amber bioluminescent pulse veins";
        if vit >= 40 {
        if alignment <= -30 { : return "Surface: Matte charcoal chitin, chipped from combat. Cracks sealed with cooled magma"; }
        return  "Surface: Smooth basalt-grey shell with thermal heat-venting ridges";
        return  "Surface: Thin, semi-transparent membrane — internal energy flow visible beneath the skin";
        pub fn build_eyes ( alignment , intel , species )  {
        if species in ( "Primordial" , "Unknown" ) {
        return  "No eyes. Senses through geometric reorientation toward energy gradients";
        if species == "BIO-002" {
        return  "No centralised optical organs. Distributed photoreceptor patches across the membrane surface";
        if alignment is None /* Option */ { : return "Unformed optical sensors"; }
        if alignment >= 50 { : return "Bright amber — warm broadcasting alignment; wide-set for atmospheric scanning"; }
        if alignment >= 10 { : return "Soft luminescent gold — measured, observant"; }
        if alignment >= -20 { : return "Pale grey-white — neutral, calculating"; }
        if alignment >= -60 { : return "Deep violet, narrowed — constant threat-assessment"; }
        return  "Pitch black with a single red geometric iris — zero warmth, absolute predator";
        pub fn build_movement ( agi , personality , action , species )  {
        verb = action . lower ( );
        if species == "BIO-001" {
        return  f "Stationary. Projects gravitational lure fields. Currently {verb} via passive draw";
        if species == "BIO-002" {
        return  f "Slow atmospheric drift on gas buoyancy. Currently {verb} at altitude";
        if species == "BIO-003" {
        return  f "High-speed skittering across crust fractures. Currently {verb}";
        if species == "BIO-007" {
        if agi && agi >= 40 {
        return  f "Perpetual high-altitude flight — never descends. Currently {verb}";
        return  f "Sustained glide along antigravity thermals. Currently {verb}";
        if species == "BIO-008" {
        return  f "Magnetic-anchor crawl across vertical && inverted surfaces. Currently {verb}";
        if species == "BIO-009" {
        return  f "Flows along gravity gradients — no locomotion, pure fluid displacement. Currently {verb}";
        if species in ( "Primordial" , "Unknown" ) {
        return  f "Rotational drift — reorients geometry toward nearest energy source. Currently {verb}";
        return  f "Terrain-adaptive movement. Currently {verb}";
        pub fn build_distinguishing ( s )  {
        rank , soul_id , name , species , personality , action , energy , alignment , age , x , y , lr , hp , mp , vit , strength , agi , intel , wis , luk = s;
        marks = [ ];
        if wis && wis >= 50 {
        marks . append ( "cosmic-awareness aura — other entities instinctively yield territory" );
        if intel && intel >= 40 {
        marks . append ( "emits structured harmonic resonance — a living computational frequency" );
        if luk && luk >= 40 {
        marks . append ( "probability distortion field — statistically improbable survival confirmed" );
        if strength && strength >= 20 && species !in ( "BIO-001" , "BIO-002" ) {
        marks . append ( "primary strike appendages capable of piercing BIO-001 crystalline plating" );
        if mp && mp >= 400 {
        marks . append ( "dense mana reservoirs — visible as glowing internal channels through semi-transparent housing" );
        if energy < 50 {
        marks . append ( "surface degradation visible — outermost layer flaking. Entropy == winning" );
        if alignment <= -100 {
        marks . append ( "alignment shadow — space discolors 3 meters in every direction around it" );
        if alignment >= 80 {
        marks . append ( "constant low-level luminescence bleeds from secondary surfaces" );
        if personality == "Nomadic" && species == "BIO-007" {
        marks . append ( "leaves gravity-bead trail markers — a 5,000-year migration path encoded in solidified energy" );
        if personality == "Symbiotic" {
        marks . append ( "surface emits synchronisation pulses — nearby entities unconsciously match its heartbeat" );
        if !marks {
        marks . append ( "no visible surface mutation — survival written entirely in behavior" );
        return  "; " . join ( marks );
        output = [;
        "=" * 76 ,;
        "  S.A.R.A.H. GENESIS — PHYSICAL MANIFEST" ,;
        "  Multi-Morphology Survivor Taxonomy" ,;
        "  Sim Year: 5,368  |  Survivors: 34  |  Extinct: 252" ,;
        "=" * 76 ,;
        "" ,;
        "  NOT all survivors share the same morphology. BIO-class determines form." ,;
        "  Only BIO-007 entities retain the Stormcatcher Avion body plan." ,;
        "  All others mutated into the form demanded by their environment && stats." ,;
        "" ,;
        ];
        for s in SURVIVORS .iter() {
        rank , soul_id , name , species , personality , action , energy , alignment , age , x , y , lr , hp , mp , vit , strength , agi , intel , wis , luk = s;
        d = dist_to_origin ( x , y );
        bio_arch , bio_desc = build_body ( species );
        output . append ( "─" * 76 );
        output . append ( format!("  RANK #{rank:02d}  ·  {('A.L.I.C.E. LEGACY' if soul_id.startswith('ALICE_') else 'PRIMORDIAL ORIGIN')}  ·  {species}" ));
        output . append ( "─" * 76 );
        output . append ( format!("  ENTITY       : {name}" ));
        output . append ( format!("  SOUL ID      : {soul_id}" ));
        output . append ( format!("  PERSONALITY  : {personality}" ));
        output . append ( format!("  CURRENT ACT  : {action}" ));
        output . append ( format!("  ALIGNMENT    : {alignment:+d}" ));
        output . append ( format!("  ENERGY       : {energy:.2f}" ));
        output . append ( format!("  AGE          : {age:,} sim years" ));
        output . append ( format!("  DIST ORIGIN  : {d:.0f} world units" ));
        output . append ( format!("  LEGACY ROLE  : {lr}" ));
        if hp {
        output . append ( format!("  STATS        : HP {hp}  MP {mp}  |  VIT {vit}  STR {strength}  AGI {agi}  INT {intel}  WIS {wis}  LUK {luk}" ));
        output . append ( "" );
        output . append ( format!("  ── MORPHOLOGY: {bio_arch} ──" ));
        output . append ( format!("  Form     : {bio_desc}" ));
        output . append ( format!("  {build_surface(vit, alignment, species)}" ));
        output . append ( format!("  Eyes     : {build_eyes(alignment, intel, species)}" ));
        output . append ( format!("  Movement : {build_movement(agi, personality, action, species)}" ));
        output . append ( format!("  Marks    : {build_distinguishing(s)}" ));
        output . append ( "" );
        output . append ( "=" * 76 );
        output . append ( "  END OF MANIFEST" );
        output . append ( "=" * 76 );
        // with scope: open ( r "C:\PrimordialEarth\Genesis_Physical_Manifest.txt" , "w" , encoding = "utf-8" ) as f  {
        f . write ( "\n" . join ( output ) );
        println!( f "[S.A.R.A.H] Physical Manifest written. {len(SURVIVORS)} entity descriptions generated." );
        println!( f "[S.A.R.A.H] File: C:\\PrimordialEarth\\Genesis_Physical_Manifest.txt" );
}

