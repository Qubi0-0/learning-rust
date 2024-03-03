#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return the corresponding Error enum if the conversion is impossible.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if to_base == 0 || to_base == 1{
        Err(Error::InvalidOutputBase)
    } else if from_base == 0 || from_base == 1 {
        Err(Error::InvalidInputBase)
    } else if from_base == to_base {
        return Ok(number.to_vec());
    } else if number.is_empty() {
        Ok(vec![0])
    } else {
        for num in number {
            if num >= &from_base {
                return Err(Error::InvalidDigit(*num));
            }
        }
        let input_sum = number
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, digit)| digit * from_base.pow(idx as u32))
            .sum::<u32>();
        if input_sum == 0 {
            return Ok(vec![0]);
        }
        let mut highest_divider: u32 = 1;
        let mut iterator: u32 = 0;
        while input_sum > highest_divider {
            iterator += 1;
            highest_divider = to_base.pow(iterator);
        }
        let mut result: Vec<u32> = vec![];
        let mut output_sum = input_sum.clone();
        while output_sum > 0 {
            if output_sum > 0 {
                if iterator > 0 {
                    iterator -= 1
                }
                highest_divider = to_base.pow(iterator);
                let result_chunk = output_sum / highest_divider;
                output_sum -= result_chunk * highest_divider;
                result.push(result_chunk);
            }
        }
        while iterator > 0 {
            result.push(0);
            iterator -= 1;
        }
        Ok(result)
    }
}
