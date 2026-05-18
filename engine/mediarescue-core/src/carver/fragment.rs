use crate::error::CarveError;

pub struct Fragment {
    pub offset: u64,
    pub data: Vec<u8>,
}

pub struct FragmentAssembler {
    fragments: Vec<Fragment>,
}

impl FragmentAssembler {
    pub fn new() -> Self {
        Self {
            fragments: Vec::new(),
        }
    }

    pub fn add_fragment(&mut self, offset: u64, data: Vec<u8>) {
        self.fragments.push(Fragment { offset, data });
    }

    pub fn assemble(&mut self) -> Result<Vec<u8>, CarveError> {
        if self.fragments.is_empty() {
            return Err(CarveError::FragmentAssembly("no fragments to assemble".to_string()));
        }

        self.fragments.sort_by_key(|f| f.offset);

        let total_size: usize = self.fragments.iter().map(|f| f.data.len()).sum();
        let mut assembled = Vec::with_capacity(total_size);

        for fragment in &self.fragments {
            assembled.extend_from_slice(&fragment.data);
        }

        Ok(assembled)
    }

    pub fn fragment_count(&self) -> usize {
        self.fragments.len()
    }
}

impl Default for FragmentAssembler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assemble_single_fragment() {
        let mut assembler = FragmentAssembler::new();
        assembler.add_fragment(0, vec![1, 2, 3, 4]);
        let result = assembler.assemble().unwrap();
        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_assemble_ordered_fragments() {
        let mut assembler = FragmentAssembler::new();
        assembler.add_fragment(100, vec![5, 6]);
        assembler.add_fragment(0, vec![1, 2]);
        assembler.add_fragment(50, vec![3, 4]);

        let result = assembler.assemble().unwrap();
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_assemble_empty() {
        let mut assembler = FragmentAssembler::new();
        let result = assembler.assemble();
        assert!(result.is_err());
    }
}
