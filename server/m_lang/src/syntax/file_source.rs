use biome_rowan::FileSourceError;
use std::{ffi::OsStr, path::Path};

#[derive(Debug, Clone, Default, Copy, Eq, PartialEq, Hash)]
pub enum ModuleKind {
    Script,

    #[default]
    Module,

    Report,
}

impl ModuleKind {
    pub const fn is_script(&self) -> bool {
        matches!(self, ModuleKind::Script)
    }
    pub const fn is_module(&self) -> bool {
        matches!(self, ModuleKind::Module)
    }

    pub const fn is_report(&self) -> bool {
        matches!(self, ModuleKind::Report)
    }
}

#[derive(Debug, Clone, Default, Copy, Eq, PartialEq, Hash)]
pub struct MFileSource {
    module_kind: ModuleKind,
}

impl MFileSource {
    pub fn module() -> Self {
        Self::default()
    }

    pub fn script() -> Self {
        Self::default().with_module_kind(ModuleKind::Script)
    }

    pub fn report() -> Self {
        Self::default().with_module_kind(ModuleKind::Report)
    }

    pub const fn with_module_kind(mut self, kind: ModuleKind) -> Self {
        self.module_kind = kind;
        self
    }

    pub fn set_module_kind(&mut self, kind: ModuleKind) {
        self.module_kind = kind;
    }

    pub const fn module_kind(&self) -> ModuleKind {
        self.module_kind
    }

    pub const fn is_module(&self) -> bool {
        self.module_kind.is_module()
    }

    pub const fn is_script(&self) -> bool {
        self.module_kind.is_script()
    }

    pub const fn is_report(&self) -> bool {
        self.module_kind.is_report()
    }

    pub fn file_extension(&self) -> &str {
        match self.module_kind {
            ModuleKind::Module => "prg",
            ModuleKind::Report => "rpt",
            ModuleKind::Script => "",
        }
    }

    /// Try to return the mlang file source corresponding to this file extension
    pub fn try_from_extension(extension: &OsStr) -> Result<Self, FileSourceError> {
        // We assume the file extension is normalized to lowercase
        match extension.as_encoded_bytes() {
            b"prg" | b"hdl" => Ok(Self::module()),
            _ if extension.to_string_lossy().starts_with("rpt") => Ok(Self::report()),
            _ if extension.to_string_lossy().starts_with("pa") => Ok(Self::report()),
            _ => Err(FileSourceError::UnknownExtension),
        }
    }
}

impl TryFrom<&Path> for MFileSource {
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
