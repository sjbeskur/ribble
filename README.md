# ribble
Ribble is an attempt at porting the popular Java based, [Mibble](https://www.mibble.org/) MIB Parser to Rust using the [lalrpop](https://github.com/lalrpop/lalrpop) parser generator.

MIBs are text files defined using [ASN.1](https://www.oss.com/asn1/resources/asn1-made-simple/introduction.html) syntax and are part of the foundation of SNMP.  MIBs describe a range of simple to complex data objects used by a particular piece of SNMP capable equipment.

The project would not be possible without the excellent work completed by the [Mibble](https://www.mibble.org) team.

Currently the project will "mostly" parse a MIB file but I still need to work out some quirks with lalrpop.   Nothing is generated from the parse tree.
