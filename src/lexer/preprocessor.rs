use std::collections::HashMap;
use std::path::PathBuf;

pub struct Preprocessor {
    defines: HashMap<String, String>,
    include_paths: Vec<PathBuf>,
}

impl Preprocessor {
    pub fn new() -> Self {
        Self {
            defines: HashMap::new(),
            include_paths: vec![PathBuf::from(".")],
        }
    }
    
    pub fn add_define(&mut self, name: String, value: String) {
        self.defines.insert(name, value);
    }
    
    pub fn add_include_path(&mut self, path: PathBuf) {
        self.include_paths.push(path);
    }
    
    pub fn process(&self, source: &str) -> Result<String, String> {
        let mut output = String::new();
        let mut lines = source.lines();
        
        while let Some(line) = lines.next() {
            if line.trim_start().starts_with('#') {
                self.process_directive(line, &mut output)?;
            } else {
                output.push_str(&self.expand_macros(line));
                output.push('\n');
            }
        }
        
        Ok(output)
    }
    
    fn process_directive(&self, line: &str, output: &mut String) -> Result<(), String> {
        let directive = line.trim_start().trim_start_matches('#').trim();
        
        if directive.starts_with("define") {
            // Handle #define
            Ok(())
        } else if directive.starts_with("include") {
            // Handle #include
            Ok(())
        } else if directive.starts_with("if") {
            // Handle #if
            Ok(())
        } else if directive.starts_with("pragma") {
            // Handle #pragma
            Ok(())
        } else {
            Ok(())
        }
    }
    
    fn expand_macros(&self, line: &str) -> String {
        let mut result = line.to_string();
        
        for (name, value) in &self.defines {
            result = result.replace(name, value);
        }
        
        result
    }
}

