use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy)]
pub struct Pagination {
    pub limit: usize,
    pub offset: Option<usize>,
    pub dir: PaginationDir,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum PaginationDir {
    Asc,
    Desc,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Paginated<T> {
    pub results: Vec<T>,
    pub has_more: bool,
    pub total: usize,
}

impl<T> Paginated<T> {
    pub fn paginate(
        items: impl ExactSizeIterator<Item = T> + DoubleEndedIterator<Item = T>,
        pagination: Pagination,
    ) -> Self {
        let Pagination { limit, offset, dir } = pagination;

        let offset = offset.unwrap_or(0);

        Self {
            has_more: items.len() > offset + limit,
            total: items.len(),
            results: match dir {
                PaginationDir::Asc => items.skip(offset).take(limit).collect(),
                PaginationDir::Desc => items.rev().skip(offset).take(limit).collect(),
            },
        }
    }

    pub fn map<U>(self, f: impl Fn(T) -> U) -> Paginated<U> {
        Paginated {
            results: self.results.into_iter().map(f).collect(),
            has_more: self.has_more,
            total: self.total,
        }
    }
}
