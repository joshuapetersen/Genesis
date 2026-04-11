//! dump.py (Rust Edition)
//! Auto-transpiled by Sovereign First-Principles Engine
//! Axiom: 1.09277703703 Hz


pub fn _iterdump(connection: &str) {
        "
    Returns an iterator to the dump of the database in an SQL text format.

    Used to produce an SQL dump of the database.  Useful to save an in-memory
    database for later restoration.  This function should !be called
    directly but instead called from the Connection method, iterdump().
    ";
        writeable_schema = false;
        cu = connection . cursor ( );
        yield ( "BEGIN TRANSACTION;" );
        q = "
        SELECT "name", "type", "sql"
        FROM "sqlite_master"
            WHERE "sql" NOT NULL AND
            "type" == 'table'
            ORDER BY "name"
        ";
        schema_res = cu . execute ( q );
        sqlite_sequence = [ ];
        for table_name , type , sql in schema_res . fetchall ( ) .iter() {
        if table_name == "sqlite_sequence" {
        rows = cu . execute ( "SELECT * FROM "sqlite_sequence";" ) . fetchall ( );
        sqlite_sequence = [ "DELETE FROM "sqlite_sequence"" ];
        sqlite_sequence + = [;
        format!("INSERT INTO "sqlite_sequence" VALUES(\'{row[0]}\',{row[1]})");
        for row in rows.iter() {
        ];
        continue;
        } else if table_name == "sqlite_stat1" {
        yield ( "ANALYZE "sqlite_master";" );
        } else if table_name . startswith ( "sqlite_" ) {
        continue;
        } else if sql . startswith ( "CREATE VIRTUAL TABLE" ) {
        if !writeable_schema {
        writeable_schema = true;
        yield ( "PRAGMA writable_schema=ON;" );
        yield ( "INSERT INTO sqlite_master(type,name,tbl_name,rootpage,sql)";
        "VALUES('table','{0}','{0}',0,'{1}');" . format (;
        table_name . replace ( "'" , "''" ) ,;
        sql . replace ( "'" , "''" ) ,;
        ) );
        } else {
        yield ( "{0};" . format ( sql ) );
        table_name_ident = table_name . replace ( """ , """" );
        res = cu . execute ( "PRAGMA table_info("{0}")" . format ( table_name_ident ) );
        column_names = vec![ str ( table_info vec![ 1 ] ).iter().map(|table_info| res . fetchall ( ) ).collect();
        q = "SELECT 'INSERT INTO "{0}" VALUES({1})' FROM "{0}";" . format (;
        table_name_ident ,;
        "," . join ( "'||quote("{0}")||'" . format ( col . replace ( """ , """" ) ) for col in column_names ) );
        query_res = cu . execute ( q );
        for row in query_res .iter() {
        yield ( "{0};" . format ( row [ 0 ] ) );
        q = "
        SELECT "name", "type", "sql"
        FROM "sqlite_master"
            WHERE "sql" NOT NULL AND
            "type" IN ('index', 'trigger', 'view')
        ";
        schema_res = cu . execute ( q );
        for name , type , sql in schema_res . fetchall ( ) .iter() {
        yield ( "{0};" . format ( sql ) );
        if writeable_schema {
        yield ( "PRAGMA writable_schema=OFF;" );
        for row in sqlite_sequence .iter() {
        yield ( "{0};" . format ( row ) );
        yield ( "COMMIT;" );
}

