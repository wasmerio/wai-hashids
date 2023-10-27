use harsh::Harsh;

wai_bindgen_rust::export!("bindings.wai");
struct Hashids {
    inner: Harsh
}

struct Bindings;

impl bindings::Bindings for Bindings {}

impl bindings::Hashids for Hashids {
    fn constructor() -> wai_bindgen_rust::Handle<Self> {
        Self {
            inner: Harsh::builder().salt("salt goes here!").build().unwrap()
        }.into()
    }
    fn encode(&self, values: Vec<u64>) -> String {
        self.inner.encode(&values)
    }

    fn decode(&self, input: String) -> Vec<u64> {
        self.inner.decode(&input).unwrap().into()
    }

    fn encode_hex(&self, hex: String) -> String {
        self.inner.encode_hex(&hex).unwrap()
    }

    fn decode_hex(&self, input: String) -> String {
        self.inner.decode_hex(&input).unwrap().into()
    }
}