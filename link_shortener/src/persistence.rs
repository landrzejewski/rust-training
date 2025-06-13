use common::id::Id;
use common::pagination::{PageRequest, PageResponse};
use link_shortener::{Error, Link, LinkRepository};
use sqlx::error::ErrorKind;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, PgPool, Row, Transaction};
use std::ops::DerefMut;

const INSERT_LINK: &str = r#"
    INSERT INTO links(id, original_url, shortened_path, is_active, created_at, expires_at)
    VALUES ($1, $2, $3, $4, $5, $6)"#;
const SELECT_LINK: &str = r#"
    SELECT l.*, COALESCE(array_agg(t.name), '{}') AS tags
    FROM links l
    LEFT JOIN links_tags lt ON l.id = lt.link_id
    LEFT JOIN tags t ON lt.tag_id = t.id
    WHERE l.id = $1
    GROUP BY l.id"#;
const SELECT_LINKS: &str = r#"
    SELECT l.*, COALESCE(array_agg(t.name), '{}') AS tags
    FROM links l
    LEFT JOIN links_tags lt ON l.id = lt.link_id
    LEFT JOIN tags t ON lt.tag_id = t.id
    GROUP BY l.id
    ORDER BY l.created_at DESC
    LIMIT $1
    OFFSET $2"#;
const SELECT_LINKS_BY_IDS: &str = r#"
    SELECT l.*, COALESCE(array_agg(t.name), '{}') AS tags
    FROM links l
    LEFT JOIN links_tags lt ON l.id = lt.link_id
    LEFT JOIN tags t ON lt.tag_id = t.id
    WHERE l.id = ANY($1)
    GROUP BY l.id
    ORDER BY l.created_at DESC
    LIMIT $2 OFFSET $3"#;
const SELECT_ACTIVE_LINK_BY_SHORTENED_PATH: &str = r#"
    SELECT *
    FROM links
    WHERE shortened_path = $1 AND is_active = TRUE"#;
const SELECT_LINK_COUNT: &str = r#"
    SELECT count(id)
    FROM links"#;
const UPDATE_LINK: &str = r#"
    UPDATE links
    SET original_url = $1, is_active = $2, expires_at = $3 WHERE id = $4"#;
const DELETE_LINK: &str = r#"
    DELETE
    FROM links
    WHERE id = $1"#;
const DELETE_EXPIRED: &str = r#"
    DELETE
    FROM links
    WHERE expires_at <= NOW()"#;
const INSERT_TAG: &str = r#"
    INSERT INTO tags(id, name)
    VALUES ($1,$2) ON CONFLICT DO NOTHING"#;
const SELECT_TAG_ID_BY_NAME: &str = r#"
    SELECT id
    FROM tags
    WHERE name = $1"#;
const DELETE_ORPHANED_TAGS: &str = r#"
    DELETE
    FROM tags
    WHERE id NOT IN (SELECT DISTINCT tag_id FROM links_tags)"#;
const INSERT_LINK_TAG: &str = r#"
    INSERT INTO links_tags(link_id, tag_id)
    VALUES ($1, $2) ON CONFLICT DO NOTHING"#;
const SELECT_LINK_IDS_BY_TAGS: &str = r"
    SELECT lt.link_id
    FROM links_tags lt
    JOIN tags t ON lt.tag_id = t.id
    WHERE t.name = ANY($1)
    GROUP BY lt.link_id
    HAVING COUNT(DISTINCT t.name) = $2";
const SELECT_LINK_COUNT_BY_IDS: &str = r#"
    SELECT COUNT(*)
    FROM links
    WHERE id = ANY($1)"#;
const DELETE_LINK_TAGS: &str = r#"
    DELETE
    FROM links_tags
    WHERE link_id = $1"#;

#[derive(Clone)]
pub struct SqlxLinkRepository {
    pool: PgPool,
}

