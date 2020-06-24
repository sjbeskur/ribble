# ribble
Ribble is an attempt at porting the popular Java based, [Mibble](https://www.mibble.org/) MIB Parser to Rust using the [lalrpop](https://github.com/lalrpop/lalrpop) parser generator.

MIB are text files defined using [ASN.1](https://www.oss.com/asn1/resources/asn1-made-simple/introduction.html) syntax and are  SNMP defines the data exposed by SNMP agents. MIBs are written in ASN.1, and parsing them is not trivial!

 text file that lists the data objects used by a particular piece of SNMP equipment. How does MIB work in 3 steps: ... Your SNMP manager will use the provided MIB data to interpret the incoming messages from your new devic

The project would not be possible without the excellent work completed by the [Mibble](https://www.mibble.org) team.

Currently the project will "mostly" parse a MIB file but I still need to work out some quirks with lalrpop.   Nothing is generated from the parse tree.
