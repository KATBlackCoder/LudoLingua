use crate::core::error::AppResult;
use crate::db::glossary::model::GlossaryTerm;
use crate::db::glossary::model::GlossaryQuery;
use crate::db::ManagedGlossaryState;

pub async fn list_terms(state: &ManagedGlossaryState, q: GlossaryQuery) -> AppResult<Vec<GlossaryTerm>> {
    crate::db::glossary::repo::find_terms(state, &q).await
}

pub async fn upsert_term(state: &ManagedGlossaryState, term: GlossaryTerm) -> AppResult<i64> {
    crate::db::glossary::repo::upsert_term(state, term).await
}

pub async fn delete_term(state: &ManagedGlossaryState, id: i64) -> AppResult<()> {
    crate::db::glossary::repo::delete_term(state, id).await
}

/// Export terms as JSON (filtered by query)
pub async fn export_terms(state: &ManagedGlossaryState, q: GlossaryQuery) -> AppResult<String> {
    crate::db::glossary::repo::export_terms_json(state, &q).await
}

/// Import terms from JSON string (upsert)
pub async fn import_terms(state: &ManagedGlossaryState, json: String) -> AppResult<usize> {
    crate::db::glossary::repo::import_terms_json(state, &json).await
}