impl SqlxLinkRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl SqlxLinkRepository {
    async fn save_link_tags(&self, link: &Link, transaction: &mut Transaction<'_, sqlx::Postgres>) -> Result<(), Error> {
        for tag in &link.tags {
            let tag_id = match self.find_tag_id(tag, transaction).await? {
                Some(id) => Id::from(id),
                None => self.save_tag(tag, transaction).await?,
            };
            sqlx::query(INSERT_LINK_TAG)
                .bind(link.id.to_str())
                .bind(tag_id.to_str())
                .execute(transaction.deref_mut())
                .await
                .map_err(sqlx_to_app_error)?;
        }
        Ok(())
    }
    async fn find_tag_id(&self, tag: &str, transaction: &mut Transaction<'_, sqlx::Postgres>) -> Result<Option<Id>, Error> {
        sqlx::query_scalar::<_, String>(SELECT_TAG_ID_BY_NAME)
            .bind(tag.to_string())
            .fetch_optional(transaction.deref_mut())
            .await
            .map(|id| id.map(Id::from))
            .map_err(sqlx_to_app_error)
    }
    async fn save_tag(&self, tag: &str, transaction: &mut Transaction<'_, sqlx::Postgres>) -> Result<Id, Error> {
        let id = Id::new();
        sqlx::query(INSERT_TAG)
            .bind(id.to_str())
            .bind(tag)
            .execute(transaction.deref_mut())
            .await
            .map_err(sqlx_to_app_error)?;
        Ok(id)
    }
    async fn delete_link_tags(&self, link_id: &Id, transaction: &mut Transaction<'_, sqlx::Postgres>) -> Result<(), Error> {
        sqlx::query(DELETE_LINK_TAGS)
            .bind(link_id.to_str())
            .execute(transaction.deref_mut())
            .await
            .map(|_| ())
            .map_err(sqlx_to_app_error)
    }
    fn calculate_total_pages(&self, page_size: usize, total_rows: i64) -> usize {
        (total_rows as f64 / page_size as f64).ceil() as usize
    }
}

