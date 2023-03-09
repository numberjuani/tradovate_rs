use reqwest::Method;

pub struct Endpoint {
    pub path: &'static str,
    pub method: Method,
}

pub const ACCESS_TOKEN: Endpoint = Endpoint {
    path: "/v1/auth/accesstokenrequest",
    method: Method::POST,
};

pub const ACCESS_TOKEN_RENEW: Endpoint = Endpoint {
    path: "/v1/auth/renewaccesstoken",
    method: Method::POST,
};

pub const CONTRACT_DEPS: Endpoint = Endpoint {
    path: "/v1/contract/deps",
    method: Method::GET,
};

pub const PRODUCTS_LIST: Endpoint = Endpoint {
    path: "/v1/product/list",
    method: Method::GET,
};

pub const CONTRACT_GROUPS_LIST: Endpoint = Endpoint {
    path: "/v1/contractGroup/list",
    method: Method::GET,
};

pub const CONTRACT_FIND: Endpoint = Endpoint {
    path: "/v1/contract/find",
    method: Method::GET,
};

pub const CONTRACT_MATURITY: Endpoint = Endpoint {
    path: "/v1/contractMaturity/item",
    method: Method::GET,
};

pub const LIST_POSITIONS: Endpoint = Endpoint {
    path: "/v1/position/list",
    method: Method::GET,
};

pub const PLACE_ORDER: Endpoint = Endpoint {
    path: "/v1/order/placeorder",
    method: Method::POST,
};

pub const ACCOUNTS_LIST: Endpoint = Endpoint {
    path: "/v1/account/list",
    method: Method::GET,
};

pub const CASH_BALANCE_LIST: Endpoint = Endpoint {
    path: "/v1/cashBalance/list",
    method: Method::GET,
};