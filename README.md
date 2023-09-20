# Rasn compiler
The `rasn-compiler` library is a parser combinator that parses ASN1 specifications and outputs
encoding-rule-agnotic rust representations of the ASN1 data elements to be used with the [`rasn`](https://github.com/XAMPPRocky/rasn) crate. 
The compiler heavily relies on the great library [nom](https://docs.rs/nom/latest/nom/) for its basic parsers. 

## Example
In order to compile ASN1 in your build process, invoke the `rasn-compiler` in your [`build.rs` build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html).

```rust
// build.rs build script
use std::path::PathBuf;
use rasn_compiler::RasnCompiler;
fn main() {
  // Initialize the compiler
  match RasnCompiler::new()
    // add a single ASN1 source file
    .add_asn_by_path(PathBuf::from("spec_1.asn"))
    // add several ASN1 source files
    .add_asn_sources_by_path(vec![
        PathBuf::from("spec_2.asn"),
        PathBuf::from("spec_3.asn"),
    ].iter())
    // set an output path for the generated rust code
    .set_output_path(PathBuf::from("./asn/generated.rs"))
    // you may also compile literal ASN1 snippets
    .add_asn_literal("My-test-integer ::= INTEGER (1..128)")
    // optionally choose to support `no_std`
    .compile() {
    Ok(warnings /* Vec<Box<dyn Error>> */) => { /* handle compilation warnings */ }
    Err(error /* Box<dyn Error> */) => { /* handle unrecoverable compilation error */ }
  }
}
```

## ASN1 Support
ASN1 is a complex standard, and not all of its features and encoding rules are supported, yet.

Currently, `rasn` supports the following encoding rules:
* Basic Encoding Rules (BER)
* Canonical Encoding Rules (CER)
* Distinguished Encoding Rules (DER)
* Aligned Packed Encoding Rules (APER)
* Unaligned Packed Encoding Rules (UPER)

`rasn` and the `rasn-compiler` support the following ASN1 features:

#### Types
* `NULL` type and value
* `BOOLEAN` type and value
* `NumericString` type and value
* `VisibleString` type and value
* `IA5String` type and value
* `TeletexString` type and value
* `GeneralString` type and value
* `UTF8String` type and value
* `BMPString` type and value
* `PrintableString` type and value
* `BIT STRING` type and value (hex- and bitstring declations)
* `OCTET STRING` type and value (hex- and bitstring declations)
* `OBJECT IDENTIFIER` type and value
* `SEQUENCE` type and value
* `SET` type and value
* `SEQUENCE OF` type
* `ENUMERATED` type and value
* `CHOICE` type and value
* `UTCTime` type and value
* `GeneralizedTime` type and value

#### Constraints
* Single value constraints
* Value range constraints
* Contained subtype constraints
* Size constraints
* Permitted alphabet constraints
* Constraint set operations

#### Misc
* `DEFAULT` member values
* extensions and extension groups
* Parameterization (the `rasn-compiler` creates rust representations for invocations of the parameterized data elements in the given spec, i.e. it does not preserve the parameterization itself)
* Information Object Classes (however, the `rasn-compiler` does not link type references in class instances)
