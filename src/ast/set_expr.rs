use super::{expr::Expr, Ident};
use crate::ast::expr::WindowSpec;
use crate::ast::table_ref::TableRef;

#[derive(Debug, Clone)]
pub enum SetExpr {
    Select {
        distinct: bool,
        projection: Vec<SelectItem>,
        from: Option<TableRef>,
        selection: Option<Expr>,
        group_by: Vec<Expr>,
        having: Option<Expr>,
        named_windows: Vec<NamedWindowDef>,
    },
}
impl std::fmt::Display for SetExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Select {
                distinct,
                projection,
                from,
                selection,
                group_by,
                having,
                named_windows,
            } => {
                write!(
                    f,
                    "SELECT {}{}",
                    if *distinct { "DISTINCT " } else { "" },
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
                if !named_windows.is_empty() {
                    write!(
                        f,
                        " WINDOW {}",
                        named_windows
                            .iter()
                            .map(|w| w.to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    )?;
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
    Wildcard(WildcardOptions),
}
impl std::fmt::Display for SelectItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnnamedExpr(expr) => write!(f, "{}", expr),
            Self::ExprWithAlias { expr, alias } => write!(f, "{} AS {}", expr, alias),
            Self::Wildcard(options) => write!(f, "*{}", options),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WildcardOptions {
    pub exclude: Vec<Ident>,
    pub except: Vec<Ident>,
}
impl std::fmt::Display for WildcardOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.exclude.is_empty() {
            write!(
                f,
                " EXCLUDE {}",
                if self.exclude.len() == 1 {
                    self.exclude.get(0).unwrap().to_string()
                } else {
                    format!(
                        "({})",
                        self.exclude
                            .iter()
                            .map(|col| col.to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                    )
                }
            )?;
        }
        if !self.except.is_empty() {
            write!(
                f,
                " EXCEPT ({})",
                self.except
                    .iter()
                    .map(|col| col.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct NamedWindowDef {
    pub name: Ident,
    pub spec: WindowSpec,
}
impl std::fmt::Display for NamedWindowDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} AS ({})", self.name, self.spec)
    }
}
