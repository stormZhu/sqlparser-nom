use super::{expr::Expr, Ident};
use crate::ast::statement::SelectStatement;

#[derive(Debug, Clone)]
pub enum SetExpr {
    Select {
        projection: Vec<SelectItem>,
        from: Option<TableRef>,
        selection: Option<Expr>,
        group_by: Vec<Expr>,
        having: Option<Expr>,
    },
}
impl std::fmt::Display for SetExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Select {
                projection,
                from,
                selection,
                group_by,
                having,
            } => {
                write!(
                    f,
                    "SELECT {}",
                    projection
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                )?;
                if let Some(from) = from {
                    write!(f, " FROM {}", from)?;
                }
                if let Some(selection) = selection {
                    write!(f, " WHERE {}", selection)?;
                }
                if !group_by.is_empty() {
                    write!(
                        f,
                        " GROUP BY {}",
                        group_by
                            .iter()
                            .map(|p| p.to_string())
                            .collect::<Vec<_>>()
                            .join(", "),
                    )?;
                }
                if let Some(expr) = having {
                    write!(f, " Having {}", expr)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum SelectItem {
    UnnamedExpr(Expr),
    ExprWithAlias { expr: Expr, alias: Ident },
    Wildcard,
}
impl std::fmt::Display for SelectItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnnamedExpr(expr) => write!(f, "{}", expr),
            Self::ExprWithAlias { expr, alias } => write!(f, "{} AS {}", expr, alias),
            Self::Wildcard => write!(f, "*"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TableRef {
    BaseTable {
        name: TableName,
        alias: Option<Ident>,
    },
    Subquery {
        subquery: Box<SelectStatement>,
        alias: Option<Ident>,
    },
}
impl std::fmt::Display for TableRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableRef::BaseTable { name, alias } => {
                write!(f, "{name}")?;
                if let Some(alias) = alias {
                    write!(f, " AS {alias}")?;
                }
                Ok(())
            }
            TableRef::Subquery { subquery, alias } => {
                write!(f, "({subquery})")?;
                if let Some(alias) = alias {
                    write!(f, " AS {alias}")?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct TableName {
    pub database: Option<Ident>,
    pub table: Ident,
}
impl std::fmt::Display for TableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(database) = self.database.as_ref() {
            write!(f, "{database}.")?;
        }
        write!(f, "{}", self.table)?;
        Ok(())
    }
}
