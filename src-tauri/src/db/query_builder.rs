// Query builder utilities for safe dynamic SQL construction
//
// SECURITY: This module provides helpers for building dynamic SQL queries
// while maintaining 100% parameterization. All user-provided values must
// be passed via the params vector with `?` placeholders.

use rusqlite::ToSql;

/// Helper for building dynamic queries with optional date filtering
///
/// This helper standardizes the pattern of adding WHERE clauses for date ranges
/// while maintaining parameterized queries for security.
///
/// # Security Note
/// This is SAFE because:
/// 1. Only static SQL strings are returned (no user input interpolated)
/// 2. User-provided date values are returned separately in the params vector
/// 3. Caller must use the params vector with `?` placeholders in the query
///
/// # Example
/// ```rust,ignore
/// let (where_clause, params) = DateFilterBuilder::new()
///     .with_from_date(from_date, "created_at")
///     .with_to_date(to_date, "created_at")
///     .build();
///
/// let query = format!("SELECT * FROM table WHERE 1=1 {}", where_clause);
/// let params_refs: Vec<&dyn ToSql> = params.iter().map(|p| p.as_ref()).collect();
/// stmt.query_map(&params_refs[..], |row| { /* ... */ })?;
/// ```
pub struct DateFilterBuilder<'a> {
    conditions: Vec<String>,
    params: Vec<Box<dyn ToSql + 'a>>,
}

impl<'a> DateFilterBuilder<'a> {
    /// Create a new date filter builder
    pub fn new() -> Self {
        Self {
            conditions: Vec::new(),
            params: Vec::new(),
        }
    }

    /// Add a "greater than or equal" date filter
    ///
    /// # Parameters
    /// - `date`: Optional date string to filter by
    /// - `column`: The column name to filter (e.g., "created_at", "completed_at")
    pub fn with_from_date(mut self, date: Option<&'a str>, column: &str) -> Self {
        if let Some(from) = date {
            self.conditions.push(format!("AND {} >= ?", column));
            self.params.push(Box::new(from.to_string()));
        }
        self
    }

    /// Add a "less than or equal" date filter
    ///
    /// # Parameters
    /// - `date`: Optional date string to filter by
    /// - `column`: The column name to filter (e.g., "created_at", "completed_at")
    pub fn with_to_date(mut self, date: Option<&'a str>, column: &str) -> Self {
        if let Some(to) = date {
            self.conditions.push(format!("AND {} <= ?", column));
            self.params.push(Box::new(to.to_string()));
        }
        self
    }

    /// Build the WHERE clause string and params vector
    ///
    /// Returns a tuple of (where_clause, params) where:
    /// - where_clause: String containing SQL conditions (may be empty)
    /// - params: Vector of parameters to bind to the query
    pub fn build(self) -> (String, Vec<Box<dyn ToSql + 'a>>) {
        let where_clause = if self.conditions.is_empty() {
            String::new()
        } else {
            format!(" {}", self.conditions.join(" "))
        };
        (where_clause, self.params)
    }

    /// Build WHERE or AND clause based on whether this is the first condition
    ///
    /// Returns a tuple of (where_clause, params) where:
    /// - where_clause: Starts with "WHERE" if first condition, empty otherwise
    /// - params: Vector of parameters to bind to the query
    pub fn build_where(self) -> (String, Vec<Box<dyn ToSql + 'a>>) {
        let where_clause = if self.conditions.is_empty() {
            String::new()
        } else {
            // Replace first "AND" with "WHERE"
            let clause = self.conditions.join(" ");
            clause.replacen("AND", "WHERE", 1)
        };
        (where_clause, self.params)
    }
}

impl<'a> Default for DateFilterBuilder<'a> {
    fn default() -> Self {
        Self::new()
    }
}

/// Generate a parameterized IN clause for batch validation
///
/// Creates a SQL IN clause with the correct number of placeholders
/// for validating multiple IDs in a single query.
///
/// # Security Note
/// This is SAFE because:
/// 1. Only generates placeholder `?` characters (no user input)
/// 2. Caller must bind actual values via params vector
///
/// # Example
/// ```rust,ignore
/// let ids = vec![1, 2, 3];
/// let in_clause = generate_in_clause(ids.len());
/// // Returns: "(?, ?, ?)"
///
/// let query = format!("SELECT * FROM table WHERE id IN {}", in_clause);
/// let params: Vec<&dyn ToSql> = ids.iter().map(|id| id as &dyn ToSql).collect();
/// stmt.query_map(&params[..], |row| { /* ... */ })?;
/// ```
pub fn generate_in_clause(count: usize) -> String {
    if count == 0 {
        return "()".to_string();
    }
    let placeholders = vec!["?"; count].join(", ");
    format!("({})", placeholders)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_filter_builder_empty() {
        let (clause, params) = DateFilterBuilder::new().build();
        assert_eq!(clause, "");
        assert_eq!(params.len(), 0);
    }

    #[test]
    fn test_date_filter_builder_from_only() {
        let from_date = "2025-01-01";
        let (clause, params) = DateFilterBuilder::new()
            .with_from_date(Some(from_date), "created_at")
            .build();

        assert_eq!(clause, " AND created_at >= ?");
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn test_date_filter_builder_to_only() {
        let to_date = "2025-12-31";
        let (clause, params) = DateFilterBuilder::new()
            .with_to_date(Some(to_date), "created_at")
            .build();

        assert_eq!(clause, " AND created_at <= ?");
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn test_date_filter_builder_both() {
        let from_date = "2025-01-01";
        let to_date = "2025-12-31";
        let (clause, params) = DateFilterBuilder::new()
            .with_from_date(Some(from_date), "created_at")
            .with_to_date(Some(to_date), "created_at")
            .build();

        assert_eq!(clause, " AND created_at >= ? AND created_at <= ?");
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn test_date_filter_builder_where() {
        let from_date = "2025-01-01";
        let (clause, params) = DateFilterBuilder::new()
            .with_from_date(Some(from_date), "created_at")
            .build_where();

        assert_eq!(clause, "WHERE created_at >= ?");
        assert_eq!(params.len(), 1);
    }

    #[test]
    fn test_generate_in_clause_empty() {
        let clause = generate_in_clause(0);
        assert_eq!(clause, "()");
    }

    #[test]
    fn test_generate_in_clause_single() {
        let clause = generate_in_clause(1);
        assert_eq!(clause, "(?)");
    }

    #[test]
    fn test_generate_in_clause_multiple() {
        let clause = generate_in_clause(5);
        assert_eq!(clause, "(?, ?, ?, ?, ?)");
    }
}
