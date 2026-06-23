pub use biome_rowan::FileSourceError;
use std::{ffi::OsStr, path::Path};

#[derive(Debug, Clone, Default, Copy, Eq, PartialEq, Hash)]
pub enum PsqlModuleKind {
    #[default]
    Query,

    Script,
}

impl PsqlModuleKind {
    pub const fn is_script(&self) -> bool {
        matches!(self, PsqlModuleKind::Script)
    }
    pub const fn is_query(&self) -> bool {
        matches!(self, PsqlModuleKind::Query)
    }
}

#[derive(Debug, Clone, Default, Copy, Eq, PartialEq, Hash)]
pub struct PsqlFileSource {
    module_kind: PsqlModuleKind,
}

impl PsqlFileSource {
    pub fn query() -> Self {
        Self::default()
    }

    pub fn script() -> Self {
        Self::default().with_module_kind(PsqlModuleKind::Script)
    }

    pub const fn with_module_kind(mut self, kind: PsqlModuleKind) -> Self {
        self.module_kind = kind;
        self
    }

    pub fn set_module_kind(&mut self, kind: PsqlModuleKind) {
        self.module_kind = kind;
    }

    pub const fn module_kind(&self) -> PsqlModuleKind {
        self.module_kind
    }

    pub const fn is_query(&self) -> bool {
        self.module_kind.is_query()
    }

    pub const fn is_script(&self) -> bool {
        self.module_kind.is_script()
    }

    pub fn file_extension(&self) -> &str {
        match self.module_kind {
            PsqlModuleKind::Query => "",
            PsqlModuleKind::Script => "sql",
        }
    }

    /// Try to return the mlang file source corresponding to this file extension
    pub fn try_from_extension(extension: &OsStr) -> Result<Self, FileSourceError> {
        // We assume the file extension is normalized to lowercase
        match extension.as_encoded_bytes() {
            b"sql" => Ok(Self::script()),
            _ => Ok(Self::query()),
        }
    }
}

impl TryFrom<&Path> for PsqlFileSource {
    type Error = FileSourceError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let extension = path
            .extension()
            // We assume the file extensions are case-insensitive.
            // Thus, we normalize the extension to lowercase.
            .map(|ext| ext.to_ascii_lowercase())
            .ok_or(FileSourceError::MissingFileExtension)?;

        Self::try_from_extension(&extension)
    }
}
