![wlc_banner](https://github.com/user-attachments/assets/ddcacbde-c3a6-45ab-92c3-aaa7dd2b35de)

# SWLC  
#### Searching the Westminster Leningrad Codex

<br>

# NOTE

For now this is just a skeleton. 




# Motivation

Although there are already websites/programs that you can use to search the Westminster Leningrad Codex (the oldest complete version of the Old Testament written in the Hebrew language).  For me personally, however, I have not yet found options that provides sufficient search capabilities. 

While learning Biblical Hebrew and Rust I decided to combine both languanges. So I started to write a search program in Rust for the Westminster Leningrad Codex. The idea is that I write this missing features.


Here are some points I miss most in existing search engines:

1. **MAQAF and word finding**   
   The maqaf is a type of hyphen used to connect two words. In Hebrew, this is not a new word, but it is more of a “sound unit” made up of two separate words. I should be able to find each of those separate words.

2. **Consonants and diacritics**  
   Finding specific combinations of consonants and diacritics is not possible. You usually get a list of the requested consonants including all diacritics. This means you still have to manually search the given list for your specific diacrtics.
   
<br>

# Layout

THIS WILL MOST CERTAINLY CHANGE DURING DEVELOPMENT!

The package layout for now will be the following:

**swcl_cli**   
A crate containing the CLI version of the search program.

**swcl_gui**  
A crate containing the CLI version of the search program.

**swcl_model**  
A library crate that will contain the model.

**swcl_gen**   
A binary crate, used to generate the model (Rust code) from text files.



