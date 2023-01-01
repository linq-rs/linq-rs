extern crate proc_macro;

use linq_sql_parser as parser;

use crate::dml;

pub fn parse_sql<S>(s: S) -> anyhow::Result<Vec<dml::DML<'static>>>
where
    S: AsRef<str>,
{
    let stream = s
        .as_ref()
        .parse::<proc_macro::TokenStream>()
        .map_err(|e| anyhow::format_err!("{}", e))?;

    let rqls = syn::parse::<parser::RQLs>(stream)?;

    let mut dmls = vec![];

    for rql in &rqls.rqls {
        dmls.push(gen_dml(rql)?);
    }

    Ok(dmls)
}

fn gen_dml(rql: &parser::RQL) -> anyhow::Result<dml::DML<'static>> {
    match rql {
        parser::RQL::Insert(insert) => {
            unimplemented!()
        }
        parser::RQL::Select(select) => Ok(dml::DML::Selecter(gen_selecter(select)?)),
        parser::RQL::Update(update) => {
            unimplemented!()
        }
        parser::RQL::Delete(delete) => {
            unimplemented!()
        }
    }
}

fn gen_selecter(select: &parser::Select) -> anyhow::Result<dml::Selecter<'static>> {
    unimplemented!()
}
