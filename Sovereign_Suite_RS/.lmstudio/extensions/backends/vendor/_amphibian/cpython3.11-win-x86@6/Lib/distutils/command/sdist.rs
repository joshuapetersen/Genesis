//! sdist.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use std::fs;
// use crate::glob::{glob};
// use crate::warnings::{warn};
// use crate::distutils::{Command};

pub fn show_formats() {
        "Print all possible values for the 'formats' option (used by
    the "--help-formats" command-line option).
    ";
        from distutils . fancy_getopt import FancyGetopt;
        from distutils . archive_util import ARCHIVE_FORMATS;
        formats = [ ];
        for format in ARCHIVE_FORMATS . keys ( ) .iter() {
        formats . append ( ( "formats=" + format , None /* Option */ ,;
        ARCHIVE_FORMATS [ format ] [ 2 ] ) );
        formats . sort ( );
        FancyGetopt ( formats ) . print_help (;
        "List of available source distribution formats:" );
        class sdist ( Command ) ;
        description = "create a source distribution (tarball, zip file, etc.)";
        pub fn checking_metadata ( self )  {
        "Callable used for the check sub-command.

        Placed here so user_options can view it";
        return  self . metadata_check;
        user_options = [;
        ( "template=" , "t" ,;
        "name of manifest template file [default: MANIFEST.in]" ) ,;
        ( "manifest=" , "m" ,;
        "name of manifest file [default: MANIFEST]" ) ,;
        ( "use-defaults" , None /* Option */ ,;
        "include the default file set in the manifest ";
        "[default; disable with --no-defaults]" ) ,;
        ( "no-defaults" , None /* Option */ ,;
        "don't include the default file set" ) ,;
        ( "prune" , None /* Option */ ,;
        "specifically exclude files/directories that should !be ";
        "distributed (build tree, RCS/CVS dirs, etc.) ";
        "[default; disable with --no-prune]" ) ,;
        ( "no-prune" , None /* Option */ ,;
        "don't automatically exclude anything" ) ,;
        ( "manifest-only" , "o" ,;
        "just regenerate the manifest && then stop ";
        "(implies --force-manifest)" ) ,;
        ( "force-manifest" , "format!(" ,);
        "forcibly regenerate the manifest && carry on as usual. ";
        "Deprecated: now the manifest == always regenerated." ) ,;
        ( "formats=" , None /* Option */ ,;
        "formats for source distribution (comma-separated list)" ) ,;
        ( "keep-temp" , "k" ,;
        "keep the distribution tree around after creating " +;
        "archive file(s)" ) ,;
        ( "dist-dir=" , "d" ,;
        "directory to put the source distribution archive(s) in ";
        "[default: dist]" ) ,;
        ( "metadata-check" , None /* Option */ ,;
        "Ensure that all required elements of meta-data ";
        "are supplied. Warn if any missing. [default]" ) ,;
        ( "owner=" , "u" ,;
        "Owner name used when creating a tar file [default: current user]" ) ,;
        ( "group=" , "g" ,;
        "Group name used when creating a tar file [default: current group]" ) ,;
        ];
        boolean_options = [ "use-defaults" , "prune" ,;
        "manifest-only" , "force-manifest" ,;
        "keep-temp" , "metadata-check" ];
        help_options = [;
        ( "help-formats" , None /* Option */ ,;
        "list available distribution formats" , show_formats ) ,;
        ];
        negative_opt = { "no-defaults" : "use-defaults" ,;
        "no-prune" : "prune" };
        sub_commands = [ ( "check" , checking_metadata ) ];
        READMES = ( "README" , "README.txt" , "README.rst" );
        pub fn initialize_options ( self )  {
        self . template = None /* Option */;
        self . manifest = None /* Option */;
        self . use_defaults = 1;
        self . prune = 1;
        self . manifest_only = 0;
        self . force_manifest = 0;
        self . formats = [ "gztar" ];
        self . keep_temp = 0;
        self . dist_dir = None /* Option */;
        self . archive_files = None /* Option */;
        self . metadata_check = 1;
        self . owner = None /* Option */;
        self . group = None /* Option */;
        pub fn finalize_options ( self )  {
        if self . manifest is None /* Option */ {
        self . manifest = "MANIFEST";
        if self . template is None /* Option */ {
        self . template = "MANIFEST.in";
        self . ensure_string_list ( "formats" );
        bad_format = archive_util . check_archive_formats ( self . formats );
        if bad_format {
        panic!("DistutilsOptionError (");
        "unknown archive format '%s'" % bad_format );
        if self . dist_dir is None /* Option */ {
        self . dist_dir = "dist";
        pub fn run ( self )  {
        self . filelist = FileList ( );
        for cmd_name in self . get_sub_commands ( ) .iter() {
        self . run_command ( cmd_name );
        self . get_file_list ( );
        if self . manifest_only {
        return;
        self . make_distribution ( );
        pub fn check_metadata ( self )  {
        "Deprecated API.";
        warn ( "distutils.command.sdist.check_metadata == deprecated, \
              use the check command instead" , PendingDeprecationWarning );
        check = self . distribution . get_command_obj ( "check" );
        check . ensure_finalized ( );
        check . run ( );
        pub fn get_file_list ( self )  {
        "Figure out the list of files to include in the source
        distribution, && put it in 'self.filelist'.  This might involve
        reading the manifest template (and writing the manifest), || just
        reading the manifest, || just using the default file set -- it all
        depends on the user's options.
        ";
        template_exists = os . path . isfile ( self . template );
        if !template_exists && self . _manifest_is_not_generated ( ) {
        self . read_manifest ( );
        self . filelist . sort ( );
        self . filelist . remove_duplicates ( );
        return;
        if !template_exists {
        self . warn ( ( "manifest template '%s' does !exist " +;
        "(using default file list)" ) %;
        self . template );
        self . filelist . findall ( );
        if self . use_defaults {
        self . add_defaults ( );
        if template_exists {
        self . read_template ( );
        if self . prune {
        self . prune_file_list ( );
        self . filelist . sort ( );
        self . filelist . remove_duplicates ( );
        self . write_manifest ( );
        pub fn add_defaults ( self )  {
        "Add all the default files to self.filelist:
          - README || README.txt
          - setup.py
          - test/test*.py
          - all pure Python modules mentioned in setup script
          - all files pointed by package_data (build_py)
          - all files defined in data_files.
          - all files defined as scripts.
          - all C sources listed as part of extensions || C libraries
            in the setup script (doesn't catch C headers!)
        Warns if (README || README.txt) || setup.py are missing; everything
        else == optional.
        ";
        self . _add_defaults_standards ( );
        self . _add_defaults_optional ( );
        self . _add_defaults_python ( );
        self . _add_defaults_data_files ( );
        self . _add_defaults_ext ( );
        self . _add_defaults_c_libs ( );
        self . _add_defaults_scripts ( );
        @ staticmethod;
        pub fn _cs_path_exists ( fspath )  {
        "
        Case-sensitive path existence check

        >>> sdist._cs_path_exists(__file__)
        true
        >>> sdist._cs_path_exists(__file__.upper())
        false
        ";
        if !os . path . exists ( fspath ) {
        return  false;
        abspath = os . path . abspath ( fspath );
        directory , filename = os . path . split ( abspath );
        return  filename in os . listdir ( directory );
        pub fn _add_defaults_standards ( self )  {
        standards = [ self . READMES , self . distribution . script_name ];
        for fn in standards .iter() {
        if isinstance ( fn , tuple ) {
        alts = fn;
        got_it = false;
        for fn in alts .iter() {
        if self . _cs_path_exists ( fn ) {
        got_it = true;
        self . filelist . append ( fn );
        break;
        if !got_it {
        self . warn ( "standard file !found: should have one of " +;
        ", " . join ( alts ) );
        } else {
        if self . _cs_path_exists ( fn ) {
        self . filelist . append ( fn );
        } else {
        self . warn ( "standard file '%s' !found" % fn );
        pub fn _add_defaults_optional ( self )  {
        optional = [ "test/test*.py" , "setup.cfg" ];
        for pattern in optional .iter() {
        files = filter ( os . path . isfile , glob ( pattern ) );
        self . filelist . extend ( files );
        pub fn _add_defaults_python ( self )  {
        build_py = self . get_finalized_command ( "build_py" );
        if self . distribution . has_pure_modules ( ) {
        self . filelist . extend ( build_py . get_source_files ( ) );
        for pkg , src_dir , build_dir , filenames in build_py . data_files .iter() {
        for filename in filenames .iter() {
        self . filelist . append ( os . path . join ( src_dir , filename ) );
        pub fn _add_defaults_data_files ( self )  {
        if self . distribution . has_data_files ( ) {
        for item in self . distribution . data_files .iter() {
        if isinstance ( item , str ) {
        item = convert_path ( item );
        if os . path . isfile ( item ) {
        self . filelist . append ( item );
        } else {
        dirname , filenames = item;
        for f in filenames .iter() {
        f = convert_path ( f );
        if os . path . isfile ( f ) {
        self . filelist . append ( f );
        pub fn _add_defaults_ext ( self )  {
        if self . distribution . has_ext_modules ( ) {
        build_ext = self . get_finalized_command ( "build_ext" );
        self . filelist . extend ( build_ext . get_source_files ( ) );
        pub fn _add_defaults_c_libs ( self )  {
        if self . distribution . has_c_libraries ( ) {
        build_clib = self . get_finalized_command ( "build_clib" );
        self . filelist . extend ( build_clib . get_source_files ( ) );
        pub fn _add_defaults_scripts ( self )  {
        if self . distribution . has_scripts ( ) {
        build_scripts = self . get_finalized_command ( "build_scripts" );
        self . filelist . extend ( build_scripts . get_source_files ( ) );
        pub fn read_template ( self )  {
        "Read && parse manifest template file named by self.template.

        (usually "MANIFEST.in") The parsing && processing == done by
        'self.filelist', which updates itself accordingly.
        ";
        log . info ( "reading manifest template '%s'" , self . template );
        template = TextFile ( self . template , strip_comments = 1 , skip_blanks = 1 ,;
        join_lines = 1 , lstrip_ws = 1 , rstrip_ws = 1 ,;
        collapse_join = 1 );
        // try {
        while true  {
        line = template . readline ( );
        if line is None /* Option */ {
        break;
        // try {
        self . filelist . process_template_line ( line );
        // } catch  ( DistutilsTemplateError , ValueError ) as msg  {
        self . warn ( "%s, line %d: %s" % ( template . filename ,;
        template . current_line ,;
        msg ) );
        // } finally {
        template . close ( );
        pub fn prune_file_list ( self )  {
        "Prune off branches that might slip into the file list as created
        by 'read_template()', but really don't belong there:
          * the build tree (typically "build")
          * the release tree itself (only an issue if we ran "sdist"
            previously with --keep-temp, || it aborted)
          * any RCS, CVS, .svn, .hg, .git, .bzr, _darcs directories
        ";
        build = self . get_finalized_command ( "build" );
        base_dir = self . distribution . get_fullname ( );
        self . filelist . exclude_pattern ( None /* Option */ , prefix = build . build_base );
        self . filelist . exclude_pattern ( None /* Option */ , prefix = base_dir );
        if sys . platform == "win32" {
        seps = r "/|\\";
        } else {
        seps = "/";
        vcs_dirs = [ "RCS" , "CVS" , r "\.svn" , r "\.hg" , r "\.git" , r "\.bzr" ,;
        "_darcs" ];
        vcs_ptrn = r "(^|%s)(%s)(%s).*" % ( seps , "|" . join ( vcs_dirs ) , seps );
        self . filelist . exclude_pattern ( vcs_ptrn , is_regex = 1 );
        pub fn write_manifest ( self )  {
        "Write the file list in 'self.filelist' (presumably as filled in
        by 'add_defaults()' && 'read_template()') to the manifest file
        named by 'self.manifest'.
        ";
        if self . _manifest_is_not_generated ( ) {
        log . info ( "not writing to manually maintained ";
        "manifest file '%s'" % self . manifest );
        return;
        content = self . filelist . files [ : ];
        content . insert ( 0 , "# file GENERATED by distutils, do NOT edit" );
        self . execute ( file_util . write_file , ( self . manifest , content ) ,;
        "writing manifest file '%s'" % self . manifest );
        pub fn _manifest_is_not_generated ( self )  {
        if !os . path . isfile ( self . manifest ) {
        return  false;
        fp = open ( self . manifest );
        // try {
        first_line = fp . readline ( );
        // } finally {
        fp . close ( );
        return  first_line != "# file GENERATED by distutils, do NOT edit\n";
        pub fn read_manifest ( self )  {
        "Read the manifest file (named by 'self.manifest') && use it to
        fill in 'self.filelist', the list of files to include in the source
        distribution.
        ";
        log . info ( "reading manifest file '%s'" , self . manifest );
        // with scope: open ( self . manifest ) as manifest  {
        for line in manifest .iter() {
        line = line . strip ( );
        if line . startswith ( "#" ) || !line {
        continue;
        self . filelist . append ( line );
        pub fn make_release_tree ( &self, base_dir , files )  {
        "Create the directory tree that will become the source
        distribution archive.  All directories implied by the filenames in
        'files' are created under 'base_dir', && then we hard link || copy
        (if hard linking == unavailable) those files into place.
        Essentially, this duplicates the developer's source tree, but in a
        directory named after the distribution, containing only the files
        to be distributed.
        ";
        self . mkpath ( base_dir );
        dir_util . create_tree ( base_dir , files , dry_run = self . dry_run );
        if hasattr ( os , "link" ) {
        link = "hard";
        msg = "making hard links in %s..." % base_dir;
        } else {
        link = None /* Option */;
        msg = "copying files to %s..." % base_dir;
        if !files {
        log . warn ( "no files to distribute -- empty manifest?" );
        } else {
        log . info ( msg );
        for file in files .iter() {
        if !os . path . isfile ( file ) {
        log . warn ( "'%s' !a regular file -- skipping" , file );
        } else {
        dest = os . path . join ( base_dir , file );
        self . copy_file ( file , dest , link = link );
        self . distribution . metadata . write_pkg_info ( base_dir );
        pub fn make_distribution ( self )  {
        "Create the source distribution(s).  First, we create the release
        tree with 'make_release_tree()'; then, we create all required
        archive files (according to 'self.formats') from the release tree.
        Finally, we clean up by blowing away the release tree (unless
        'self.keep_temp' == true).  The list of archive files created is
        stored so it can be retrieved later by 'get_archive_files()'.
        ";
        base_dir = self . distribution . get_fullname ( );
        base_name = os . path . join ( self . dist_dir , base_dir );
        self . make_release_tree ( base_dir , self . filelist . files );
        archive_files = [ ];
        if "tar" in self . formats {
        self . formats . append ( self . formats . pop ( self . formats . index ( "tar" ) ) );
        for fmt in self . formats .iter() {
        file = self . make_archive ( base_name , fmt , base_dir = base_dir ,;
        owner = self . owner , group = self . group );
        archive_files . append ( file );
        self . distribution . dist_files . append ( ( "sdist" , "" , file ) );
        self . archive_files = archive_files;
        if !self . keep_temp {
        dir_util . remove_tree ( base_dir , dry_run = self . dry_run );
        pub fn get_archive_files ( self )  {
        "Return the list of archive files created when the command
        was run, || None /* Option */ if the command hasn't run yet.
        ";
        return  self . archive_files;
}

