#[macro_use] extern crate lalrpop_util;


lalrpop_mod!(pub asn1_grammar); // synthesized by LALRPOP
pub mod ast;

#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}


#[test]
fn module_parser() {
    
    let ast = asn1_grammar::StartParser::new().parse("RuckusMib  DEFINITIONS ::= BEGIN  END").unwrap();
    println!("-------------------");
    println!("{:#?}",ast);

    assert!(asn1_grammar::StartParser::new().parse("RuckusMib  DEFINITIONS ::= BEGIN  END").is_ok());

    assert!(asn1_grammar::StartParser::new().parse("RuckusMib  DEFINITIONS EXPLICIT TAGS ::= BEGIN  END").is_ok());
    assert!(asn1_grammar::StartParser::new().parse("RuckusMib  DEFINITIONS IMPLICIT TAGS ::= BEGIN  END").is_ok());

}

//#[test]
fn module_body_parser() {    
    assert!(asn1_grammar::StartParser::new().parse("RuckusMib  DEFINITIONS ::= BEGIN EXPORTS;   END").is_ok());
    assert!(asn1_grammar::StartParser::new().parse("RuckusMib  DEFINITIONS ::= BEGIN IMPORTS;   END").is_ok());
}

#[test]
fn macro_assignment_body_parser() {
//    MacroDefinition = { MacroReference "MACRO" "::=" MacroBody };
    assert!(asn1_grammar::StartParser::new().parse("RuckusMib  DEFINITIONS ::= BEGIN EXPORTS;  sams_macro MACRO ::= BEGIN macrobody END END").is_ok());

}

//#[test]
fn type_assignment_body_parser() {
//    MacroDefinition = { MacroReference "MACRO" "::=" MacroBody };
    assert!(asn1_grammar::StartParser::new().parse(
        r"RuckusMib  DEFINITIONS ::= 
                BEGIN 
                    EXPORTS;  
                    myType ::= NULL
                    myBool ::= BOOLEAN 
                    myOid ::= OBJECT IDENTIFIER
                    foReal ::= REAL
                    myString ::= OCTET STRING
                    myInt ::= INTEGER
                    mySeq ::= SEQUENCE {  mycomp COMPONENTS OF REAL
                        , INTEGER
                        , OCTET STRING
                        , NULL
                        , myOptional_NullType NULL OPTIONAL
                        , myDefault BOOLEAN DEFAULT hello
                        , myDefault BOOLEAN DEFAULT myHello hello
                    }
                    myBitString ::= BIT STRING
                    myBits ::= BITS
                    mySeqOf ::= SEQUENCE     OF NULL
                    mySetType ::= SET {  mycomp COMPONENTS OF REAL
                        , INTEGER
                        , OCTET STRING
                        , NULL
                        , myOptional_NullType NULL OPTIONAL
                        , myDefault BOOLEAN DEFAULT hello
                        , myDefault BOOLEAN DEFAULT myHello hello
                    }
                    mySetOfType ::= SET OF  BOOLEAN
                    myChoiceType ::= CHOICE {  COMPONENTS OF REAL }
                    myChoiceType ::= CHOICE {  mycomp COMPONENTS OF REAL
                                                , INTEGER
                                                , OCTET STRING
                                                , NULL
                                                , myOptional_NullType NULL OPTIONAL
                                                , myDefault BOOLEAN DEFAULT hello
                                                , myDefault BOOLEAN DEFAULT myHello hello
                                            }

                END").is_ok());

}
