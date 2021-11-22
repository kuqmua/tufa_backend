#[derive(Debug)]
pub enum Resource {
    Local,
    Mongodb,
    PostgreSql,
}

#[derive(Debug)]
pub enum ResourceError {
    NoLinkParts(Resource),
    Other,
}