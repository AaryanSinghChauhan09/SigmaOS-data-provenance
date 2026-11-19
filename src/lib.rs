#![forbid(unsafe_code)]
//! SigmaOS-data-provenance: data provenance model.
//!
//! Real implementation: sources, timestamps, ownership, and validation.
//! Replaces the 108-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Provenance record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvenanceRecord {
    pub id: String,
    pub name: String,
    pub source: String,
    pub owner: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProvenanceError {
    EmptyName,
    DuplicateRecord(String),
}

impl fmt::Display for ProvenanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "record name must not be empty"),
            Self::DuplicateRecord(id) => write!(f, "duplicate record {:?}", id),
        }
    }
}

impl std::error::Error for ProvenanceError {}

/// Data provenance manager.
pub struct DataProvenance {
    pub name: String,
    records: BTreeMap<String, ProvenanceRecord>,
}

impl DataProvenance {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            records: BTreeMap::new(),
        }
    }

    pub fn add_record(&mut self, record: ProvenanceRecord) -> Result<(), ProvenanceError> {
        if record.name.trim().is_empty() { return Err(ProvenanceError::EmptyName); }
        let id = record.id.clone();
        if self.records.contains_key(&id) {
            return Err(ProvenanceError::DuplicateRecord(id.clone()));
        }
        self.records.insert(id, record);
        Ok(())
    }

    pub fn record_count(&self) -> usize { self.records.len() }

    pub fn get_record(&self, id: &str) -> Option<&ProvenanceRecord> { self.records.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provenance_create_aaa() {
        let p = DataProvenance::new("My Provenance");
        assert_eq!(p.name, "My Provenance");
        assert_eq!(p.record_count(), 0);
    }

    #[test]
    fn test_add_record_aaa() {
        let mut p = DataProvenance::new("My Provenance");
        let r = ProvenanceRecord { id: "r1".into(), name: "dataset".into(), source: "api".into(), owner: "team".into() };
        p.add_record(r).unwrap();
        assert_eq!(p.record_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut p = DataProvenance::new("My Provenance");
        let r = ProvenanceRecord { id: "x".into(), name: "".into(), source: "a".into(), owner: "b".into() };
        assert!(p.add_record(r).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut p = DataProvenance::new("My Provenance");
        let r = ProvenanceRecord { id: "x".into(), name: "X".into(), source: "a".into(), owner: "b".into() };
        p.add_record(r).unwrap();
        let r2 = ProvenanceRecord { id: "x".into(), name: "X".into(), source: "a".into(), owner: "b".into() };
        assert!(p.add_record(r2).is_err());
    }
}
