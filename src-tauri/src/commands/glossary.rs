use crate::core::error::AppResult;
use crate::glossaries::{GlossaryQuery, GlossaryState};
use crate::glossaries::model::GlossaryTerm;

pub async fn list_terms(state: &GlossaryState, q: GlossaryQuery) -> AppResult<Vec<GlossaryTerm>> {
    crate::glossaries::repo::find_terms(state, &q).await
}

pub async fn upsert_term(state: &GlossaryState, term: GlossaryTerm) -> AppResult<i64> {
    crate::glossaries::repo::upsert_term(state, term).await
}

pub async fn delete_term(state: &GlossaryState, id: i64) -> AppResult<()> {
    crate::glossaries::repo::delete_term(state, id).await
}

/// Export terms as JSON (filtered by query)
pub async fn export_terms(state: &GlossaryState, q: GlossaryQuery) -> AppResult<String> {
    crate::glossaries::repo::export_terms_json(state, &q).await
}

/// Import terms from JSON string (upsert)
pub async fn import_terms(state: &GlossaryState, json: String) -> AppResult<usize> {
    crate::glossaries::repo::import_terms_json(state, &json).await
}


