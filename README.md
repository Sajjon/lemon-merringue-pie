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

# Good
* It works! we can have a non-trivial hiearchy of UniFFI crates in a workspace! 🥳
* We need not declare UniFFI types of "grandparents", meaning foo needs only declare the external types of f, but ???? hmm what if foo uses some type from e which f did not use?!

# Bad
* We MUST use udl files, even for a 100% proc macro based project

# Ugly
* We have to type `ExternalType` many types *per crate* instead of once per crate (and then just list each typedef extern we wanna use from that crate)
