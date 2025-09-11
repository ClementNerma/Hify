mod convert;
mod routes;
mod types;
mod xml;

use axum::{
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde::Deserialize;

pub use self::routes::router;
use self::xml::{XMLChildren, XMLCompatible, XMLElement};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct OSCommonParams {
    #[serde(default = "ResponseFormat::default")]
    f: ResponseFormat,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "lowercase")]
enum ResponseFormat {
    Json,
    #[default]
    Xml,
}

type OSResult<T> = Result<OSNestedResponse<T>, OSError>;

type OSError = (StatusCode, &'static str);

struct OSNestedResponse<T: XMLCompatible>(ResponseFormat, &'static str, T);

impl<T: XMLCompatible> IntoResponse for OSNestedResponse<T> {
    fn into_response(self) -> Response {
        let Self(f, key, value) = self;
        OSResponse(f, Some((key, value.to_xml_element()))).into_response()
    }
}

struct OSEmptyResponse(ResponseFormat);

impl IntoResponse for OSEmptyResponse {
    fn into_response(self) -> Response {
        let Self(f) = self;
        OSResponse(f, None).into_response()
    }
}

struct OSResponse(ResponseFormat, Option<(&'static str, XMLElement)>);

impl IntoResponse for OSResponse {
    fn into_response(self) -> Response {
        let Self(format, nested) = self;

        let response = XMLElement {
            fields: vec![
                ("status".into(), "ok".into()),
                ("version".into(), "1.16.1".into()),
                ("type".into(), "HifyServer".into()),
                ("serverVersion".into(), env!("CARGO_PKG_VERSION").into()),
                ("openSubsonic".into(), true.into()),
            ],
            content_fields: vec![],
            children: nested
                .map(|(key, value)| vec![(key.into(), XMLChildren::Single(value))])
                .unwrap_or_default(),
            content: None,
        };

        match format {
            ResponseFormat::Json => {
                let mut headers = HeaderMap::new();
                headers.insert(
                    "Content-Type",
                    HeaderValue::from_str("application/json").unwrap(),
                );

                let response = XMLElement {
                    fields: vec![],
                    content_fields: vec![],
                    children: vec![("subsonic-response".into(), XMLChildren::Single(response))],
                    content: None,
                };

                let mut out = String::new();
                response.serialize_to_json(&mut out).unwrap();

                (headers, out).into_response()
            }

            ResponseFormat::Xml => {
                let mut headers = HeaderMap::new();
                headers.insert(
                    "Content-Type",
                    HeaderValue::from_str("application/xml").unwrap(),
                );

                let mut out =
                    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes" ?>"#.to_owned();

                response
                    .serialize_to_xml("subsonic-response", &mut out)
                    .unwrap();

                (headers, out).into_response()
            }
        }
    }
}
