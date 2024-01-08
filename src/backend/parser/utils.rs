type _IntermediateResult = Result<String, String>;

// For [`Statement`]s and [`Expression`]s.
#[derive(Debug)]
pub enum IntermediateResult {
    Ok(String),
    Err(String)
}
