impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        // need to remove any white spaces from the string, and any not alphanumeric characters, and also case sensitivity
        // but this will make us create a new string in memory, while we cna just skip the increment the pointer and not the other one when it lands on that
        // make a pointer on the beginning and one at the end, and match each if one of the pairs is not matching return false
        // if not return true
        // the loop stopping condition when left > right because of even string
        // convert it into indexable string using .chars().collect()
        // is_alphanumeric()
        // to_lowercase() 
        // for the skip inner loop for chars are not to be evaluated it can be like 
        
        if s.len() <= 1 {
            return true;
        }

        let mut s1 : Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut right = s1.len() - 1;

        while left < right { 
            while !s1[left].is_alphanumeric() && left < right
                {
                        left = left + 1;
                }
            while !s1[right].is_alphanumeric() && left < right
                {
                        right = right - 1;
                }
            if s1[left].to_ascii_lowercase()  == s1[right].to_ascii_lowercase()  {
                if left != right {
                    left = left + 1;
                    right = right - 1;
                }
            }else{
                return false;
            }

        }
        true   
    }
}
