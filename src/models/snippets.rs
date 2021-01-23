use std::fs::create_dir_all;

use crate::db::DbConn;
use chrono::{NaiveDateTime, Utc};

use super::{last_insert_rowid, ModelError};

#[derive(Debug, Queryable)]
pub struct Snippet {
    pub id: i32,
    pub creator_id: i32,

    pub taxonomy: String,
    pub hidden: bool,

    pub title: String,
    pub icon: Option<String>,
    pub shared_by: String,
    pub shared_on: NaiveDateTime,
    pub summary: String,
    pub description: String,
    pub href: String,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Snippet {
    pub fn create(
        conn: &DbConn,
        the_creator_id: i32,
        the_taxonomy: &str,
        is_hidden: bool,
        the_icon: Option<&str>,
        the_title: &str,
        the_shared_by: &str,
        the_shared_on: &NaiveDateTime,
        the_summary: &str,
        the_description: &str,
        the_href: &str,
    ) -> Result<Self, ModelError> {
        use crate::schema::snippets::dsl::{
            created_at, creator_id, description, hidden, href, icon, shared_by, shared_on,
            snippets, summary, taxonomy, title, updated_at,
        };
        use diesel::prelude::*;

        let snippet = conn.transaction::<Self, ModelError, _>(|| {
            let inserted_at = Utc::now().naive_utc();

            diesel::insert_into(snippets)
                .values((
                    creator_id.eq(the_creator_id),
                    taxonomy.eq(&the_taxonomy),
                    hidden.eq(is_hidden),
                    title.eq(&the_title),
                    icon.eq(&the_icon),
                    shared_by.eq(&the_shared_by),
                    shared_on.eq(the_shared_on),
                    summary.eq(&the_summary),
                    description.eq(&the_description),
                    href.eq(&the_href),
                    created_at.eq(&inserted_at),
                    updated_at.eq(&inserted_at),
                ))
                .execute(conn)?;

            let rowid = diesel::select(last_insert_rowid).get_result::<i32>(conn)?;

            Ok(Self::find_by_id(conn, rowid)?)
        })?;

        Ok(snippet)
    }

    pub fn find_by_id(conn: &DbConn, the_id: i32) -> Result<Self, ModelError> {
        use crate::schema::snippets::dsl::{id, snippets};
        use diesel::prelude::*;

        let snippet = snippets
            .filter(id.eq(the_id))
            .limit(1)
            .first::<Snippet>(conn)?;

        Ok(snippet)
    }

    pub fn find_all(
        conn: &DbConn,
        visible_only: bool,
        page: i64,
        page_size: i64,
    ) -> Result<Vec<Self>, ModelError> {
        use crate::schema::snippets::dsl::{hidden, shared_on, snippets};
        use diesel::prelude::*;

        let q = snippets
            .order(shared_on.desc())
            .limit(page_size)
            .offset(page * page_size);

        let r = if visible_only {
            q.filter(hidden.eq(false)).load::<Snippet>(conn)?
        } else {
            q.load::<Snippet>(conn)?
        };

        Ok(r)
    }

    pub fn find_all_by_taxonomy(
        conn: &DbConn,
        visible_only: bool,
        the_taxonomy: &str,
        page: i64,
        page_size: i64,
    ) -> Result<Vec<Self>, ModelError> {
        use crate::schema::snippets::dsl::{hidden, shared_on, snippets, taxonomy};
        use diesel::prelude::*;

        let q = snippets
            .filter(taxonomy.eq(the_taxonomy))
            .order(shared_on.desc())
            .limit(page_size)
            .offset(page * page_size);

        let r = if visible_only {
            q.filter(hidden.eq(false)).load::<Snippet>(conn)?
        } else {
            q.load::<Snippet>(conn)?
        };

        Ok(r)
    }

    pub fn update(&self, conn: &DbConn) -> Result<(), ModelError> {
        use crate::schema::snippets::dsl::{
            creator_id, description, hidden, href, icon, shared_by, shared_on, snippets, summary,
            taxonomy, title, updated_at,
        };
        use diesel::prelude::*;

        diesel::update(snippets.find(self.id))
            .set((
                creator_id.eq(self.creator_id),
                taxonomy.eq(&self.taxonomy),
                hidden.eq(self.hidden),
                title.eq(&self.title),
                icon.eq(&self.icon),
                shared_by.eq(&self.shared_by),
                shared_on.eq(&self.shared_on),
                summary.eq(&self.summary),
                description.eq(&self.description),
                href.eq(&self.href),
                updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)?;

        Ok(())
    }

    pub fn delete(&self, conn: &DbConn) -> Result<usize, ModelError> {
        use crate::schema::snippets::dsl::{id, snippets};
        use diesel::prelude::*;

        let r = diesel::delete(snippets.filter(id.eq(self.id))).execute(conn)?;

        Ok(r)
    }
}

impl Default for Snippet {
    fn default() -> Self {
        Snippet {
            id: 0,
            creator_id: 0,
            taxonomy: "None".to_owned(),
            hidden: true,
            title: "The title of this snippet".to_owned(),
            icon: None,
            shared_by: "Who shared this, in like Discord or something".to_owned(),
            shared_on: Utc::now().naive_utc(),
            summary: "A sentence or two about this snippet".to_owned(),
            description: "A paragraph or so about this snippet. Currently unused.".to_owned(),
            href: "The URL where this snippet goes!".to_owned(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        }
    }
}
