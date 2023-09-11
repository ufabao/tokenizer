use std::collections::HashMap;
use std::fs;

fn main() {
    let path = "/Users/Darrell/Desktop/tiny-shakespeare.txt";
    let mut contents = fs::read_to_string(path).expect("Can't read file").to_ascii_lowercase();
    contents = contents.replace("\n", " \n ");
    contents = contents.replace(".", " [SEP]");
    //contents = contents.replace(",", " ");



    let mut vocab = get_vocab(&contents);
    
    let word_counts = count_words(&contents);
    let mut word_splits = create_splits(&word_counts);

    let pairs = pair_scores(&word_counts, &word_splits);

    let mut key_with_max_value = z.iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(k, _v)| k);
    let max_pair = key_with_max_value.unwrap().0.clone() + &key_with_max_value.unwrap().1;
    vocab.push(max_pair);
        
    println!("Vocab is: {:?}", vocab);
    //println!("Max key is {:?}", max_pair);
}


fn get_vocab(corpus: &String) -> Vec<String>{
    let mut vocab = Vec::new();
    for char in corpus.chars(){
        if vocab.contains(&char.to_string()){
            continue;
        }
        vocab.push(char.to_string());
    }
    vocab.push("[SEP]".to_string());
    vocab
}

fn count_words(corpus: &String) -> HashMap<String, u32>{
    let mut word_counts = HashMap::new();
    for char in corpus.split(" ") {
        *word_counts.entry(char.to_owned()).or_insert(0) += 1;
    }
    word_counts
}

fn create_splits(words: &HashMap<String, u32>) -> HashMap<String, Vec<String>> {
    let mut splits = HashMap::new();
    for word in words.keys(){
        splits.insert(word.to_owned(), word.chars().map(|c| c.to_string()).collect());
    }
    splits
}

fn pair_scores<'a>(word_counts: &HashMap<String, u32>, split: &'a HashMap<String, Vec<String>>) -> HashMap<(&'a String, &'a String), f64>{
    let mut letter_freq = HashMap::new();
    let mut pair_freqs = HashMap::new();

    for (word, freq) in word_counts{
        let splits = &split[word];
        if splits.len() == 0{
            continue;
        }
        if splits.len() == 1{
            *letter_freq.entry(splits[0].to_owned()).or_insert(0) += freq;
            continue;
        }
        for i in 0..(&splits.len()-1){
            let pair = (&splits[i], &splits[i+1]);
            *letter_freq.entry(splits[i].to_owned()).or_insert(0) += freq;
            *pair_freqs.entry(pair.to_owned()).or_insert(0.0) += f64::from(*freq);
        }
        *letter_freq.entry(splits[&splits.len()-1].to_owned()).or_insert(0) += freq;

    }
    let mut scores = HashMap::new();
    for (pair, freq) in pair_freqs{
        //let z = f64::from(letter_freq[pair.0] * letter_freq[pair.1]);
        let z = 1.0;
        if z != 0.0 {
            *scores.entry(pair.to_owned()).or_insert(0.0) = freq / z;
        }
    }
    scores
}