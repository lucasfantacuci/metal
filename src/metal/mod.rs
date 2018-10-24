
use std::net::TcpListener;
use std::io::prelude::*;
use std::thread;

pub struct Metal;

impl Metal {
    pub fn listen(&self, address: String) {
        let listener = TcpListener::bind(address).expect("Unable to bind the server into this port");
        self.handle_incomming_messages(listener);
    }

    fn handle_incomming_messages(&self, listener: TcpListener) {
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            thread::spawn(move || {
                let mut buffer: [u8; 2048] = [0; 2048];
                stream.read(&mut buffer).unwrap();
                let request = Metal::parse_incomming_message(format!("{}", String::from_utf8_lossy(&buffer[..])));
                let response = Response;
                //self.dispatch_to_route(request, response);
                stream.write(Metal::parse_output_message(response));
            });
        }
    }

    fn parse_incomming_message(message: String) -> Request {
        println!("{}", message);
        let x = Request {
            method: HttpMethod::GET
        };
        x
    }

    fn parse_output_message(response: Response) -> &'static [u8; 98] {
        b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n<html><body>Hello world</body></html>\r\n"
    }
}

pub struct Request {
    method: HttpMethod
}

pub struct Response;

enum HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}

enum HttpStatus {
    CONTINUE, //100
    SWITCHING_PROTOCOL, //101
    PROCESSING, //102
    OK, //200
    CREATED, //201
    ACCEPTED, //202
    NON_AUTHORITATIVE_INFORMATION, //203
    NO_CONTENT, //204
    RESET_CONTENT, //205
    PARTIAL_CONTENT, //206
    MULTI_STATUS, //207
    IM_USED, //226
    MULTIPLE_CHOICE, //300
    MOVED_PERMANENTLY, //301
    FOUND, //302
    SEE_OTHER, //303
    NOT_MODIFIED, //304
    USE_PROXY, //305
    UNUSED, //306
    TEMPORARY_REDIRECT, //307
    PERMANENT_REDIRECT, //308
    BAD_REQUEST, //400
    UNAUTHORIZED, //401
    PAYMENT_REQUIRED, //402
    FORBIDDEN, //403
    NOT_FOUND, //404
    METHOD_NOT_ALLOWED, //405
    NOT_ACCEPTABLE, //406
    PROXY_AUTHENTICATION_REQUIRED, //407
    REQUEST_TIMEOUT, //408
    CONFLICT, //408
    GONE, //410
    LENGTH_REQUIRED, //411
    PRECONDITION_FAILED, //412
    PAYLOAD_TOO_LARGE, //413
    URI_TOO_LONG, //414
    UNSUPPORTED_MEDIA_TYPE, //415
    REQUESTED_RANGE_NOT_SATISFIED, //416
    EXPECTATION_FAILED, //417
    IM_A_TEAPOT, //418
    MISDIRECTED_REQUEST, //421
    UNPROCESSABLE_ENTITY, //422
    LOCKED, //423
    FAILED_DEPENDENCY, //424
    UPGRADE_REQUIRED, //426
    PRECONDITION_REQUIRED, //428
    TOO_MANY_REQUESTS, //429
    REQUEST_HEADER_FIELDS_TOO_LARGE, //431
    UNAVAILABLE_FOR_LEGAL_REASONS, //451
    INTERNAL_SERVER_ERROR, //500
    NOT_IMPLEMENTED, //501
    BAD_GATEWAY, //502
    SERVICE_UNAVAILABLE, //503
    GATEWAY_TIMEOUT, //504
    HTTP_VERSION_NOT_SUPPORTED, //505
    VARIANT_ALSO_NEGOTIATES, //506
    INSUFFICIENT_STORAGE, //507
    LOOP_DETECTED, //508
    NOT_EXTENDED, //510
    NETWORK_AUTHENTICATION_REQUIRED, //511
}