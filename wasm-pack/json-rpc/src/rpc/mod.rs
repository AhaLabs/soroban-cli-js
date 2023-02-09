use serde_json::json;
use soroban_env_host::xdr::Error as XdrError;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::JsFuture;
use web_sys::{self, Request, RequestInit, RequestMode, Response};

pub mod server;

const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("xdr processing error: {0}")]
    Xdr(#[from] XdrError),
    #[error("json decoding error: {0}")]
    Serde(#[from] serde_json::Error),
    #[error("transaction submission failed")]
    TransactionSubmissionFailed,
    #[error("expected transaction status: {0}")]
    UnexpectedTransactionStatus(String),
    #[error("transaction submission timeout")]
    TransactionSubmissionTimeout,
    #[error("transaction simulation failed: {0}")]
    TransactionSimulationFailed(String),

    #[error("JS failed with {0:#?}")]
    JsError(JsValue),
}

// TODO: this should also be used by serve
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct GetAccountResponse {
    pub id: String,
    pub sequence: String,
    // TODO: add balances
}

// TODO: this should also be used by serve
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct SendTransactionResponse {
    pub id: String,
    pub status: String,
    // TODO: add error
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct TransactionStatusResult {
    pub xdr: String,
}

// TODO: this should also be used by serve
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct GetTransactionStatusResponse {
    pub id: String,
    pub status: String,
    #[serde(
        rename = "envelopeXdr",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub envelope_xdr: Option<String>,
    #[serde(rename = "resultXdr", skip_serializing_if = "Option::is_none", default)]
    pub result_xdr: Option<String>,
    #[serde(
        rename = "resultMetaXdr",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub result_meta_xdr: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub results: Vec<TransactionStatusResult>,
}

// TODO: this should also be used by serve
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct GetLedgerEntryResponse {
    pub xdr: String,
    // TODO: add lastModifiedLedgerSeq and latestLedger
}

// TODO: this should also be used by serve
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Cost {
    #[serde(rename = "cpuInsns")]
    pub cpu_insns: String,
    #[serde(rename = "memBytes")]
    pub mem_bytes: String,
}
#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct SimulateTransactionResponse {
    pub footprint: String,
    pub cost: Cost,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub error: Option<String>,
    // TODO: add results and latestLedger
}

impl From<JsValue> for Error {
    fn from(value: JsValue) -> Self {
        Error::JsError(value)
    }
}

pub struct Client {
    base_url: String,
}

impl Client {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    pub async fn request(&self, name: &str, params: &str) -> Result<String, Error> {
        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.mode(RequestMode::Cors);
        let body = json!({
            "id": 1,
            "jsonrpc": "2.0",
            "method": name,
            "params": vec![params],
        });

        opts.body(Some(&JsValue::from_str(&body.to_string())));

        let request = Request::new_with_str_and_init(&self.base_url, &opts)?;
        request
            .headers()
            .set("X-Client-Name", "soroban-cli")
            .unwrap();
        request
            .headers()
            .set("Accept", "application/json, text/plain, */*")
            .unwrap();
        request
            .headers()
            .set("Allow-Post", "application/json")
            .unwrap();
        request
            .headers()
            .set("Referer", "http://localhost:8000/")
            .unwrap();
        request
            .headers()
            .set("Origin", "http://localhost:8000/")
            .unwrap();

        request
            .headers()
            .set("X-Client-Version", VERSION.unwrap_or("devel"))?;
        let window = web_sys::window().unwrap();
        web_sys::console::log_1(&"hell".to_string().into());
        let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;
        web_sys::console::log_1(&"hell".to_string().into());

        // `resp_value` is a `Response` object.
        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().unwrap();

        // Convert this other `Promise` into a rust `Future`.
        let json = JsFuture::from(resp.json()?).await?;
        web_sys::console::log_1(&json);
        Ok("hello".to_string())
    }

    // fn client(&self) -> Result<HttpClient, Error> {
    //     let url = self.base_url.clone();
    //     let mut headers = HeaderMap::new();
    //     headers.insert("X-Client-Name", "soroban-cli".parse().unwrap());
    //     let version = VERSION.unwrap_or("devel");
    //     headers.insert("X-Client-Version", version.parse().unwrap());
    //     // TODO: We should consider migrating the server subcommand to jsonrpsee
    //     Ok(HttpClientBuilder::default()
    //         .set_headers(headers)
    //         .build(url)?)
    // }

    pub async fn get_account(&self, account_id: &str) -> Result<String, Error> {
        self.request("getAccount", account_id).await
    }

    // pub async fn send_transaction(
    //     &self,
    //     tx: &TransactionEnvelope,
    // ) -> Result<Vec<TransactionStatusResult>, Error> {
    //     let client = self.client()?;
    //     let SendTransactionResponse { id, status } = client
    //         .request("sendTransaction", rpc_params![tx.to_xdr_base64()?])
    //         .await
    //         .map_err(|_| Error::TransactionSubmissionFailed)?;

    //     if status == "error" {
    //         return Err(Error::TransactionSubmissionFailed);
    //     }
    //     // even if status == "success" we need to query the transaction status in order to get the result

    //     // Poll the transaction status
    //     let start = Instant::now();
    //     loop {
    //         let response = self.get_transaction_status(&id).await?;
    //         match response.status.as_str() {
    //             "success" => {
    //                 // TODO: the caller should probably be printing this
    //                 eprintln!("{}", response.status);
    //                 return Ok(response.results);
    //             }
    //             "error" => {
    //                 // TODO: provide a more elaborate error
    //                 return Err(Error::TransactionSubmissionFailed);
    //             }
    //             "pending" => (),
    //             _ => {
    //                 return Err(Error::UnexpectedTransactionStatus(response.status));
    //             }
    //         };
    //         let duration = start.elapsed();
    //         // TODO: parameterize the timeout instead of using a magic constant
    //         if duration.as_secs() > 10 {
    //             return Err(Error::TransactionSubmissionTimeout);
    //         }
    //         #[cfg(not(feature = "wasm"))]
    //         sleep(Duration::from_secs(1)).await;
    //     }
    // }

    // pub async fn simulate_transaction(
    //     &self,
    //     tx: &TransactionEnvelope,
    // ) -> Result<SimulateTransactionResponse, Error> {
    //     let base64_tx = tx.to_xdr_base64()?;
    //     let response: SimulateTransactionResponse = self
    //         .client()?
    //         .request("simulateTransaction", rpc_params![base64_tx])
    //         .await?;
    //     match response.error {
    //         None => Ok(response),
    //         Some(e) => Err(Error::TransactionSimulationFailed(e)),
    //     }
    // }

    // pub async fn get_transaction_status(
    //     &self,
    //     tx_id: &str,
    // ) -> Result<GetTransactionStatusResponse, Error> {
    //     Ok(self
    //         .client()?
    //         .request("getTransactionStatus", rpc_params![tx_id])
    //         .await?)
    // }

    // pub async fn get_ledger_entry(&self, key: LedgerKey) -> Result<GetLedgerEntryResponse, Error> {
    //     let base64_key = key.to_xdr_base64()?;
    //     Ok(self
    //         .client()?
    //         .request("getLedgerEntry", rpc_params![base64_key])
    //         .await?)
    // }
}
