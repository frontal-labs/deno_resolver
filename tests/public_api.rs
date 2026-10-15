use deno_resolver::sloppy_imports::SloppyImportsResolution;
use deno_resolver::sloppy_imports::SloppyImportsResolutionKind;
use url::Url;

#[test]
fn sloppy_import_resolution_exposes_specifier_and_messages() {
  let resolution = SloppyImportsResolution::JsToTs(
    Url::parse("file:///project/mod.ts").unwrap(),
  );
  assert_eq!(resolution.as_specifier().path(), "/project/mod.ts");
  assert_eq!(
    resolution.as_suggestion_message(),
    "Maybe change the extension to '.ts'"
  );
  assert_eq!(
    resolution.as_quick_fix_message(),
    "Change the extension to '.ts'."
  );
  assert_eq!(resolution.into_specifier().path(), "/project/mod.ts");
}

#[test]
fn resolution_kind_distinguishes_type_analysis() {
  assert!(SloppyImportsResolutionKind::Types.is_types());
  assert!(!SloppyImportsResolutionKind::Execution.is_types());
}
