use std::collections::HashMap;
fn tf(term: &str, tf: HashMap<String, usize>) -> f32 {
    let a = tf.get(term).cloned().unwrap_or(0) as f32;
    let b = tf.iter().map(|(_, f)| *f).sum::<usize>() as f32;
    a / b
}

fn idf(term: &str, tf: &HashMap<String, HashMap<String, usize>>) -> f32 {
    let n = tf.len() as f32;
    let m = tf.values().filter(|tf| tf.contains_key(term)).count() as f32;
    n / m
}

pub fn tfidf(
    term: &str,
    tf_table: HashMap<String, usize>,
    tf_index: &HashMap<String, HashMap<String, usize>>,
) -> f32 {
    tf(&term, tf_table) * idf(&term, &tf_index)
}
