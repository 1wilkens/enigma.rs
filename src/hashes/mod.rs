pub mod md2;

pub trait HashFunction {
    /// Set the input bytes of the HashFunction
    ///
    /// Also effectively performs a reset of the inner state,
    /// this means retrieving a previously calculated hash is no
    /// longer possible.
    ///
    /// # Arguments
    /// * `input` - The input bytes to hash
    fn set_input(&mut self, input: &[u8]);

    /// Convenience method to set a &str as input
    ///
    /// # Arguments
    /// * `input_str` - The input &str to hash
    fn set_input_str(&mut self, input_str: &str) {
        self.set_input(input_str.as_bytes());
    }

    /// Performs all necessary hashing operations
    ///
    /// After this method returns the hash calculations
    /// are done and the output can be retrieved.
    fn hash(&mut self);

    /// Retrieve the output of the applied HashFunction
    ///
    /// Can only be called after hash() returned successfully, but
    /// multiple times.
    ///
    /// # Arguments
    /// * `output` - The vector to write the output to. Must be
    ///     big enough to store at least get_output_length bytes
    fn get_output(&mut self, output: &mut [u8]);

    /// Convenience method to retrieve the output of the HashFunction
    /// as the commonly used hex-String
    ///
    /// Can also be called multiple times.
    ///
    /// # Returns
    /// * The resulting hex-String
    //FIXME: figure out how to properly document return values
    fn get_output_str(&mut self) -> String {
        String::new()
    }

    /// Returns the blocksize of the HashFunction used in bytes
    ///
    /// # Returns
    /// * The used blocksize
    fn get_blocksize(&self) -> uint;

    /// Returns the length of the result vector in bits as commonly used
    /// in hash specifications
    ///
    /// # Returns
    /// * The length if the output vector in bits
    fn get_output_length_in_bits(&self) -> uint;

    /// Convenience method to calculate the required size of a output vector in bytes
    ///
    /// # Returns
    /// * The length of the output vector in bytes
    fn get_output_length(&self) -> uint {
        (self.get_output_length_in_bits() + 7) / 8
    }
}

#[cfg(test)]
mod test {
    use hashes::HashFunction;

    pub struct HashTestCase {
        pub input:      &'static str,
        pub output:     Vec<u8>,
        pub output_str: &'static str
    }

    pub fn perform_hash_test(hash: &mut HashFunction, test: &HashTestCase) {
        hash.set_input_str(test.input);
        hash.hash();
        let mut result = Vec::from_elem(hash.get_output_length(), 0u8);
        hash.get_output(result.as_mut_slice());
        println!("result:")
        for r in result.iter() {
            print!("{:u} ", *r)
        }
        println!("")
        println!("output:")
        for o in test.output.iter() {
            print!("{:u} ", *o)
        }
        println!("")
        let result_str = hash.get_output_str();
        assert!(result == test.output)
        //assert!(result_str == test.output_str.to_string())
    }
}
