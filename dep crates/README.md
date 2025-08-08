# Dependency Crates

## base32

Use the script to build it:

```
# from https://github.com/andreasots/base32/tree/master
crate  =base32
main=src${~/~}lib
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

Paths may need to be corrected in the script in a case of a different directories structure.  