#![feature(slice_patterns)]
use std::str::FromStr;

/// Stores roll parameters.
/// 
/// ** Parameters **
/// - Count: the number of dice to be rolled
/// - Range: the highest value on each dice
#[derive(Eq, PartialEq)]
pub struct RollCommand {
    count: u32, // unsigned, 32bit integer 
    range: u32, 
}

impl RollCommand {
    /// Constructs a new RollCommand with basic parameters.
    pub fn new(c: u32, r: u32) -> RollCommand {
        RollCommand { count: c, range: r }
    }

    /// Generates a RollResult based on a command.
    ///
    /// Each command can be used any number of times; this function will
    /// generate new results each time.
    /// Higher order function -> up to the caller to provide an appropriate
    /// function to generate random values, any function will be used
    ///
    /// # Examples
    /// ```
    /// use rcmd::RollCommand;
    /// 
    /// let cmd = RollCommand::new(2, 6);
    /// let result = cmd.result(|max| max);
    /// assert!([6, 6] == result.values());
    /// ```
    /// 
    /// Here we have a function that provides values from an iterator instead
    /// of generatring random values each time:
    ///
    /// ```
    /// use rcmd::RollCommand;
    /// 
    /// let rng_src = [1,2,3,4];
    /// let mut rng = rng_src.iter();
    /// let cmd = RollCommand::new(4, 6);
    /// let result = cmd.result(|_| *rng.next().unwrap());
    ///
    /// assert!([1,2,3,4] == result.values());
    /// ```

    pub fn result<F: FnMut(u32) -> u32>(&self, mut f: F) -> RollResult {
        RollResult((0..self.count).map(|_| f(self.range)).collect())
        
        /* alternative
        let mut vec = Vec::new();
        for _ in (0..self.count) {
            vec.push(f(self.range));
        }
        RollResult(vec)
        */
    }
}

/// Converts a string roll command to a roll command struct.
/// 
/// 2d6 => RollCommand {count: 2, range: 6}, etc
impl FromStr for RollCommand {
    type Err = String;

    fn from_str(s: &str) -> Result<RollCommand, <RollCommand as FromStr>::Err> {
        // 2d6: [2, 6]
        let split: Vec<_> = s.split('d').filter_map(|n| n.parse().ok()).collect();
        // match slice by splitting the string and return some valid result
        match split[..] {
            [ref count, ref range] => Ok(RollCommand::new(*count, *range)), 
            [ref range] => Ok(RollCommand::new(1, *range)), 
            _ => Err(format!("Invalid command: {}.", s)), 
        }
    }
}

/// RollResult is a vector of unsigned integers.
///
/// RollResult wraps a vector of unsinged integers representing the result
/// of a roll command once executed. Wrapping the vector allows us to provice
/// specialised function implementations for dealing with roll results.
pub struct RollResult(Vec<u32>);

impl RollResult {
    /// Returns an iterator over the result of a roll.
    /// 
    /// This function actually just returns an iterator on the 
    /// underlying vectory used to store the result values.
    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, u32> {
        self.0.iter()
    }

    /// Returns the total value of the roll
    ///
    /// This function folds over the internal vector of the RollResult to sum.
    pub fn total(&self) -> u32 {
        self.0.iter().fold(0, |a, b| a + b)
    }

    pub fn values(&self) -> &[u32] {
        &self.0
    }
}

impl std::fmt::Display for RollResult {
    /// Implements Display for RollResult.
    /// 
    /// # Examples
    /// [1, 2, 3] => "1, 2, 3 (6)"
    /// 
    /// ```
    /// use rcmd::RollResult;
    /// let vec: vec![1,2,3];
    /// let result = RollResult(vec);
    /// assert!("1, 2, 3 (6)" == result.to_string());
    /// ```
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let as_strings: Vec<_> = self.0.iter().map(|n| n.to_string()).collect();
        write!(f, "{} ({})", as_strings.connect(", "), self.total())
    }
}

#[cfg(test)]
mod rollcommand_test {
    use super::*; // pulls in code from this mod

    #[test]
    fn can_parse_full_rollcommands() {
        let cmd = RollCommand::new(2, 6);
        assert!(cmd == "2d6".parse().unwrap());
    }

    #[test]
    fn can_parse_short_rollcommands() {
        let cmd = RollCommand::new(1, 6);
        assert!(cmd == "6".parse().unwrap());
    }
}