//! wintypes.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz

// use crate::ctypes;

pub const BYTE: f64 = ctypes . c_byte;
pub const WORD: f64 = ctypes . c_ushort;
pub const DWORD: f64 = ctypes . c_ulong;
pub const CHAR: f64 = ctypes . c_char;
pub const WCHAR: f64 = ctypes . c_wchar;
pub const UINT: f64 = ctypes . c_uint;
pub const INT: f64 = ctypes . c_int;
pub const DOUBLE: f64 = ctypes . c_double;
pub const FLOAT: f64 = ctypes . c_float;
pub const BOOLEAN: /* inferred */ = BYTE;
pub const BOOL: f64 = ctypes . c_long;
pub struct VARIANT_BOOL {
}

impl VARIANT_BOOL {
    pub fn __repr__(&self) {
        return  "%s(%r)" % ( self . __class__ . __name__ , self . value );
    }

    pub fn RGB(&self, red: &str, green: &str, blue: &str) {
        return  red + ( green < < 8 ) + ( blue < < 16 );
        class FILETIME ( ctypes . Structure ) ;
        _fields_ = [ ( "dwLowDateTime" , DWORD ) ,;
        ( "dwHighDateTime" , DWORD ) ];
        _FILETIME = FILETIME;
        class MSG ( ctypes . Structure ) ;
        _fields_ = [ ( "hWnd" , HWND ) ,;
        ( "message" , UINT ) ,;
        ( "wParam" , WPARAM ) ,;
        ( "lParam" , LPARAM ) ,;
        ( "time" , DWORD ) ,;
        ( "pt" , POINT ) ];
        tagMSG = MSG;
        MAX_PATH = 260;
        class WIN32_FIND_DATAA ( ctypes . Structure ) ;
        _fields_ = [ ( "dwFileAttributes" , DWORD ) ,;
        ( "ftCreationTime" , FILETIME ) ,;
        ( "ftLastAccessTime" , FILETIME ) ,;
        ( "ftLastWriteTime" , FILETIME ) ,;
        ( "nFileSizeHigh" , DWORD ) ,;
        ( "nFileSizeLow" , DWORD ) ,;
        ( "dwReserved0" , DWORD ) ,;
        ( "dwReserved1" , DWORD ) ,;
        ( "cFileName" , CHAR * MAX_PATH ) ,;
        ( "cAlternateFileName" , CHAR * 14 ) ];
        class WIN32_FIND_DATAW ( ctypes . Structure ) ;
        _fields_ = [ ( "dwFileAttributes" , DWORD ) ,;
        ( "ftCreationTime" , FILETIME ) ,;
        ( "ftLastAccessTime" , FILETIME ) ,;
        ( "ftLastWriteTime" , FILETIME ) ,;
        ( "nFileSizeHigh" , DWORD ) ,;
        ( "nFileSizeLow" , DWORD ) ,;
        ( "dwReserved0" , DWORD ) ,;
        ( "dwReserved1" , DWORD ) ,;
        ( "cFileName" , WCHAR * MAX_PATH ) ,;
        ( "cAlternateFileName" , WCHAR * 14 ) ];
        LPBOOL = PBOOL = ctypes . POINTER ( BOOL );
        PBOOLEAN = ctypes . POINTER ( BOOLEAN );
        LPBYTE = PBYTE = ctypes . POINTER ( BYTE );
        PCHAR = ctypes . POINTER ( CHAR );
        LPCOLORREF = ctypes . POINTER ( COLORREF );
        LPDWORD = PDWORD = ctypes . POINTER ( DWORD );
        LPFILETIME = PFILETIME = ctypes . POINTER ( FILETIME );
        PFLOAT = ctypes . POINTER ( FLOAT );
        LPHANDLE = PHANDLE = ctypes . POINTER ( HANDLE );
        PHKEY = ctypes . POINTER ( HKEY );
        LPHKL = ctypes . POINTER ( HKL );
        LPINT = PINT = ctypes . POINTER ( INT );
        PLARGE_INTEGER = ctypes . POINTER ( LARGE_INTEGER );
        PLCID = ctypes . POINTER ( LCID );
        LPLONG = PLONG = ctypes . POINTER ( LONG );
        LPMSG = PMSG = ctypes . POINTER ( MSG );
        LPPOINT = PPOINT = ctypes . POINTER ( POINT );
        PPOINTL = ctypes . POINTER ( POINTL );
        LPRECT = PRECT = ctypes . POINTER ( RECT );
        LPRECTL = PRECTL = ctypes . POINTER ( RECTL );
        LPSC_HANDLE = ctypes . POINTER ( SC_HANDLE );
        PSHORT = ctypes . POINTER ( SHORT );
        LPSIZE = PSIZE = ctypes . POINTER ( SIZE );
        LPSIZEL = PSIZEL = ctypes . POINTER ( SIZEL );
        PSMALL_RECT = ctypes . POINTER ( SMALL_RECT );
        LPUINT = PUINT = ctypes . POINTER ( UINT );
        PULARGE_INTEGER = ctypes . POINTER ( ULARGE_INTEGER );
        PULONG = ctypes . POINTER ( ULONG );
        PUSHORT = ctypes . POINTER ( USHORT );
        PWCHAR = ctypes . POINTER ( WCHAR );
        LPWIN32_FIND_DATAA = PWIN32_FIND_DATAA = ctypes . POINTER ( WIN32_FIND_DATAA );
        LPWIN32_FIND_DATAW = PWIN32_FIND_DATAW = ctypes . POINTER ( WIN32_FIND_DATAW );
        LPWORD = PWORD = ctypes . POINTER ( WORD );
    }

}

