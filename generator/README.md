# cdevents rust code generator

Goals: generate rust code for cdevents from jsonschema provided as part of cdevents specs.

- The generator take read jsonschema as json apply them to a set of templates
- The generator is very basic (no json schema semantic, no `$ref` resolution) like [eventuallyconsultant/codegenr: Fast handlebars templates based code generator, ingesting swagger/openapi and other json/yaml documents with $refs, or graphql schema, outputs whatever you template](https://github.com/eventuallyconsultant/codegenr/)
- The generator is currently used to generate Subjects

## Why not use a jsonschema to rust generator?

- I tried some and they failed (no error), maybe too early, or do not support the version of jsonschema used by cdevents (often they support jsonschema draft-4)
- The jsonschemas (v0.3) are not connected (a set of independent schemas), so a lot of duplication (context,...), so classical generators will create as many Context types as Event types,... In our implementation, only parts of the schema are extracted to generate what is different aka the `content` of subjects.

## Run

To generate the `subjects` into sibling crate `cdevents/src/generated` from the content of `cdevents-specs/spec-v0.4/schemas`, from the root workspace

```sh
cargo run -p generator -- --help
cargo run -p generator -- --templates-dir "generator/templates" --jsonschema-dir "cdevents-specs/spec-v0.4/schemas" --dest "cdevents-sdk/src/generated"
```
