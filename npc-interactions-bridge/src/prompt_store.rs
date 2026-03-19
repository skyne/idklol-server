use std::collections::BTreeMap;
use std::fmt;
use std::fs;
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone)]
pub struct PromptReloadSummary {
    pub prompt_count: usize,
    pub active_prompt_key: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PromptStore {
    root_dir: PathBuf,
    prompts: BTreeMap<String, String>,
    active_prompt_key: Option<String>,
}

impl PromptStore {
    pub fn load(
        root_dir: PathBuf,
        preferred_active_prompt: Option<String>,
    ) -> Result<Self, PromptStoreError> {
        let prompts = load_prompt_map(&root_dir)?;

        let active_prompt_key = match preferred_active_prompt {
            Some(preferred) => {
                if prompts.contains_key(&preferred) {
                    Some(preferred)
                } else {
                    return Err(PromptStoreError::MissingPromptKey(preferred));
                }
            }
            None => prompts.keys().next().cloned(),
        };

        Ok(Self {
            root_dir,
            prompts,
            active_prompt_key,
        })
    }

    pub fn summary(&self) -> PromptReloadSummary {
        PromptReloadSummary {
            prompt_count: self.prompts.len(),
            active_prompt_key: self.active_prompt_key.clone(),
        }
    }

    pub fn list_prompt_keys(&self) -> Vec<String> {
        self.prompts.keys().cloned().collect()
    }

    pub fn active_prompt_key(&self) -> Option<&str> {
        self.active_prompt_key.as_deref()
    }

    pub fn active_prompt_preview(&self, max_chars: usize) -> Option<String> {
        let active_key = self.active_prompt_key()?;
        let text = self.prompts.get(active_key)?;
        Some(compact_preview(text, max_chars))
    }

    pub fn set_active_prompt(&mut self, key: &str) -> Result<(), PromptStoreError> {
        if !self.prompts.contains_key(key) {
            return Err(PromptStoreError::MissingPromptKey(key.to_string()));
        }

        self.active_prompt_key = Some(key.to_string());
        Ok(())
    }

    pub fn resolve_system_prompt(
        &self,
        requested_prompt_key: Option<&str>,
    ) -> Result<(Option<String>, Option<String>), PromptStoreError> {
        if let Some(requested_key) = requested_prompt_key {
            let text = self
                .prompts
                .get(requested_key)
                .ok_or_else(|| PromptStoreError::MissingPromptKey(requested_key.to_string()))?;
            return Ok((Some(requested_key.to_string()), Some(text.clone())));
        }

        let Some(active_key) = self.active_prompt_key.as_ref() else {
            return Ok((None, None));
        };

        let Some(text) = self.prompts.get(active_key) else {
            return Ok((None, None));
        };

        Ok((Some(active_key.clone()), Some(text.clone())))
    }

    pub fn reload(&mut self) -> Result<PromptReloadSummary, PromptStoreError> {
        let prompts = load_prompt_map(&self.root_dir)?;
        let previous_active = self.active_prompt_key.clone();

        self.prompts = prompts;

        self.active_prompt_key = match previous_active {
            Some(previous) if self.prompts.contains_key(&previous) => Some(previous),
            _ => self.prompts.keys().next().cloned(),
        };

        Ok(self.summary())
    }

    pub fn root_dir(&self) -> &Path {
        &self.root_dir
    }
}

fn load_prompt_map(root_dir: &Path) -> Result<BTreeMap<String, String>, PromptStoreError> {
    if !root_dir.exists() {
        return Err(PromptStoreError::RootNotFound(root_dir.to_path_buf()));
    }

    let mut files = Vec::new();
    collect_prompt_files(root_dir, &mut files)?;
    files.sort();

    let mut prompts = BTreeMap::new();
    for file_path in files {
        let key = path_to_prompt_key(root_dir, &file_path)?;
        let content = fs::read_to_string(&file_path)
            .map_err(|source| PromptStoreError::Io {
                path: file_path.clone(),
                source,
            })?;

        prompts.insert(key, content);
    }

    Ok(prompts)
}

fn collect_prompt_files(dir: &Path, output: &mut Vec<PathBuf>) -> Result<(), PromptStoreError> {
    let entries = fs::read_dir(dir).map_err(|source| PromptStoreError::Io {
        path: dir.to_path_buf(),
        source,
    })?;

    for entry_result in entries {
        let entry = entry_result.map_err(|source| PromptStoreError::Io {
            path: dir.to_path_buf(),
            source,
        })?;

        let path = entry.path();

        if path.is_dir() {
            collect_prompt_files(&path, output)?;
        } else if is_prompt_file(&path) {
            output.push(path);
        }
    }

    Ok(())
}

fn is_prompt_file(path: &Path) -> bool {
    let Some(extension) = path.extension().and_then(|value| value.to_str()) else {
        return false;
    };

    matches!(
        extension.to_ascii_lowercase().as_str(),
        "prompt" | "txt" | "md"
    )
}

fn path_to_prompt_key(root_dir: &Path, prompt_path: &Path) -> Result<String, PromptStoreError> {
    let relative = prompt_path
        .strip_prefix(root_dir)
        .map_err(|_| PromptStoreError::InvalidPromptPath(prompt_path.to_path_buf()))?;

    let mut no_extension = relative.to_path_buf();
    no_extension.set_extension("");

    let key_parts: Vec<String> = no_extension
        .components()
        .filter_map(component_to_string)
        .collect();

    if key_parts.is_empty() {
        return Err(PromptStoreError::InvalidPromptPath(prompt_path.to_path_buf()));
    }

    Ok(key_parts.join("/"))
}

fn component_to_string(component: Component<'_>) -> Option<String> {
    match component {
        Component::Normal(value) => Some(value.to_string_lossy().to_string()),
        _ => None,
    }
}

fn compact_preview(text: &str, max_chars: usize) -> String {
    let single_line = text.split_whitespace().collect::<Vec<_>>().join(" ");
    if single_line.chars().count() <= max_chars {
        return single_line;
    }

    single_line.chars().take(max_chars).collect::<String>() + "…"
}

#[derive(Debug)]
pub enum PromptStoreError {
    RootNotFound(PathBuf),
    MissingPromptKey(String),
    InvalidPromptPath(PathBuf),
    Io { path: PathBuf, source: std::io::Error },
}

impl fmt::Display for PromptStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RootNotFound(path) => {
                write!(f, "prompt root directory not found: {}", path.display())
            }
            Self::MissingPromptKey(key) => write!(f, "prompt key not found: {key}"),
            Self::InvalidPromptPath(path) => {
                write!(f, "invalid prompt file path: {}", path.display())
            }
            Self::Io { path, source } => {
                write!(f, "io error at {}: {}", path.display(), source)
            }
        }
    }
}

impl std::error::Error for PromptStoreError {}
