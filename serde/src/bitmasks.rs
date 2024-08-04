use bitflags::bitflags;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug)]
    pub(crate) struct MessageType: u16 {
        const Query = 0 << 15;
        const Response = 1 << 15;
    }

    #[repr(transparent)]
    #[derive(Debug)]
    pub(crate) struct Opcode: u16 {
        const Standard = 0 << 11;
    }

    #[repr(transparent)]
    #[derive(Debug)]
    pub(crate) struct AuthoritativeAnswer: u16 {
        const NotAuthoritative = 0 << 10;
    }


    #[repr(transparent)]
    #[derive(Debug)]
    pub(crate) struct Truncation: u16 {
        const False = 0 << 9;
    }

    #[repr(transparent)]
    #[derive(Debug)]
    pub(crate) struct RecursionDesired: u16 {
        const True = 1 << 8;
    }

    #[repr(transparent)]
    #[derive(Debug)]
    pub(crate) struct RecursionAvailable: u16 {
        const False = 0 << 7;
    }

    #[repr(transparent)]
    #[derive(Debug)]
    pub(crate) struct ResponseCode: u16 {
        const NoError = 0 << 0;
        const FormatError = 1 << 0;
        const ServerFailure = 2 << 0;
        const NameError = 3 << 0;
        const NotImplemented = 4 << 0;
        const Refused = 5 << 0;
    }

    #[repr(transparent)]
    #[derive(Debug)]
    pub struct RecordType: u16 {
        const A = 1;
        const Null = 10;
    }


    #[repr(transparent)]
    #[derive(Debug)]
    pub(crate) struct QueryClass: u16 {
        const Internet = 1;
    }
}

pub const STANDARD_REQUEST_FLAGS: u16 = MessageType::Query.bits() | Opcode::Standard.bits() |
    AuthoritativeAnswer::NotAuthoritative.bits() | Truncation::False.bits() | RecursionDesired::True.bits() | RecursionAvailable::False.bits() |
    ResponseCode::NoError.bits();
