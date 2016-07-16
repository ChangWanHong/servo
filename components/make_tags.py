import os
import re
import subprocess
cargo_tomls = subprocess.Popen("find . -name 'Cargo.toml'", shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE).communicate()[0].split('\n')[:-1]
cargo_dirs = map(lambda x: re.sub('/Cargo.toml$', '', x) , cargo_tomls)
print "Run to ..."
print cargo_dirs
for directory in cargo_dirs:
    os.chdir(directory)
    print directory
    print subprocess.Popen("rusty-tags vi", shell=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE).communicate()[0]
    os.chdir("..")
