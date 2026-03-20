# RFreq
A word frequency program made in Rust

# Credits
1. [w3Schools Python](w3schools.com/python)

# How to Install

1. Clone the repository on your IDE
2. [Install Python](https://www.python.org/)
3. Run the file
~~~bash
python3 main.py
~~~

# How it Works

Concepts used
- Print
- File handling
- String replace and split
- Dictionary
- For loop
- Conditional logic
- Increment operator

1. Ask the user for paragraph input
2. Split the text by spaces to make an array of words
3. Declare a frequency dictionary
4. For each item in the array of words
    1. If the word is a common word (a, an, the, it, etc.)
        1. Do nothing
    2. Else if the item is a key in the dictionary
        1. Increment it's frequency by 1
    3. Else
        1. Declare the word key with a value of 1 
    4. For each key in the dictionary
        1. Print each word key along with it's frequency

# Instructions
Before pasting your text, run it through a punctuation remover. I tried removing punctuation and many other junk characters via Python's replace() and strip() methods, but both failed to actually remove the characters.

Then paste your text into input.txt and run the command. After that, let the program do its magic!