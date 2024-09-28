use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

#[derive(Debug)]
struct Backend {
    client: Client,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                definition_provider: Some(OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let uri = params.text_document_position_params.text_document.uri;

        // Origin range (where the definition is requested from)
        let origin_start = params.text_document_position_params.position;
        let origin_end = params.text_document_position_params.position;
        let origin_range = Range::new(origin_start, origin_end);

        // Target range (where the definition is located)
        let target_start = Position::new(2, 2);
        let target_end = Position::new(2, 4);
        let target_range = Range::new(target_start, target_end);

        // Target selection range (more specific location within the target range)
        let target_selection_start = Position::new(2, 2);
        let target_selection_end = Position::new(2, 2);
        let target_selection_range = Range::new(target_selection_start, target_selection_end);

        // Create a LocationLink with detailed information
        let location_link = LocationLink {
            origin_selection_range: Some(origin_range),
            target_uri: uri.clone(),
            target_range,
            target_selection_range,
        };

        // Return both the detailed LocationLink and the simple Location
        return Ok(Some(GotoDefinitionResponse::Link(vec![location_link])));

        // let uri = params.text_document_position_params.text_document.uri;
        // let pos = Position::new(2, 2);
        // let range = Range::new(pos, pos);
        // return Ok(Some(GotoDefinitionResponse::Array(vec![Location::new(
        //     uri, range,
        // )])));
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend { client });
    Server::new(stdin, stdout, socket).serve(service).await;
}
