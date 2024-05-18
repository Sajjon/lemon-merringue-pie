This is a small demo project that showcase 
that it is possible to use a workspace - 
multicrate setup together with UniFFI with
a non-trivial hierachy of inter-dependent 
crates.

This project has this "dependency tree"

```
  / bar ~~~~> foobar
e > f > foo ~~/
    o /
```

Or in words - list of dependencies per crate:
foobar: ALL
bar: e
foo: e, f, o
f: e
e: NONE
o: NONE

All crates declare UniFFI Records and Objects.
