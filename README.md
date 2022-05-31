# Simple Regex DFA
An alternative approach to regex checker, written in rust. The problem is hard difficulty and this implementation is faster than 100% rust solutions, but slightly crap for memory usage (this is meant to be more for fun than for performance)

You can compile this simply with ``rustc regex.rs`` and run it with ``./regex``

Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:

'.' Matches any single character.​​​​
'*' Matches zero or more of the preceding element.
The matching should cover the entire input string (not partial).

 

**Example 1:**

``Input: s = "aa", p = "a"``

``Output: false``

``Explanation: "a" does not match the entire string "aa".``


**Example 2:**

``Input: s = "aa", p = "a*"``

``Output: true``

``Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".``

**Example 3:**

``Input: s = "ab", p = ".*"``

``Output: true``

``Explanation: ".*" means "zero or more (*) of any character (.)".``
