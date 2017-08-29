#![allow(dead_code)]
use ::error::*;
use ::rusqlite;
use ::std::path::{Path, PathBuf};
use ::std::str::ToString;
use ::std::fmt::{self, Display, Formatter}

pub struct Car {
    pub sku: String,
    pub name: String,
    pub collector_num: u16,
    pub series: String,
    pub description: String,
}

impl Car {
    pub fn in_db(sku: &str, conn: &rusqlite::Connection) -> Result<bool> {
        let stmt = conn.prepare("SELECT 1 FROM cars WHERE sku=?1)")?;
        let mut rows = stmt.query(&[&sku]);
        Ok(match rows.next() {
            Some(_) => true,
            None => false
        })
    }

    pub fn from_row(row: &rusqlite::Row) -> Result<Self> {
        Self {
            row.get_checked(0),
            row.get_checked(1),
            row.get_checked(2),
            row.get_checked(3),
            row.get_checked(4)
        }
    }

    pub fn push_to_db(&self, conn: &rusqlite::Connection) -> Result<()> {
        dbc.execute(
            r"INSERT OR REPLACE INTO cars(sku, name, collector_num, series, description)
                VALUES(?1, ?2, ?3, ?4, ?5)",
            &[&self.sku, &self.name, &self.collector_num, &self.series, &self.description]
        )?;
    }

    pub fn get_by_sku(conn: &rusqlite::Connection, sku: &str) -> Option<Result<Self>> {
        if Self::in_db(sku)? {
            conn.query_row("SELECT * FROM cars WHERE sku=?1", &[&sku], |row| {
                Self::from_row(row)
            })?;
        } else {
            None
        }
    }

    pub fn rel_path(&str sku) -> Path {
        let mut pb = PathBuf::from("index");
        pb.push(sku);
        pb.as_path()
    }

    pub fn abs_path(&str sku) -> Path {
        let mut PathBuf::from("/");
        pb.push(Self::rel_path());
        pb.as_path()
    }
    
    pub fn rel_img_path(&str sku) -> Path {
        let mut pb = PathBuf::from("index/images");
        pb.push();
        pb.as_path()
    }
    
    pub fn abs_img_path(&str sku) -> Path {
     let mut pb = PathBuf::from(Self::abs_path());
        pb.push(Self::rel_img_path(sku));
        pb.as_path()
    } 
}

impl Display for Car {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_fmt(format_args!("SKU#: {}\nC#: {}\nSeries: {}\nDescription: {}"))?;
    }
}
