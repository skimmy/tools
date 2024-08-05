import sys
import sqlite3


queries = {
    "all-files": """SELECT md5,path,name FROM file ORDER BY md5;"""
}

def db_open_and_check(file: str):
    try:
        conn = sqlite3.connect(file)
        return conn
    except:
        print("Error opening database")
        sys.exit(1)
    

def compare_from_scan() -> list:
    print("Compare from scan not yet implemented")
    sys.exit(1)
    
    
def merge_lists(list1: list, list2: list) -> list:
    merged = []
    i1 = 0
    i2 = 0
    while (i1 < len(list1)) and (i2 < len(list2)):
        if (list1[i1] == list2[i2]):
            merged.append((list1[1], 3))
            i1 += 1
            i2 += 1
        elif list1[i1] < list2[i2]:
            merged.append((list1[i1],1))
            i1 += 1
        else:
            merged.append((list2[i2], 2))
            i2 += 1
    while(i1 < len(list1)):
        merged.append((list2[i1],1))
        i1 += 1
    while(i2 < len(list2)):
        merged.append((list2[i2],2))
        i2 += 1
    return merged
            


def compare_from_files(file1: str, file2: str) -> list:
    conn1 = db_open_and_check(file1)
    conn2 = db_open_and_check(file2)
    cur1 = conn1.cursor()
    res = cur1.execute(queries["all-files"])
    file_list_1 = res.fetchall()
    cur2 = conn2.cursor()
    res = cur2.execute(queries["all-files"])
    file_list_2 = res.fetchall()
    
    merged = merge_lists(file_list_1[1:3], file_list_2[2:5])
    [print(l) for l in merged]
    
    
    

def main():
    compare_result = None
    if len(sys.argv) >= 3:
        compare_result = compare_from_files(sys.argv[1], sys.argv[2])
    else:
        compare_result = compare_from_scan()

if __name__ == "__main__":
    main()