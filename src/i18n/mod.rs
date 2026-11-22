//! Translation of the user interface.
//!
//! Every text shown to the user is a field of [`Translations`]. Each
//! supported [`Language`] provides one complete instance of that struct
//! ([`ENGLISH`], [`FRENCH`]), so the compiler refuses a language with a
//! missing text.
//!
//! Texts may contain placeholders such as `{name}`, replaced at run time by
//! [`fill_placeholders`]. Errors are translated by the `describe_*`
//! functions, which keeps the rest of the application free of any language
//! concern.
//!
//! To add a language, see `docs/ARCHITECTURE.md`.

mod activity_kind_label;
mod describe_app_error;
mod describe_presence_error;
mod describe_validation_error;
mod english;
mod fill_placeholders;
mod french;
mod language;
mod language_name;
mod text_field_label;
mod translations;
mod translations_for;

pub use activity_kind_label::activity_kind_label;
pub use describe_app_error::describe_app_error;
pub use describe_presence_error::describe_presence_error;
pub use describe_validation_error::describe_validation_error;
pub use english::ENGLISH;
pub use fill_placeholders::fill_placeholders;
pub use french::FRENCH;
pub use language::Language;
pub use language_name::language_name;
pub use text_field_label::text_field_label;
pub use translations::Translations;
pub use translations_for::translations_for;
