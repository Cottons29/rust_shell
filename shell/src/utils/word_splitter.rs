
type Output = Result<Vec<String>, Box<dyn std::error::Error>>;

pub(crate) trait WordSplitter{
    fn split_words_by_space(&self) -> Output;
    fn split_quote(&self) -> Output;
    fn split_double_quote(&self) -> Output;
    fn split_path(&self) -> Output;

}

impl WordSplitter for String{
    fn split_words_by_space(&self) -> Output{
        let words: Vec<&str> = self.split_whitespace().collect();
        let result: Vec<String> = words.iter().map(|s| s.to_string()).collect();
        Ok(result)
    }

    fn split_quote(&self) -> Output {
        todo!()
    }

    fn split_double_quote(&self) -> Output {
        todo!()
    }

    fn split_path(&self) -> Output {
        let temp: Vec<String> = self.trim().split('/').map(|s| s.to_string()).collect();
        Ok(temp)
    }
}