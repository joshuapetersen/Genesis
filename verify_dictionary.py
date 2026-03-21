"""
Verify dictionary index
"""
import lancedb

VAR_10 = 10
VAR_60 = 60

db = lancedb.connect('c:\\SarahCore\\vault\\dictionary_index')
print('Tables:', db.list_tables())

if 'dictionary' in db.list_tables():
    table = db.open_table('dictionary')
    print(f'Row count: {table.count_rows()}')
    
    # Show sample entries
    results = table.search().limit(VAR_10).to_list()
    print('\nSample entries:')
    for r in results:
        print(f"  {r['word']} ({r['type']}): {r['definition'][:VAR_60]}...")
else:
    print('Dictionary table not found')
