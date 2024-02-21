use crate::{
    configuration::{Configuration, ValidTransformerBackend},
    worker::{
        DoCompletionResponse, DoGenerateResponse, DoGenerateStreamResponse, GenerateStreamRequest,
    },
};

pub mod llama_cpp;

pub trait TransformerBackend {
    // Should all take an enum of chat messages or just a string for completion
    fn do_completion(&self, prompt: &str) -> anyhow::Result<DoCompletionResponse>;
    fn do_generate(&self, prompt: &str) -> anyhow::Result<DoGenerateResponse>;
    fn do_generate_stream(
        &self,
        request: &GenerateStreamRequest,
    ) -> anyhow::Result<DoGenerateStreamResponse>;
}

impl TryFrom<Configuration> for Box<dyn TransformerBackend + Send> {
    type Error = anyhow::Error;

    fn try_from(configuration: Configuration) -> Result<Self, Self::Error> {
        match configuration.get_transformer_backend()? {
            ValidTransformerBackend::LlamaCPP => {
                Ok(Box::new(llama_cpp::LlamaCPP::new(configuration)?))
            }
            _ => unimplemented!(),
        }
    }
}
