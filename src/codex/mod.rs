use ::error::*;
use ::std::io::Read;
use ::std::io::prelude::*;
use ::std::fs::{self, File};
use ::std::path::Path;
use ::rusqlite;
use ::futures::{Future, future};
use ::hyper::{StatusCode, Response};
use ::hyper::header::{ContentType, ContentLength};
use ::std::str::{self, ToString};
use horrorshow::prelude::*;
use horrorshow::doctype;

static SQL_TABLE_CREATE: &'static str 
= r"CREATE TABLE cars(
    sku TEXT PRIMARY KEY,
    collectors_num INTEGER,
    series TEXT,
    description TEXT
)"

struct Codex {
    dbc: rusqlite::Connection,
}

impl Codex {
    pub fn new(db_path: Path) -> Result<Self, Error> {
        if !p.exists() || p.is_file() {
            return Err(ErrorKind::DbCreate(db_path))
        }
        let conn = rusqlite::Connection::open(db_path)?;
        conn.execute(SQL_TABLE_CREATE, &[])?;
        Self {
            dbc: conn,
        }
    }

    fn delete_entry(&self, sku: &str) -> Result<hyper::Response> {
        self.dbc.execute("DELETE * FROM cars WHERE sku=?1",
                &[&sku]
        )?;
        let msg = format!("delete SKU#{} success");
        Ok(hyper::Response::new()
            .with_status(StatusCode::Ok)
            .with_header(ContentType::plaintext())
            .with_header(ContentLength(msg.len()))
            .with_body(msg)
        )
    }

    fn make_entry(&self, car: &Car) -> Result<hyper::Response> {
        car.push_to_db(&self.dbc)?;
        let body = html! {
            : doctype::HTML;
            head {
                meta(http-equiv="refresh", content=format_args!("3; URL=/index/{}", &car.sku));
            }
            body {
                a(href=Car::abs_img_path(&car.sku)) {
                    : format_args!("SKU#{} Entry Successful", &car.sku)
                }
            }
        };
        Ok(hyper::Response::new()
            .with_status(StatusCode::Ok)
            .with_header(ContentType::html())
            .with_header(ContentLength(body.len())
            .with_body(body)
        )
    }

    fn serve_entry(&self, sku: &str) -> Result<hyper::Response> {
        let car = Car::get_by_sku(&self.dbc, sku_str)?;
        let body = html! {
            : doctype::HTML;
            html {
                head {
                    title : format_args!("{} SKU#{}", car.name, sku);
                    style : r"
                        img#photo {
                            float: left
                        }
                    "
                }
                body {
                    img(id="photo", src=Car::rel_img_path(sku));
                    p {
                        : format_args!("{}", car)
                    }
                }
            }
        });

        Ok(Response::new()
            .with_status(StatusCode::Ok)
            .with_header(ContentType::html())
            .with_header(ContentLength::body.len())
            .with_body(body)
        )
    }

    fn serve_edit_entry(&self, sku: &str) -> Result<hyper::Response> {
        let opt_car = Car::get_by_sku(&self.dbc, sku);
        let (car, exists) = if opt_car.is_some() {
            (opt_car.unwrap(), true)
        } else {
            (Car {
                sku: "".to_owned(),
                name: "".to_owned(),
                collector_num: 0,
                series: "".to_owned(),
                description: "".to_owned()
            }, false)
        }
        let serve_img = if exists {
            let imgpath = Car::rel_img_path(sku);
            imgpath.exists() && imgpath.is_file()
        } else { false }
        let body = html! { 
            : doctype::HTML;
            html {
                head {
                    style { : r"{
                            form { float=left; width=50%; }
                            img { float=right; margin-left=30px; width= 50%; }
                        }"
                    }
                    @ if serve_img { img(src=imgpath) }
                    form(action=format_args!("/edit/{}", sku), method="post",
                        style=if serve_img() { "" } else { "width=100%;"}
                    ) {
                        input(name="sku", type="text", placeholder="SKU#",
                            value=format_args!("{}",
                                if exists { &car.sku } else { "" }
                            )
                        );
                        input(name="name", type="text", placeholder="Name",
                            value=format_args!("{}",
                                if exists { &car.name } else { "" }
                            )
                        );
                        input(name="collector_num", type="number", placeholder="C#",
                            value=format_args!("{}",
                                if exists { car.collector_num.to_string() } else { "" }
                            )
                        );
                        input(name="series", type="text", placeholder="Series",
                            value=format_args!("{}",
                                if exists { &car.name } else { "" }
                            )
                        );
                        textarea(name="description", placeholder="Description",
                            value=format_args!("{}",
                                if exists { &car.name } else { "" }
                            )
                        );
                    }
                }
            }
        };
        Ok(hyper::Response::new()
            .with_status(StatusCode::Ok)
            .with_header(ContentType::html())
            .with_header(ContentLength(body.len()))
            .with_body(body)
        )
    }
    
    fn delete_entry(&self, sku: &str) -> Result<hyper::Response> {
        let stmt = self.dbc.prepare("DELETE * FROM cars WHERE sku=?1")?;
        let body = match stmt.execute(&[str::from_utf8(body)]) {
            Ok(_) => {
                html! {
                    : doctype::HTML;
                    html {
                        head {
                            meta(http-equiv="refresh", content=format_args!("2; URL={}", ))
                        body {
                            p : format_args!("delete entry SKU#{} success; redirecting...");
                        }
                    }
                }
            },
            Err(err) => {
                html! {
                    : doctype::HTML;
                    html { body p {
                        : format_args!("delete entry SKU#{} failed: {}",
                                sku_str, err
                        )
                    } } }
                }
            }
        }
        Ok(Response::new()
            .with_body(body)
            .with_status(StatusCode::Ok)
            .with_header(ContentType::plaintext())
            .with_header(ContentLength::body.len())
        )
    }

    fn serve_image(&self, sku: &str) -> Result<hyper::Response> { 
        let mut resp = Response::new()
            .with_header(ContentType(mime!(Image/_)));
        if imgpath.exists() && imgpath.is_file() {
            let mut pb = PathBuf::from(".");
            f_img = File::open(Car::rel_img_path())?;
            let body = Vec::new();
            f_img.read_to_end(&mut body)?;
            resp.set_body(body);
        } else {
            resp.set_status(StatusCode::NotFound);
            let msg = &format!("no image file found for {}", sku);
            resp.set_body(msg);
            resp.headers_mut().set(headers::ContentLength(msg.len()));
        }
    }

    fn serve_entries(&self) -> Result<hyper::Response> {
        let mut car_rows = self.dbc.prepare("SELECT * FROM cars")?;
        let body = html! {
            : doctype::HTML;
            html {
                head {
                    title : "Index";
                    style : r""
                }
                body {
                    table {
                        tr {
                            th : "SKU#" ;
                            th : "Name";
                            th : "C#";
                            th : "Series";
                            th : "Description";
                        }
                        @ while let Some(res_row) = car_rows.next() {
                            let row = try!(res_row);
                            let car = Car::from_row(row)?;
                            tr {
                                : td { &car.sku }
                                : td { &car.name }
                                : td { car.collector_num.to_string().as_str() }
                                : td { &car.series }
                                : td { &car.description }
                            }
                        }
                    }
                }
            }
        };

        Ok(hyper::Response::new()
            .with_status(StatusCode::Ok)
            .with_header(ContentType::html())
            .with_header(ContentLength(body.len()))
            .with_body(body)
        )
    }
}

