from hashlib import md5
import os
from pathlib import Path
import sqlite3
import sys

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
    "allowed_extensions": ["mp3", "flac", "m4a", "wma", "ogg", "wav"],
    "blocked_extensions": [],
}

def is_indexed(path: str, name: str) -> bool:
    ext = os.path.splitext(name)[1]
    print(ext, (not ext[1:] in filters["allowed_extensions"]))
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

def create_records(entries, next_id=1):
    records = []
    for entry in entries:
        for file in entry[2]:
            if is_indexed(entry[0], file):
                file_path = os.path.join(entry[0],file)
                records.append((next_id, str(entry[0]), file, md5_for_file(file_path), os.path.getsize(file_path)))
                next_id += 1
    return records
    
    

def md5_for_file(file):
    with open(file, 'rb') as file_to_check:
        data = file_to_check.read()    
        md5_returned = md5(data).hexdigest()
    return md5_returned

def content_of(dir: str) -> list:
    path = Path(dir)
    return [x for x in path.walk()]

if __name__ == "__main__":
    root = "."
    db_path = "db.sqlite"
    if len(sys.argv) > 1:
        root = os.path.expanduser(sys.argv[1])
    if len(sys.argv) > 2:
        db_path = sys.argv[2]
    verbosity = 3 # 0 Min (no output) -- 3 Max (debug output)
        
    db_conn, db_cur = db_open_and_init(db_path)    
    content = content_of(root)
        
    if verbosity >= 2:
        for c in content:
            for file in c[2]:
                file_path = os.path.join(c[0],file)
                print(f"{str(c[0]):<40}{file:<40}{md5_for_file(file_path)}\t{os.path.getsize(file_path):>12} Byte")
            
    # db_add_entries(db_cur, content)
    records = create_records(content)
    db_add_records(db_conn, db_cur, records)
    
            
    db_conn.close()