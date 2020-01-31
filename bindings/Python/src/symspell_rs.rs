use std::fs::File;
use std::io::{BufRead, BufReader};
extern crate symspell as model;
use model::string_strategy::UnicodeStringStrategy;
use symspell::{SymSpell, SymSpellBuilder, Verbosity};
use model::edit_distance::{DistanceAlgorithm};
use pyo3::prelude::*;
use pyo3::exceptions; 

#[pyclass()]
pub struct PySuggestion {
    term: String,
    distance: i32,
    count: i32,
}
#[pymethods]
impl PySuggestion {
    #[getter]
    fn term(&self)-> PyResult<&String>{
        Ok(&self.term)
    }
    #[getter]
    fn distance(&self)-> PyResult<i32>{
        Ok(self.distance)
    }
    #[getter]
    fn count(&self)-> PyResult<i32>{
        Ok(self.count)
    }
}


#[pyclass(module = "symspell_rs")]
pub struct SymspellPy {
    symspell1: SymSpell<UnicodeStringStrategy>,
}

#[pymethods]
impl SymspellPy {
    #[new]
    fn new(obj: &PyRawObject,
        max_distance:Option<i64>,
        prefix_length:Option<i64>,
        count_threshold:Option<i64>,
        algorithm:Option<&str>)->PyResult<()> { 
        // word_count:Option<i64>,
        // strategy:Option<&str>,
        
        let mut builder = SymSpellBuilder::default();
        if let Some(max_distance) = max_distance {
            builder.max_dictionary_edit_distance(max_distance);    
            }
        if let Some(prefix_length) = prefix_length {
            builder.prefix_length(prefix_length);    
            }
        if let Some(count_threshold) = count_threshold {
            builder.count_threshold(count_threshold);
            }
        if let Some(algorithm) = algorithm {
            let distance_algo = match algorithm {
                "damerau" => DistanceAlgorithm::Damerau,
                _ => return Err(exceptions::Exception::py_err("Not a valid edit distance algorithm")),
            };
            builder.distance_algorithm(distance_algo);
        }
        /*if Some(strategy) = strategy {
            let string_strategy = match strategy {
                "Unicode" => string_strategy::UnicodeStringStrategy,
                "Ascii" => string_strategy::AsciiStringStrategy,
                _ => return Err(exceptions::Exception::py_err("Not a valid String Strategy")),
            };
            builder.string_strategy(string_strategy);
        }
        
        if let Some(word_count) = word_count {
            builder.corpus_word_count(word_count);
                }*/
            Ok(obj.init({SymspellPy{symspell1: builder.build().unwrap()}})) 
        }

    

    fn load_dictionary(&mut self, file:&str, term_index:i64, count_index:i64, separator:&str) -> PyResult<bool> {

        let obj = File::open(file).expect("Not a valid file");
        
        let corpus = BufReader::new(obj);
        
        for line in corpus.lines() {
            self.symspell1.load_dictionary_line(
                &line?.to_string(),
                term_index,
                count_index,
                &separator,
            );
        }
        Ok(true)
    }
   
    // // ________________________________________________________________________________________
    
    // // ________________________________________________________________________________________    

 
    fn load_bigram_dictionary(&mut self, file:&str, term_index:i64, count_index:i64, separator:&str) -> PyResult<bool> {

        let obj = File::open(file).expect("Not a valid file");
        
        let corpus = BufReader::new(obj);//.expect("Unable to read file");
        
        for line in corpus.lines() {
            self.symspell1.load_dictionary_line(
                &line?.to_string(),
                term_index,
                count_index,
                &separator,
            );
        }
        Ok(true)
    }

    pub fn lookup_compound(
        &mut self,
        input: &str,
        max_edit_distance: i32,
    ) -> PyResult<Vec<PySuggestion>> {
        let res = self.symspell1.lookup_compound(input, max_edit_distance as i64);
        Ok(res
            .into_iter()
            .map(|sugg| {
                let temp = PySuggestion {
                    term: sugg.term,
                    distance: sugg.distance as i32,
                    count: sugg.count as i32,
                };
                temp
            })
            .collect())
        
    }

    pub fn lookup(
        &self,
        input: &str,
        verbosity: i8,
        max_edit_distance: i32,
    ) -> PyResult<Vec<PySuggestion>> {
        let sym_verbosity = match verbosity {
            0 => Verbosity::Top,
            1 => Verbosity::Closest,
            2 => Verbosity::All,
            _ => return Err(exceptions::Exception::py_err("Verbosity must be between 0 and 2")),
        };

        let res = self
            .symspell1
            .lookup(&input, sym_verbosity, max_edit_distance as i64);

        Ok(res
            .into_iter()
            .map(|sugg| {
                let temp = PySuggestion {
                    term: sugg.term,
                    distance: sugg.distance as i32,
                    count: sugg.count as i32,
                };
                temp
            })
            .collect())
    }
}


#[pymodule]
fn symspell_rs(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<SymspellPy>()?;
    Ok(())
}