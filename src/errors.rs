use salvo::http::StatusCode;
use salvo::prelude::Json;
use salvo::{Depot, Request, Response, Writer, async_trait};
use serde::{Serialize, Serializer};
use std::fmt::Display;

fn serialize_status_code<S>(code: &StatusCode, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u16(code.as_u16())
}

#[derive(Debug, Serialize)]
pub struct FogSpaceError {
    #[serde(serialize_with = "serialize_status_code")]
    pub code: StatusCode,
    pub message: String,
}

#[async_trait]
impl Writer for FogSpaceError {
    async fn write(self, _: &mut Request, _: &mut Depot, res: &mut Response) {
        res.status_code(self.code);
        res.render(Json(self));
    }
}

impl Display for FogSpaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FogSpaceError! code:{}, err:{}", self.code, self.message)
    }
}
