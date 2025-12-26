# Dependency Crates

## base32

Use below script to build the crate:

```
# check out the repository from https://github.com/andreasots/base32/tree/master
crate  =base32
crate_src=src
crate main=lib
dep_crates=[]
comp opts=[]
test file=test
common =..${~/~}..${~/~}simscript${~/~}comm-crate.7b:file
common_test =..${~/~}..${~/~}simscript${~/~}comm-test.7b:file
crate_dir=..${~/~}..${~/~}crates

include(common);

```

The script assumes the following directories structure:

```
├─projects
   ....
   ├─simscript
   ├─crates
   ├─simtotp
   ├─side
      ├─base32
      ....
   .....
....
```

Paths may need to be corrected in the script in the case of a different directories structure.  