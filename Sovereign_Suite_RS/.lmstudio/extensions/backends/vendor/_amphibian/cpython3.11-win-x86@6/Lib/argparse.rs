//! argparse.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use std::env;
// use crate::warnings;
// use crate::gettext::{gettext, _, ngettext};
// use crate::copy;
// use crate::shutil;
// use crate::textwrap;

pub const __version__: &str = "1.1";
pub const __all__: f64 = [;
pub const SUPPRESS: &str = "==SUPPRESS==";
pub const OPTIONAL: &str = "?";
pub const ZERO_OR_MORE: &str = "*";
pub const ONE_OR_MORE: &str = "+";
pub const PARSER: &str = "A...";
pub const REMAINDER: &str = "...";
pub const _UNRECOGNIZED_ARGS_ATTR: &str = "_unrecognized_args";
pub struct _AttributeHolder {
    pub _prog: String, // TODO: infer type
    pub _indent_increment: String, // TODO: infer type
    pub _max_help_position: String, // TODO: infer type
    pub _width: String, // TODO: infer type
    pub _current_indent: String, // TODO: infer type
    pub _level: String, // TODO: infer type
    pub _action_max_length: String, // TODO: infer type
    pub _root_section: String, // TODO: infer type
    pub _current_section: String, // TODO: infer type
    pub _whitespace_matcher: String, // TODO: infer type
    pub _long_break_matcher: String, // TODO: infer type
    pub formatter: String, // TODO: infer type
    pub parent: String, // TODO: infer type
    pub heading: String, // TODO: infer type
    pub items: String, // TODO: infer type
    pub argument_name: String, // TODO: infer type
    pub message: String, // TODO: infer type
    pub option_strings: String, // TODO: infer type
    pub dest: String, // TODO: infer type
    pub nargs: String, // TODO: infer type
    pub const: String, // TODO: infer type
    pub default: String, // TODO: infer type
    pub type: String, // TODO: infer type
    pub choices: String, // TODO: infer type
    pub required: String, // TODO: infer type
    pub help: String, // TODO: infer type
    pub metavar: String, // TODO: infer type
    pub version: String, // TODO: infer type
    pub _prog_prefix: String, // TODO: infer type
    pub _parser_class: String, // TODO: infer type
    pub _name_parser_map: String, // TODO: infer type
    pub _choices_actions: String, // TODO: infer type
    pub _mode: String, // TODO: infer type
    pub _bufsize: String, // TODO: infer type
    pub _encoding: String, // TODO: infer type
    pub _errors: String, // TODO: infer type
    pub description: String, // TODO: infer type
    pub argument_default: String, // TODO: infer type
    pub prefix_chars: String, // TODO: infer type
    pub conflict_handler: String, // TODO: infer type
    pub _registries: String, // TODO: infer type
    pub _actions: String, // TODO: infer type
    pub _option_string_actions: String, // TODO: infer type
    pub _action_groups: String, // TODO: infer type
    pub _mutually_exclusive_groups: String, // TODO: infer type
    pub _defaults: String, // TODO: infer type
    pub _negative_number_matcher: String, // TODO: infer type
    pub _has_negative_number_optionals: String, // TODO: infer type
    pub title: String, // TODO: infer type
    pub _group_actions: String, // TODO: infer type
    pub _container: String, // TODO: infer type
    pub prog: String, // TODO: infer type
    pub usage: String, // TODO: infer type
    pub epilog: String, // TODO: infer type
    pub formatter_class: String, // TODO: infer type
    pub fromfile_prefix_chars: String, // TODO: infer type
    pub add_help: String, // TODO: infer type
    pub allow_abbrev: String, // TODO: infer type
    pub exit_on_error: String, // TODO: infer type
    pub _positionals: String, // TODO: infer type
    pub _optionals: String, // TODO: infer type
    pub _subparsers: String, // TODO: infer type
}

impl _AttributeHolder {
}

pub fn _copy_items(items: &str) {
        if items is None /* Option */ {
        return  [ ];
        if type ( items ) is list {
        return  items [ : ];
        import copy;
        return  copy . copy ( items );
        class HelpFormatter ( object ) ;
        "Formatter for generating usage messages && argument help strings.

    Only the name of this class == considered a public API. All the methods
    provided by the class are considered an implementation detail.
    ";
        pub fn __init__ ( &self, {
        prog ,;
        indent_increment = 2 ,;
        max_help_position = 24 ,;
        width = None /* Option */ ) ;
        if width is None /* Option */ {
        import shutil;
        width = shutil . get_terminal_size ( ) . columns;
        width - = 2;
        self . _prog = prog;
        self . _indent_increment = indent_increment;
        self . _max_help_position = min ( max_help_position ,;
        max ( width - 20 , indent_increment * 2 ) );
        self . _width = width;
        self . _current_indent = 0;
        self . _level = 0;
        self . _action_max_length = 0;
        self . _root_section = self . _Section ( self , None /* Option */ );
        self . _current_section = self . _root_section;
        self . _whitespace_matcher = _re . compile ( r "\s+" , _re . ASCII );
        self . _long_break_matcher = _re . compile ( r "\n\n\n+" );
        pub fn _indent ( self )  {
        self . _current_indent + = self . _indent_increment;
        self . _level + = 1;
        pub fn _dedent ( self )  {
        self . _current_indent - = self . _indent_increment;
        assert self . _current_indent >= 0 , "Indent decreased below 0.";
        self . _level - = 1;
        class _Section ( object ) ;
        pub fn __init__ ( &self, formatter , parent , heading = None /* Option */ )  {
        self . formatter = formatter;
        self . parent = parent;
        self . heading = heading;
        self . items = [ ];
        pub fn format_help ( self )  {
        if self . parent is !None /* Option */ {
        self . formatter . _indent ( );
        join = self . formatter . _join_parts;
        item_help = join ( vec![ func ( * args ).iter().map(|func , args| self . items ] );
        if self . parent is !None /* Option */ {
        self . formatter . _dedent ( );
        if !item_help {
        return  "";
        if self . heading is !SUPPRESS && self . heading is !None /* Option */ {
        current_indent = self . formatter . _current_indent;
        heading_text = _ ( "%(heading)s:" ) % dict ( heading = self . heading );
        heading = "%*s%s\n" % ( current_indent , "" , heading_text );
        } else {
        heading = "";
        return  join ( [ "\n" , heading , item_help , "\n" ] );
        pub fn _add_item ( &self, func , args )  {
        self . _current_section . items . append ( ( func , args ) );
        pub fn start_section ( &self, heading )  {
        self . _indent ( );
        section = self . _Section ( self , self . _current_section , heading );
        self . _add_item ( section . format_help , [ ] );
        self . _current_section = section;
        pub fn end_section ( self )  {
        self . _current_section = self . _current_section . parent;
        self . _dedent ( );
        pub fn add_text ( &self, text )  {
        if text is !SUPPRESS && text is !None /* Option */ {
        self . _add_item ( self . _format_text , [ text ] );
        pub fn add_usage ( &self, usage , actions , groups , prefix = None /* Option */ )  {
        if usage is !SUPPRESS {
        args = usage , actions , groups , prefix;
        self . _add_item ( self . _format_usage , args );
        pub fn add_argument ( &self, action )  {
        if action . help is !SUPPRESS {
        get_invocation = self . _format_action_invocation;
        invocations = [ get_invocation ( action ) ];
        for subaction in self . _iter_indented_subactions ( action ) .iter() {
        invocations . append ( get_invocation ( subaction ) );
        invocation_length = max ( map ( len , invocations ) );
        action_length = invocation_length + self . _current_indent;
        self . _action_max_length = max ( self . _action_max_length ,;
        action_length );
        self . _add_item ( self . _format_action , [ action ] );
        pub fn add_arguments ( &self, actions )  {
        for action in actions .iter() {
        self . add_argument ( action );
        pub fn format_help ( self )  {
        help = self . _root_section . format_help ( );
        if help {
        help = self . _long_break_matcher . sub ( "\n\n" , help );
        help = help . strip ( "\n" ) + "\n";
        return  help;
        pub fn _join_parts ( &self, part_strings )  {
        return  "" . join ( [ part;
        for part in part_strings.iter() {
        if part && part is !SUPPRESS ] ) {
        pub fn _format_usage ( &self, usage , actions , groups , prefix )  {
        if prefix is None /* Option */ {
        prefix = _ ( "usage: " );
        if usage is !None /* Option */ {
        usage = usage % dict ( prog = self . _prog );
        } else if usage is None /* Option */ && !actions {
        usage = "%(prog)s" % dict ( prog = self . _prog );
        } else if usage is None /* Option */ {
        prog = "%(prog)s" % dict ( prog = self . _prog );
        optionals = [ ];
        positionals = [ ];
        for action in actions .iter() {
        if action . option_strings {
        optionals . append ( action );
        } else {
        positionals . append ( action );
        format = self . _format_actions_usage;
        action_usage = format ( optionals + positionals , groups );
        usage = " " . join ( vec![ s.iter().map(|s| vec![ prog , action_usage ] if s ] );
        text_width = self . _width - self . _current_indent;
        if len ( prefix ) + len ( usage ) > text_width {
        part_regexp = (;
        r "\(.*?\)+(?=\s|$)|";
        r "\[.*?\]+(?=\s|$)|";
        r "\S+";
        );
        opt_usage = format ( optionals , groups );
        pos_usage = format ( positionals , groups );
        opt_parts = _re . findall ( part_regexp , opt_usage );
        pos_parts = _re . findall ( part_regexp , pos_usage );
        assert " " . join ( opt_parts ) == opt_usage;
        assert " " . join ( pos_parts ) == pos_usage;
        pub fn get_lines ( parts , indent , prefix = None /* Option */ )  {
        lines = [ ];
        line = [ ];
        if prefix is !None /* Option */ {
        line_len = len ( prefix ) - 1;
        } else {
        line_len = len ( indent ) - 1;
        for part in parts .iter() {
        if line_len + 1 + len ( part ) > text_width && line {
        lines . append ( indent + " " . join ( line ) );
        line = [ ];
        line_len = len ( indent ) - 1;
        line . append ( part );
        line_len + = len ( part ) + 1;
        if line {
        lines . append ( indent + " " . join ( line ) );
        if prefix is !None /* Option */ {
        lines [ 0 ] = lines [ 0 ] [ len ( indent ) : ];
        return  lines;
        if len ( prefix ) + len ( prog ) <= 0.75 * text_width {
        indent = " " * ( len ( prefix ) + len ( prog ) + 1 );
        if opt_parts {
        lines = get_lines ( [ prog ] + opt_parts , indent , prefix );
        lines . extend ( get_lines ( pos_parts , indent ) );
        } else if pos_parts {
        lines = get_lines ( [ prog ] + pos_parts , indent , prefix );
        } else {
        lines = [ prog ];
        } else {
        indent = " " * len ( prefix );
        parts = opt_parts + pos_parts;
        lines = get_lines ( parts , indent );
        if len ( lines ) > 1 {
        lines = [ ];
        lines . extend ( get_lines ( opt_parts , indent ) );
        lines . extend ( get_lines ( pos_parts , indent ) );
        lines = [ prog ] + lines;
        usage = "\n" . join ( lines );
        return  "%s%s\n\n" % ( prefix , usage );
        pub fn _format_actions_usage ( &self, actions , groups )  {
        group_actions = set ( );
        inserts = { };
        for group in groups .iter() {
        if !group . _group_actions {
        panic!("ValueError ( f "empty group {group}" )");
        // try {
        start = actions . index ( group . _group_actions [ 0 ] );
        // } catch  ValueError  {
        continue;
        } else {
        group_action_count = len ( group . _group_actions );
        end = start + group_action_count;
        if actions [ start { : end ] == group . _group_actions ; }
        suppressed_actions_count = 0;
        for action in group . _group_actions .iter() {
        group_actions . add ( action );
        if action . help is SUPPRESS {
        suppressed_actions_count + = 1;
        exposed_actions_count = group_action_count - suppressed_actions_count;
        if !exposed_actions_count {
        continue;
        if !group . required {
        if start in inserts {
        inserts [ start ] + = " [";
        } else {
        inserts [ start ] = "[";
        if end in inserts {
        inserts [ end ] + = "]";
        } else {
        inserts [ end ] = "]";
        } else if exposed_actions_count > 1 {
        if start in inserts {
        inserts [ start ] + = " (";
        } else {
        inserts [ start ] = "(";
        if end in inserts {
        inserts [ end ] + = ")";
        } else {
        inserts [ end ] = ")";
        for i in range ( start + 1 , end ) .iter() {
        inserts [ i ] = "|";
        parts = [ ];
        for i , action in enumerate ( actions ) .iter() {
        if action . help is SUPPRESS {
        parts . append ( None /* Option */ );
        if inserts . get ( i ) == "|" {
        inserts . pop ( i );
        } else if inserts . get ( i + 1 ) == "|" {
        inserts . pop ( i + 1 );
        } else if !action . option_strings {
        default = self . _get_default_metavar_for_positional ( action );
        part = self . _format_args ( action , default );
        if action in group_actions {
        if part [ 0 ] == "[" && part [ -1 ] == "]" {
        part = part [ 1 : -1 ];
        parts . append ( part );
        } else {
        option_string = action . option_strings [ 0 ];
        if action . nargs == 0 {
        part = action . format_usage ( );
        } else {
        default = self . _get_default_metavar_for_optional ( action );
        args_string = self . _format_args ( action , default );
        part = "%s %s" % ( option_string , args_string );
        if !action . required && action !in group_actions {
        part = "[%s]" % part;
        parts . append ( part );
        for i in sorted ( inserts , reverse = true ) .iter() {
        parts [ i : i ] = [ inserts [ i ] ];
        text = " " . join ( vec![ item.iter().map(|item| parts if item == !None /* Option */ ] );
        open = r "[\[(]";
        close = r "[\])]";
        text = _re . sub ( r "(%s) " % open , r "\1" , text );
        text = _re . sub ( r " (%s)" % close , r "\1" , text );
        text = _re . sub ( r "%s *%s" % ( open , close ) , r "" , text );
        text = text . strip ( );
        return  text;
        pub fn _format_text ( &self, text )  {
        if "%(prog)" in text {
        text = text % dict ( prog = self . _prog );
        text_width = max ( self . _width - self . _current_indent , 11 );
        indent = " " * self . _current_indent;
        return  self . _fill_text ( text , text_width , indent ) + "\n\n";
        pub fn _format_action ( &self, action )  {
        help_position = min ( self . _action_max_length + 2 ,;
        self . _max_help_position );
        help_width = max ( self . _width - help_position , 11 );
        action_width = help_position - self . _current_indent - 2;
        action_header = self . _format_action_invocation ( action );
        if !action . help {
        tup = self . _current_indent , "" , action_header;
        action_header = "%*s%s\n" % tup;
        } else if len ( action_header ) <= action_width {
        tup = self . _current_indent , "" , action_width , action_header;
        action_header = "%*s%-*s  " % tup;
        indent_first = 0;
        } else {
        tup = self . _current_indent , "" , action_header;
        action_header = "%*s%s\n" % tup;
        indent_first = help_position;
        parts = [ action_header ];
        if action . help && action . help . strip ( ) {
        help_text = self . _expand_help ( action );
        if help_text {
        help_lines = self . _split_lines ( help_text , help_width );
        parts . append ( "%*s%s\n" % ( indent_first , "" , help_lines [ 0 ] ) );
        for line in help_lines [ 1 : ] .iter() {
        parts . append ( "%*s%s\n" % ( help_position , "" , line ) );
        } else if !action_header . endswith ( "\n" ) {
        parts . append ( "\n" );
        for subaction in self . _iter_indented_subactions ( action ) .iter() {
        parts . append ( self . _format_action ( subaction ) );
        return  self . _join_parts ( parts );
        pub fn _format_action_invocation ( &self, action )  {
        if !action . option_strings {
        default = self . _get_default_metavar_for_positional ( action );
        metavar , = self . _metavar_formatter ( action , default ) ( 1 );
        return  metavar;
        } else {
        parts = [ ];
        if action . nargs == 0 {
        parts . extend ( action . option_strings );
        } else {
        default = self . _get_default_metavar_for_optional ( action );
        args_string = self . _format_args ( action , default );
        for option_string in action . option_strings .iter() {
        parts . append ( "%s %s" % ( option_string , args_string ) );
        return  ", " . join ( parts );
        pub fn _metavar_formatter ( &self, action , default_metavar )  {
        if action . metavar is !None /* Option */ {
        result = action . metavar;
        } else if action . choices is !None /* Option */ {
        choice_strs = vec![ str ( choice ).iter().map(|choice| action . choices ).collect();
        result = "{%s}" % "," . join ( choice_strs );
        } else {
        result = default_metavar;
        pub fn format ( tuple_size )  {
        if isinstance ( result , tuple ) {
        return  result;
        } else {
        return  ( result , ) * tuple_size;
        return  format;
        pub fn _format_args ( &self, action , default_metavar )  {
        get_metavar = self . _metavar_formatter ( action , default_metavar );
        if action . nargs is None /* Option */ {
        result = "%s" % get_metavar ( 1 );
        } else if action . nargs == OPTIONAL {
        result = "[%s]" % get_metavar ( 1 );
        } else if action . nargs == ZERO_OR_MORE {
        metavar = get_metavar ( 1 );
        if len ( metavar ) == 2 {
        result = "[%s [%s ...]]" % metavar;
        } else {
        result = "[%s ...]" % metavar;
        } else if action . nargs == ONE_OR_MORE {
        result = "%s [%s ...]" % get_metavar ( 2 );
        } else if action . nargs == REMAINDER {
        result = "...";
        } else if action . nargs == PARSER {
        result = "%s ..." % get_metavar ( 1 );
        } else if action . nargs == SUPPRESS {
        result = "";
        } else {
        // try {
        formats = vec![ "%s".iter().map(|_| range ( action . nargs ) ).collect();
        // } catch  TypeError  {
        panic!("ValueError ( "invalid nargs value" ) from None /* Option */");
        result = " " . join ( formats ) % get_metavar ( action . nargs );
        return  result;
        pub fn _expand_help ( &self, action )  {
        params = dict ( vars ( action ) , prog = self . _prog );
        for name in list ( params ) .iter() {
        if params [ name ] is SUPPRESS {
        del params [ name ];
        for name in list ( params ) .iter() {
        if hasattr ( params [ name ] , "__name__" ) {
        params [ name ] = params [ name ] . __name__;
        if params . get ( "choices" ) is !None /* Option */ {
        choices_str = ", " . join ( vec![ str ( c ).iter().map(|c| params vec![ "choices" ] ] );
        params [ "choices" ] = choices_str;
        return  self . _get_help_string ( action ) % params;
        pub fn _iter_indented_subactions ( &self, action )  {
        // try {
        get_subactions = action . _get_subactions;
        // } catch  AttributeError  {
        // pass
        } else {
        self . _indent ( );
        yield from get_subactions ( );
        self . _dedent ( );
        pub fn _split_lines ( &self, text , width )  {
        text = self . _whitespace_matcher . sub ( " " , text ) . strip ( );
        import textwrap;
        return  textwrap . wrap ( text , width );
        pub fn _fill_text ( &self, text , width , indent )  {
        text = self . _whitespace_matcher . sub ( " " , text ) . strip ( );
        import textwrap;
        return  textwrap . fill ( text , width ,;
        initial_indent = indent ,;
        subsequent_indent = indent );
        pub fn _get_help_string ( &self, action )  {
        return  action . help;
        pub fn _get_default_metavar_for_optional ( &self, action )  {
        return  action . dest . upper ( );
        pub fn _get_default_metavar_for_positional ( &self, action )  {
        return  action . dest;
        class RawDescriptionHelpFormatter ( HelpFormatter ) ;
        "Help message formatter which retains any formatting in descriptions.

    Only the name of this class == considered a public API. All the methods
    provided by the class are considered an implementation detail.
    ";
        pub fn _fill_text ( &self, text , width , indent )  {
        return  "" . join ( indent + line for line in text . splitlines ( keepends = true ) );
        class RawTextHelpFormatter ( RawDescriptionHelpFormatter ) ;
        "Help message formatter which retains formatting of all help text.

    Only the name of this class == considered a public API. All the methods
    provided by the class are considered an implementation detail.
    ";
        pub fn _split_lines ( &self, text , width )  {
        return  text . splitlines ( );
        class ArgumentDefaultsHelpFormatter ( HelpFormatter ) ;
        "Help message formatter which adds default values to argument help.

    Only the name of this class == considered a public API. All the methods
    provided by the class are considered an implementation detail.
    ";
        pub fn _get_help_string ( &self, action )  {
        "
        Add the default value to the option help message.

        ArgumentDefaultsHelpFormatter && BooleanOptionalAction when it isn't
        already present. This code will do that, detecting cornercases to
        prevent duplicates || cases where it wouldn't make sense to the end
        user.
        ";
        help = action . help;
        if help is None /* Option */ {
        help = "";
        if "%(default)" !in help {
        if action . default is !SUPPRESS {
        defaulting_nargs = [ OPTIONAL , ZERO_OR_MORE ];
        if action . option_strings || action . nargs in defaulting_nargs {
        help + = _ ( " (default: %(default)s)" );
        return  help;
        class MetavarTypeHelpFormatter ( HelpFormatter ) ;
        "Help message formatter which uses the argument 'type' as the default
    metavar value (instead of the argument 'dest')

    Only the name of this class == considered a public API. All the methods
    provided by the class are considered an implementation detail.
    ";
        pub fn _get_default_metavar_for_optional ( &self, action )  {
        return  action . type . __name__;
        pub fn _get_default_metavar_for_positional ( &self, action )  {
        return  action . type . __name__;
        pub fn _get_action_name ( argument )  {
        if argument is None /* Option */ {
        return;
        } else if argument . option_strings {
        return  "/" . join ( argument . option_strings );
        } else if argument . metavar !in ( None /* Option */ , SUPPRESS ) {
        return  argument . metavar;
        } else if argument . dest !in ( None /* Option */ , SUPPRESS ) {
        return  argument . dest;
        } else if argument . choices {
        return  "{" + "," . join ( argument . choices ) + "}";
        } else {
        return;
        class ArgumentError ( Exception ) ;
        "An error from creating || using an argument (optional || positional).

    The string value of this exception == the message, augmented with
    information about the argument that caused it.
    ";
        pub fn __init__ ( &self, argument , message )  {
        self . argument_name = _get_action_name ( argument );
        self . message = message;
        pub fn __str__ ( self )  {
        if self . argument_name is None /* Option */ {
        format = "%(message)s";
        } else {
        format = _ ( "argument %(argument_name)s: %(message)s" );
        return  format % dict ( message = self . message ,;
        argument_name = self . argument_name );
        class ArgumentTypeError ( Exception ) ;
        "An error from trying to convert a command line string to a type.";
        // pass
        class Action ( _AttributeHolder ) ;
        "Information about how to convert command line strings to Python objects.

    Action objects are used by an ArgumentParser to represent the information
    needed to parse a single argument from one || more strings from the
    command line. The keyword arguments to the Action constructor are also
    all attributes of Action instances.

    Keyword Arguments:

        - option_strings -- A list of command-line option strings which
            should be associated with this action.

        - dest -- The name of the attribute to hold the created object(s)

        - nargs -- The number of command-line arguments that should be
            consumed. By default, one argument will be consumed && a single
            value will be produced.  Other values include:
                - N (an integer) consumes N arguments (and produces a list)
                - '?' consumes zero || one arguments
                - '*' consumes zero || more arguments (and produces a list)
                - '+' consumes one || more arguments (and produces a list)
            Note that the difference between the default && nargs=1 == that
            with the default, a single value will be produced, while with
            nargs=1, a list containing a single value will be produced.

        - const -- The value to be produced if the option == specified && the
            option uses an action that takes no values.

        - default -- The value to be produced if the option == !specified.

        - type -- A callable that accepts a single string argument, and
            returns the converted value.  The standard Python types str, int,
            float, && complex are useful examples of such callables.  If None /* Option */,
            str == used.

        - choices -- A container of values that should be allowed. If !None /* Option */,
            after a command-line argument has been converted to the appropriate
            type, an exception will be raised if it == !a member of this
            collection.

        - required -- true if the action must always be specified at the
            command line. This == only meaningful for optional command-line
            arguments.

        - help -- The help string describing the argument.

        - metavar -- The name to be used for the option's argument with the
            help string. If None /* Option */, the 'dest' value will be used as the name.
    ";
        pub fn __init__ ( &self, {
        option_strings ,;
        dest ,;
        nargs = None /* Option */ ,;
        const = None /* Option */ ,;
        default = None /* Option */ ,;
        type = None /* Option */ ,;
        choices = None /* Option */ ,;
        required = false ,;
        help = None /* Option */ ,;
        metavar = None /* Option */ ) ;
        self . option_strings = option_strings;
        self . dest = dest;
        self . nargs = nargs;
        self . const = const;
        self . default = default;
        self . type = type;
        self . choices = choices;
        self . required = required;
        self . help = help;
        self . metavar = metavar;
        pub fn _get_kwargs ( self )  {
        names = [;
        "option_strings" ,;
        "dest" ,;
        "nargs" ,;
        "const" ,;
        "default" ,;
        "type" ,;
        "choices" ,;
        "required" ,;
        "help" ,;
        "metavar" ,;
        ];
        return  [ ( name , getattr ( self , name ) ) for name in names ];
        pub fn format_usage ( self )  {
        return  self . option_strings [ 0 ];
        pub fn __call__ ( &self, parser , namespace , values , option_string = None /* Option */ )  {
        panic!("NotImplementedError ( _ ( ".__call__() !defined" ) )");
        class BooleanOptionalAction ( Action ) ;
        pub fn __init__ ( &self, {
        option_strings ,;
        dest ,;
        default = None /* Option */ ,;
        type = None /* Option */ ,;
        choices = None /* Option */ ,;
        required = false ,;
        help = None /* Option */ ,;
        metavar = None /* Option */ ) ;
        _option_strings = [ ];
        for option_string in option_strings .iter() {
        _option_strings . append ( option_string );
        if option_string . startswith ( "--" ) {
        option_string = "--no-" + option_string [ 2 : ];
        _option_strings . append ( option_string );
        super ( ) . __init__ (;
        option_strings = _option_strings ,;
        dest = dest ,;
        nargs = 0 ,;
        default = default ,;
        type = type ,;
        choices = choices ,;
        required = required ,;
        help = help ,;
        metavar = metavar );
        pub fn __call__ ( &self, parser , namespace , values , option_string = None /* Option */ )  {
        if option_string in self . option_strings {
        setattr ( namespace , self . dest , !option_string . startswith ( "--no-" ) );
        pub fn format_usage ( self )  {
        return  " | " . join ( self . option_strings );
        class _StoreAction ( Action ) ;
        pub fn __init__ ( &self, {
        option_strings ,;
        dest ,;
        nargs = None /* Option */ ,;
        const = None /* Option */ ,;
        default = None /* Option */ ,;
        type = None /* Option */ ,;
        choices = None /* Option */ ,;
        required = false ,;
        help = None /* Option */ ,;
        metavar = None /* Option */ ) ;
        if nargs == 0 {
        panic!("ValueError ( "nargs for store actions must be != 0; if you "");
        "have nothing to store, actions such as store ";
        "true || store const may be more appropriate" );
        if const is !None /* Option */ && nargs != OPTIONAL {
        panic!("ValueError ( "nargs must be %r to supply const" % OPTIONAL )");
        super ( _StoreAction , self ) . __init__ (;
        option_strings = option_strings ,;
        dest = dest ,;
        nargs = nargs ,;
        const = const ,;
        default = default ,;
        type = type ,;
        choices = choices ,;
        required = required ,;
        help = help ,;
        metavar = metavar );
        pub fn __call__ ( &self, parser , namespace , values , option_string = None /* Option */ )  {
        setattr ( namespace , self . dest , values );
        class _StoreConstAction ( Action ) ;
        pub fn __init__ ( &self, {
        option_strings ,;
        dest ,;
        const = None /* Option */ ,;
        default = None /* Option */ ,;
        required = false ,;
        help = None /* Option */ ,;
        metavar = None /* Option */ ) ;
        super ( _StoreConstAction , self ) . __init__ (;
        option_strings = option_strings ,;
        dest = dest ,;
        nargs = 0 ,;
        const = const ,;
        default = default ,;
        required = required ,;
        help = help );
        pub fn __call__ ( &self, parser , namespace , values , option_string = None /* Option */ )  {
        setattr ( namespace , self . dest , self . const );
        class _StoretrueAction ( _StoreConstAction ) ;
        pub fn __init__ ( &self, {
        option_strings ,;
        dest ,;
        default = false ,;
        required = false ,;
        help = None /* Option */ ) ;
        super ( _StoretrueAction , self ) . __init__ (;
        option_strings = option_strings ,;
        dest = dest ,;
        const = true ,;
        default = default ,;
        required = required ,;
        help = help );
        class _StorefalseAction ( _StoreConstAction ) ;
        pub fn __init__ ( &self, {
        option_strings ,;
        dest ,;
        default = true ,;
        required = false ,;
        help = None /* Option */ ) ;
        super ( _StorefalseAction , self ) . __init__ (;
        option_strings = option_strings ,;
        dest = dest ,;
        const = false ,;
        default = default ,;
        required = required ,;
        help = help );
        class _AppendAction ( Action ) ;
        pub fn __init__ ( &self, {
        option_strings ,;
        dest ,;
        nargs = None /* Option */ ,;
        const = None /* Option */ ,;
        default = None /* Option */ ,;
        type = None /* Option */ ,;
        choices = None /* Option */ ,;
        required = false ,;
        help = None /* Option */ ,;
        metavar = None /* Option */ ) ;
        if nargs == 0 {
        panic!("ValueError ( "nargs for append actions must be != 0; if arg "");
        "strings are !supplying the value to append, ";
        "the append const action may be more appropriate" );
        if const is !None /* Option */ && nargs != OPTIONAL {
        panic!("ValueError ( "nargs must be %r to supply const" % OPTIONAL )");
        super ( _AppendAction , self ) . __init__ (;
        option_strings = option_strings ,;
        dest = dest ,;
        nargs = nargs ,;
        const = const ,;
        default = default ,;
        type = type ,;
        choices = choices ,;
        required = required ,;
        help = help ,;
        metavar = metavar );
        pub fn __call__ ( &self, parser , namespace , values , option_string = None /* Option */ )  {
        items = getattr ( namespace , self . dest , None /* Option */ );
        items = _copy_items ( items );
        items . append ( values );
        setattr ( namespace , self . dest , items );
        class _AppendConstAction ( Action ) ;
        pub fn __init__ ( &self, {
        option_strings ,;
        dest ,;
        const = None /* Option */ ,;
        default = None /* Option */ ,;
        required = false ,;
        help = None /* Option */ ,;
        metavar = None /* Option */ ) ;
        super ( _AppendConstAction , self ) . __init__ (;
        option_strings = option_strings ,;
        dest = dest ,;
        nargs = 0 ,;
        const = const ,;
        default = default ,;
        required = required ,;
        help = help ,;
        metavar = metavar );
        pub fn __call__ ( &self, parser , namespace , values , option_string = None /* Option */ )  {
        items = getattr ( namespace , self . dest , None /* Option */ );
        items = _copy_items ( items );
        items . append ( self . const );
        setattr ( namespace , self . dest , items );
        class _CountAction ( Action ) ;
        pub fn __init__ ( &self, {
        option_strings ,;
        dest ,;
        default = None /* Option */ ,;
        required = false ,;
        help = None /* Option */ ) ;
        super ( _CountAction , self ) . __init__ (;
        option_strings = option_strings ,;
        dest = dest ,;
        nargs = 0 ,;
        default = default ,;
        required = required ,;
        help = help );
        pub fn __call__ ( &self, parser , namespace , values , option_string = None /* Option */ )  {
        count = getattr ( namespace , self . dest , None /* Option */ );
        if count is None /* Option */ {
        count = 0;
        setattr ( namespace , self . dest , count + 1 );
        class _HelpAction ( Action ) ;
        pub fn __init__ ( &self, {
        option_strings ,;
        dest = SUPPRESS ,;
        default = SUPPRESS ,;
        help = None /* Option */ ) ;
        super ( _HelpAction , self ) . __init__ (;
        option_strings = option_strings ,;
        dest = dest ,;
        default = default ,;
        nargs = 0 ,;
        help = help );
        pub fn __call__ ( &self, parser , namespace , values , option_string = None /* Option */ )  {
        parser . print_help ( );
        parser . exit ( );
        class _VersionAction ( Action ) ;
        pub fn __init__ ( &self, {
        option_strings ,;
        version = None /* Option */ ,;
        dest = SUPPRESS ,;
        default = SUPPRESS ,;
        help = None /* Option */ ) ;
        if help is None /* Option */ {
        help = _ ( "show program's version number && exit" );
        super ( _VersionAction , self ) . __init__ (;
        option_strings = option_strings ,;
        dest = dest ,;
        default = default ,;
        nargs = 0 ,;
        help = help );
        self . version = version;
        pub fn __call__ ( &self, parser , namespace , values , option_string = None /* Option */ )  {
        version = self . version;
        if version is None /* Option */ {
        version = parser . version;
        formatter = parser . _get_formatter ( );
        formatter . add_text ( version );
        parser . _print_message ( formatter . format_help ( ) , _sys . stdout );
        parser . exit ( );
        class _SubParsersAction ( Action ) ;
        class _ChoicesPseudoAction ( Action ) ;
        pub fn __init__ ( &self, name , aliases , help )  {
        metavar = dest = name;
        if aliases {
        metavar + = " (%s)" % ", " . join ( aliases );
        sup = super ( _SubParsersAction . _ChoicesPseudoAction , self );
        sup . __init__ ( option_strings = [ ] , dest = dest , help = help ,;
        metavar = metavar );
        pub fn __init__ ( &self, {
        option_strings ,;
        prog ,;
        parser_class ,;
        dest = SUPPRESS ,;
        required = false ,;
        help = None /* Option */ ,;
        metavar = None /* Option */ ) ;
        self . _prog_prefix = prog;
        self . _parser_class = parser_class;
        self . _name_parser_map = { };
        self . _choices_actions = [ ];
        super ( _SubParsersAction , self ) . __init__ (;
        option_strings = option_strings ,;
        dest = dest ,;
        nargs = PARSER ,;
        choices = self . _name_parser_map ,;
        required = required ,;
        help = help ,;
        metavar = metavar );
        pub fn add_parser ( &self, name , ** kwargs )  {
        if kwargs . get ( "prog" ) is None /* Option */ {
        kwargs [ "prog" ] = "%s %s" % ( self . _prog_prefix , name );
        aliases = kwargs . pop ( "aliases" , ( ) );
        if name in self . _name_parser_map {
        panic!("ArgumentError ( self , _ ( "conflicting subparser: %s" ) % name )");
        for alias in aliases .iter() {
        if alias in self . _name_parser_map {
        panic!("ArgumentError (");
        self , _ ( "conflicting subparser alias: %s" ) % alias );
        if "help" in kwargs {
        help = kwargs . pop ( "help" );
        choice_action = self . _ChoicesPseudoAction ( name , aliases , help );
        self . _choices_actions . append ( choice_action );
        parser = self . _parser_class ( ** kwargs );
        self . _name_parser_map [ name ] = parser;
        for alias in aliases .iter() {
        self . _name_parser_map [ alias ] = parser;
        return  parser;
        pub fn _get_subactions ( self )  {
        return  self . _choices_actions;
        pub fn __call__ ( &self, parser , namespace , values , option_string = None /* Option */ )  {
        parser_name = values [ 0 ];
        arg_strings = values [ 1 : ];
        if self . dest is !SUPPRESS {
        setattr ( namespace , self . dest , parser_name );
        // try {
        parser = self . _name_parser_map [ parser_name ];
        // } catch  KeyError  {
        args = { "parser_name" : parser_name ,;
        "choices" : ", " . join ( self . _name_parser_map ) };
        msg = _ ( "unknown parser %(parser_name)r (choices: %(choices)s)" ) % args;
        panic!("ArgumentError ( self , msg )");
        subnamespace , arg_strings = parser . parse_known_args ( arg_strings , None /* Option */ );
        for key , value in vars ( subnamespace ) . items ( ) .iter() {
        setattr ( namespace , key , value );
        if arg_strings {
        vars ( namespace ) . setdefault ( _UNRECOGNIZED_ARGS_ATTR , [ ] );
        getattr ( namespace , _UNRECOGNIZED_ARGS_ATTR ) . extend ( arg_strings );
        class _ExtendAction ( _AppendAction ) ;
        pub fn __call__ ( &self, parser , namespace , values , option_string = None /* Option */ )  {
        items = getattr ( namespace , self . dest , None /* Option */ );
        items = _copy_items ( items );
        items . extend ( values );
        setattr ( namespace , self . dest , items );
        class FileType ( object ) ;
        "Factory for creating file object types

    Instances of FileType are typically passed as type= arguments to the
    ArgumentParser add_argument() method.

    Keyword Arguments:
        - mode -- A string indicating how the file == to be opened. Accepts the
            same values as the builtin open() function.
        - bufsize -- The file's desired buffer size. Accepts the same values as
            the builtin open() function.
        - encoding -- The file's encoding. Accepts the same values as the
            builtin open() function.
        - errors -- A string indicating how encoding && decoding errors are to
            be handled. Accepts the same value as the builtin open() function.
    ";
        pub fn __init__ ( &self, mode = "r" , bufsize = -1 , encoding = None /* Option */ , errors = None /* Option */ )  {
        self . _mode = mode;
        self . _bufsize = bufsize;
        self . _encoding = encoding;
        self . _errors = errors;
        pub fn __call__ ( &self, string )  {
        if string == "-" {
        if "r" in self . _mode {
        return  _sys . stdin . buffer if "b" in self . _mode else _sys . stdin;
        } else if any ( c in self . _mode for c in "wax" ) {
        return  _sys . stdout . buffer if "b" in self . _mode else _sys . stdout;
        } else {
        msg = _ ( "argument "-" with mode %r" ) % self . _mode;
        panic!("ValueError ( msg )");
        // try {
        return  open ( string , self . _mode , self . _bufsize , self . _encoding ,;
        self . _errors );
        // } catch  OSError as e  {
        args = { "filename" : string , "error" : e };
        message = _ ( "can't open '%(filename)s': %(error)s" );
        panic!("ArgumentTypeError ( message % args )");
        pub fn __repr__ ( self )  {
        args = self . _mode , self . _bufsize;
        kwargs = [ ( "encoding" , self . _encoding ) , ( "errors" , self . _errors ) ];
        args_str = ", " . join ( vec![ repr ( arg ).iter().map(|arg| args if arg != -1 ] +;
        vec![ "%s=%r" % ( kw , arg ).iter().map(|kw , arg| kwargs;
        if arg is !None /* Option */ ] ) {
        return  "%s(%s)" % ( type ( self ) . __name__ , args_str );
        class Namespace ( _AttributeHolder ) ;
        "Simple object for storing attributes.

    Implements equality by attribute names && values, && provides a simple
    string representation.
    ";
        pub fn __init__ ( &self, ** kwargs )  {
        for name in kwargs .iter() {
        setattr ( self , name , kwargs [ name ] );
        pub fn __eq__ ( &self, other )  {
        if !isinstance ( other , Namespace ) {
        return  NotImplemented;
        return  vars ( self ) == vars ( other );
        pub fn __contains__ ( &self, key )  {
        return  key in self . __dict__;
        class _ActionsContainer ( object ) ;
        pub fn __init__ ( &self, {
        description ,;
        prefix_chars ,;
        argument_default ,;
        conflict_handler ) ;
        super ( _ActionsContainer , self ) . __init__ ( );
        self . description = description;
        self . argument_default = argument_default;
        self . prefix_chars = prefix_chars;
        self . conflict_handler = conflict_handler;
        self . _registries = { };
        self . register ( "action" , None /* Option */ , _StoreAction );
        self . register ( "action" , "store" , _StoreAction );
        self . register ( "action" , "store_const" , _StoreConstAction );
        self . register ( "action" , "store_true" , _StoretrueAction );
        self . register ( "action" , "store_false" , _StorefalseAction );
        self . register ( "action" , "append" , _AppendAction );
        self . register ( "action" , "append_const" , _AppendConstAction );
        self . register ( "action" , "count" , _CountAction );
        self . register ( "action" , "help" , _HelpAction );
        self . register ( "action" , "version" , _VersionAction );
        self . register ( "action" , "parsers" , _SubParsersAction );
        self . register ( "action" , "extend" , _ExtendAction );
        self . _get_handler ( );
        self . _actions = [ ];
        self . _option_string_actions = { };
        self . _action_groups = [ ];
        self . _mutually_exclusive_groups = [ ];
        self . _defaults = { };
        self . _negative_number_matcher = _re . compile ( r "^-\d+$|^-\d*\.\d+$" );
        self . _has_negative_number_optionals = [ ];
        pub fn register ( &self, registry_name , value , object )  {
        registry = self . _registries . setdefault ( registry_name , { } );
        registry [ value ] = object;
        pub fn _registry_get ( &self, registry_name , value , default = None /* Option */ )  {
        return  self . _registries [ registry_name ] . get ( value , default );
        pub fn set_defaults ( &self, ** kwargs )  {
        self . _defaults . update ( kwargs );
        for action in self . _actions .iter() {
        if action . dest in kwargs {
        action . default = kwargs [ action . dest ];
        pub fn get_default ( &self, dest )  {
        for action in self . _actions .iter() {
        if action . dest == dest && action . default is !None /* Option */ {
        return  action . default;
        return  self . _defaults . get ( dest , None /* Option */ );
        pub fn add_argument ( &self, * args , ** kwargs )  {
        "
        add_argument(dest, ..., name=value, ...)
        add_argument(option_string, option_string, ..., name=value, ...)
        ";
        chars = self . prefix_chars;
        if !args || len ( args ) == 1 && args [ 0 ] [ 0 ] !in chars {
        if args && "dest" in kwargs {
        panic!("ValueError ( "dest supplied twice for positional argument" )");
        kwargs = self . _get_positional_kwargs ( * args , ** kwargs );
        } else {
        kwargs = self . _get_optional_kwargs ( * args , ** kwargs );
        if "default" !in kwargs {
        dest = kwargs [ "dest" ];
        if dest in self . _defaults {
        kwargs [ "default" ] = self . _defaults [ dest ];
        } else if self . argument_default is !None /* Option */ {
        kwargs [ "default" ] = self . argument_default;
        action_class = self . _pop_action_class ( kwargs );
        if !callable ( action_class ) {
        panic!("ValueError ( "unknown action "%s"" % ( action_class , ) )");
        action = action_class ( ** kwargs );
        type_func = self . _registry_get ( "type" , action . type , action . type );
        if !callable ( type_func ) {
        panic!("ValueError ( "%r is !callable" % ( type_func , ) )");
        if type_func is FileType {
        panic!("ValueError ( "%r is a FileType class object, instance of it"");
        " must be passed" % ( type_func , ) );
        if hasattr ( self , "_get_formatter" ) {
        // try {
        self . _get_formatter ( ) . _format_args ( action , None /* Option */ );
        // } catch  TypeError  {
        panic!("ValueError ( "length of metavar tuple does !match nargs" )");
        return  self . _add_action ( action );
        pub fn add_argument_group ( &self, * args , ** kwargs )  {
        group = _ArgumentGroup ( self , * args , ** kwargs );
        self . _action_groups . append ( group );
        return  group;
        pub fn add_mutually_exclusive_group ( &self, ** kwargs )  {
        group = _MutuallyExclusiveGroup ( self , ** kwargs );
        self . _mutually_exclusive_groups . append ( group );
        return  group;
        pub fn _add_action ( &self, action )  {
        self . _check_conflict ( action );
        self . _actions . append ( action );
        action . container = self;
        for option_string in action . option_strings .iter() {
        self . _option_string_actions [ option_string ] = action;
        for option_string in action . option_strings .iter() {
        if self . _negative_number_matcher . match ( option_string ) {
        if !self . _has_negative_number_optionals {
        self . _has_negative_number_optionals . append ( true );
        return  action;
        pub fn _remove_action ( &self, action )  {
        self . _actions . remove ( action );
        pub fn _add_container_actions ( &self, container )  {
        title_group_map = { };
        for group in self . _action_groups .iter() {
        if group . title in title_group_map {
        msg = _ ( "cannot merge actions - two groups are named %r" );
        panic!("ValueError ( msg % ( group . title ) )");
        title_group_map [ group . title ] = group;
        group_map = { };
        for group in container . _action_groups .iter() {
        if group . title !in title_group_map {
        title_group_map [ group . title ] = self . add_argument_group (;
        title = group . title ,;
        description = group . description ,;
        conflict_handler = group . conflict_handler );
        for action in group . _group_actions .iter() {
        group_map [ action ] = title_group_map [ group . title ];
        for group in container . _mutually_exclusive_groups .iter() {
        mutex_group = self . add_mutually_exclusive_group (;
        required = group . required );
        for action in group . _group_actions .iter() {
        group_map [ action ] = mutex_group;
        for action in container . _actions .iter() {
        group_map . get ( action , self ) . _add_action ( action );
        pub fn _get_positional_kwargs ( &self, dest , ** kwargs )  {
        if "required" in kwargs {
        msg = _ ( "'required' == an invalid argument for positionals" );
        panic!("TypeError ( msg )");
        if kwargs . get ( "nargs" ) !in [ OPTIONAL , ZERO_OR_MORE ] {
        kwargs [ "required" ] = true;
        if kwargs . get ( "nargs" ) == ZERO_OR_MORE && "default" !in kwargs {
        kwargs [ "required" ] = true;
        return  dict ( kwargs , dest = dest , option_strings = [ ] );
        pub fn _get_optional_kwargs ( &self, * args , ** kwargs )  {
        option_strings = [ ];
        long_option_strings = [ ];
        for option_string in args .iter() {
        if !option_string [ 0 ] in self . prefix_chars {
        args = { "option" : option_string ,;
        "prefix_chars" : self . prefix_chars };
        msg = _ ( "invalid option string %(option)r: ";
        "must start with a character %(prefix_chars)r" );
        panic!("ValueError ( msg % args )");
        option_strings . append ( option_string );
        if len ( option_string ) > 1 && option_string [ 1 ] in self . prefix_chars {
        long_option_strings . append ( option_string );
        dest = kwargs . pop ( "dest" , None /* Option */ );
        if dest is None /* Option */ {
        if long_option_strings {
        dest_option_string = long_option_strings [ 0 ];
        } else {
        dest_option_string = option_strings [ 0 ];
        dest = dest_option_string . lstrip ( self . prefix_chars );
        if !dest {
        msg = _ ( "dest= == required for options like %r" );
        panic!("ValueError ( msg % option_string )");
        dest = dest . replace ( "-" , "_" );
        return  dict ( kwargs , dest = dest , option_strings = option_strings );
        pub fn _pop_action_class ( &self, kwargs , default = None /* Option */ )  {
        action = kwargs . pop ( "action" , default );
        return  self . _registry_get ( "action" , action , action );
        pub fn _get_handler ( self )  {
        handler_func_name = "_handle_conflict_%s" % self . conflict_handler;
        // try {
        return  getattr ( self , handler_func_name );
        // } catch  AttributeError  {
        msg = _ ( "invalid conflict_resolution value: %r" );
        panic!("ValueError ( msg % self . conflict_handler )");
        pub fn _check_conflict ( &self, action )  {
        confl_optionals = [ ];
        for option_string in action . option_strings .iter() {
        if option_string in self . _option_string_actions {
        confl_optional = self . _option_string_actions [ option_string ];
        confl_optionals . append ( ( option_string , confl_optional ) );
        if confl_optionals {
        conflict_handler = self . _get_handler ( );
        conflict_handler ( action , confl_optionals );
        pub fn _handle_conflict_error ( &self, action , conflicting_actions )  {
        message = ngettext ( "conflicting option string: %s" ,;
        "conflicting option strings: %s" ,;
        len ( conflicting_actions ) );
        conflict_string = ", " . join ( [ option_string;
        for option_string , action;
        in conflicting_actions ] );
        panic!("ArgumentError ( action , message % conflict_string )");
        pub fn _handle_conflict_resolve ( &self, action , conflicting_actions )  {
        for option_string , action in conflicting_actions .iter() {
        action . option_strings . remove ( option_string );
        self . _option_string_actions . pop ( option_string , None /* Option */ );
        if !action . option_strings {
        action . container . _remove_action ( action );
        class _ArgumentGroup ( _ActionsContainer ) ;
        pub fn __init__ ( &self, container , title = None /* Option */ , description = None /* Option */ , ** kwargs )  {
        update = kwargs . setdefault;
        update ( "conflict_handler" , container . conflict_handler );
        update ( "prefix_chars" , container . prefix_chars );
        update ( "argument_default" , container . argument_default );
        super_init = super ( _ArgumentGroup , self ) . __init__;
        super_init ( description = description , ** kwargs );
        self . title = title;
        self . _group_actions = [ ];
        self . _registries = container . _registries;
        self . _actions = container . _actions;
        self . _option_string_actions = container . _option_string_actions;
        self . _defaults = container . _defaults;
        self . _has_negative_number_optionals = \;
        container . _has_negative_number_optionals;
        self . _mutually_exclusive_groups = container . _mutually_exclusive_groups;
        pub fn _add_action ( &self, action )  {
        action = super ( _ArgumentGroup , self ) . _add_action ( action );
        self . _group_actions . append ( action );
        return  action;
        pub fn _remove_action ( &self, action )  {
        super ( _ArgumentGroup , self ) . _remove_action ( action );
        self . _group_actions . remove ( action );
        pub fn add_argument_group ( &self, * args , ** kwargs )  {
        warnings . warn (;
        "Nesting argument groups == deprecated." ,;
        category = DeprecationWarning ,;
        stacklevel = 2;
        );
        return  super ( ) . add_argument_group ( * args , ** kwargs );
        class _MutuallyExclusiveGroup ( _ArgumentGroup ) ;
        pub fn __init__ ( &self, container , required = false )  {
        super ( _MutuallyExclusiveGroup , self ) . __init__ ( container );
        self . required = required;
        self . _container = container;
        pub fn _add_action ( &self, action )  {
        if action . required {
        msg = _ ( "mutually exclusive arguments must be optional" );
        panic!("ValueError ( msg )");
        action = self . _container . _add_action ( action );
        self . _group_actions . append ( action );
        return  action;
        pub fn _remove_action ( &self, action )  {
        self . _container . _remove_action ( action );
        self . _group_actions . remove ( action );
        pub fn add_mutually_exclusive_group ( &self, * args , ** kwargs )  {
        warnings . warn (;
        "Nesting mutually exclusive groups == deprecated." ,;
        category = DeprecationWarning ,;
        stacklevel = 2;
        );
        return  super ( ) . add_mutually_exclusive_group ( * args , ** kwargs );
        class ArgumentParser ( _AttributeHolder , _ActionsContainer ) ;
        "Object for parsing command line strings into Python objects.

    Keyword Arguments:
        - prog -- The name of the program (default:
            ``os.path.basename(sys.argv[0])``)
        - usage -- A usage message (default: auto-generated from arguments)
        - description -- A description of what the program does
        - epilog -- Text following the argument descriptions
        - parents -- Parsers whose arguments should be copied into this one
        - formatter_class -- HelpFormatter class for printing help messages
        - prefix_chars -- Characters that prefix optional arguments
        - fromfile_prefix_chars -- Characters that prefix files containing
            additional arguments
        - argument_default -- The default value for all arguments
        - conflict_handler -- String indicating how to handle conflicts
        - add_help -- Add a -h/-help option
        - allow_abbrev -- Allow long options to be abbreviated unambiguously
        - exit_on_error -- Determines whether || !ArgumentParser exits with
            error info when an error occurs
    ";
        pub fn __init__ ( &self, {
        prog = None /* Option */ ,;
        usage = None /* Option */ ,;
        description = None /* Option */ ,;
        epilog = None /* Option */ ,;
        parents = [ ] ,;
        formatter_class = HelpFormatter ,;
        prefix_chars = "-" ,;
        fromfile_prefix_chars = None /* Option */ ,;
        argument_default = None /* Option */ ,;
        conflict_handler = "error" ,;
        add_help = true ,;
        allow_abbrev = true ,;
        exit_on_error = true ) ;
        superinit = super ( ArgumentParser , self ) . __init__;
        superinit ( description = description ,;
        prefix_chars = prefix_chars ,;
        argument_default = argument_default ,;
        conflict_handler = conflict_handler );
        if prog is None /* Option */ {
        prog = _os . path . basename ( _sys . argv [ 0 ] );
        self . prog = prog;
        self . usage = usage;
        self . epilog = epilog;
        self . formatter_class = formatter_class;
        self . fromfile_prefix_chars = fromfile_prefix_chars;
        self . add_help = add_help;
        self . allow_abbrev = allow_abbrev;
        self . exit_on_error = exit_on_error;
        add_group = self . add_argument_group;
        self . _positionals = add_group ( _ ( "positional arguments" ) );
        self . _optionals = add_group ( _ ( "options" ) );
        self . _subparsers = None /* Option */;
        pub fn identity ( string )  {
        return  string;
        self . register ( "type" , None /* Option */ , identity );
        default_prefix = "-" iformat!("-" in prefix_chars else prefix_chars [ 0 ]);
        if self . add_help {
        self . add_argument (;
        default_prefix + "h" , default_prefix * 2 + "help" ,;
        action = "help" , default = SUPPRESS ,;
        help = _ ( "show this help message && exit" ) );
        for parent in parents .iter() {
        self . _add_container_actions ( parent );
        // try {
        defaults = parent . _defaults;
        // } catch  AttributeError  {
        // pass
        } else {
        self . _defaults . update ( defaults );
        pub fn _get_kwargs ( self )  {
        names = [;
        "prog" ,;
        "usage" ,;
        "description" ,;
        "formatter_class" ,;
        "conflict_handler" ,;
        "add_help" ,;
        ];
        return  [ ( name , getattr ( self , name ) ) for name in names ];
        pub fn add_subparsers ( &self, ** kwargs )  {
        if self . _subparsers is !None /* Option */ {
        self . error ( _ ( "cannot have multiple subparser arguments" ) );
        kwargs . setdefault ( "parser_class" , type ( self ) );
        if "title" in kwargs || "description" in kwargs {
        title = _ ( kwargs . pop ( "title" , "subcommands" ) );
        description = _ ( kwargs . pop ( "description" , None /* Option */ ) );
        self . _subparsers = self . add_argument_group ( title , description );
        } else {
        self . _subparsers = self . _positionals;
        if kwargs . get ( "prog" ) is None /* Option */ {
        formatter = self . _get_formatter ( );
        positionals = self . _get_positional_actions ( );
        groups = self . _mutually_exclusive_groups;
        formatter . add_usage ( self . usage , positionals , groups , "" );
        kwargs [ "prog" ] = formatter . format_help ( ) . strip ( );
        parsers_class = self . _pop_action_class ( kwargs , "parsers" );
        action = parsers_class ( option_strings = [ ] , ** kwargs );
        self . _subparsers . _add_action ( action );
        return  action;
        pub fn _add_action ( &self, action )  {
        if action . option_strings {
        self . _optionals . _add_action ( action );
        } else {
        self . _positionals . _add_action ( action );
        return  action;
        pub fn _get_optional_actions ( self )  {
        return  [ action;
        for action in self . _actions.iter() {
        if action . option_strings ] {
        pub fn _get_positional_actions ( self )  {
        return  [ action;
        for action in self . _actions.iter() {
        if !action . option_strings ] {
        pub fn parse_args ( &self, args = None /* Option */ , namespace = None /* Option */ )  {
        args , argv = self . parse_known_args ( args , namespace );
        if argv {
        msg = _ ( "unrecognized arguments: %s" );
        self . error ( msg % " " . join ( argv ) );
        return  args;
        pub fn parse_known_args ( &self, args = None /* Option */ , namespace = None /* Option */ )  {
        if args is None /* Option */ {
        args = _sys . argv [ 1 : ];
        } else {
        args = list ( args );
        if namespace is None /* Option */ {
        namespace = Namespace ( );
        for action in self . _actions .iter() {
        if action . dest is !SUPPRESS {
        if !hasattr ( namespace , action . dest ) {
        if action . default is !SUPPRESS {
        setattr ( namespace , action . dest , action . default );
        for dest in self . _defaults .iter() {
        if !hasattr ( namespace , dest ) {
        setattr ( namespace , dest , self . _defaults [ dest ] );
        if self . exit_on_error {
        // try {
        namespace , args = self . _parse_known_args ( args , namespace );
        // } catch  ArgumentError as err  {
        self . error ( str ( err ) );
        } else {
        namespace , args = self . _parse_known_args ( args , namespace );
        if hasattr ( namespace , _UNRECOGNIZED_ARGS_ATTR ) {
        args . extend ( getattr ( namespace , _UNRECOGNIZED_ARGS_ATTR ) );
        delattr ( namespace , _UNRECOGNIZED_ARGS_ATTR );
        return  namespace , args;
        pub fn _parse_known_args ( &self, arg_strings , namespace )  {
        if self . fromfile_prefix_chars is !None /* Option */ {
        arg_strings = self . _read_args_from_files ( arg_strings );
        action_conflicts = { };
        for mutex_group in self . _mutually_exclusive_groups .iter() {
        group_actions = mutex_group . _group_actions;
        for i , mutex_action in enumerate ( mutex_group . _group_actions ) .iter() {
        conflicts = action_conflicts . setdefault ( mutex_action , [ ] );
        conflicts . extend ( group_actions [ : i ] );
        conflicts . extend ( group_actions [ i + 1 : ] );
        option_string_indices = { };
        arg_string_pattern_parts = [ ];
        arg_strings_iter = iter ( arg_strings );
        for i , arg_string in enumerate ( arg_strings_iter ) .iter() {
        if arg_string == "--" {
        arg_string_pattern_parts . append ( "-" );
        for arg_string in arg_strings_iter .iter() {
        arg_string_pattern_parts . append ( "A" );
        } else {
        option_tuple = self . _parse_optional ( arg_string );
        if option_tuple is None /* Option */ {
        pattern = "A";
        } else {
        option_string_indices [ i ] = option_tuple;
        pattern = "O";
        arg_string_pattern_parts . append ( pattern );
        arg_strings_pattern = "" . join ( arg_string_pattern_parts );
        seen_actions = set ( );
        seen_non_default_actions = set ( );
        pub fn take_action ( action , argument_strings , option_string = None /* Option */ )  {
        seen_actions . add ( action );
        argument_values = self . _get_values ( action , argument_strings );
        if argument_values is !action . default {
        seen_non_default_actions . add ( action );
        for conflict_action in action_conflicts . get ( action , [ ] ) .iter() {
        if conflict_action in seen_non_default_actions {
        msg = _ ( "not allowed with argument %s" );
        action_name = _get_action_name ( conflict_action );
        panic!("ArgumentError ( action , msg % action_name )");
        if argument_values is !SUPPRESS {
        action ( self , namespace , argument_values , option_string );
        pub fn consume_optional ( start_index )  {
        option_tuple = option_string_indices [ start_index ];
        action , option_string , sep , explicit_arg = option_tuple;
        match_argument = self . _match_argument;
        action_tuples = [ ];
        while true  {
        if action is None /* Option */ {
        extras . append ( arg_strings [ start_index ] );
        return  start_index + 1;
        if explicit_arg is !None /* Option */ {
        arg_count = match_argument ( action , "A" );
        chars = self . prefix_chars;
        if ( {
        arg_count == 0;
        and option_string [ 1 ] !in chars;
        and explicit_arg != "";
        ) ;
        if sep || explicit_arg [ 0 ] in chars {
        msg = _ ( "ignored explicit argument %r" );
        panic!("ArgumentError ( action , msg % explicit_arg )");
        action_tuples . append ( ( action , [ ] , option_string ) );
        char = option_string [ 0 ];
        option_string = char + explicit_arg [ 0 ];
        optionals_map = self . _option_string_actions;
        if option_string in optionals_map {
        action = optionals_map [ option_string ];
        explicit_arg = explicit_arg [ 1 : ];
        if !explicit_arg {
        sep = explicit_arg = None /* Option */;
        } else if explicit_arg [ 0 ] == "=" {
        sep = "=";
        explicit_arg = explicit_arg [ 1 : ];
        } else {
        sep = "";
        } else {
        extras . append ( char + explicit_arg );
        stop = start_index + 1;
        break;
        } else if arg_count == 1 {
        stop = start_index + 1;
        args = [ explicit_arg ];
        action_tuples . append ( ( action , args , option_string ) );
        break;
        } else {
        msg = _ ( "ignored explicit argument %r" );
        panic!("ArgumentError ( action , msg % explicit_arg )");
        } else {
        start = start_index + 1;
        selected_patterns = arg_strings_pattern [ start : ];
        arg_count = match_argument ( action , selected_patterns );
        stop = start + arg_count;
        args = arg_strings [ start : stop ];
        action_tuples . append ( ( action , args , option_string ) );
        break;
        assert action_tuples;
        for action , args , option_string in action_tuples .iter() {
        take_action ( action , args , option_string );
        return  stop;
        positionals = self . _get_positional_actions ( );
        pub fn consume_positionals ( start_index )  {
        match_partial = self . _match_arguments_partial;
        selected_pattern = arg_strings_pattern [ start_index : ];
        arg_counts = match_partial ( positionals , selected_pattern );
        for action , arg_count in zip ( positionals , arg_counts ) .iter() {
        args = arg_strings [ start_index : start_index + arg_count ];
        start_index + = arg_count;
        take_action ( action , args );
        positionals [ : ] = positionals [ len ( arg_counts ) : ];
        return  start_index;
        extras = [ ];
        start_index = 0;
        if option_string_indices {
        max_option_string_index = max ( option_string_indices );
        } else {
        max_option_string_index = -1;
        while start_index <= max_option_string_index  {
        next_option_string_index = min ( [;
        index;
        for index in option_string_indices.iter() {
        if index >= start_index ] ) {
        if start_index != next_option_string_index {
        positionals_end_index = consume_positionals ( start_index );
        if positionals_end_index > start_index {
        start_index = positionals_end_index;
        continue;
        } else {
        start_index = positionals_end_index;
        if start_index !in option_string_indices {
        strings = arg_strings [ start_index : next_option_string_index ];
        extras . extend ( strings );
        start_index = next_option_string_index;
        start_index = consume_optional ( start_index );
        stop_index = consume_positionals ( start_index );
        extras . extend ( arg_strings [ stop_index : ] );
        required_actions = [ ];
        for action in self . _actions .iter() {
        if action !in seen_actions {
        if action . required {
        required_actions . append ( _get_action_name ( action ) );
        } else {
        if ( action . default is !None /* Option */ and {
        isinstance ( action . default , str ) and;
        hasattr ( namespace , action . dest ) and;
        action . default == getattr ( namespace , action . dest ) ) ;
        setattr ( namespace , action . dest ,;
        self . _get_value ( action , action . default ) );
        if required_actions {
        self . error ( _ ( "the following arguments are required: %s" ) %;
        ", " . join ( required_actions ) );
        for group in self . _mutually_exclusive_groups .iter() {
        if group . required {
        for action in group . _group_actions .iter() {
        if action in seen_non_default_actions {
        break;
        } else {
        names = [ _get_action_name ( action );
        for action in group . _group_actions.iter() {
        if action . help is !SUPPRESS ] {
        msg = _ ( "one of the arguments %s == required" );
        self . error ( msg % " " . join ( names ) );
        return  namespace , extras;
        pub fn _read_args_from_files ( &self, arg_strings )  {
        new_arg_strings = [ ];
        for arg_string in arg_strings .iter() {
        if !arg_string || arg_string [ 0 ] !in self . fromfile_prefix_chars {
        new_arg_strings . append ( arg_string );
        } else {
        // try {
        // with scope: open ( arg_string [ 1 : ] ) as args_file  {
        arg_strings = [ ];
        for arg_line in args_file . read ( ) . splitlines ( ) .iter() {
        for arg in self . convert_arg_line_to_args ( arg_line ) .iter() {
        arg_strings . append ( arg );
        arg_strings = self . _read_args_from_files ( arg_strings );
        new_arg_strings . extend ( arg_strings );
        // } catch  OSError as err  {
        self . error ( str ( err ) );
        return  new_arg_strings;
        pub fn convert_arg_line_to_args ( &self, arg_line )  {
        return  [ arg_line ];
        pub fn _match_argument ( &self, action , arg_strings_pattern )  {
        nargs_pattern = self . _get_nargs_pattern ( action );
        match = _re . match ( nargs_pattern , arg_strings_pattern );
        if match is None /* Option */ {
        nargs_errors = {;
        None /* Option */ : _ ( "expected one argument" ) ,;
        OPTIONAL : _ ( "expected at most one argument" ) ,;
        ONE_OR_MORE : _ ( "expected at least one argument" ) ,;
        };
        msg = nargs_errors . get ( action . nargs );
        if msg is None /* Option */ {
        msg = ngettext ( "expected %s argument" ,;
        "expected %s arguments" ,;
        action . nargs ) % action . nargs;
        panic!("ArgumentError ( action , msg )");
        return  len ( match . group ( 1 ) );
        pub fn _match_arguments_partial ( &self, actions , arg_strings_pattern )  {
        result = [ ];
        for i in range ( len ( actions ) , 0 , -1 ) .iter() {
        actions_slice = actions [ : i ];
        pattern = "" . join ( [ self . _get_nargs_pattern ( action );
        for action in actions_slice ] ).iter() {
        match = _re . match ( pattern , arg_strings_pattern );
        if match is !None /* Option */ {
        result . extend ( vec![ len ( string ).iter().map(|string| match . groups ( ) ] );
        break;
        return  result;
        pub fn _parse_optional ( &self, arg_string )  {
        if !arg_string {
        return;
        if !arg_string [ 0 ] in self . prefix_chars {
        return;
        if arg_string in self . _option_string_actions {
        action = self . _option_string_actions [ arg_string ];
        return  action , arg_string , None /* Option */ , None /* Option */;
        if len ( arg_string ) == 1 {
        return;
        option_string , sep , explicit_arg = arg_string . partition ( "=" );
        if sep && option_string in self . _option_string_actions {
        action = self . _option_string_actions [ option_string ];
        return  action , option_string , sep , explicit_arg;
        option_tuples = self . _get_option_tuples ( arg_string );
        if len ( option_tuples ) > 1 {
        options = ", " . join ( [ option_string;
        for action , option_string , sep , explicit_arg in option_tuples ] ).iter() {
        args = { "option" : arg_string , "matches" : options };
        msg = _ ( "ambiguous option: %(option)s could match %(matches)s" );
        self . error ( msg % args );
        } else if len ( option_tuples ) == 1 {
        option_tuple , = option_tuples;
        return  option_tuple;
        if self . _negative_number_matcher . match ( arg_string ) {
        if !self . _has_negative_number_optionals {
        return;
        if " " in arg_string {
        return;
        return  None /* Option */ , arg_string , None /* Option */ , None /* Option */;
        pub fn _get_option_tuples ( &self, option_string )  {
        result = [ ];
        chars = self . prefix_chars;
        if option_string [ 0 ] in chars && option_string [ 1 ] in chars {
        if self . allow_abbrev {
        option_prefix , sep , explicit_arg = option_string . partition ( "=" );
        if !sep {
        sep = explicit_arg = None /* Option */;
        for option_string in self . _option_string_actions .iter() {
        if option_string . startswith ( option_prefix ) {
        action = self . _option_string_actions [ option_string ];
        tup = action , option_string , sep , explicit_arg;
        result . append ( tup );
        } else if option_string [ 0 ] in chars && option_string [ 1 ] !in chars {
        option_prefix = option_string;
        short_option_prefix = option_string [ : 2 ];
        short_explicit_arg = option_string [ 2 : ];
        for option_string in self . _option_string_actions .iter() {
        if option_string == short_option_prefix {
        action = self . _option_string_actions [ option_string ];
        tup = action , option_string , "" , short_explicit_arg;
        result . append ( tup );
        } else if option_string . startswith ( option_prefix ) {
        action = self . _option_string_actions [ option_string ];
        tup = action , option_string , None /* Option */ , None /* Option */;
        result . append ( tup );
        } else {
        self . error ( _ ( "unexpected option string: %s" ) % option_string );
        return  result;
        pub fn _get_nargs_pattern ( &self, action )  {
        nargs = action . nargs;
        if nargs is None /* Option */ {
        nargs_pattern = "(-*A-*)";
        } else if nargs == OPTIONAL {
        nargs_pattern = "(-*A?-*)";
        } else if nargs == ZERO_OR_MORE {
        nargs_pattern = "(-*[A-]*)";
        } else if nargs == ONE_OR_MORE {
        nargs_pattern = "(-*A[A-]*)";
        } else if nargs == REMAINDER {
        nargs_pattern = "([-AO]*)";
        } else if nargs == PARSER {
        nargs_pattern = "(-*A[-AO]*)";
        } else if nargs == SUPPRESS {
        nargs_pattern = "(-*-*)";
        } else {
        nargs_pattern = "(-*%s-*)" % "-*" . join ( "A" * nargs );
        if action . option_strings {
        nargs_pattern = nargs_pattern . replace ( "-*" , "" );
        nargs_pattern = nargs_pattern . replace ( "-" , "" );
        return  nargs_pattern;
        pub fn parse_intermixed_args ( &self, args = None /* Option */ , namespace = None /* Option */ )  {
        args , argv = self . parse_known_intermixed_args ( args , namespace );
        if argv {
        msg = _ ( "unrecognized arguments: %s" );
        self . error ( msg % " " . join ( argv ) );
        return  args;
        pub fn parse_known_intermixed_args ( &self, args = None /* Option */ , namespace = None /* Option */ )  {
        positionals = self . _get_positional_actions ( );
        a = vec![ action.iter().map(|action| positionals;
        if action . nargs in [ PARSER , REMAINDER ] ] {
        if a {
        panic!("TypeError ( "parse_intermixed_args: positional arg"");
        " with nargs=%s" % a [ 0 ] . nargs );
        if [ action . dest for group in self . _mutually_exclusive_groups {
        for action in group . _group_actions if action in positionals ] .iter() {
        panic!("TypeError ( "parse_intermixed_args: positional in"");
        " mutuallyExclusiveGroup" );
        // try {
        save_usage = self . usage;
        // try {
        if self . usage is None /* Option */ {
        self . usage = self . format_usage ( ) [ 7 : ];
        for action in positionals .iter() {
        action . save_nargs = action . nargs;
        action . nargs = SUPPRESS;
        action . save_default = action . default;
        action . default = SUPPRESS;
        namespace , remaining_args = self . parse_known_args ( args ,;
        namespace );
        for action in positionals .iter() {
        if ( hasattr ( namespace , action . dest ) {
        and getattr ( namespace , action . dest ) == [ ] ) ;
        from warnings import warn;
        warn ( "Do !expect %s in %s" % ( action . dest , namespace ) );
        delattr ( namespace , action . dest );
        // } finally {
        for action in positionals .iter() {
        action . nargs = action . save_nargs;
        action . default = action . save_default;
        optionals = self . _get_optional_actions ( );
        // try {
        for action in optionals .iter() {
        action . save_required = action . required;
        action . required = false;
        for group in self . _mutually_exclusive_groups .iter() {
        group . save_required = group . required;
        group . required = false;
        namespace , extras = self . parse_known_args ( remaining_args ,;
        namespace );
        // } finally {
        for action in optionals .iter() {
        action . required = action . save_required;
        for group in self . _mutually_exclusive_groups .iter() {
        group . required = group . save_required;
        // } finally {
        self . usage = save_usage;
        return  namespace , extras;
        pub fn _get_values ( &self, action , arg_strings )  {
        if !action . option_strings && action . nargs !in [ PARSER , REMAINDER ] {
        // try {
        arg_strings . remove ( "--" );
        // } catch  ValueError  {
        // pass
        if !arg_strings && action . nargs == OPTIONAL {
        if action . option_strings {
        value = action . const;
        } else {
        value = action . default;
        if isinstance ( value , str ) {
        value = self . _get_value ( action , value );
        self . _check_value ( action , value );
        } else if ( !arg_strings && action . nargs == ZERO_OR_MORE and {
        not action . option_strings ) ;
        if action . default is !None /* Option */ {
        value = action . default;
        } else {
        value = arg_strings;
        self . _check_value ( action , value );
        } else if len ( arg_strings ) == 1 && action . nargs in [ None /* Option */ , OPTIONAL ] {
        arg_string , = arg_strings;
        value = self . _get_value ( action , arg_string );
        self . _check_value ( action , value );
        } else if action . nargs == REMAINDER {
        value = vec![ self . _get_value ( action , v ).iter().map(|v| arg_strings ).collect();
        } else if action . nargs == PARSER {
        value = vec![ self . _get_value ( action , v ).iter().map(|v| arg_strings ).collect();
        self . _check_value ( action , value [ 0 ] );
        } else if action . nargs == SUPPRESS {
        value = SUPPRESS;
        } else {
        value = vec![ self . _get_value ( action , v ).iter().map(|v| arg_strings ).collect();
        for v in value .iter() {
        self . _check_value ( action , v );
        return  value;
        pub fn _get_value ( &self, action , arg_string )  {
        type_func = self . _registry_get ( "type" , action . type , action . type );
        if !callable ( type_func ) {
        msg = _ ( "%r == !callable" );
        panic!("ArgumentError ( action , msg % type_func )");
        // try {
        result = type_func ( arg_string );
        // } catch  ArgumentTypeError as err  {
        name = getattr ( action . type , "__name__" , repr ( action . type ) );
        msg = str ( err );
        panic!("ArgumentError ( action , msg )");
        // } catch  ( TypeError , ValueError )  {
        name = getattr ( action . type , "__name__" , repr ( action . type ) );
        args = { "type" : name , "value" : arg_string };
        msg = _ ( "invalid %(type)s value: %(value)r" );
        panic!("ArgumentError ( action , msg % args )");
        return  result;
        pub fn _check_value ( &self, action , value )  {
        if action . choices is !None /* Option */ && value !in action . choices {
        args = { "value" : value ,;
        "choices" : ", " . join ( map ( repr , action . choices ) ) };
        msg = _ ( "invalid choice: %(value)r (choose from %(choices)s)" );
        panic!("ArgumentError ( action , msg % args )");
        pub fn format_usage ( self )  {
        formatter = self . _get_formatter ( );
        formatter . add_usage ( self . usage , self . _actions ,;
        self . _mutually_exclusive_groups );
        return  formatter . format_help ( );
        pub fn format_help ( self )  {
        formatter = self . _get_formatter ( );
        formatter . add_usage ( self . usage , self . _actions ,;
        self . _mutually_exclusive_groups );
        formatter . add_text ( self . description );
        for action_group in self . _action_groups .iter() {
        formatter . start_section ( action_group . title );
        formatter . add_text ( action_group . description );
        formatter . add_arguments ( action_group . _group_actions );
        formatter . end_section ( );
        formatter . add_text ( self . epilog );
        return  formatter . format_help ( );
        pub fn _get_formatter ( self )  {
        return  self . formatter_class ( prog = self . prog );
        pub fn print_usage ( &self, file = None /* Option */ )  {
        if file is None /* Option */ {
        file = _sys . stdout;
        self . _print_message ( self . format_usage ( ) , file );
        pub fn print_help ( &self, file = None /* Option */ )  {
        if file is None /* Option */ {
        file = _sys . stdout;
        self . _print_message ( self . format_help ( ) , file );
        pub fn _print_message ( &self, message , file = None /* Option */ )  {
        if message {
        file = file || _sys . stderr;
        // try {
        file . write ( message );
        // } catch  ( AttributeError , OSError )  {
        // pass
        pub fn exit ( &self, status = 0 , message = None /* Option */ )  {
        if message {
        self . _print_message ( message , _sys . stderr );
        _sys . exit ( status );
        pub fn error ( &self, message )  {
        "error(message: string)

        Prints a usage message incorporating the message to stderr and
        exits.

        If you override this in a subclass, it should !return -- it
        should either exit || raise an exception.
        ";
        self . print_usage ( _sys . stderr );
        args = { "prog" : self . prog , "message" : message };
        self . exit ( 2 , _ ( "%(prog)s: error: %(message)s\n" ) % args );
}

