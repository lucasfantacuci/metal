#[allow(dead_code)]
#[derive(Debug)]
pub enum Method {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch
}

pub fn match_method(method: &str) -> Result<Method, String> {
    println!("{}", method);
    match method {
        "GET" => Ok(Method::Get),
        "HEAD" => Ok(Method::Head),
        "POST" => Ok(Method::Post),
        "PUT" => Ok(Method::Put),
        "DELETE" => Ok(Method::Delete),
        "CONNECT" => Ok(Method::Connect),
        "OPTIONS" => Ok(Method::Options),
        "TRACE" => Ok(Method::Trace),
        "PATCH" => Ok(Method::Patch),
        &_ => Err(String::from("Invalid HTTP Method"))
    }
}

pub enum Status {
    Continue, //100
    SwitchingProtocol, //101
    Processing, //102
    Ok, //200
    Created, //201
    Accepted, //202
    NonAuthoritativeInformation, //203
    NoContent, //204
    ResetContent, //205
    PartialContent, //206
    MultiStatus, //207
    ImUsed, //226
    MultipleChoice, //300
    MovedPermanently, //301
    Found, //302
    SeeOther, //303
    NotModified, //304
    UseProxy, //305
    Unused, //306
    TemporaryRedirect, //307
    PermanentRedirect, //308
    BadRequest, //400
    Unauthorized, //401
    PaymentRequired, //402
    Forbidden, //403
    NotFound, //404
    MethodNotAllowed, //405
    NotAcceptable, //406
    ProxyAuthenticationRequired, //407
    RequestTimeot, //408
    Conflict, //408
    Gone, //410
    LengthRequired, //411
    PreconditionFailed, //412
    PayloadTooLarge, //413
    UriTooLong, //414
    UnsupportedMediaType, //415
    RequestedRangeNotSatisfied, //416
    ExpectationFailed, //417
    ImATeapot, //418
    MisdirectedRequest, //421
    UnprocessableEntity, //422
    Locked, //423
    FailedDependency, //424
    UpgradeRequired, //426
    PreconditionRequired, //428
    TooManyRequests, //429
    RequestHeaderFieldsTooLarge, //431
    UnavailableForLegalReasons, //451
    InternalServerError, //500
    NotImplemented, //501
    BadGateway, //502
    ServiceUnavailable, //503
    GatewayTimeout, //504
    HttpVersionNotSupported, //505
    VariantAlsoNegotiates, //506
    InsufficientStorage, //507
    LoopDetected, //508
    NotExtended, //510
    NetworkAuthenticationRequired, //511
}