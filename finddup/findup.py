import argparse
import sqlite3

queries = {
    "find-duplicate": """SELECT md5, COUNT(id) AS c FROM file GROUP BY md5 HAVING c > 1;""",
    "file-with-md5": """SELECT * FROM file WHERE md5=?;"""
}

def parse_arguments() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Finds duplicate files in DB.")
    parser.add_argument("db", type=str, help="The SQLite DB to scan (see dir2sql tool)")
    parser.add_argument("-v", dest="verb", type=int, default=1, help="Set verbosity level from 0 to 3 (default 1)")
    parser.add_argument("--prefix", required=False, help="Common path prefix, outputs are relative to this")
    return parser.parse_args()

def open_db(file: str) -> sqlite3.Connection:
    return sqlite3.connect(file)

def format_duplicates(records: list, conn: sqlite3.Connection) -> str:
    # TODO: It would be better to sort the output by path and name
    s = ""
    for record in records:
        # print(record[0])
        cursor = conn.cursor()
        cursor.execute(queries["file-with-md5"], (record[0],))
        results = cursor.fetchall()
        s+= f"{record[0]} ({len(results)})\n"
        for tuple in results:
            s+= f"\t{tuple[1]}/{tuple[2]}\n"
    return s

def main():
    args = parse_arguments()
    if args.verb > 1:
        print(f"DB: {args.db}")
    conn = open_db(args.db)
    cursor = conn.cursor()
    cursor.execute(queries["find-duplicate"])
    results = cursor.fetchall()
    output = format_duplicates(results, conn)
    if args.prefix:
        output = output.replace(args.prefix, "./")
    print(output)
    

if __name__ == "__main__":
    main()
