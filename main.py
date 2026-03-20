text_file = open("input.txt")

paragraph = ""

for line in text_file:
    paragraph += line

text_file.close()

paragraph_array = paragraph.split()

print(paragraph_array)