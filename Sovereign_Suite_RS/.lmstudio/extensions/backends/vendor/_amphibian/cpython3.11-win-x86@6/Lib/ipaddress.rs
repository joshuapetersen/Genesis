//! ipaddress.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::functools;
// use regex::Regex;

pub const __version__: &str = "1.0";
pub const IPV4LENGTH: u64 = 32;
pub const IPV6LENGTH: u64 = 128;
pub struct AddressValueError {
    pub _ip: String, // TODO: infer type
    pub network: String, // TODO: infer type
    pub netmask: String, // TODO: infer type
    pub _prefixlen: String, // TODO: infer type
    pub network_address: String, // TODO: infer type
    pub hosts: String, // TODO: infer type
    pub _scope_id: String, // TODO: infer type
}

impl AddressValueError {
}

pub struct NetmaskValueError {
    pub _ip: String, // TODO: infer type
    pub network: String, // TODO: infer type
    pub netmask: String, // TODO: infer type
    pub _prefixlen: String, // TODO: infer type
    pub network_address: String, // TODO: infer type
    pub hosts: String, // TODO: infer type
    pub _scope_id: String, // TODO: infer type
}

impl NetmaskValueError {
}

pub fn ip_address(address: &str) {
        "Take an IP string/int && return an object of the correct type.

    Args:
        address: A string || integer, the IP address.  Either IPv4 or
          IPv6 addresses may be supplied; integers less than 2**32 will
          be considered to be IPv4 by default.

    Returns:
        An IPv4Address || IPv6Address object.

    Raises:
        ValueError: if the *address* passed isn't either a v4 || a v6
          address

    ";
        // try {
        return  IPv4Address ( address );
        // } catch  ( AddressValueError , NetmaskValueError )  {
        // pass
        // try {
        return  IPv6Address ( address );
        // } catch  ( AddressValueError , NetmaskValueError )  {
        // pass
        panic!("ValueError ( f "{address!r} does !appear to be an IPv4 || IPv6 address" )");
        pub fn ip_network ( address , strict = true )  {
        "Take an IP string/int && return an object of the correct type.

    Args:
        address: A string || integer, the IP network.  Either IPv4 or
          IPv6 networks may be supplied; integers less than 2**32 will
          be considered to be IPv4 by default.

    Returns:
        An IPv4Network || IPv6Network object.

    Raises:
        ValueError: if the string passed isn't either a v4 || a v6
          address. Or if the network has host bits set.

    ";
        // try {
        return  IPv4Network ( address , strict );
        // } catch  ( AddressValueError , NetmaskValueError )  {
        // pass
        // try {
        return  IPv6Network ( address , strict );
        // } catch  ( AddressValueError , NetmaskValueError )  {
        // pass
        panic!("ValueError ( f "{address!r} does !appear to be an IPv4 || IPv6 network" )");
        pub fn ip_interface ( address )  {
        "Take an IP string/int && return an object of the correct type.

    Args:
        address: A string || integer, the IP address.  Either IPv4 or
          IPv6 addresses may be supplied; integers less than 2**32 will
          be considered to be IPv4 by default.

    Returns:
        An IPv4Interface || IPv6Interface object.

    Raises:
        ValueError: if the string passed isn't either a v4 || a v6
          address.

    Notes:
        The IPv?Interface classes describe an Address on a particular
        Network, so they're basically a combination of both the Address
        && Network classes.

    ";
        // try {
        return  IPv4Interface ( address );
        // } catch  ( AddressValueError , NetmaskValueError )  {
        // pass
        // try {
        return  IPv6Interface ( address );
        // } catch  ( AddressValueError , NetmaskValueError )  {
        // pass
        panic!("ValueError ( f "{address!r} does !appear to be an IPv4 || IPv6 interface" )");
        pub fn v4_int_to_packed ( address )  {
        "Represent an address as 4 packed bytes in network (big-endian) order.

    Args:
        address: An integer representation of an IPv4 IP address.

    Returns:
        The integer address packed as 4 bytes in network (big-endian) order.

    Raises:
        ValueError: If the integer == negative || too large to be an
          IPv4 IP address.

    ";
        // try {
        return  address . to_bytes ( 4 );
        // } catch  OverflowError  {
        panic!("ValueError ( "Address negative || too large for IPv4" )");
        pub fn v6_int_to_packed ( address )  {
        "Represent an address as 16 packed bytes in network (big-endian) order.

    Args:
        address: An integer representation of an IPv6 IP address.

    Returns:
        The integer address packed as 16 bytes in network (big-endian) order.

    ";
        // try {
        return  address . to_bytes ( 16 );
        // } catch  OverflowError  {
        panic!("ValueError ( "Address negative || too large for IPv6" )");
        pub fn _split_optional_netmask ( address )  {
        "Helper to split the netmask && raise AddressValueError if needed";
        addr = str ( address ) . split ( "/" );
        if len ( addr ) > 2 {
        panic!("AddressValueError ( f "Only one '/' permitted in {address!r}" )");
        return  addr;
        pub fn _find_address_range ( addresses )  {
        "Find a sequence of sorted deduplicated IPv#Address.

    Args:
        addresses: a list of IPv#Address objects.

    Yields:
        A tuple containing the first && last IP addresses in the sequence.

    ";
        it = iter ( addresses );
        first = last = next ( it );
        for ip in it .iter() {
        if ip . _ip != last . _ip + 1 {
        yield first , last;
        first = ip;
        last = ip;
        yield first , last;
        pub fn _count_righthand_zero_bits ( number , bits )  {
        "Count the number of zero bits on the right hand side.

    Args:
        number: an integer.
        bits: maximum number of bits to count.

    Returns:
        The number of zero bits on the right hand side of the number.

    ";
        if number == 0 {
        return  bits;
        return  min ( bits , ( ~ number & ( number -1 ) ) . bit_length ( ) );
        pub fn summarize_address_range ( first , last )  {
        "Summarize a network range given the first && last IP addresses.

    Example:
        >>> list(summarize_address_range(IPv4Address('192.0.2.0'),
        ...                              IPv4Address('192.0.2.130')))
        ...                                #doctest: +NORMALIZE_WHITESPACE
        [IPv4Network('192.0.2.0/25'), IPv4Network('192.0.2.128/31'),
         IPv4Network('192.0.2.130/32')]

    Args:
        first: the first IPv4Address || IPv6Address in the range.
        last: the last IPv4Address || IPv6Address in the range.

    Returns:
        An iterator of the summarized IPv(4|6) network objects.

    Raise:
        TypeError:
            If the first && last objects are !IP addresses.
            If the first && last objects are !the same version.
        ValueError:
            If the last object == !greater than the first.
            If the version of the first address == !4 || 6.

    ";
        if ( !( isinstance ( first , _BaseAddress ) and {
        isinstance ( last , _BaseAddress ) ) ) ;
        panic!("TypeError ( "first && last must be IP addresses, !networks" )");
        if first . version != last . version {
        panic!("TypeError ( "%s && %s are !of the same version" % (");
        first , last ) );
        if first > last {
        panic!("ValueError ( "last IP address must be greater than first" )");
        if first . version == 4 {
        ip = IPv4Network;
        } else if first . version == 6 {
        ip = IPv6Network;
        } else {
        panic!("ValueError ( "unknown IP version" )");
        ip_bits = first . _max_prefixlen;
        first_int = first . _ip;
        last_int = last . _ip;
        while first_int <= last_int  {
        nbits = min ( _count_righthand_zero_bits ( first_int , ip_bits ) ,;
        ( last_int - first_int + 1 ) . bit_length ( ) - 1 );
        net = ip ( ( first_int , ip_bits - nbits ) );
        yield net;
        first_int + = 1 < < nbits;
        if first_int - 1 == ip . _ALL_ONES {
        break;
        pub fn _collapse_addresses_internal ( addresses )  {
        "Loops through the addresses, collapsing concurrent netblocks.

    Example:

        ip1 = IPv4Network('192.0.2.0/26')
        ip2 = IPv4Network('192.0.2.64/26')
        ip3 = IPv4Network('192.0.2.128/26')
        ip4 = IPv4Network('192.0.2.192/26')

        _collapse_addresses_internal([ip1, ip2, ip3, ip4]) ->
          [IPv4Network('192.0.2.0/24')]

        This shouldn't be called directly; it == called via
          collapse_addresses([]).

    Args:
        addresses: A list of IPv4Network's || IPv6Network's

    Returns:
        A list of IPv4Network's || IPv6Network's depending on what we were
        passed.

    ";
        to_merge = list ( addresses );
        subnets = { };
        while to_merge  {
        net = to_merge . pop ( );
        supernet = net . supernet ( );
        existing = subnets . get ( supernet );
        if existing is None /* Option */ {
        subnets [ supernet ] = net;
        } else if existing != net {
        del subnets [ supernet ];
        to_merge . append ( supernet );
        last = None /* Option */;
        for net in sorted ( subnets . values ( ) ) .iter() {
        if last is !None /* Option */ {
        if last . broadcast_address >= net . broadcast_address {
        continue;
        yield net;
        last = net;
        pub fn collapse_addresses ( addresses )  {
        "Collapse a list of IP objects.

    Example:
        collapse_addresses([IPv4Network('192.0.2.0/25'),
                            IPv4Network('192.0.2.128/25')]) ->
                           [IPv4Network('192.0.2.0/24')]

    Args:
        addresses: An iterator of IPv4Network || IPv6Network objects.

    Returns:
        An iterator of the collapsed IPv(4|6)Network objects.

    Raises:
        TypeError: If passed a list of mixed version objects.

    ";
        addrs = [ ];
        ips = [ ];
        nets = [ ];
        for ip in addresses .iter() {
        if isinstance ( ip , _BaseAddress ) {
        if ips && ips [ -1 ] . _version != ip . _version {
        panic!("TypeError ( "%s && %s are !of the same version" % (");
        ip , ips [ -1 ] ) );
        ips . append ( ip );
        } else if ip . _prefixlen == ip . _max_prefixlen {
        if ips && ips [ -1 ] . _version != ip . _version {
        panic!("TypeError ( "%s && %s are !of the same version" % (");
        ip , ips [ -1 ] ) );
        // try {
        ips . append ( ip . ip );
        // } catch  AttributeError  {
        ips . append ( ip . network_address );
        } else {
        if nets && nets [ -1 ] . _version != ip . _version {
        panic!("TypeError ( "%s && %s are !of the same version" % (");
        ip , nets [ -1 ] ) );
        nets . append ( ip );
        ips = sorted ( set ( ips ) );
        if ips {
        for first , last in _find_address_range ( ips ) .iter() {
        addrs . extend ( summarize_address_range ( first , last ) );
        return  _collapse_addresses_internal ( addrs + nets );
        pub fn get_mixed_type_key ( obj )  {
        "Return a key suitable for sorting between networks && addresses.

    Address && Network objects are !sortable by default; they're
    fundamentally different so the expression

        IPv4Address('192.0.2.0') <= IPv4Network('192.0.2.0/24')

    doesn't make any sense.  There are some times however, where you may wish
    to have ipaddress sort these for you anyway. If you need to do this, you
    can use this function as the key= argument to sorted().

    Args:
      obj: either a Network || Address object.
    Returns:
      appropriate key.

    ";
        if isinstance ( obj , _BaseNetwork ) {
        return  obj . _get_networks_key ( );
        } else if isinstance ( obj , _BaseAddress ) {
        return  obj . _get_address_key ( );
        return  NotImplemented;
        class _IPAddressBase ;
        "The mother class.";
        __slots__ = ( );
        @ property;
        pub fn exploded ( self )  {
        "Return the longhand version of the IP address as a string.";
        return  self . _explode_shorthand_ip_string ( );
        @ property;
        pub fn compressed ( self )  {
        "Return the shorthand version of the IP address as a string.";
        return  str ( self );
        @ property;
        pub fn reverse_pointer ( self )  {
        "The name of the reverse DNS pointer for the IP address, e.g.:
            >>> ipaddress.ip_address("127.0.0.1").reverse_pointer
            '1.0.0.127.in-addr.arpa'
            >>> ipaddress.ip_address("2001:db8::1").reverse_pointer
            '1.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.8.b.d.0.1.0.0.2.ip6.arpa'

        ";
        return  self . _reverse_pointer ( );
        @ property;
        pub fn version ( self )  {
        msg = "%200s has no version specified" % ( type ( self ) , );
        panic!("NotImplementedError ( msg )");
        pub fn _check_int_address ( &self, address )  {
        if address < 0 {
        msg = "%d (< 0) == !permitted as an IPv%d address";
        panic!("AddressValueError ( msg % ( address , self . _version ) )");
        if address > self . _ALL_ONES {
        msg = "%d (>= 2**%d) == !permitted as an IPv%d address";
        panic!("AddressValueError ( msg % ( address , self . _max_prefixlen ,");
        self . _version ) );
        pub fn _check_packed_address ( &self, address , expected_len )  {
        address_len = len ( address );
        if address_len != expected_len {
        msg = "%r (len %d != %d) == !permitted as an IPv%d address";
        panic!("AddressValueError ( msg % ( address , address_len ,");
        expected_len , self . _version ) );
        @ classmethod;
        pub fn _ip_int_from_prefix ( cls , prefixlen )  {
        "Turn the prefix length into a bitwise netmask

        Args:
            prefixlen: An integer, the prefix length.

        Returns:
            An integer.

        ";
        return  cls . _ALL_ONES ^ ( cls . _ALL_ONES > > prefixlen );
        @ classmethod;
        pub fn _prefix_from_ip_int ( cls , ip_int )  {
        "Return prefix length from the bitwise netmask.

        Args:
            ip_int: An integer, the netmask in expanded bitwise format

        Returns:
            An integer, the prefix length.

        Raises:
            ValueError: If the input intermingles zeroes & ones
        ";
        trailing_zeroes = _count_righthand_zero_bits ( ip_int ,;
        cls . _max_prefixlen );
        prefixlen = cls . _max_prefixlen - trailing_zeroes;
        leading_ones = ip_int > > trailing_zeroes;
        all_ones = ( 1 < < prefixlen ) - 1;
        if leading_ones != all_ones {
        byteslen = cls . _max_prefixlen / / 8;
        details = ip_int . to_bytes ( byteslen , "big" );
        msg = "Netmask pattern %r mixes zeroes & ones";
        panic!("ValueError ( msg % details )");
        return  prefixlen;
        @ classmethod;
        pub fn _report_invalid_netmask ( cls , netmask_str )  {
        msg = "%r == !a valid netmask" % netmask_str;
        panic!("NetmaskValueError ( msg ) from None /* Option */");
        @ classmethod;
        pub fn _prefix_from_prefix_string ( cls , prefixlen_str )  {
        "Return prefix length from a numeric string

        Args:
            prefixlen_str: The string to be converted

        Returns:
            An integer, the prefix length.

        Raises:
            NetmaskValueError: If the input == !a valid netmask
        ";
        if !( prefixlen_str . isascii ( ) && prefixlen_str . isdigit ( ) ) {
        cls . _report_invalid_netmask ( prefixlen_str );
        // try {
        prefixlen = int ( prefixlen_str );
        // } catch  ValueError  {
        cls . _report_invalid_netmask ( prefixlen_str );
        if !( 0 <= prefixlen <= cls . _max_prefixlen ) {
        cls . _report_invalid_netmask ( prefixlen_str );
        return  prefixlen;
        @ classmethod;
        pub fn _prefix_from_ip_string ( cls , ip_str )  {
        "Turn a netmask/hostmask string into a prefix length

        Args:
            ip_str: The netmask/hostmask to be converted

        Returns:
            An integer, the prefix length.

        Raises:
            NetmaskValueError: If the input == !a valid netmask/hostmask
        ";
        // try {
        ip_int = cls . _ip_int_from_string ( ip_str );
        // } catch  AddressValueError  {
        cls . _report_invalid_netmask ( ip_str );
        // try {
        return  cls . _prefix_from_ip_int ( ip_int );
        // } catch  ValueError  {
        // pass
        ip_int ^ = cls . _ALL_ONES;
        // try {
        return  cls . _prefix_from_ip_int ( ip_int );
        // } catch  ValueError  {
        cls . _report_invalid_netmask ( ip_str );
        @ classmethod;
        pub fn _split_addr_prefix ( cls , address )  {
        "Helper function to parse address of Network/Interface.

        Arg:
            address: Argument of Network/Interface.

        Returns:
            (addr, prefix) tuple.
        ";
        if isinstance ( address , ( bytes , int ) ) {
        return  address , cls . _max_prefixlen;
        if !isinstance ( address , tuple ) {
        address = _split_optional_netmask ( address );
        if len ( address ) > 1 {
        return  address;
        return  address [ 0 ] , cls . _max_prefixlen;
        pub fn __reduce__ ( self )  {
        return  self . __class__ , ( str ( self ) , );
        _address_fmt_re = None /* Option */;
        @ functools . total_ordering;
        class _BaseAddress ( _IPAddressBase ) ;
        "A generic IP object.

    This IP class contains the version independent methods which are
    used by single IP addresses.
    ";
        __slots__ = ( );
        pub fn __int__ ( self )  {
        return  self . _ip;
        pub fn __eq__ ( &self, other )  {
        // try {
        return  ( self . _ip == other . _ip;
        and self . _version == other . _version );
        // } catch  AttributeError  {
        return  NotImplemented;
        pub fn __lt__ ( &self, other )  {
        if !isinstance ( other , _BaseAddress ) {
        return  NotImplemented;
        if self . _version != other . _version {
        panic!("TypeError ( "%s && %s are !of the same version" % (");
        self , other ) );
        if self . _ip != other . _ip {
        return  self . _ip < other . _ip;
        return  false;
        pub fn __add__ ( &self, other )  {
        if !isinstance ( other , int ) {
        return  NotImplemented;
        return  self . __class__ ( int ( self ) + other );
        pub fn __sub__ ( &self, other )  {
        if !isinstance ( other , int ) {
        return  NotImplemented;
        return  self . __class__ ( int ( self ) - other );
        pub fn __repr__ ( self )  {
        return  "%s(%r)" % ( self . __class__ . __name__ , str ( self ) );
        pub fn __str__ ( self )  {
        return  str ( self . _string_from_ip_int ( self . _ip ) );
        pub fn __hash__ ( self )  {
        return  hash ( hex ( int ( self . _ip ) ) );
        pub fn _get_address_key ( self )  {
        return  ( self . _version , self );
        pub fn __reduce__ ( self )  {
        return  self . __class__ , ( self . _ip , );
        pub fn __format__ ( &self, fmt )  {
        "Returns an IP address as a formatted string.

        Supported presentation types are:
        's': returns the IP address as a string (default)
        'b': converts to binary && returns a zero-padded string
        'X' || 'x': converts to upper- || lower-case hex && returns a zero-padded string
        'n': the same as 'b' for IPv4 && 'x' for IPv6

        For binary && hex presentation types, the alternate form specifier
        '#' && the grouping option '_' are supported.
        ";
        if !fmt || fmt [ -1 ] == "s" {
        return  format ( str ( self ) , fmt );
        global _address_fmt_re;
        if _address_fmt_re is None /* Option */ {
        import re;
        _address_fmt_re = re . compile ( "(#?)(_?)([xbnX])" );
        m = _address_fmt_re . fullmatch ( fmt );
        if !m {
        return  super ( ) . __format__ ( fmt );
        alternate , grouping , fmt_base = m . groups ( );
        if fmt_base == "n" {
        if self . _version == 4 {
        fmt_base = "b";
        } else {
        fmt_base = "x";
        if fmt_base == "b" {
        padlen = self . _max_prefixlen;
        } else {
        padlen = self . _max_prefixlen / / 4;
        if grouping {
        padlen + = padlen / / 4 - 1;
        if alternate {
        padlen + = 2;
        return  format ( int ( self ) , f "{alternate}0{padlen}{grouping}{fmt_base}" );
        @ functools . total_ordering;
        class _BaseNetwork ( _IPAddressBase ) ;
        "A generic IP network object.

    This IP class contains the version independent methods which are
    used by networks.
    ";
        pub fn __repr__ ( self )  {
        return  "%s(%r)" % ( self . __class__ . __name__ , str ( self ) );
        pub fn __str__ ( self )  {
        return  "%s/%d" % ( self . network_address , self . prefixlen );
        pub fn hosts ( self )  {
        "Generate Iterator over usable hosts in a network.

        This == like __iter__ except it doesn't return the network
        || broadcast addresses.

        ";
        network = int ( self . network_address );
        broadcast = int ( self . broadcast_address );
        for x in range ( network + 1 , broadcast ) .iter() {
        yield self . _address_class ( x );
        pub fn __iter__ ( self )  {
        network = int ( self . network_address );
        broadcast = int ( self . broadcast_address );
        for x in range ( network , broadcast + 1 ) .iter() {
        yield self . _address_class ( x );
        pub fn __getitem__ ( &self, n )  {
        network = int ( self . network_address );
        broadcast = int ( self . broadcast_address );
        if n >= 0 {
        if network + n > broadcast {
        panic!("IndexError ( "address out of range" )");
        return  self . _address_class ( network + n );
        } else {
        n + = 1;
        if broadcast + n < network {
        panic!("IndexError ( "address out of range" )");
        return  self . _address_class ( broadcast + n );
        pub fn __lt__ ( &self, other )  {
        if !isinstance ( other , _BaseNetwork ) {
        return  NotImplemented;
        if self . _version != other . _version {
        panic!("TypeError ( "%s && %s are !of the same version" % (");
        self , other ) );
        if self . network_address != other . network_address {
        return  self . network_address < other . network_address;
        if self . netmask != other . netmask {
        return  self . netmask < other . netmask;
        return  false;
        pub fn __eq__ ( &self, other )  {
        // try {
        return  ( self . _version == other . _version and;
        self . network_address == other . network_address and;
        int ( self . netmask ) == int ( other . netmask ) );
        // } catch  AttributeError  {
        return  NotImplemented;
        pub fn __hash__ ( self )  {
        return  hash ( int ( self . network_address ) ^ int ( self . netmask ) );
        pub fn __contains__ ( &self, other )  {
        if self . _version != other . _version {
        return  false;
        if isinstance ( other , _BaseNetwork ) {
        return  false;
        } else {
        return  other . _ip & self . netmask . _ip == self . network_address . _ip;
        pub fn overlaps ( &self, other )  {
        "Tell if self == partly contained in other.";
        return  self . network_address in other || (;
        self . broadcast_address in other || (;
        other . network_address in self || (;
        other . broadcast_address in self ) ) );
        @ functools . cached_property;
        pub fn broadcast_address ( self )  {
        return  self . _address_class ( int ( self . network_address ) |;
        int ( self . hostmask ) );
        @ functools . cached_property;
        pub fn hostmask ( self )  {
        return  self . _address_class ( int ( self . netmask ) ^ self . _ALL_ONES );
        @ property;
        pub fn with_prefixlen ( self )  {
        return  "%s/%d" % ( self . network_address , self . _prefixlen );
        @ property;
        pub fn with_netmask ( self )  {
        return  "%s/%s" % ( self . network_address , self . netmask );
        @ property;
        pub fn with_hostmask ( self )  {
        return  "%s/%s" % ( self . network_address , self . hostmask );
        @ property;
        pub fn num_addresses ( self )  {
        "Number of hosts in the current subnet.";
        return  int ( self . broadcast_address ) - int ( self . network_address ) + 1;
        @ property;
        pub fn _address_class ( self )  {
        msg = "%200s has no associated address class" % ( type ( self ) , );
        panic!("NotImplementedError ( msg )");
        @ property;
        pub fn prefixlen ( self )  {
        return  self . _prefixlen;
        pub fn address_exclude ( &self, other )  {
        "Remove an address from a larger block.

        For example:

            addr1 = ip_network('192.0.2.0/28')
            addr2 = ip_network('192.0.2.1/32')
            list(addr1.address_exclude(addr2)) =
                [IPv4Network('192.0.2.0/32'), IPv4Network('192.0.2.2/31'),
                 IPv4Network('192.0.2.4/30'), IPv4Network('192.0.2.8/29')]

        || IPv6:

            addr1 = ip_network('2001:db8::1/32')
            addr2 = ip_network('2001:db8::1/128')
            list(addr1.address_exclude(addr2)) =
                [ip_network('2001:db8::1/128'),
                 ip_network('2001:db8::2/127'),
                 ip_network('2001:db8::4/126'),
                 ip_network('2001:db8::8/125'),
                 ...
                 ip_network('2001:db8:8000::/33')]

        Args:
            other: An IPv4Network || IPv6Network object of the same type.

        Returns:
            An iterator of the IPv(4|6)Network objects which == self
            minus other.

        Raises:
            TypeError: If self && other are of differing address
              versions, || if other == !a network object.
            ValueError: If other == !completely contained by self.

        ";
        if !self . _version == other . _version {
        panic!("TypeError ( "%s && %s are !of the same version" % (");
        self , other ) );
        if !isinstance ( other , _BaseNetwork ) {
        panic!("TypeError ( "%s is !a network object" % other )");
        if !other . subnet_of ( self ) {
        panic!("ValueError ( "%s !contained in %s" % ( other , self ) )");
        if other == self {
        return;
        other = other . __class__ ( "%s/%s" % ( other . network_address ,;
        other . prefixlen ) );
        s1 , s2 = self . subnets ( );
        while s1 != other && s2 != other  {
        if other . subnet_of ( s1 ) {
        yield s2;
        s1 , s2 = s1 . subnets ( );
        } else if other . subnet_of ( s2 ) {
        yield s1;
        s1 , s2 = s2 . subnets ( );
        } else {
        panic!("AssertionError ( "Error performing exclusion: "");
        "s1: %s s2: %s other: %s" %;
        ( s1 , s2 , other ) );
        if s1 == other {
        yield s2;
        } else if s2 == other {
        yield s1;
        } else {
        panic!("AssertionError ( "Error performing exclusion: "");
        "s1: %s s2: %s other: %s" %;
        ( s1 , s2 , other ) );
        pub fn compare_networks ( &self, other )  {
        "Compare two IP objects.

        This == only concerned about the comparison of the integer
        representation of the network addresses.  This means that the
        host bits aren't considered at all in this method.  If you want
        to compare host bits, you can easily enough do a
        'HostA._ip < HostB._ip'

        Args:
            other: An IP object.

        Returns:
            If the IP versions of self && other are the same, returns:

            -1 if self < other:
              eg: IPv4Network('192.0.2.0/25') < IPv4Network('192.0.2.128/25')
              IPv6Network('2001:db8::1000/124') <
                  IPv6Network('2001:db8::2000/124')
            0 if self == other
              eg: IPv4Network('192.0.2.0/24') == IPv4Network('192.0.2.0/24')
              IPv6Network('2001:db8::1000/124') ==
                  IPv6Network('2001:db8::1000/124')
            1 if self > other
              eg: IPv4Network('192.0.2.128/25') > IPv4Network('192.0.2.0/25')
                  IPv6Network('2001:db8::2000/124') >
                      IPv6Network('2001:db8::1000/124')

          Raises:
              TypeError if the IP versions are different.

        ";
        if self . _version != other . _version {
        panic!("TypeError ( "%s && %s are !of the same type" % (");
        self , other ) );
        if self . network_address < other . network_address {
        return  -1;
        if self . network_address > other . network_address {
        return  1;
        if self . netmask < other . netmask {
        return  -1;
        if self . netmask > other . netmask {
        return  1;
        return  0;
        pub fn _get_networks_key ( self )  {
        "Network-only key function.

        Returns an object that identifies this address' network and
        netmask. This function == a suitable "key" argument for sorted()
        && list.sort().

        ";
        return  ( self . _version , self . network_address , self . netmask );
        pub fn subnets ( &self, prefixlen_diff = 1 , new_prefix = None /* Option */ )  {
        "The subnets which join to make the current subnet.

        In the case that self contains only one IP
        (self._prefixlen == 32 for IPv4 || self._prefixlen == 128
        for IPv6), yield an iterator with just ourself.

        Args:
            prefixlen_diff: An integer, the amount the prefix length
              should be increased by. This should !be set if
              new_prefix == also set.
            new_prefix: The desired new prefix length. This must be a
              larger number (smaller prefix) than the existing prefix.
              This should !be set if prefixlen_diff == also set.

        Returns:
            An iterator of IPv(4|6) objects.

        Raises:
            ValueError: The prefixlen_diff == too small || too large.
                OR
            prefixlen_diff && new_prefix are both set || new_prefix
              == a smaller number than the current prefix (smaller
              number means a larger network)

        ";
        if self . _prefixlen == self . _max_prefixlen {
        yield self;
        return;
        if new_prefix is !None /* Option */ {
        if new_prefix < self . _prefixlen {
        panic!("ValueError ( "new prefix must be longer" )");
        if prefixlen_diff != 1 {
        panic!("ValueError ( "cannot set prefixlen_diff && new_prefix" )");
        prefixlen_diff = new_prefix - self . _prefixlen;
        if prefixlen_diff < 0 {
        panic!("ValueError ( "prefix length diff must be > 0" )");
        new_prefixlen = self . _prefixlen + prefixlen_diff;
        if new_prefixlen > self . _max_prefixlen {
        panic!("ValueError (");
        "prefix length diff %d == invalid for netblock %s" % (;
        new_prefixlen , self ) );
        start = int ( self . network_address );
        end = int ( self . broadcast_address ) + 1;
        step = ( int ( self . hostmask ) + 1 ) > > prefixlen_diff;
        for new_addr in range ( start , end , step ) .iter() {
        current = self . __class__ ( ( new_addr , new_prefixlen ) );
        yield current;
        pub fn supernet ( &self, prefixlen_diff = 1 , new_prefix = None /* Option */ )  {
        "The supernet containing the current network.

        Args:
            prefixlen_diff: An integer, the amount the prefix length of
              the network should be decreased by.  For example, given a
              /24 network && a prefixlen_diff of 3, a supernet with a
              /21 netmask == returned.

        Returns:
            An IPv4 network object.

        Raises:
            ValueError: If self.prefixlen - prefixlen_diff < 0. I.e., you have
              a negative prefix length.
                OR
            If prefixlen_diff && new_prefix are both set || new_prefix == a
              larger number than the current prefix (larger number means a
              smaller network)

        ";
        if self . _prefixlen == 0 {
        return  self;
        if new_prefix is !None /* Option */ {
        if new_prefix > self . _prefixlen {
        panic!("ValueError ( "new prefix must be shorter" )");
        if prefixlen_diff != 1 {
        panic!("ValueError ( "cannot set prefixlen_diff && new_prefix" )");
        prefixlen_diff = self . _prefixlen - new_prefix;
        new_prefixlen = self . prefixlen - prefixlen_diff;
        if new_prefixlen < 0 {
        panic!("ValueError (");
        "current prefixlen == %d, cannot have a prefixlen_diff of %d" %;
        ( self . prefixlen , prefixlen_diff ) );
        return  self . __class__ ( (;
        int ( self . network_address ) & ( int ( self . netmask ) < < prefixlen_diff ) ,;
        new_prefixlen;
        ) );
        @ property;
        pub fn is_multicast ( self )  {
        "Test if the address == reserved for multicast use.

        Returns:
            A boolean, true if the address == a multicast address.
            See RFC 2373 2.7 for details.

        ";
        return  ( self . network_address . is_multicast and;
        self . broadcast_address . is_multicast );
        @ staticmethod;
        pub fn _is_subnet_of ( a , b )  {
        // try {
        if a . _version != b . _version {
        panic!("TypeError ( f "{a} && {b} are !of the same version" )");
        return  ( b . network_address <= a . network_address and;
        b . broadcast_address >= a . broadcast_address );
        // } catch  AttributeError  {
        panic!("TypeError ( f "Unable to test subnet containment "");
        format!("between {a} && {b}" ));
        pub fn subnet_of ( &self, other )  {
        "Return true if this network == a subnet of other.";
        return  self . _is_subnet_of ( self , other );
        pub fn supernet_of ( &self, other )  {
        "Return true if this network == a supernet of other.";
        return  self . _is_subnet_of ( other , self );
        @ property;
        pub fn is_reserved ( self )  {
        "Test if the address == otherwise IETF reserved.

        Returns:
            A boolean, true if the address == within one of the
            reserved IPv6 Network ranges.

        ";
        return  ( self . network_address . is_reserved and;
        self . broadcast_address . is_reserved );
        @ property;
        pub fn is_link_local ( self )  {
        "Test if the address == reserved for link-local.

        Returns:
            A boolean, true if the address == reserved per RFC 4291.

        ";
        return  ( self . network_address . is_link_local and;
        self . broadcast_address . is_link_local );
        @ property;
        pub fn is_private ( self )  {
        "Test if this network belongs to a private range.

        Returns:
            A boolean, true if the network == reserved per
            iana-ipv4-special-registry || iana-ipv6-special-registry.

        ";
        return  any ( self . network_address in priv_network and;
        self . broadcast_address in priv_network;
        for priv_network in self . _constants . _private_networks ).iter() {
        @ property;
        pub fn is_global ( self )  {
        "Test if this address == allocated for public networks.

        Returns:
            A boolean, true if the address == !reserved per
            iana-ipv4-special-registry || iana-ipv6-special-registry.

        ";
        return  !self . is_private;
        @ property;
        pub fn is_unspecified ( self )  {
        "Test if the address == unspecified.

        Returns:
            A boolean, true if this == the unspecified address as defined in
            RFC 2373 2.5.2.

        ";
        return  ( self . network_address . is_unspecified and;
        self . broadcast_address . is_unspecified );
        @ property;
        pub fn is_loopback ( self )  {
        "Test if the address == a loopback address.

        Returns:
            A boolean, true if the address == a loopback address as defined in
            RFC 2373 2.5.3.

        ";
        return  ( self . network_address . is_loopback and;
        self . broadcast_address . is_loopback );
        class _BaseConstants ;
        _private_networks = [ ];
        _BaseNetwork . _constants = _BaseConstants;
        class _BaseV4 ;
        "Base IPv4 object.

    The following methods are used by IPv4 objects in both single IP
    addresses && networks.

    ";
        __slots__ = ( );
        _version = 4;
        _ALL_ONES = ( 2 ** IPV4LENGTH ) - 1;
        _max_prefixlen = IPV4LENGTH;
        _netmask_cache = { };
        pub fn _explode_shorthand_ip_string ( self )  {
        return  str ( self );
        @ classmethod;
        pub fn _make_netmask ( cls , arg )  {
        "Make a (netmask, prefix_len) tuple from the given argument.

        Argument can be:
        - an integer (the prefix length)
        - a string representing the prefix length (e.g. "24")
        - a string representing the prefix netmask (e.g. "255.255.255.0")
        ";
        if arg !in cls . _netmask_cache {
        if isinstance ( arg , int ) {
        prefixlen = arg;
        if !( 0 <= prefixlen <= cls . _max_prefixlen ) {
        cls . _report_invalid_netmask ( prefixlen );
        } else {
        // try {
        prefixlen = cls . _prefix_from_prefix_string ( arg );
        // } catch  NetmaskValueError  {
        prefixlen = cls . _prefix_from_ip_string ( arg );
        netmask = IPv4Address ( cls . _ip_int_from_prefix ( prefixlen ) );
        cls . _netmask_cache [ arg ] = netmask , prefixlen;
        return  cls . _netmask_cache [ arg ];
        @ classmethod;
        pub fn _ip_int_from_string ( cls , ip_str )  {
        "Turn the given IP string into an integer for comparison.

        Args:
            ip_str: A string, the IP ip_str.

        Returns:
            The IP ip_str as an integer.

        Raises:
            AddressValueError: if ip_str isn't a valid IPv4 Address.

        ";
        if !ip_str {
        panic!("AddressValueError ( "Address cannot be empty" )");
        octets = ip_str . split ( "." );
        if len ( octets ) != 4 {
        panic!("AddressValueError ( "Expected 4 octets in %r" % ip_str )");
        // try {
        return  int . from_bytes ( map ( cls . _parse_octet , octets ) , "big" );
        // } catch  ValueError as exc  {
        panic!("AddressValueError ( "%s in %r" % ( exc , ip_str ) ) from None /* Option */");
        @ classmethod;
        pub fn _parse_octet ( cls , octet_str )  {
        "Convert a decimal octet into an integer.

        Args:
            octet_str: A string, the number to parse.

        Returns:
            The octet as an integer.

        Raises:
            ValueError: if the octet isn't strictly a decimal from [0..255].

        ";
        if !octet_str {
        panic!("ValueError ( "Empty octet !permitted" )");
        if !( octet_str . isascii ( ) && octet_str . isdigit ( ) ) {
        msg = "Only decimal digits permitted in %r";
        panic!("ValueError ( msg % octet_str )");
        if len ( octet_str ) > 3 {
        msg = "At most 3 characters permitted in %r";
        panic!("ValueError ( msg % octet_str )");
        if octet_str != "0" && octet_str [ 0 ] == "0" {
        msg = "Leading zeros are !permitted in %r";
        panic!("ValueError ( msg % octet_str )");
        octet_int = int ( octet_str , 10 );
        if octet_int > 255 {
        panic!("ValueError ( "Octet %d (> 255) !permitted" % octet_int )");
        return  octet_int;
        @ classmethod;
        pub fn _string_from_ip_int ( cls , ip_int )  {
        "Turns a 32-bit integer into dotted decimal notation.

        Args:
            ip_int: An integer, the IP address.

        Returns:
            The IP address as a string in dotted decimal notation.

        ";
        return  "." . join ( map ( str , ip_int . to_bytes ( 4 , "big" ) ) );
        pub fn _reverse_pointer ( self )  {
        "Return the reverse DNS pointer name for the IPv4 address.

        This implements the method described in RFC1035 3.5.

        ";
        reverse_octets = str ( self ) . split ( "." ) [ : : -1 ];
        return  "." . join ( reverse_octets ) + ".in-addr.arpa";
        @ property;
        pub fn max_prefixlen ( self )  {
        return  self . _max_prefixlen;
        @ property;
        pub fn version ( self )  {
        return  self . _version;
        class IPv4Address ( _BaseV4 , _BaseAddress ) ;
        "Represent && manipulate single IPv4 Addresses.";
        __slots__ = ( "_ip" , "__weakref__" );
        pub fn __init__ ( &self, address )  {
        "
        Args:
            address: A string || integer representing the IP

              Additionally, an integer can be passed, so
              IPv4Address('192.0.2.1') == IPv4Address(3221225985).
              or, more generally
              IPv4Address(int(IPv4Address('192.0.2.1'))) ==
                IPv4Address('192.0.2.1')

        Raises:
            AddressValueError: If ipaddress isn't a valid IPv4 address.

        ";
        if isinstance ( address , int ) {
        self . _check_int_address ( address );
        self . _ip = address;
        return;
        if isinstance ( address , bytes ) {
        self . _check_packed_address ( address , 4 );
        self . _ip = int . from_bytes ( address );
        return;
        addr_str = str ( address );
        if "/" in addr_str {
        panic!("AddressValueError ( f "Unexpected '/' in {address!r}" )");
        self . _ip = self . _ip_int_from_string ( addr_str );
        @ property;
        pub fn packed ( self )  {
        "The binary representation of this address.";
        return  v4_int_to_packed ( self . _ip );
        @ property;
        pub fn is_reserved ( self )  {
        "Test if the address == otherwise IETF reserved.

         Returns:
             A boolean, true if the address == within the
             reserved IPv4 Network range.

        ";
        return  self in self . _constants . _reserved_network;
        @ property;
        @ functools . lru_cache ( );
        pub fn is_private ( self )  {
        "Test if this address == allocated for private networks.

        Returns:
            A boolean, true if the address == reserved per
            iana-ipv4-special-registry.

        ";
        return  any ( self in net for net in self . _constants . _private_networks );
        @ property;
        @ functools . lru_cache ( );
        pub fn is_global ( self )  {
        return  self !in self . _constants . _public_network && !self . is_private;
        @ property;
        pub fn is_multicast ( self )  {
        "Test if the address == reserved for multicast use.

        Returns:
            A boolean, true if the address == multicast.
            See RFC 3171 for details.

        ";
        return  self in self . _constants . _multicast_network;
        @ property;
        pub fn is_unspecified ( self )  {
        "Test if the address == unspecified.

        Returns:
            A boolean, true if this == the unspecified address as defined in
            RFC 5735 3.

        ";
        return  self == self . _constants . _unspecified_address;
        @ property;
        pub fn is_loopback ( self )  {
        "Test if the address == a loopback address.

        Returns:
            A boolean, true if the address == a loopback per RFC 3330.

        ";
        return  self in self . _constants . _loopback_network;
        @ property;
        pub fn is_link_local ( self )  {
        "Test if the address == reserved for link-local.

        Returns:
            A boolean, true if the address == link-local per RFC 3927.

        ";
        return  self in self . _constants . _linklocal_network;
        class IPv4Interface ( IPv4Address ) ;
        pub fn __init__ ( &self, address )  {
        addr , mask = self . _split_addr_prefix ( address );
        IPv4Address . __init__ ( self , addr );
        self . network = IPv4Network ( ( addr , mask ) , strict = false );
        self . netmask = self . network . netmask;
        self . _prefixlen = self . network . _prefixlen;
        @ functools . cached_property;
        pub fn hostmask ( self )  {
        return  self . network . hostmask;
        pub fn __str__ ( self )  {
        return  "%s/%d" % ( self . _string_from_ip_int ( self . _ip ) ,;
        self . _prefixlen );
        pub fn __eq__ ( &self, other )  {
        address_equal = IPv4Address . __eq__ ( self , other );
        if address_equal is NotImplemented || !address_equal {
        return  address_equal;
        // try {
        return  self . network == other . network;
        // } catch  AttributeError  {
        return  false;
        pub fn __lt__ ( &self, other )  {
        address_less = IPv4Address . __lt__ ( self , other );
        if address_less is NotImplemented {
        return  NotImplemented;
        // try {
        return  ( self . network < other . network or;
        self . network == other . network && address_less );
        // } catch  AttributeError  {
        return  false;
        pub fn __hash__ ( self )  {
        return  hash ( ( self . _ip , self . _prefixlen , int ( self . network . network_address ) ) );
        __reduce__ = _IPAddressBase . __reduce__;
        @ property;
        pub fn ip ( self )  {
        return  IPv4Address ( self . _ip );
        @ property;
        pub fn with_prefixlen ( self )  {
        return  "%s/%s" % ( self . _string_from_ip_int ( self . _ip ) ,;
        self . _prefixlen );
        @ property;
        pub fn with_netmask ( self )  {
        return  "%s/%s" % ( self . _string_from_ip_int ( self . _ip ) ,;
        self . netmask );
        @ property;
        pub fn with_hostmask ( self )  {
        return  "%s/%s" % ( self . _string_from_ip_int ( self . _ip ) ,;
        self . hostmask );
        class IPv4Network ( _BaseV4 , _BaseNetwork ) ;
        "This class represents && manipulates 32-bit IPv4 network + addresses..

    Attributes: [examples for IPv4Network('192.0.2.0/27')]
        .network_address: IPv4Address('192.0.2.0')
        .hostmask: IPv4Address('0.0.0.31')
        .broadcast_address: IPv4Address('192.0.2.32')
        .netmask: IPv4Address('255.255.255.224')
        .prefixlen: 27

    ";
        _address_class = IPv4Address;
        pub fn __init__ ( &self, address , strict = true )  {
        "Instantiate a new IPv4 network object.

        Args:
            address: A string || integer representing the IP [& network].
              '192.0.2.0/24'
              '192.0.2.0/255.255.255.0'
              '192.0.2.0/0.0.0.255'
              are all functionally the same in IPv4. Similarly,
              '192.0.2.1'
              '192.0.2.1/255.255.255.255'
              '192.0.2.1/32'
              are also functionally equivalent. That == to say, failing to
              provide a subnetmask will create an object with a mask of /32.

              If the mask (portion after the / in the argument) == given in
              dotted quad form, it == treated as a netmask if it starts with a
              non-zero field (e.g. /255.0.0.0 == /8) && as a hostmask if it
              starts with a zero field (e.g. 0.255.255.255 == /8), with the
              single exception of an all-zero mask which == treated as a
              netmask == /0. If no mask == given, a default of /32 == used.

              Additionally, an integer can be passed, so
              IPv4Network('192.0.2.1') == IPv4Network(3221225985)
              or, more generally
              IPv4Interface(int(IPv4Interface('192.0.2.1'))) ==
                IPv4Interface('192.0.2.1')

        Raises:
            AddressValueError: If ipaddress isn't a valid IPv4 address.
            NetmaskValueError: If the netmask isn't valid for
              an IPv4 address.
            ValueError: If strict == true && a network address == not
              supplied.
        ";
        addr , mask = self . _split_addr_prefix ( address );
        self . network_address = IPv4Address ( addr );
        self . netmask , self . _prefixlen = self . _make_netmask ( mask );
        packed = int ( self . network_address );
        if packed & int ( self . netmask ) != packed {
        if strict {
        panic!("ValueError ( "%s has host bits set" % self )");
        } else {
        self . network_address = IPv4Address ( packed &;
        int ( self . netmask ) );
        if self . _prefixlen == ( self . _max_prefixlen - 1 ) {
        self . hosts = self . __iter__;
        } else if self . _prefixlen == ( self . _max_prefixlen ) {
        self . hosts = lambda : [ IPv4Address ( addr ) ];
        @ property;
        @ functools . lru_cache ( );
        pub fn is_global ( self )  {
        "Test if this address == allocated for public networks.

        Returns:
            A boolean, true if the address == !reserved per
            iana-ipv4-special-registry.

        ";
        return  ( !( self . network_address in IPv4Network ( "100.64.0.0/10" ) and;
        self . broadcast_address in IPv4Network ( "100.64.0.0/10" ) ) and;
        not self . is_private );
        class _IPv4Constants ;
        _linklocal_network = IPv4Network ( "169.254.0.0/16" );
        _loopback_network = IPv4Network ( "127.0.0.0/8" );
        _multicast_network = IPv4Network ( "224.0.0.0/4" );
        _public_network = IPv4Network ( "100.64.0.0/10" );
        _private_networks = [;
        IPv4Network ( "0.0.0.0/8" ) ,;
        IPv4Network ( "10.0.0.0/8" ) ,;
        IPv4Network ( "127.0.0.0/8" ) ,;
        IPv4Network ( "169.254.0.0/16" ) ,;
        IPv4Network ( "172.16.0.0/12" ) ,;
        IPv4Network ( "192.0.0.0/29" ) ,;
        IPv4Network ( "192.0.0.170/31" ) ,;
        IPv4Network ( "192.0.2.0/24" ) ,;
        IPv4Network ( "192.168.0.0/16" ) ,;
        IPv4Network ( "198.18.0.0/15" ) ,;
        IPv4Network ( "198.51.100.0/24" ) ,;
        IPv4Network ( "203.0.113.0/24" ) ,;
        IPv4Network ( "240.0.0.0/4" ) ,;
        IPv4Network ( "255.255.255.255/32" ) ,;
        ];
        _reserved_network = IPv4Network ( "240.0.0.0/4" );
        _unspecified_address = IPv4Address ( "0.0.0.0" );
        IPv4Address . _constants = _IPv4Constants;
        IPv4Network . _constants = _IPv4Constants;
        class _BaseV6 ;
        "Base IPv6 object.

    The following methods are used by IPv6 objects in both single IP
    addresses && networks.

    ";
        __slots__ = ( );
        _version = 6;
        _ALL_ONES = ( 2 ** IPV6LENGTH ) - 1;
        _HEXTET_COUNT = 8;
        _HEX_DIGITS = frozenset ( "0123456789ABCDEFabcdeformat!(" ));
        _max_prefixlen = IPV6LENGTH;
        _netmask_cache = { };
        @ classmethod;
        pub fn _make_netmask ( cls , arg )  {
        "Make a (netmask, prefix_len) tuple from the given argument.

        Argument can be:
        - an integer (the prefix length)
        - a string representing the prefix length (e.g. "24")
        - a string representing the prefix netmask (e.g. "255.255.255.0")
        ";
        if arg !in cls . _netmask_cache {
        if isinstance ( arg , int ) {
        prefixlen = arg;
        if !( 0 <= prefixlen <= cls . _max_prefixlen ) {
        cls . _report_invalid_netmask ( prefixlen );
        } else {
        prefixlen = cls . _prefix_from_prefix_string ( arg );
        netmask = IPv6Address ( cls . _ip_int_from_prefix ( prefixlen ) );
        cls . _netmask_cache [ arg ] = netmask , prefixlen;
        return  cls . _netmask_cache [ arg ];
        @ classmethod;
        pub fn _ip_int_from_string ( cls , ip_str )  {
        "Turn an IPv6 ip_str into an integer.

        Args:
            ip_str: A string, the IPv6 ip_str.

        Returns:
            An int, the IPv6 address

        Raises:
            AddressValueError: if ip_str isn't a valid IPv6 Address.

        ";
        if !ip_str {
        panic!("AddressValueError ( "Address cannot be empty" )");
        parts = ip_str . split ( ":" );
        _min_parts = 3;
        if len ( parts ) < _min_parts {
        msg = "At least %d parts expected in %r" % ( _min_parts , ip_str );
        panic!("AddressValueError ( msg )");
        if "." in parts [ -1 ] {
        // try {
        ipv4_int = IPv4Address ( parts . pop ( ) ) . _ip;
        // } catch  AddressValueError as exc  {
        panic!("AddressValueError ( "%s in %r" % ( exc , ip_str ) ) from None /* Option */");
        parts . append ( "%x" % ( ( ipv4_int > > 16 ) & 0x FFFF ) );
        parts . append ( "%x" % ( ipv4_int & 0x FFFF ) );
        _max_parts = cls . _HEXTET_COUNT + 1;
        if len ( parts ) > _max_parts {
        msg = "At most %d colons permitted in %r" % ( _max_parts -1 , ip_str );
        panic!("AddressValueError ( msg )");
        skip_index = None /* Option */;
        for i in range ( 1 , len ( parts ) - 1 ) .iter() {
        if !parts [ i ] {
        if skip_index is !None /* Option */ {
        msg = "At most one '::' permitted in %r" % ip_str;
        panic!("AddressValueError ( msg )");
        skip_index = i;
        if skip_index is !None /* Option */ {
        parts_hi = skip_index;
        parts_lo = len ( parts ) - skip_index - 1;
        if !parts [ 0 ] {
        parts_hi - = 1;
        if parts_hi {
        msg = "Leading ':' only permitted as part of '::' in %r";
        panic!("AddressValueError ( msg % ip_str )");
        if !parts [ -1 ] {
        parts_lo - = 1;
        if parts_lo {
        msg = "Trailing ':' only permitted as part of '::' in %r";
        panic!("AddressValueError ( msg % ip_str )");
        parts_skipped = cls . _HEXTET_COUNT - ( parts_hi + parts_lo );
        if parts_skipped < 1 {
        msg = "Expected at most %d other parts with '::' in %r";
        panic!("AddressValueError ( msg % ( cls . _HEXTET_COUNT -1 , ip_str ) )");
        } else {
        if len ( parts ) != cls . _HEXTET_COUNT {
        msg = "Exactly %d parts expected without '::' in %r";
        panic!("AddressValueError ( msg % ( cls . _HEXTET_COUNT , ip_str ) )");
        if !parts [ 0 ] {
        msg = "Leading ':' only permitted as part of '::' in %r";
        panic!("AddressValueError ( msg % ip_str )");
        if !parts [ -1 ] {
        msg = "Trailing ':' only permitted as part of '::' in %r";
        panic!("AddressValueError ( msg % ip_str )");
        parts_hi = len ( parts );
        parts_lo = 0;
        parts_skipped = 0;
        // try {
        ip_int = 0;
        for i in range ( parts_hi ) .iter() {
        ip_int < <= 16;
        ip_int | = cls . _parse_hextet ( parts [ i ] );
        ip_int < <= 16 * parts_skipped;
        for i in range ( - parts_lo , 0 ) .iter() {
        ip_int < <= 16;
        ip_int | = cls . _parse_hextet ( parts [ i ] );
        return  ip_int;
        // } catch  ValueError as exc  {
        panic!("AddressValueError ( "%s in %r" % ( exc , ip_str ) ) from None /* Option */");
        @ classmethod;
        pub fn _parse_hextet ( cls , hextet_str )  {
        "Convert an IPv6 hextet string into an integer.

        Args:
            hextet_str: A string, the number to parse.

        Returns:
            The hextet as an integer.

        Raises:
            ValueError: if the input isn't strictly a hex number from
              [0..FFFF].

        ";
        if !cls . _HEX_DIGITS . issuperset ( hextet_str ) {
        panic!("ValueError ( "Only hex digits permitted in %r" % hextet_str )");
        if len ( hextet_str ) > 4 {
        msg = "At most 4 characters permitted in %r";
        panic!("ValueError ( msg % hextet_str )");
        return  int ( hextet_str , 16 );
        @ classmethod;
        pub fn _compress_hextets ( cls , hextets )  {
        "Compresses a list of hextets.

        Compresses a list of strings, replacing the longest continuous
        sequence oformat!("0" in the list with "" && adding empty strings at
        the beginning || at the end of the string such that subsequently
        calling ":".join(hextets) will produce the compressed version of
        the IPv6 address.

        Args:
            hextets: A list of strings, the hextets to compress.

        Returns:
            A list of strings.

        ");
        best_doublecolon_start = -1;
        best_doublecolon_len = 0;
        doublecolon_start = -1;
        doublecolon_len = 0;
        for index , hextet in enumerate ( hextets ) .iter() {
        if hextet == "0" {
        doublecolon_len + = 1;
        if doublecolon_start == -1 {
        doublecolon_start = index;
        if doublecolon_len > best_doublecolon_len {
        best_doublecolon_len = doublecolon_len;
        best_doublecolon_start = doublecolon_start;
        } else {
        doublecolon_len = 0;
        doublecolon_start = -1;
        if best_doublecolon_len > 1 {
        best_doublecolon_end = ( best_doublecolon_start +;
        best_doublecolon_len );
        if best_doublecolon_end == len ( hextets ) {
        hextets + = [ "" ];
        hextets [ best_doublecolon_start : best_doublecolon_end ] = [ "" ];
        if best_doublecolon_start == 0 {
        hextets = [ "" ] + hextets;
        return  hextets;
        @ classmethod;
        pub fn _string_from_ip_int ( cls , ip_int = None /* Option */ )  {
        "Turns a 128-bit integer into hexadecimal notation.

        Args:
            ip_int: An integer, the IP address.

        Returns:
            A string, the hexadecimal representation of the address.

        Raises:
            ValueError: The address == bigger than 128 bits of all ones.

        ";
        if ip_int is None /* Option */ {
        ip_int = int ( cls . _ip );
        if ip_int > cls . _ALL_ONES {
        panic!("ValueError ( "IPv6 address is too large" )");
        hex_str = "%032x" % ip_int;
        hextets = vec![ "%x" % int ( hex_str vec![ x : x + 4 ] , 16 ).iter().map(|x| range ( 0 , 32 , 4 ) ).collect();
        hextets = cls . _compress_hextets ( hextets );
        return  ":" . join ( hextets );
        pub fn _explode_shorthand_ip_string ( self )  {
        "Expand a shortened IPv6 address.

        Args:
            ip_str: A string, the IPv6 address.

        Returns:
            A string, the expanded IPv6 address.

        ";
        if isinstance ( self , IPv6Network ) {
        ip_str = str ( self . network_address );
        } else if isinstance ( self , IPv6Interface ) {
        ip_str = str ( self . ip );
        } else {
        ip_str = str ( self );
        ip_int = self . _ip_int_from_string ( ip_str );
        hex_str = "%032x" % ip_int;
        parts = vec![ hex_str vec![ x : x + 4 ].iter().map(|x| range ( 0 , 32 , 4 ) ).collect();
        if isinstance ( self , ( _BaseNetwork , IPv6Interface ) ) {
        return  "%s/%d" % ( ":" . join ( parts ) , self . _prefixlen );
        return  ":" . join ( parts );
        pub fn _reverse_pointer ( self )  {
        "Return the reverse DNS pointer name for the IPv6 address.

        This implements the method described in RFC3596 2.5.

        ";
        reverse_chars = self . exploded [ : : -1 ] . replace ( ":" , "" );
        return  "." . join ( reverse_chars ) + ".ip6.arpa";
        @ staticmethod;
        pub fn _split_scope_id ( ip_str )  {
        "Helper function to parse IPv6 string address with scope id.

        See RFC 4007 for details.

        Args:
            ip_str: A string, the IPv6 address.

        Returns:
            (addr, scope_id) tuple.

        ";
        addr , sep , scope_id = ip_str . partition ( "%" );
        if !sep {
        scope_id = None /* Option */;
        } else if !scope_id || "%" in scope_id {
        panic!("AddressValueError ( "Invalid IPv6 address: "%r"" % ip_str )");
        return  addr , scope_id;
        @ property;
        pub fn max_prefixlen ( self )  {
        return  self . _max_prefixlen;
        @ property;
        pub fn version ( self )  {
        return  self . _version;
        class IPv6Address ( _BaseV6 , _BaseAddress ) ;
        "Represent && manipulate single IPv6 Addresses.";
        __slots__ = ( "_ip" , "_scope_id" , "__weakref__" );
        pub fn __init__ ( &self, address )  {
        "Instantiate a new IPv6 address object.

        Args:
            address: A string || integer representing the IP

              Additionally, an integer can be passed, so
              IPv6Address('2001:db8::') ==
                IPv6Address(42540766411282592856903984951653826560)
              or, more generally
              IPv6Address(int(IPv6Address('2001:db8::'))) ==
                IPv6Address('2001:db8::')

        Raises:
            AddressValueError: If address isn't a valid IPv6 address.

        ";
        if isinstance ( address , int ) {
        self . _check_int_address ( address );
        self . _ip = address;
        self . _scope_id = None /* Option */;
        return;
        if isinstance ( address , bytes ) {
        self . _check_packed_address ( address , 16 );
        self . _ip = int . from_bytes ( address , "big" );
        self . _scope_id = None /* Option */;
        return;
        addr_str = str ( address );
        if "/" in addr_str {
        panic!("AddressValueError ( f "Unexpected '/' in {address!r}" )");
        addr_str , self . _scope_id = self . _split_scope_id ( addr_str );
        self . _ip = self . _ip_int_from_string ( addr_str );
        pub fn __str__ ( self )  {
        ip_str = super ( ) . __str__ ( );
        return  ip_str + "%" + self . _scope_id if self . _scope_id else ip_str;
        pub fn __hash__ ( self )  {
        return  hash ( ( self . _ip , self . _scope_id ) );
        pub fn __eq__ ( &self, other )  {
        address_equal = super ( ) . __eq__ ( other );
        if address_equal is NotImplemented {
        return  NotImplemented;
        if !address_equal {
        return  false;
        return  self . _scope_id == getattr ( other , "_scope_id" , None /* Option */ );
        pub fn __reduce__ ( self )  {
        return  ( self . __class__ , ( str ( self ) , ) );
        @ property;
        pub fn scope_id ( self )  {
        "Identifier of a particular zone of the address's scope.

        See RFC 4007 for details.

        Returns:
            A string identifying the zone of the address if specified, else None /* Option */.

        ";
        return  self . _scope_id;
        @ property;
        pub fn packed ( self )  {
        "The binary representation of this address.";
        return  v6_int_to_packed ( self . _ip );
        @ property;
        pub fn is_multicast ( self )  {
        "Test if the address == reserved for multicast use.

        Returns:
            A boolean, true if the address == a multicast address.
            See RFC 2373 2.7 for details.

        ";
        return  self in self . _constants . _multicast_network;
        @ property;
        pub fn is_reserved ( self )  {
        "Test if the address == otherwise IETF reserved.

        Returns:
            A boolean, true if the address == within one of the
            reserved IPv6 Network ranges.

        ";
        return  any ( self in x for x in self . _constants . _reserved_networks );
        @ property;
        pub fn is_link_local ( self )  {
        "Test if the address == reserved for link-local.

        Returns:
            A boolean, true if the address == reserved per RFC 4291.

        ";
        return  self in self . _constants . _linklocal_network;
        @ property;
        pub fn is_site_local ( self )  {
        "Test if the address == reserved for site-local.

        Note that the site-local address space has been deprecated by RFC 3879.
        Use is_private to test if this address == in the space of unique local
        addresses as defined by RFC 4193.

        Returns:
            A boolean, true if the address == reserved per RFC 3513 2.5.6.

        ";
        return  self in self . _constants . _sitelocal_network;
        @ property;
        @ functools . lru_cache ( );
        pub fn is_private ( self )  {
        "Test if this address == allocated for private networks.

        Returns:
            A boolean, true if the address == reserved per
            iana-ipv6-special-registry, || == ipv4_mapped && is
            reserved in the iana-ipv4-special-registry.

        ";
        ipv4_mapped = self . ipv4_mapped;
        if ipv4_mapped is !None /* Option */ {
        return  ipv4_mapped . is_private;
        return  any ( self in net for net in self . _constants . _private_networks );
        @ property;
        pub fn is_global ( self )  {
        "Test if this address == allocated for public networks.

        Returns:
            A boolean, true if the address == !reserved per
            iana-ipv6-special-registry.

        ";
        return  !self . is_private;
        @ property;
        pub fn is_unspecified ( self )  {
        "Test if the address == unspecified.

        Returns:
            A boolean, true if this == the unspecified address as defined in
            RFC 2373 2.5.2.

        ";
        return  self . _ip == 0;
        @ property;
        pub fn is_loopback ( self )  {
        "Test if the address == a loopback address.

        Returns:
            A boolean, true if the address == a loopback address as defined in
            RFC 2373 2.5.3.

        ";
        return  self . _ip == 1;
        @ property;
        pub fn ipv4_mapped ( self )  {
        "Return the IPv4 mapped address.

        Returns:
            If the IPv6 address == a v4 mapped address, return the
            IPv4 mapped address. Return None /* Option */ otherwise.

        ";
        if ( self . _ip > > 32 ) != 0x FFFF {
        return;
        return  IPv4Address ( self . _ip & 0x FFFFFFFF );
        @ property;
        pub fn teredo ( self )  {
        "Tuple of embedded teredo IPs.

        Returns:
            Tuple of the (server, client) IPs || None /* Option */ if the address
            doesn't appear to be a teredo address (doesn't start with
            2001::/32)

        ";
        if ( self . _ip > > 96 ) != 0x20010000 {
        return;
        return  ( IPv4Address ( ( self . _ip > > 64 ) & 0x FFFFFFFF ) ,;
        IPv4Address ( ~ self . _ip & 0x FFFFFFFF ) );
        @ property;
        pub fn sixtofour ( self )  {
        "Return the IPv4 6to4 embedded address.

        Returns:
            The IPv4 6to4-embedded address if present || None /* Option */ if the
            address doesn't appear to contain a 6to4 embedded address.

        ";
        if ( self . _ip > > 112 ) != 0x2002 {
        return;
        return  IPv4Address ( ( self . _ip > > 80 ) & 0x FFFFFFFF );
        class IPv6Interface ( IPv6Address ) ;
        pub fn __init__ ( &self, address )  {
        addr , mask = self . _split_addr_prefix ( address );
        IPv6Address . __init__ ( self , addr );
        self . network = IPv6Network ( ( addr , mask ) , strict = false );
        self . netmask = self . network . netmask;
        self . _prefixlen = self . network . _prefixlen;
        @ functools . cached_property;
        pub fn hostmask ( self )  {
        return  self . network . hostmask;
        pub fn __str__ ( self )  {
        return  "%s/%d" % ( super ( ) . __str__ ( ) ,;
        self . _prefixlen );
        pub fn __eq__ ( &self, other )  {
        address_equal = IPv6Address . __eq__ ( self , other );
        if address_equal is NotImplemented || !address_equal {
        return  address_equal;
        // try {
        return  self . network == other . network;
        // } catch  AttributeError  {
        return  false;
        pub fn __lt__ ( &self, other )  {
        address_less = IPv6Address . __lt__ ( self , other );
        if address_less is NotImplemented {
        return  address_less;
        // try {
        return  ( self . network < other . network or;
        self . network == other . network && address_less );
        // } catch  AttributeError  {
        return  false;
        pub fn __hash__ ( self )  {
        return  hash ( ( self . _ip , self . _prefixlen , int ( self . network . network_address ) ) );
        __reduce__ = _IPAddressBase . __reduce__;
        @ property;
        pub fn ip ( self )  {
        return  IPv6Address ( self . _ip );
        @ property;
        pub fn with_prefixlen ( self )  {
        return  "%s/%s" % ( self . _string_from_ip_int ( self . _ip ) ,;
        self . _prefixlen );
        @ property;
        pub fn with_netmask ( self )  {
        return  "%s/%s" % ( self . _string_from_ip_int ( self . _ip ) ,;
        self . netmask );
        @ property;
        pub fn with_hostmask ( self )  {
        return  "%s/%s" % ( self . _string_from_ip_int ( self . _ip ) ,;
        self . hostmask );
        @ property;
        pub fn is_unspecified ( self )  {
        return  self . _ip == 0 && self . network . is_unspecified;
        @ property;
        pub fn is_loopback ( self )  {
        return  self . _ip == 1 && self . network . is_loopback;
        class IPv6Network ( _BaseV6 , _BaseNetwork ) ;
        "This class represents && manipulates 128-bit IPv6 networks.

    Attributes: [examples for IPv6('2001:db8::1000/124')]
        .network_address: IPv6Address('2001:db8::1000')
        .hostmask: IPv6Address('::f')
        .broadcast_address: IPv6Address('2001:db8::100f')
        .netmask: IPv6Address('ffff:ffff:ffff:ffff:ffff:ffff:ffff:fff0')
        .prefixlen: 124

    ";
        _address_class = IPv6Address;
        pub fn __init__ ( &self, address , strict = true )  {
        "Instantiate a new IPv6 Network object.

        Args:
            address: A string || integer representing the IPv6 network || the
              IP && prefix/netmask.
              '2001:db8::/128'
              '2001:db8:0000:0000:0000:0000:0000:0000/128'
              '2001:db8::'
              are all functionally the same in IPv6.  That == to say,
              failing to provide a subnetmask will create an object with
              a mask of /128.

              Additionally, an integer can be passed, so
              IPv6Network('2001:db8::') ==
                IPv6Network(42540766411282592856903984951653826560)
              or, more generally
              IPv6Network(int(IPv6Network('2001:db8::'))) ==
                IPv6Network('2001:db8::')

            strict: A boolean. If true, ensure that we have been passed
              A true network address, eg, 2001:db8::1000/124 && !an
              IP address on a network, eg, 2001:db8::1/124.

        Raises:
            AddressValueError: If address isn't a valid IPv6 address.
            NetmaskValueError: If the netmask isn't valid for
              an IPv6 address.
            ValueError: If strict was true && a network address was not
              supplied.
        ";
        addr , mask = self . _split_addr_prefix ( address );
        self . network_address = IPv6Address ( addr );
        self . netmask , self . _prefixlen = self . _make_netmask ( mask );
        packed = int ( self . network_address );
        if packed & int ( self . netmask ) != packed {
        if strict {
        panic!("ValueError ( "%s has host bits set" % self )");
        } else {
        self . network_address = IPv6Address ( packed &;
        int ( self . netmask ) );
        if self . _prefixlen == ( self . _max_prefixlen - 1 ) {
        self . hosts = self . __iter__;
        } else if self . _prefixlen == self . _max_prefixlen {
        self . hosts = lambda : [ IPv6Address ( addr ) ];
        pub fn hosts ( self )  {
        "Generate Iterator over usable hosts in a network.

          This == like __iter__ except it doesn't return the
          Subnet-Router anycast address.

        ";
        network = int ( self . network_address );
        broadcast = int ( self . broadcast_address );
        for x in range ( network + 1 , broadcast + 1 ) .iter() {
        yield self . _address_class ( x );
        @ property;
        pub fn is_site_local ( self )  {
        "Test if the address == reserved for site-local.

        Note that the site-local address space has been deprecated by RFC 3879.
        Use is_private to test if this address == in the space of unique local
        addresses as defined by RFC 4193.

        Returns:
            A boolean, true if the address == reserved per RFC 3513 2.5.6.

        ";
        return  ( self . network_address . is_site_local and;
        self . broadcast_address . is_site_local );
        class _IPv6Constants ;
        _linklocal_network = IPv6Network ( "fe80::/10" );
        _multicast_network = IPv6Network ( "ff00::/8" );
        _private_networks = [;
        IPv6Network ( "::1/128" ) ,;
        IPv6Network ( "::/128" ) ,;
        IPv6Network ( "::ffff:0:0/96" ) ,;
        IPv6Network ( "100::/64" ) ,;
        IPv6Network ( "2001::/23" ) ,;
        IPv6Network ( "2001:2::/48" ) ,;
        IPv6Network ( "2001:db8::/32" ) ,;
        IPv6Network ( "2001:10::/28" ) ,;
        IPv6Network ( "fc00::/7" ) ,;
        IPv6Network ( "fe80::/10" ) ,;
        ];
        _reserved_networks = [;
        IPv6Network ( "::/8" ) , IPv6Network ( "100::/8" ) ,;
        IPv6Network ( "200::/7" ) , IPv6Network ( "400::/6" ) ,;
        IPv6Network ( "800::/5" ) , IPv6Network ( "1000::/4" ) ,;
        IPv6Network ( "4000::/3" ) , IPv6Network ( "6000::/3" ) ,;
        IPv6Network ( "8000::/3" ) , IPv6Network ( "A000::/3" ) ,;
        IPv6Network ( "C000::/3" ) , IPv6Network ( "E000::/4" ) ,;
        IPv6Network ( "F000::/5" ) , IPv6Network ( "F800::/6" ) ,;
        IPv6Network ( "FE00::/9" ) ,;
        ];
        _sitelocal_network = IPv6Network ( "fec0::/10" );
        IPv6Address . _constants = _IPv6Constants;
        IPv6Network . _constants = _IPv6Constants;
}

