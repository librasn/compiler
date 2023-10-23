/// This suite contains tests to test x681 Information Object Classes and its
/// associated notations

const ATTRIBUTE: &'static str = r#"ATTRIBUTE ::= CLASS {
    &derivation            ATTRIBUTE OPTIONAL,
    &Type                  OPTIONAL, -- either &Type or &derivation required 
    &equality-match        MATCHING-RULE OPTIONAL,
    &ordering-match        MATCHING-RULE OPTIONAL,
    &substrings-match      MATCHING-RULE OPTIONAL,
    &single-valued         BOOLEAN DEFAULT FALSE,
    &collective            BOOLEAN DEFAULT FALSE,
    -- operational extensions 
    &no-user-modification  BOOLEAN DEFAULT FALSE,
    &usage                 AttributeUsage DEFAULT userApplications,
    &id                    OBJECT IDENTIFIER UNIQUE
  }
  WITH SYNTAX {
    [SUBTYPE OF &derivation]
    [WITH SYNTAX &Type]
    [EQUALITY MATCHING RULE &equality-match]
    [ORDERING MATCHING RULE &ordering-match]
    [SUBSTRINGS MATCHING RULE &substrings-match]
    [SINGLE VALUE &single-valued]
    [COLLECTIVE &collective]
    [NO USER MODIFICATION &no-user-modification]
    [USAGE &usage]
    ID &id
  }"#;