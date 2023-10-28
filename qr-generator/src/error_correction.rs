use reed_solomon::{Polynomial, IntMod, ReedSolomonEncoder, GaloisField};

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
        Ok(())
    }

    pub fn generate_error_correction(&mut self) {
        for ec_block in self.blocks.iter_mut() {
            ec_block.generate_error_correction();
        }
        println!("{:?}", self.blocks);
    }

    pub fn interleave(&self) -> BlockInterleaver {
        BlockInterleaver::new(self)
    }
}

impl ErrorCorrectionBlock {
    pub fn generate_error_correction(&mut self) {
        type Element = Polynomial<IntMod<2>>;
        type GF256 = GaloisField<2, 8, 285, 2>;
        let rs = ReedSolomonEncoder::<GF256>::new();

        let block_polys = self.data_codewords.iter().map(|&cw| Element::from(cw as u32)).collect::<Vec<Element>>();
        let encoded_polys: Vec<Element> = rs.encode(block_polys, self.error_correction_codeword_count);
        self.ec_codewords = encoded_polys.into_iter().skip(self.data_codeword_count).map(|cw| u32::from(cw) as u8).collect();
    }
}

enum BlockType {
    Data,
    ErrorCorrection
}
pub struct BlockInterleaver<'a> {
    corrected_blocks: &'a ErrorCorrector,
    block_type: BlockType,
    block_number: usize,
    word_number: usize
}
impl<'a> BlockInterleaver<'a> {
    pub fn new(corrected_blocks: &'a ErrorCorrector) -> Self {
        Self {
            corrected_blocks,
            block_type: BlockType::Data,
            block_number: 0,
            word_number: 0
        }
    }
}
impl<'a> Iterator for BlockInterleaver<'a> {
    type Item = u8;
    fn next(&mut self) -> Option<Self::Item> {
        match self.block_type {
            BlockType::Data => {
                if self.block_number < self.corrected_blocks.blocks.len() {
                    if self.word_number < self.corrected_blocks.blocks[self.block_number].data_codewords.len() {
                        let use_block = self.block_number;
                        self.block_number += 1;
                        Some(self.corrected_blocks.blocks[use_block].data_codewords[self.word_number])
                    }
                    else {
                        self.block_number += 1;
                        self.next()
                    }
                } else {
                    self.block_number = 0;
                    if self.word_number < self.corrected_blocks.blocks.last().unwrap().data_codewords.len() {
                        self.word_number += 1;
                    } else {
                        self.word_number = 0;
                        self.block_type = BlockType::ErrorCorrection;
                    }
                    self.next()
                }
            },
            BlockType::ErrorCorrection => {
                if self.block_number < self.corrected_blocks.blocks.len() {
                    if self.word_number < self.corrected_blocks.blocks[self.block_number].ec_codewords.len() {
                        let use_block = self.block_number;
                        self.block_number += 1;
                        Some(self.corrected_blocks.blocks[use_block].ec_codewords[self.word_number])
                    }
                    else {
                        self.block_number += 1;
                        self.next()
                    }
                } else {
                    self.block_number = 0;
                    if self.word_number < self.corrected_blocks.blocks.last().unwrap().ec_codewords.len() {
                        self.word_number += 1;
                        self.next()
                    } else {
                        None
                    }
                }
            }
        }
    }
}