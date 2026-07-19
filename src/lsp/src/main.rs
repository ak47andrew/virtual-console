use std::{fs::read_to_string};

use tower_lsp::{Client, LanguageServer, LspService, Server, lsp_types::{Hover, HoverParams, InitializeParams, InitializeResult, InitializedParams, MarkedString, MessageType, Position, Range, ServerCapabilities}};
use tower_lsp::jsonrpc::Result;

include!(concat!(env!("OUT_DIR"), "/opcodes.rs"));

#[derive(Debug)]
struct Backend {
    client: Client,
}

fn is_numeric(s: &str) -> bool {
    s.trim().parse::<u64>().is_ok()
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(tower_lsp::lsp_types::HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "VEA-LSP initialized")
            .await;
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri.to_file_path().unwrap();
        let line_num = params.text_document_position_params.position.line;
        let col = params.text_document_position_params.position.character;
        let text = read_to_string(uri).unwrap();
        let mut line = text.split('\n').take(line_num as usize + 1).last().unwrap().to_string();
        eprintln!("Got hover: line - {}, col - {}", line_num, col);

        if line.contains(";") {
            line = line.split(";").collect::<Vec<_>>().first().unwrap().to_string();
        }

        if line.is_empty() {
            return Ok(None);
        }

        let mut output = get_hovered_token(line, line_num, col);

        if output.is_errored {
            return Ok(None);
        }

        if let Some(docs) = get_opcode_docs(&output.token) {
            output.token = docs.to_string();
        }
        if is_numeric(&output.token) {
            output.token = format!("### {}\n`Immediate`", output.token);
        }
        if output.token.starts_with("&") && is_numeric(&output.token[1..]) {
            output.token = format!("### {}\n`LongImmediate`", output.token);
        }
        if output.token.starts_with("^") && is_numeric(&output.token[1..]) {
            output.token = format!("### {}\n`LongImmediate`", output.token);
        }

        if output.token.ends_with(":") {
            output.token = output.token.strip_suffix(":").unwrap().to_string();
            output.range = update_range(output.range, 0, 0, 0, -1);
        }

        Ok(Some(Hover{
            contents: tower_lsp::lsp_types::HoverContents::Scalar(
                MarkedString::String(output.token)
            ),
            range: Some(output.range)
        }))
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    } 
}

struct HoveredToken {
    token: String,
    range: Range,
    is_errored: bool
}
const ERROR_HOVER_TOKEN: HoveredToken = HoveredToken {token: String::new(), range: Range { start: Position { line: 0, character: 0 }, end: Position { line: 0, character: 0 } }, is_errored: true};

fn get_hovered_token(line: String, line_num: u32, col: u32) -> HoveredToken {
    let text_line = line.split(' ').collect::<Vec<_>>();
    eprintln!("text_line: {:?}", text_line);

    let mut curr: u32 = 0;
    for token in text_line {
        eprintln!("token: {} ({}-{})", token, curr, curr + token.len() as u32);
        if curr <= col && col <= curr + token.len() as u32 {
            return HoveredToken {
                token: token.to_string(),
                range: Range { 
                    start: Position { line: line_num, character: curr }, 
                    end: Position { line: line_num, character: curr + token.len() as u32 }
                },
                is_errored: false
            };
        }
        curr += token.len() as u32 + 1;
    }
    
    ERROR_HOVER_TOKEN
}

fn update_range(init: Range, line_start_delta: i32, char_start_delta: i32, line_end_delta: i32, char_end_delta: i32) -> Range {
    Range { 
        start: Position {
            line: (init.start.line as i32 + line_start_delta) as u32, 
            character: (init.start.character as i32 + char_start_delta) as u32, 
        }, 
        end: Position { 
            line: (init.end.line as i32 + line_end_delta) as u32, 
            character: (init.end.character as i32 + char_end_delta) as u32, 
        }
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {client});
    Server::new(stdin, stdout, socket).serve(service).await;
}
