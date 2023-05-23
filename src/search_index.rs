use std::collections::HashMap;
use deunicode::deunicode;

static STOP_WORDS: &[ &'static str ] = &[
    "and".to_string(),
    "or".to_string(), 
    "a".to_string(), 
    "an".to_string(), 
    "the".to_string() 
];

pub struct SearchIndex {
    index: HashMap< String, Vec<String> >,
    human_index: HashMap< String, Vec<String> >,
}

impl SearchIndex {
    pub fn new() -> Self {
        SearchIndex {
            index: HashMap::new(),
            human_index: HashMap::new(), // human-readable version of the index;
                                         // includes stop-words
        }
    }

    pub fn add_document( &mut self, document_id: String, terms: &[String] ) {
        for term in terms {
            let normalized_term = normalize( term.to_string(), &STOP_WORDS );
            self.human_index.entry( term.to_owned() ).or_insert_with( Vec::new ).push( document_id.to_owned() );
            self.index.entry( normalized_term.to_owned() ).or_insert_with( Vec::new ).push( document_id.to_owned() );
        }
    }

    pub fn search( &self, term: String ) -> Option< Vec<String> > {
        let normalized_term = normalize( term, &STOP_WORDS );
        match self.index.get( &normalized_term ) {
            Some( documents ) => Some( documents.clone() ),
            None => None,
        }
    }
}

fn normalize( s: String, stop_words: &[String] ) -> String {
    let words: Vec<&str> = s.split_whitespace().collect();
    let filtered_words: Vec<&str> = words.into_iter()
        .filter( |word| !stop_words.contains( &word.to_string() ) )
        .collect();
    
    str::to_ascii_lowercase( &deunicode( &filtered_words.join( " " ) ) )
}
