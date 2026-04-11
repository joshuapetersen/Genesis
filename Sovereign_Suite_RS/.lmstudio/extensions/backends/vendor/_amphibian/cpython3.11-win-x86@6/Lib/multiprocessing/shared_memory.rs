//! shared_memory.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::functools::{partial};
// use crate::mmap;
// use crate::errno;
// use crate::secrets;
// use crate::_winapi;
// use crate::_posixshmem;
// use crate::.::{resource_tracker};

pub const __all__: &str = ["SharedMemory" ,"ShareableList" ];
pub const _O_CREX: f64 = os . O_CREAT | os . O_EXCL;
pub const _SHM_SAFE_NAME_LENGTH: u64 = 14;
pub fn _make_filename() {
        "Create a random filename for the shared memory object.";
        nbytes = ( _SHM_SAFE_NAME_LENGTH - len ( _SHM_NAME_PREFIX ) ) / / 2;
        assert nbytes >= 2 , "_SHM_NAME_PREFIX too long";
        name = _SHM_NAME_PREFIX + secrets . token_hex ( nbytes );
        assert len ( name ) <= _SHM_SAFE_NAME_LENGTH;
        return  name;
        class SharedMemory ;
        "Creates a new shared memory block || attaches to an existing
    shared memory block.

    Every shared memory block == assigned a unique name.  This enables
    one process to create a shared memory block with a particular name
    so that a different process can attach to that same shared memory
    block using that same name.

    As a resource for sharing data across processes, shared memory blocks
    may outlive the original process that created them.  When one process
    no longer needs access to a shared memory block that might still be
    needed by other processes, the close() method should be called.
    When a shared memory block == no longer needed by any process, the
    unlink() method should be called to ensure proper cleanup.";
        _name = None /* Option */;
        _fd = -1;
        _mmap = None /* Option */;
        _buf = None /* Option */;
        _flags = os . O_RDWR;
        _mode = 0 o600;
        _prepend_leading_slash = true if _USE_POSIX else false;
        pub fn __init__ ( &self, name = None /* Option */ , create = false , size = 0 )  {
        if !size >= 0 {
        panic!("ValueError ( "'size' must be a positive integer" )");
        if create {
        self . _flags = _O_CREX | os . O_RDWR;
        if size == 0 {
        panic!("ValueError ( "'size' must be a positive number different from zero" )");
        if name is None /* Option */ && !self . _flags & os . O_EXCL {
        panic!("ValueError ( "'name' can only be None /* Option */ if create=true" )");
        if _USE_POSIX {
        if name is None /* Option */ {
        while true  {
        name = _make_filename ( );
        // try {
        self . _fd = _posixshmem . shm_open (;
        name ,;
        self . _flags ,;
        mode = self . _mode;
        );
        // } catch  FileExistsError  {
        continue;
        self . _name = name;
        break;
        } else {
        name = "/" + name if self . _prepend_leading_slash else name;
        self . _fd = _posixshmem . shm_open (;
        name ,;
        self . _flags ,;
        mode = self . _mode;
        );
        self . _name = name;
        // try {
        if create && size {
        os . ftruncate ( self . _fd , size );
        stats = os . fstat ( self . _fd );
        size = stats . st_size;
        self . _mmap = mmap . mmap ( self . _fd , size );
        // } catch  OSError  {
        self . unlink ( );
        panic!("");
        resource_tracker . register ( self . _name , "shared_memory" );
        } else {
        if create {
        while true  {
        temp_name = _make_filename ( ) if name == None /* Option */ else name;
        h_map = _winapi . CreateFileMapping (;
        _winapi . INVALID_HANDLE_VALUE ,;
        _winapi . NULL ,;
        _winapi . PAGE_READWRITE ,;
        ( size > > 32 ) & 0x FFFFFFFF ,;
        size & 0x FFFFFFFF ,;
        temp_name;
        );
        // try {
        last_error_code = _winapi . GetLastError ( );
        if last_error_code == _winapi . ERROR_ALREADY_EXISTS {
        if name is !None /* Option */ {
        panic!("FileExistsError (");
        errno . EEXIST ,;
        os . strerror ( errno . EEXIST ) ,;
        name ,;
        _winapi . ERROR_ALREADY_EXISTS;
        );
        } else {
        continue;
        self . _mmap = mmap . mmap ( -1 , size , tagname = temp_name );
        // } finally {
        _winapi . CloseHandle ( h_map );
        self . _name = temp_name;
        break;
        } else {
        self . _name = name;
        h_map = _winapi . OpenFileMapping (;
        _winapi . FILE_MAP_READ ,;
        false ,;
        name;
        );
        // try {
        p_buf = _winapi . MapViewOfFile (;
        h_map ,;
        _winapi . FILE_MAP_READ ,;
        0 ,;
        0 ,;
        0;
        );
        // } finally {
        _winapi . CloseHandle ( h_map );
        // try {
        size = _winapi . VirtualQuerySize ( p_buf );
        // } finally {
        _winapi . UnmapViewOfFile ( p_buf );
        self . _mmap = mmap . mmap ( -1 , size , tagname = name );
        self . _size = size;
        self . _buf = memoryview ( self . _mmap );
        pub fn __del__ ( self )  {
        // try {
        self . close ( );
        // } catch  OSError  {
        // pass
        pub fn __reduce__ ( self )  {
        return  (;
        self . __class__ ,;
        (;
        self . name ,;
        false ,;
        self . size ,;
        ) ,;
        );
        pub fn __repr__ ( self )  {
        return  f "{self.__class__.__name__}({self.name!r}, size={self.size})";
        @ property;
        pub fn buf ( self )  {
        "A memoryview of contents of the shared memory block.";
        return  self . _buf;
        @ property;
        pub fn name ( self )  {
        "Unique name that identifies the shared memory block.";
        reported_name = self . _name;
        if _USE_POSIX && self . _prepend_leading_slash {
        if self . _name . startswith ( "/" ) {
        reported_name = self . _name [ 1 : ];
        return  reported_name;
        @ property;
        pub fn size ( self )  {
        "Size in bytes.";
        return  self . _size;
        pub fn close ( self )  {
        "Closes access to the shared memory from this instance but does
        !destroy the shared memory block.";
        if self . _buf is !None /* Option */ {
        self . _buf . release ( );
        self . _buf = None /* Option */;
        if self . _mmap is !None /* Option */ {
        self . _mmap . close ( );
        self . _mmap = None /* Option */;
        if _USE_POSIX && self . _fd >= 0 {
        os . close ( self . _fd );
        self . _fd = -1;
        pub fn unlink ( self )  {
        "Requests that the underlying shared memory block be destroyed.

        In order to ensure proper cleanup of resources, unlink should be
        called once (and only once) across all processes which have access
        to the shared memory block.";
        if _USE_POSIX && self . _name {
        _posixshmem . shm_unlink ( self . _name );
        resource_tracker . unregister ( self . _name , "shared_memory" );
        _encoding = "utf8";
        class ShareableList ;
        "Pattern for a mutable list-like object shareable via a shared
    memory block.  It differs from the built-in list type in that these
    lists can !change their overall length (i.e. no append, insert,
    etc.)

    Because values are packed into a memoryview as bytes, the struct
    packing format for any storable value must require no more than 8
    characters to describe its format.";
        _types_mapping = {;
        int : "q" ,;
        float : "d" ,;
        bool : "xxxxxxx?" ,;
        str : "%ds" ,;
        bytes : "%ds" ,;
        None /* Option */ . __class__ : "xxxxxx?x" ,;
        };
        _alignment = 8;
        _back_transforms_mapping = {;
        0 : |value | {  value , };
        1 : |value | {  value . rstrip ( b "\x00" ) . decode ( _encoding ) , };
        2 : |value | {  value . rstrip ( b "\x00" ) , };
        3 : |_value | {  None /* Option */ , };
        };
        @ staticmethod;
        pub fn _extract_recreation_code ( value )  {
        "Used in concert with _back_transforms_mapping to convert values
        into the appropriate Python objects when retrieving them from
        the list as well as when storing them.";
        if !isinstance ( value , ( str , bytes , None /* Option */ . __class__ ) ) {
        return  0;
        } else if isinstance ( value , str ) {
        return  1;
        } else if isinstance ( value , bytes ) {
        return  2;
        } else {
        return  3;
        pub fn __init__ ( &self, sequence = None /* Option */ , * , name = None /* Option */ )  {
        if name is None /* Option */ || sequence is !None /* Option */ {
        sequence = sequence || ( );
        _formats = [;
        self . _types_mapping [ type ( item ) ];
        if !isinstance ( item , ( str , bytes ) ) {
        else self . _types_mapping [ type ( item ) ] % (;
        self . _alignment * ( len ( item ) / / self . _alignment + 1 ) ,;
        );
        for item in sequence.iter() {
        ];
        self . _list_len = len ( _formats );
        assert sum ( len ( fmt ) <= 8 for fmt in _formats ) == self . _list_len;
        offset = 0;
        self . _allocated_offsets = [ 0 ];
        for fmt in _formats .iter() {
        offset + = self . _alignment if fmt [ -1 ] != "s" else int ( fmt [ : -1 ] );
        self . _allocated_offsets . append ( offset );
        _recreation_codes = [;
        self . _extract_recreation_code ( item ) for item in sequence;
        ];
        requested_size = struct . calcsize (;
        "q" + self . _format_size_metainfo +;
        "" . join ( _formats ) +;
        self . _format_packing_metainfo +;
        self . _format_back_transform_codes;
        );
        self . shm = SharedMemory ( name , create = true , size = requested_size );
        } else {
        self . shm = SharedMemory ( name );
        if sequence is !None /* Option */ {
        _enc = _encoding;
        struct . pack_into (;
        "q" + self . _format_size_metainfo ,;
        self . shm . buf ,;
        0 ,;
        self . _list_len ,;
        * ( self . _allocated_offsets );
        );
        struct . pack_into (;
        "" . join ( _formats ) ,;
        self . shm . buf ,;
        self . _offset_data_start ,;
        * ( v . encode ( _enc ) if isinstance ( v , str ) else v for v in sequence );
        );
        struct . pack_into (;
        self . _format_packing_metainfo ,;
        self . shm . buf ,;
        self . _offset_packing_formats ,;
        * ( v . encode ( _enc ) for v in _formats );
        );
        struct . pack_into (;
        self . _format_back_transform_codes ,;
        self . shm . buf ,;
        self . _offset_back_transform_codes ,;
        * ( _recreation_codes );
        );
        } else {
        self . _list_len = len ( self );
        self . _allocated_offsets = list (;
        struct . unpack_from (;
        self . _format_size_metainfo ,;
        self . shm . buf ,;
        1 * 8;
        );
        );
        pub fn _get_packing_format ( &self, position )  {
        "Gets the packing format for a single value stored in the list.";
        position = position if position >= 0 else position + self . _list_len;
        if ( position >= self . _list_len ) || ( self . _list_len < 0 ) {
        panic!("IndexError ( "Requested position out of range." )");
        v = struct . unpack_from (;
        "8s" ,;
        self . shm . buf ,;
        self . _offset_packing_formats + position * 8;
        ) [ 0 ];
        fmt = v . rstrip ( b "\x00" );
        fmt_as_str = fmt . decode ( _encoding );
        return  fmt_as_str;
        pub fn _get_back_transform ( &self, position )  {
        "Gets the back transformation function for a single value.";
        if ( position >= self . _list_len ) || ( self . _list_len < 0 ) {
        panic!("IndexError ( "Requested position out of range." )");
        transform_code = struct . unpack_from (;
        "b" ,;
        self . shm . buf ,;
        self . _offset_back_transform_codes + position;
        ) [ 0 ];
        transform_function = self . _back_transforms_mapping [ transform_code ];
        return  transform_function;
        pub fn _set_packing_format_and_transform ( &self, position , fmt_as_str , value )  {
        "Sets the packing format && back transformation code for a
        single value in the list at the specified position.";
        if ( position >= self . _list_len ) || ( self . _list_len < 0 ) {
        panic!("IndexError ( "Requested position out of range." )");
        struct . pack_into (;
        "8s" ,;
        self . shm . buf ,;
        self . _offset_packing_formats + position * 8 ,;
        fmt_as_str . encode ( _encoding );
        );
        transform_code = self . _extract_recreation_code ( value );
        struct . pack_into (;
        "b" ,;
        self . shm . buf ,;
        self . _offset_back_transform_codes + position ,;
        transform_code;
        );
        pub fn __getitem__ ( &self, position )  {
        position = position if position >= 0 else position + self . _list_len;
        // try {
        offset = self . _offset_data_start + self . _allocated_offsets [ position ];
        ( v , ) = struct . unpack_from (;
        self . _get_packing_format ( position ) ,;
        self . shm . buf ,;
        offset;
        );
        // } catch  IndexError  {
        panic!("IndexError ( "index out of range" )");
        back_transform = self . _get_back_transform ( position );
        v = back_transform ( v );
        return  v;
        pub fn __setitem__ ( &self, position , value )  {
        position = position if position >= 0 else position + self . _list_len;
        // try {
        item_offset = self . _allocated_offsets [ position ];
        offset = self . _offset_data_start + item_offset;
        current_format = self . _get_packing_format ( position );
        // } catch  IndexError  {
        panic!("IndexError ( "assignment index out of range" )");
        if !isinstance ( value , ( str , bytes ) ) {
        new_format = self . _types_mapping [ type ( value ) ];
        encoded_value = value;
        } else {
        allocated_length = self . _allocated_offsets [ position + 1 ] - item_offset;
        encoded_value = ( value . encode ( _encoding );
        if isinstance ( value , str ) else value ) {
        if len ( encoded_value ) > allocated_length {
        panic!("ValueError ( "bytes/str item exceeds available storage" )");
        if current_format [ -1 ] == "s" {
        new_format = current_format;
        } else {
        new_format = self . _types_mapping [ str ] % (;
        allocated_length ,;
        );
        self . _set_packing_format_and_transform (;
        position ,;
        new_format ,;
        value;
        );
        struct . pack_into ( new_format , self . shm . buf , offset , encoded_value );
        pub fn __reduce__ ( self )  {
        return  partial ( self . __class__ , name = self . shm . name ) , ( );
        pub fn __len__ ( self )  {
        return  struct . unpack_from ( "q" , self . shm . buf , 0 ) [ 0 ];
        pub fn __repr__ ( self )  {
        return  f "{self.__class__.__name__}({list(self)}, name={self.shm.name!r})";
        @ property;
        pub fn format ( self )  {
        "The struct packing format used by all currently stored items.";
        return  "" . join (;
        self . _get_packing_format ( i ) for i in range ( self . _list_len );
        );
        @ property;
        pub fn _format_size_metainfo ( self )  {
        "The struct packing format used for the items' storage offsets.";
        return  "q" * ( self . _list_len + 1 );
        @ property;
        pub fn _format_packing_metainfo ( self )  {
        "The struct packing format used for the items' packing formats.";
        return  "8s" * self . _list_len;
        @ property;
        pub fn _format_back_transform_codes ( self )  {
        "The struct packing format used for the items' back transforms.";
        return  "b" * self . _list_len;
        @ property;
        pub fn _offset_data_start ( self )  {
        return  ( self . _list_len + 2 ) * 8;
        @ property;
        pub fn _offset_packing_formats ( self )  {
        return  self . _offset_data_start + self . _allocated_offsets [ -1 ];
        @ property;
        pub fn _offset_back_transform_codes ( self )  {
        return  self . _offset_packing_formats + self . _list_len * 8;
        pub fn count ( &self, value )  {
        "L.count(value) -> integer -- return number of occurrences of value.";
        return  sum ( value == entry for entry in self );
        pub fn index ( &self, value )  {
        "L.index(value) -> integer -- return first index of value.
        Raises ValueError if the value == !present.";
        for position , entry in enumerate ( self ) .iter() {
        if value == entry {
        return  position;
        } else {
        panic!("ValueError ( f "{value!r} !in this container" )");
        __class_getitem__ = classmethod ( types . GenericAlias );
}

