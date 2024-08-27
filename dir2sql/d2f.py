import argparse
from hashlib import md5
import os
from pathlib import Path
import sqlite3

def parse_arguments():
    parser = argparse.ArgumentParser(description="Recursively scans directory hashing files and outputting SQLite DB.")
    parser.add_argument("dirname", type=str, help="The directory to scan")
    parser.add_argument("sqlfile", type=str, default="db.sqlite", help="The destination SQLIte file")
    parser.add_argument("-v", dest="verb", type=int, default=1, help="Set verbosity level from 0 to 3 (default 1)")
    return parser.parse_args()

file_filters = []
dir_filters = []

sql_stmts = {
    "create": r"""CREATE TABLE IF NOT EXISTS file(
	id INT PRIMARY KEY,
	path VARCHAR(256),
	name VARCHAR(128),
	md5 CHAR(24),
	size INT);"""
}

# allowed_extensions and blocked_extensions are mutually exclusive
filters = {
    # "allowed_extensions": ["mp3", "flac", "m4a", "wma", "ogg", "wav"],
    "allowed_extensions": [],
    "blocked_extensions": [],
}

def is_indexed(path: str, name: str) -> bool:
    ext = os.path.splitext(name)[1]
    # print(ext, (not ext[1:] in filters["allowed_extensions"]))
    if (len(filters["allowed_extensions"]) > 0) and (not ext[1:] in filters["allowed_extensions"]):
        return False
    return True

def db_open_and_init(path: str) -> list:
    conn = sqlite3.connect(path)
    cur = conn.cursor()
    cur.execute(sql_stmts["create"])
    conn.commit()
    return [conn, cur]

def get_max_id(cur):
    res = cur.execute("SELECT MAX(id) FROM file")
    return res.fetchone()[0]

def db_add_records(conn, cur, records):
    cur.executemany("INSERT INTO file VALUES (?,?,?,?,?)", records)
    conn.commit()

def create_records(entries, next_id=1, verbosity=3):
    records = []
    n = len(entries)
    i = 0
    for entry in entries:
        i += 1
        if verbosity > 2:
            print(f"{int(100*i/n)}% {str(entry[0])}")
        for file in entry[2]:
            if is_indexed(entry[0], file):
                file_path = os.path.join(entry[0],file)
                try:
                    md5 = md5_for_file(file_path)
                    records.append((next_id, str(entry[0]), file, md5, os.path.getsize(file_path)))
                    next_id += 1
                except:
                    print(f"Error with entry {file_path} (Ignoring)")
    return records
    
    

def md5_for_file(file):
    with open(file, 'rb') as file_to_check:
        data = file_to_check.read()    
        md5_returned = md5(data).hexdigest()
    
    
    return md5_returned

def content_of(dir: str) -> list:
    path = Path(dir)
    return [x for x in path.walk()]

def main():
    args = parse_arguments()
    root = os.path.expanduser(args.dirname)
    db_path = os.path.expanduser(args.sqlfile)

        
    db_conn, db_cur = db_open_and_init(db_path)    
    content = content_of(root)
            
    # TODO: User the next_id key parameter (this should solve issue #1)
    records = create_records(content)
    db_add_records(db_conn, db_cur, records)
    db_conn.close()
    
if __name__ == "__main__":
    main()