impl LinkRepository for SqlxLinkRepository {
    async fn save(&self, link: Link) -> Result<Link, Error> {
        let mut transaction = self.pool.begin().await.map_err(|_| Error::DataAccessError)?;
        sqlx::query(INSERT_LINK)
            .bind(link.id.to_str())
            .bind(link.original_url.as_str())
            .bind(link.shortened_path.as_str())
            .bind(link.is_active)
            .bind(link.created_at)
            .bind(link.expires_at)
            .execute(&mut *transaction)
            .await
            .map_err(sqlx_to_app_error)?;
        self.save_link_tags(&link, &mut transaction).await?;
        transaction.commit().await.map_err(|_| Error::DataAccessError)?;
        Ok(link)
    }
    async fn find_all(&self, page_request: &PageRequest) -> Result<PageResponse<Link>, Error> {
        let total_rows = sqlx::query_scalar::<_, i64>(SELECT_LINK_COUNT).fetch_one(&self.pool).await.map_err(sqlx_to_app_error)?;
        let total_pages = self.calculate_total_pages(page_request.size, total_rows);
        sqlx::query_as::<_, LinkRow>(SELECT_LINKS)
            .bind(page_request.size as i64)
            .bind(page_request.offset() as i64)
            .fetch_all(&self.pool)
            .await
            .map(|rows| rows.into_iter().map(LinkRow::into_inner).collect())
            .map(|content| PageResponse { content, total_pages })
            .map_err(sqlx_to_app_error)
    }
    async fn find_by_id(&self, id: &Id) -> Result<Option<Link>, Error> {
        sqlx::query_as::<_, LinkRow>(SELECT_LINK)
            .bind(id.to_str())
            .fetch_optional(&self.pool)
            .await
            .map(|link_row| link_row.map(LinkRow::into_inner))
            .map_err(sqlx_to_app_error)
    }
    async fn find_by_shortened_path(&self, shortened_path: &str) -> Result<Option<Link>, Error> {
        sqlx::query_as::<_, LinkRow>(SELECT_ACTIVE_LINK_BY_SHORTENED_PATH)
            .bind(shortened_path)
            .fetch_optional(&self.pool)
            .await
            .map(|link_row| link_row.map(LinkRow::into_inner))
            .map_err(sqlx_to_app_error)
    }
    async fn find_by_tags(&self, tags: &Vec<String>, page_request: &PageRequest) -> Result<PageResponse<Link>, Error> {
        let link_ids: Vec<String> = sqlx::query(SELECT_LINK_IDS_BY_TAGS)
            .bind(tags)
            .bind(tags.len() as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(sqlx_to_app_error)?
            .into_iter()
            .map(|row| row.get::<String, _>("link_id"))
            .collect();
        if link_ids.is_empty() {
            return Ok(PageResponse::empty());
        }
        let total_rows = sqlx::query_scalar::<_, i64>(SELECT_LINK_COUNT_BY_IDS)
            .bind(&link_ids)
            .fetch_one(&self.pool)
            .await
            .map_err(sqlx_to_app_error)?;
        let total_pages = self.calculate_total_pages(page_request.size, total_rows);
        sqlx::query_as::<_, LinkRow>(SELECT_LINKS_BY_IDS)
            .bind(&link_ids)
            .bind(page_request.size as i64)
            .bind(page_request.offset() as i64) // Offset
            .fetch_all(&self.pool)
            .await
            .map_err(sqlx_to_app_error)
            .map(|rows| rows.into_iter().map(LinkRow::into_inner).collect())
            .map(|content| PageResponse { content, total_pages })
    }
    async fn update(&self, link: Link) -> Result<Link, Error> {
        let mut transaction = self.pool.begin().await.map_err(|_| Error::DataAccessError)?;
        sqlx::query(UPDATE_LINK)
            .bind(link.original_url.as_str())
            .bind(link.is_active)
            .bind(&link.expires_at)
            .bind(link.id.to_str())
            .execute(&mut *transaction)
            .await
            .map_err(sqlx_to_app_error)?;
        self.delete_link_tags(&link.id, &mut transaction).await?;
        self.save_link_tags(&link, &mut transaction).await?;
        transaction.commit().await.map_err(|_| Error::DataAccessError)?;
        Ok(link)
    }
    async fn delete_by_id(&self, id: &Id) -> Result<(), Error> {
        sqlx::query(DELETE_LINK).bind(id.to_str()).execute(&self.pool).await.map(|_| ()).map_err(sqlx_to_app_error)
    }
    async fn delete_expired(&self) -> Result<(), Error> {
        sqlx::query(DELETE_EXPIRED).execute(&self.pool).await.map(|_| ()).map_err(sqlx_to_app_error)
    }
    async fn delete_orphaned_tags(&self) -> Result<(), Error> {
        sqlx::query(DELETE_ORPHANED_TAGS).execute(&self.pool).await.map(|_| Ok(())).map_err(sqlx_to_app_error)?
    }
}

struct LinkRow(Link);

impl LinkRow {
    pub fn into_inner(self) -> Link {
        self.0
    }
}

impl<'a> FromRow<'a, PgRow> for LinkRow {
    fn from_row(row: &PgRow) -> Result<Self, sqlx::Error> {
        let link = Link {
            id: Id::from(row.try_get::<String, _>("id")?),
            original_url: row.try_get::<String, _>("original_url")?.parse().map_err(|error| sqlx::Error::ColumnDecode {
                index: "original_url".to_string(),
                source: Box::new(error),
            })?,
            shortened_path: row.try_get("shortened_path")?,
            is_active: row.try_get("is_active")?,
            tags: row.try_get::<Vec<String>, _>("tags").unwrap_or_default(),
            created_at: row.try_get("created_at")?,
            expires_at: row.try_get("expires_at")?,
        };
        Ok(LinkRow(link))
    }
}

fn sqlx_to_app_error(error: sqlx::Error) -> Error {
    tracing::error!("DataAccess error: {}", error);
    let Some(db_error) = error.into_database_error() else {
        return Error::DataAccessError;
    };
    match db_error.kind() {
        ErrorKind::UniqueViolation => Error::NonUniqueShortenedPath,
        _ => {
            tracing::error!("Db error: {}", db_error);
            Error::DataAccessError
        }
    }
}
