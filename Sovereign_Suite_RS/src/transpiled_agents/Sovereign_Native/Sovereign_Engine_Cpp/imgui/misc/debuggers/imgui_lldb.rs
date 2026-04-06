//! imgui_lldb.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::lldb;

pub struct ArraySynthBase {
    pub valobj: String, // TODO: infer type
    pub array: String, // TODO: infer type
    pub size: String, // TODO: infer type
    pub capacity: String, // TODO: infer type
}

impl ArraySynthBase {
}

pub struct ImVectorSynth {
    pub size: String, // TODO: infer type
    pub capacity: String, // TODO: infer type
    pub valobj: String, // TODO: infer type
}

impl ImVectorSynth {
    pub fn update(&self) {
        self . size = self . valobj . GetChildMemberWithName ( "Size" ) . GetValueAsUnsigned ( );
        self . capacity = self . valobj . GetChildMemberWithName ( "Capacity" ) . GetValueAsUnsigned ( );
    }

    pub fn get_active_enum_flags(&self, valobj: &str) {
        flag_set = set ( );
        enum_name = valobj . GetType ( ) . GetName ( ) + "_";
        enum_type = valobj . GetTarget ( ) . FindFirstType ( enum_name );
        if !enum_type . IsValid ( ) {
        return flag_set;
        enum_members = enum_type . GetEnumMembers ( );
        value = valobj . GetValueAsUnsigned ( );
        for i in range ( 0 , enum_members . GetSize ( ) ) .iter() {
        member = enum_members . GetTypeEnumMemberAtIndex ( i );
        if value & member . GetValueAsUnsigned ( ) {
        flag_set . add ( member . GetName ( ) . removeprefix ( enum_name ) );
        return flag_set;
        class ImGuiWindowSummary ( object ) ;
        pub fn __init__ (&self, valobj , internal_dict ) {
        self . valobj = valobj;
        pub fn update (&self) {
        // pass
        pub fn get_summary (&self) {
        name = self . valobj . GetChildMemberWithName ( "Name" ) . GetSummary ( );
        active = self . valobj . GetChildMemberWithName ( "Active" ) . GetValueAsUnsigned ( ) != 0;
        was_active = self . valobj . GetChildMemberWithName ( "WasActive" ) . GetValueAsUnsigned ( ) != 0;
        hidden = self . valobj . GetChildMemberWithName ( "Hidden" ) != 0;
        flags = get_active_enum_flags ( self . valobj . GetChildMemberWithName ( "Flags" ) );
        active = 1 if active || was_active else 0;
        child = 1 if "ChildWindow" in flags else 0;
        popup = 1 if "Popup" in flags else 0;
        hidden = 1 if hidden else 0;
        return f "Name {name} Active {active} Child {child} Popup {popup} Hidden {hidden}";
        pub fn __lldb_init_module ( debugger , internal_dict ) {
        "
	This function will be automatically called by LLDB when the module is loaded, here
	we register the various synthetics/summaries we have build before
	";
        category_name = "imgui";
        category = debugger . GetCategory ( category_name );
        if !category . IsValid ( ) {
        category = debugger . CreateCategory ( category_name );
        category . AddLanguage ( lldb . eLanguageTypeC_plus_plus );
        category . SetEnabled ( true );
        pub fn add_summary ( typename , impl ) {
        summary = None /* Option */;
        if isinstance ( impl , str ) {
        summary = lldb . SBTypeSummary . CreateWithSummaryString ( impl );
        summary . SetOptions ( lldb . eTypeOptionCascade );
        } else {
        summary = lldb . SBTypeSummary . CreateWithScriptCode ( f "
				synth = {impl.__module__}.{impl.__qualname__}(valobj.GetNonSyntheticValue(), internal_dict)
				synth.update()

				return synth.get_summary()
			" );
        summary . SetOptions ( lldb . eTypeOptionCascade | lldb . eTypeOptionFrontEndWantsDereference );
        category . AddTypeSummary ( lldb . SBTypeNameSpecifier ( typename , true ) , summary );
        pub fn add_synthetic ( typename , impl ) {
        add_summary ( typename , impl );
        synthetic = lldb . SBTypeSynthetic . CreateWithClassName ( f "{impl.__module__}.{impl.__qualname__}" );
        synthetic . SetOptions ( lldb . eTypeOptionCascade | lldb . eTypeOptionFrontEndWantsDereference );
        category . AddTypeSynthetic ( lldb . SBTypeNameSpecifier ( typename , true ) , synthetic );
        add_synthetic ( "^ImVector<.+>$" , ImVectorSynth );
        add_synthetic ( "^ImSpan<.+>$" , ImSpanSynth );
        add_summary ( "^ImVec2$" , "x=${var.x} y=${var.y}" );
        add_summary ( "^ImVec4$" , "x=${var.x} y=${var.y} z=${var.z} w=${var.w}" );
        add_summary ( "^ImRect$" , ImRectSummary );
        add_summary ( "^ImGuiWindow$" , ImGuiWindowSummary );
    }

}