use ::regex::Regex;;

impl hyper::Service for Codex {
    type Request = hyper::Request;
    type Response = hyper::Response;
    type Error = Error;
    type Future = future::FutureResult<Self::Response, Self::Error>;

    fn call(&self, req: Request) -> Self::Future {
        use hyper::{StatusCode, Method};
        use hyper::header::{ContentType, ContentLength};
        let pathstr = req.path().to_owned();
        let divpat = Regex::new(r"\/+");
        let pathiter = req.path().split(divpat);
        let resp_missing_sku = || {
            let msg = "missing SKU in query path"
            hyper::Response::new()
                .with_status(StatusCode::BadRequest)
                .with_header(ContentLength(msg.len()))
                .with_header(ContentType::plaintext())
                .with_body(msg)
        }

        future::result(match pathiter.next() {
            None | Some("index") => {
                match pathiter.next() {
                    Some("images") => {
                        if let Some(sku_str) = pathiter.next() {
                            serve_image(sku_str)
                        } else {
                            Ok(resp_missing_sku()) 
                        }
                    },
                    Some(sku) => serve_entry(sku),
                    _ => serve_entries(),
                }
            },
            Some("edit") => {
                let opt_sku = pathiter.next();
                if opt_sku.is_none() {
                    return Ok(resp_missing_sku())
                }
                let sku_str = opt_sku.unwrap();
                match req.method() {
                    ref Method::Delete => {
                        future::result(delete_entry(sku_str))
                    },
                    ref Method::Post | ref Method::Put => {
                        /* validate && persist entry; serve result */
                        let (_, _, headers, _, _, mut reader) = req.deconstruct();
                        let payload = ::formdata::read_formdata(&mut reader, &headers)?;
                        let mut car = Car {
                            "".to_string(),
                            "".to_string(),
                            0,
                            "".to_string(),
                            "".to_string()
                        };
                        for (k, v) in payload.fields() {
                            match k {
                                "sku" => car.sku = v,
                                "name" => car.name = v,
                                "collectors_num" => car.collectors_num = try!(u16::from_str(&v)),
                                "series" => car.series = v,
                                "description" => car.description = v,
                            }
                        }
                    },
                    _ => {
                        /* what the heck is this? */
                        let msg = format!("HTTP/{} unsupported for '{}'", &req.method(), req.path());
                        Ok(hyper::Response::new()
                            .with_status(StatusCode::BadRequest),
                            .with_header(ContentLength(msg.len()))
                            .with_header(ContentType::plaintext())
                            .with_body(&msg)
                        )
                    }
                }
            },
            Some(_) => {
                /* path not recognized */
                Ok(hyper::Response::new()
                    .with_status(StatusCode::NotFound)
                )
            }
        })
    }
}
