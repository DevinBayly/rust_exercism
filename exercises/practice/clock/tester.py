import os
import subprocess as sp
import time

last_time = 0
f_path = "./src/lib.rs"
while True:
  mtime = os.stat(f_path).st_mtime
  if mtime > last_time:
    last_time = mtime
    command = "cargo test -- --nocapture"
    res = sp.run(command,shell=True)
  time.sleep(2)


  
