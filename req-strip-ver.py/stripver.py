# Removes the version from input requirements.txt formatted files 
import sys

file_name = "requirements.txt"
if len(sys.argv) > 1:
    file_name = sys.argv[1]
replace = False
out_name = file_name if replace else file_name + ".stripped.txt"
stripped = []

with open(file_name) as f:
    stripped = [line.split("==")[0] + "\n" for line in f]
    
    
print(stripped)
        
with open(out_name, "w") as f:
    f.writelines(stripped)