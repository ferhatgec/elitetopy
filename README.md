# [elite](https://github.com/ferhatgec/elite)topy
## [elite](https://github.com/ferhatgec/elite) -> python3 converter

### input:
```rs
required_version is 0.1

set ProjectName as "elitetopy"

set HOME        as env "HOME"

set COMPILER_PATH as "/usr/bin/{COMPILER}"

for argument "install" [
    use exec "cargo install --path ."

    for exists "{HOME}.cargo/bin/{ProjectName}" [
        println "{ProjectName} installed to {HOME}.cargo/bin/{ProjectName}."
    ]

    use signal "exit"
]
```

### output
```py
#!/usr/bin/env python3
import platform
import sys
from pathlib import Path
from os import system
if '0.1' != '0.1':
 print('elite: Required higher version')
 exit(1)
ProjectName = 'elitetopy'
HOME = '/home/gech'
COMPILER_PATH = '/usr/bin/'
if len(sys.argv) >= 2 and sys.argv[1] == 'install':
 system('cargo install --path .')
 if Path('/home/gech/.cargo/bin/elitetopy').exists():
  print('elitetopy installed to /home/gech/.cargo/bin/elitetopy.')
 exit(0)
```

### elitetopy licensed under the terms of MIT License
