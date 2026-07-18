impl Solution {
        pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        // use HashMap to store character and its occurence and for each string to compare with try to decrement exisiting ones till it resolves to zero this means 
        // they are anagrams and create a list of them to be part of the overall list being returned
        // but while comparing I can insert in the map so in h=just in one pass can do the comparisons and know the answer
        // so we can sort each string and store it in the map so now the comparison happen in the map level not strings with each other
        // HashMap the key will be the sorted version of the word and the value will be the matching strings list for it 

        let mut map: HashMap<String, Vec<String>> = HashMap::new();

        for s in &strs {
            let mut chars: Vec<char> = s.chars().collect();
            chars.sort();
            let key: String = chars.into_iter().collect(); // converting back list of chars into a String

            map.entry(key).or_insert_with(Vec::new).push(s.clone());
        }

        map.into_values().collect() // converting the list of values of the map into the expected output type and returning it
    }
}
