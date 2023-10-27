use crate::qr_errors::EncodingError;

#[derive(PartialEq, PartialOrd, Eq, Ord, Debug)]
pub enum CorrectionLevels {
    DetectionOnly,
    L,
    M,
    Q,
    H,
}

pub(crate) struct ErrorCorrector {
    blocks: Vec<ErrorCorrectionBlock>,
}

#[derive(Clone, Debug)]
struct ErrorCorrectionBlock {
    data_codeword_count: usize,
    error_correction_codeword_count: usize,
    data_codewords: Vec<u8>,
    ec_codewords: Vec<u8>,
}

impl From<&Vec<(usize, usize, usize)>> for ErrorCorrector {
    fn from(block_descriptors: &Vec<(usize, usize, usize)>) -> Self {
        let capacity = block_descriptors
            .iter()
            .map(|descriptor| descriptor.2)
            .sum();
        let mut blocks = Vec::<ErrorCorrectionBlock>::with_capacity(capacity);

        for descriptor in block_descriptors {
            let exemplar = ErrorCorrectionBlock {
                data_codeword_count: descriptor.1,
                error_correction_codeword_count: descriptor.0 - descriptor.1,
                data_codewords: vec![],
                ec_codewords: vec![],
            };
            let mut blocks_repeated = vec![exemplar; descriptor.2];
            blocks.append(&mut blocks_repeated);
        }

        Self { blocks }
    }
}

impl ErrorCorrector {
    pub fn fill_data_into_blocks(&mut self, data_codewords: Vec<u8>) -> Result<(), EncodingError> {
        if data_codewords.len() != self.blocks.iter().map(|b| b.data_codeword_count).sum() {
            return Err(EncodingError::new("Data vs ec blocks len mismatch"));
        }

        let mut data_iter = data_codewords.into_iter();
        self.blocks.iter_mut().for_each(|block| {
            block.data_codewords = (&mut data_iter)
                .take(block.data_codeword_count)
                .collect::<Vec<u8>>();
        });
        println!("{:?}", self.blocks);
        Ok(())
    }
}
