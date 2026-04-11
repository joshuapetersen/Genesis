//! tracemalloc.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::collections::{Sequence, Iterable};
// use crate::functools::{total_ordering};
// use crate::fnmatch;
// use std::fs;
// use crate::_tracemalloc::{};

pub fn _format_size(size: &str, sign: &str) {
        for unit in ( "B" , "KiB" , "MiB" , "GiB" , "TiB" ) .iter() {
        if abs ( size ) < 100 && unit != "B" {
        if sign {
        return  "%+.1f %s" % ( size , unit );
        } else {
        return  "%.1f %s" % ( size , unit );
        if abs ( size ) < 10 * 1024 || unit == "TiB" {
        if sign {
        return  "%+.0f %s" % ( size , unit );
        } else {
        return  "%.0f %s" % ( size , unit );
        size / = 1024;
        class Statistic ;
        "
    Statistic difference on memory allocations between two Snapshot instance.
    ";
        __slots__ = ( "traceback" , "size" , "count" );
        pub fn __init__ ( &self, traceback , size , count )  {
        self . traceback = traceback;
        self . size = size;
        self . count = count;
        pub fn __hash__ ( self )  {
        return  hash ( ( self . traceback , self . size , self . count ) );
        pub fn __eq__ ( &self, other )  {
        if !isinstance ( other , Statistic ) {
        return  NotImplemented;
        return  ( self . traceback == other . traceback;
        and self . size == other . size;
        and self . count == other . count );
        pub fn __str__ ( self )  {
        text = ( "%s: size=%s, count=%i";
        % ( self . traceback ,;
        _format_size ( self . size , false ) ,;
        self . count ) );
        if self . count {
        average = self . size / self . count;
        text + = ", average=%s" % _format_size ( average , false );
        return  text;
        pub fn __repr__ ( self )  {
        return  ( "<Statistic traceback=%r size=%i count=%i>";
        % ( self . traceback , self . size , self . count ) );
        pub fn _sort_key ( self )  {
        return  ( self . size , self . count , self . traceback );
        class StatisticDiff ;
        "
    Statistic difference on memory allocations between an old && a new
    Snapshot instance.
    ";
        __slots__ = ( "traceback" , "size" , "size_difformat!(" , "count" , "count_difformat!(" ));
        pub fn __init__ ( &self, traceback , size , size_diff , count , count_diff )  {
        self . traceback = traceback;
        self . size = size;
        self . size_diff = size_diff;
        self . count = count;
        self . count_diff = count_diff;
        pub fn __hash__ ( self )  {
        return  hash ( ( self . traceback , self . size , self . size_diff ,;
        self . count , self . count_diff ) );
        pub fn __eq__ ( &self, other )  {
        if !isinstance ( other , StatisticDiff ) {
        return  NotImplemented;
        return  ( self . traceback == other . traceback;
        and self . size == other . size;
        and self . size_diff == other . size_diff;
        and self . count == other . count;
        and self . count_diff == other . count_diff );
        pub fn __str__ ( self )  {
        text = ( "%s: size=%s (%s), count=%i (%+i)";
        % ( self . traceback ,;
        _format_size ( self . size , false ) ,;
        _format_size ( self . size_diff , true ) ,;
        self . count ,;
        self . count_diff ) );
        if self . count {
        average = self . size / self . count;
        text + = ", average=%s" % _format_size ( average , false );
        return  text;
        pub fn __repr__ ( self )  {
        return  ( "<StatisticDiff traceback=%r size=%i (%+i) count=%i (%+i)>";
        % ( self . traceback , self . size , self . size_diff ,;
        self . count , self . count_diff ) );
        pub fn _sort_key ( self )  {
        return  ( abs ( self . size_diff ) , self . size ,;
        abs ( self . count_diff ) , self . count ,;
        self . traceback );
        pub fn _compare_grouped_stats ( old_group , new_group )  {
        statistics = [ ];
        for traceback , stat in new_group . items ( ) .iter() {
        previous = old_group . pop ( traceback , None /* Option */ );
        if previous is !None /* Option */ {
        stat = StatisticDiff ( traceback ,;
        stat . size , stat . size - previous . size ,;
        stat . count , stat . count - previous . count );
        } else {
        stat = StatisticDiff ( traceback ,;
        stat . size , stat . size ,;
        stat . count , stat . count );
        statistics . append ( stat );
        for traceback , stat in old_group . items ( ) .iter() {
        stat = StatisticDiff ( traceback , 0 , - stat . size , 0 , - stat . count );
        statistics . append ( stat );
        return  statistics;
        @ total_ordering;
        class Frame ;
        "
    Frame of a traceback.
    ";
        __slots__ = ( "_frame" , );
        pub fn __init__ ( &self, frame )  {
        self . _frame = frame;
        @ property;
        pub fn filename ( self )  {
        return  self . _frame [ 0 ];
        @ property;
        pub fn lineno ( self )  {
        return  self . _frame [ 1 ];
        pub fn __eq__ ( &self, other )  {
        if !isinstance ( other , Frame ) {
        return  NotImplemented;
        return  ( self . _frame == other . _frame );
        pub fn __lt__ ( &self, other )  {
        if !isinstance ( other , Frame ) {
        return  NotImplemented;
        return  ( self . _frame < other . _frame );
        pub fn __hash__ ( self )  {
        return  hash ( self . _frame );
        pub fn __str__ ( self )  {
        return  "%s:%s" % ( self . filename , self . lineno );
        pub fn __repr__ ( self )  {
        return  "<Frame filename=%r lineno=%r>" % ( self . filename , self . lineno );
        @ total_ordering;
        class Traceback ( Sequence ) ;
        "
    Sequence of Frame instances sorted from the oldest frame
    to the most recent frame.
    ";
        __slots__ = ( "_frames" , "_total_nframe" );
        pub fn __init__ ( &self, frames , total_nframe = None /* Option */ )  {
        Sequence . __init__ ( self );
        self . _frames = tuple ( reversed ( frames ) );
        self . _total_nframe = total_nframe;
        @ property;
        pub fn total_nframe ( self )  {
        return  self . _total_nframe;
        pub fn __len__ ( self )  {
        return  len ( self . _frames );
        pub fn __getitem__ ( &self, index )  {
        if isinstance ( index , slice ) {
        return  tuple ( Frame ( trace ) for trace in self . _frames [ index ] );
        } else {
        return  Frame ( self . _frames [ index ] );
        pub fn __contains__ ( &self, frame )  {
        return  frame . _frame in self . _frames;
        pub fn __hash__ ( self )  {
        return  hash ( self . _frames );
        pub fn __eq__ ( &self, other )  {
        if !isinstance ( other , Traceback ) {
        return  NotImplemented;
        return  ( self . _frames == other . _frames );
        pub fn __lt__ ( &self, other )  {
        if !isinstance ( other , Traceback ) {
        return  NotImplemented;
        return  ( self . _frames < other . _frames );
        pub fn __str__ ( self )  {
        return  str ( self [ 0 ] );
        pub fn __repr__ ( self )  {
        s = format!("<Traceback {tuple(self)}");
        if self . _total_nframe is None /* Option */ {
        s + = ">";
        } else {
        s + = format!(" total_nframe={self.total_nframe}>");
        return  s;
        pub fn format ( &self, limit = None /* Option */ , most_recent_first = false )  {
        lines = [ ];
        if limit is !None /* Option */ {
        if limit > 0 {
        frame_slice = self [ - limit : ];
        } else {
        frame_slice = self [ : limit ];
        } else {
        frame_slice = self;
        if most_recent_first {
        frame_slice = reversed ( frame_slice );
        for frame in frame_slice .iter() {
        lines . append ( "  File "%s", line %s";
        % ( frame . filename , frame . lineno ) );
        line = linecache . getline ( frame . filename , frame . lineno ) . strip ( );
        if line {
        lines . append ( "    %s" % line );
        return  lines;
        pub fn get_object_traceback ( obj )  {
        "
    Get the traceback where the Python object *obj* was allocated.
    Return a Traceback instance.

    Return None /* Option */ if the tracemalloc module == !tracing memory allocations or
    did !trace the allocation of the object.
    ";
        frames = _get_object_traceback ( obj );
        if frames is !None /* Option */ {
        return  Traceback ( frames );
        } else {
        return;
        class Trace ;
        "
    Trace of a memory block.
    ";
        __slots__ = ( "_trace" , );
        pub fn __init__ ( &self, trace )  {
        self . _trace = trace;
        @ property;
        pub fn domain ( self )  {
        return  self . _trace [ 0 ];
        @ property;
        pub fn size ( self )  {
        return  self . _trace [ 1 ];
        @ property;
        pub fn traceback ( self )  {
        return  Traceback ( * self . _trace [ 2 : ] );
        pub fn __eq__ ( &self, other )  {
        if !isinstance ( other , Trace ) {
        return  NotImplemented;
        return  ( self . _trace == other . _trace );
        pub fn __hash__ ( self )  {
        return  hash ( self . _trace );
        pub fn __str__ ( self )  {
        return  "%s: %s" % ( self . traceback , _format_size ( self . size , false ) );
        pub fn __repr__ ( self )  {
        return  ( "<Trace domain=%s size=%s, traceback=%r>";
        % ( self . domain , _format_size ( self . size , false ) , self . traceback ) );
        class _Traces ( Sequence ) ;
        pub fn __init__ ( &self, traces )  {
        Sequence . __init__ ( self );
        self . _traces = traces;
        pub fn __len__ ( self )  {
        return  len ( self . _traces );
        pub fn __getitem__ ( &self, index )  {
        if isinstance ( index , slice ) {
        return  tuple ( Trace ( trace ) for trace in self . _traces [ index ] );
        } else {
        return  Trace ( self . _traces [ index ] );
        pub fn __contains__ ( &self, trace )  {
        return  trace . _trace in self . _traces;
        pub fn __eq__ ( &self, other )  {
        if !isinstance ( other , _Traces ) {
        return  NotImplemented;
        return  ( self . _traces == other . _traces );
        pub fn __repr__ ( self )  {
        return  "<Traces len=%s>" % len ( self );
        pub fn _normalize_filename ( filename )  {
        filename = os . path . normcase ( filename );
        if filename . endswith ( ".pyc" ) {
        filename = filename [ : -1 ];
        return  filename;
        class BaseFilter ;
        pub fn __init__ ( &self, inclusive )  {
        self . inclusive = inclusive;
        pub fn _match ( &self, trace )  {
        panic!("NotImplementedError");
        class Filter ( BaseFilter ) ;
        pub fn __init__ ( &self, inclusive , filename_pattern , {
        lineno = None /* Option */ , all_frames = false , domain = None /* Option */ ) ;
        super ( ) . __init__ ( inclusive );
        self . inclusive = inclusive;
        self . _filename_pattern = _normalize_filename ( filename_pattern );
        self . lineno = lineno;
        self . all_frames = all_frames;
        self . domain = domain;
        @ property;
        pub fn filename_pattern ( self )  {
        return  self . _filename_pattern;
        pub fn _match_frame_impl ( &self, filename , lineno )  {
        filename = _normalize_filename ( filename );
        if !fnmatch . fnmatch ( filename , self . _filename_pattern ) {
        return  false;
        if self . lineno is None /* Option */ {
        return  true;
        } else {
        return  ( lineno == self . lineno );
        pub fn _match_frame ( &self, filename , lineno )  {
        return  self . _match_frame_impl ( filename , lineno ) ^ ( !self . inclusive );
        pub fn _match_traceback ( &self, traceback )  {
        if self . all_frames {
        if any ( self . _match_frame_impl ( filename , lineno ) {
        for filename , lineno in traceback ) .iter() {
        return  self . inclusive;
        } else {
        return  ( !self . inclusive );
        } else {
        filename , lineno = traceback [ 0 ];
        return  self . _match_frame ( filename , lineno );
        pub fn _match ( &self, trace )  {
        domain , size , traceback , total_nframe = trace;
        res = self . _match_traceback ( traceback );
        if self . domain is !None /* Option */ {
        if self . inclusive {
        return  res && ( domain == self . domain );
        } else {
        return  res || ( domain != self . domain );
        return  res;
        class DomainFilter ( BaseFilter ) ;
        pub fn __init__ ( &self, inclusive , domain )  {
        super ( ) . __init__ ( inclusive );
        self . _domain = domain;
        @ property;
        pub fn domain ( self )  {
        return  self . _domain;
        pub fn _match ( &self, trace )  {
        domain , size , traceback , total_nframe = trace;
        return  ( domain == self . domain ) ^ ( !self . inclusive );
        class Snapshot ;
        "
    Snapshot of traces of memory blocks allocated by Python.
    ";
        pub fn __init__ ( &self, traces , traceback_limit )  {
        self . traces = _Traces ( traces );
        self . traceback_limit = traceback_limit;
        pub fn dump ( &self, filename )  {
        "
        Write the snapshot into a file.
        ";
        // with scope: open ( filename , "wb" ) as fp  {
        pickle . dump ( self , fp , pickle . HIGHEST_PROTOCOL );
        @ staticmethod;
        pub fn load ( filename )  {
        "
        Load a snapshot from a file.
        ";
        // with scope: open ( filename , "rb" ) as fp  {
        return  pickle . load ( fp );
        pub fn _filter_trace ( &self, include_filters , exclude_filters , trace )  {
        if include_filters {
        if !any ( trace_filter . _match ( trace ) {
        for trace_filter in include_filters ) .iter() {
        return  false;
        if exclude_filters {
        if any ( !trace_filter . _match ( trace ) {
        for trace_filter in exclude_filters ) .iter() {
        return  false;
        return  true;
        pub fn filter_traces ( &self, filters )  {
        "
        Create a new Snapshot instance with a filtered traces sequence, filters
        == a list of Filter || DomainFilter instances.  If filters == an empty
        list, return a new Snapshot instance with a copy of the traces.
        ";
        if !isinstance ( filters , Iterable ) {
        panic!("TypeError ( "filters must be a list of filters, !%s"");
        % type ( filters ) . __name__ );
        if filters {
        include_filters = [ ];
        exclude_filters = [ ];
        for trace_filter in filters .iter() {
        if trace_filter . inclusive {
        include_filters . append ( trace_filter );
        } else {
        exclude_filters . append ( trace_filter );
        new_traces = vec![ trace.iter().map(|trace| self . traces . _traces;
        if self . _filter_trace ( include_filters , {
        exclude_filters ,;
        trace ) ];
        } else {
        new_traces = self . traces . _traces . copy ( );
        return  Snapshot ( new_traces , self . traceback_limit );
        pub fn _group_by ( &self, key_type , cumulative )  {
        if key_type !in ( "traceback" , "filename" , "lineno" ) {
        panic!("ValueError ( "unknown key_type: %r" % ( key_type , ) )");
        if cumulative && key_type !in ( "lineno" , "filename" ) {
        panic!("ValueError ( "cumulative mode cannot by used "");
        "with key type %r" % key_type );
        stats = { };
        tracebacks = { };
        if !cumulative {
        for trace in self . traces . _traces .iter() {
        domain , size , trace_traceback , total_nframe = trace;
        // try {
        traceback = tracebacks [ trace_traceback ];
        // } catch  KeyError  {
        if key_type == "traceback" {
        frames = trace_traceback;
        } else if key_type == "lineno" {
        frames = trace_traceback [ : 1 ];
        } else {
        frames = ( ( trace_traceback [ 0 ] [ 0 ] , 0 ) , );
        traceback = Traceback ( frames );
        tracebacks [ trace_traceback ] = traceback;
        // try {
        stat = stats [ traceback ];
        stat . size + = size;
        stat . count + = 1;
        // } catch  KeyError  {
        stats [ traceback ] = Statistic ( traceback , size , 1 );
        } else {
        for trace in self . traces . _traces .iter() {
        domain , size , trace_traceback , total_nframe = trace;
        for frame in trace_traceback .iter() {
        // try {
        traceback = tracebacks [ frame ];
        // } catch  KeyError  {
        if key_type == "lineno" {
        frames = ( frame , );
        } else {
        frames = ( ( frame [ 0 ] , 0 ) , );
        traceback = Traceback ( frames );
        tracebacks [ frame ] = traceback;
        // try {
        stat = stats [ traceback ];
        stat . size + = size;
        stat . count + = 1;
        // } catch  KeyError  {
        stats [ traceback ] = Statistic ( traceback , size , 1 );
        return  stats;
        pub fn statistics ( &self, key_type , cumulative = false )  {
        "
        Group statistics by key_type. Return a sorted list of Statistic
        instances.
        ";
        grouped = self . _group_by ( key_type , cumulative );
        statistics = list ( grouped . values ( ) );
        statistics . sort ( reverse = true , key = Statistic . _sort_key );
        return  statistics;
        pub fn compare_to ( &self, old_snapshot , key_type , cumulative = false )  {
        "
        Compute the differences with an old snapshot old_snapshot. Get
        statistics as a sorted list of StatisticDiff instances, grouped by
        group_by.
        ";
        new_group = self . _group_by ( key_type , cumulative );
        old_group = old_snapshot . _group_by ( key_type , cumulative );
        statistics = _compare_grouped_stats ( old_group , new_group );
        statistics . sort ( reverse = true , key = StatisticDiff . _sort_key );
        return  statistics;
        pub fn take_snapshot ( )  {
        "
    Take a snapshot of traces of memory blocks allocated by Python.
    ";
        if !is_tracing ( ) {
        panic!("RuntimeError ( "the tracemalloc module must be tracing memory "");
        "allocations to take a snapshot" );
        traces = _get_traces ( );
        traceback_limit = get_traceback_limit ( );
        return  Snapshot ( traces , traceback_limit );
}

